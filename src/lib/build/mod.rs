use std::{fs, str::FromStr};

use ethers::prelude::Abigen;
use ethers_solc::{
    artifacts::{output_selection::OutputSelection, Libraries, Optimizer, Settings, Severity},
    output, Artifact, ConfigurableArtifacts, Project, ProjectCompileOutput, ProjectPathsConfig,
    Solc, SolcConfig,
};
use eyre::Context;

use ethers::{abi, types::Bytes};
use eyre::{eyre, ContextCompat};
use std::path::PathBuf;
use tokio::sync::{Mutex, OnceCell};

/// Generate the rust bindings and ABI and bytecode for the smart contracts
///
/// # Returns
///
/// * `eyre::Result<()>`
pub async fn smart_contract_code_generation() -> eyre::Result<()> {
    // Compile smart contracts
    let project = get_compiled_contracts().await.lock().await.clone();

    // Generate the rust bindings for the smart contracts needed
    let bindings_for = vec![
        "SoulWallet",
        "SoulWalletFactory",
        // "EntryPoint",
        // "TokenPaymaster",
        // "GuardianMultiSigWallet",
        // "ClutchWalletProxy",
    ];
    generate_bindings(bindings_for.clone(), project.clone()).await?;

    // Generate ABI and bytecode for the smart contracts needed
    generate_abi_bytecode(bindings_for, project).await?;

    Ok(())
}

/// Write the ABI and bytecode to file for required contracts
///
/// # Arguments
///
/// * `bindings_for` - The contracts to generate the ABI and bytecode for
/// * `project` - The compiled contracts
///
/// # Returns
///
/// * `eyre::Result<()>`
async fn generate_abi_bytecode(
    bindings_for: Vec<&str>,
    project: ProjectCompileOutput,
) -> eyre::Result<()> {
    for contract in bindings_for {
        let (abi, bytecode) = get_abi_and_bytecode(String::from(contract), project.clone()).await?;
        let json_abi = serde_json::to_string(&abi).unwrap();
        fs::write(
            format!(
                "src/lib/generated/abi/contract_{}.json",
                contract.to_lowercase()
            ),
            json_abi,
        )?;
        // wrtie bytecode to file
        fs::write(
            format!(
                "src/lib/generated/abi/contract_{}.bytecode",
                contract.to_lowercase()
            ),
            bytecode.to_string(),
        )?;
    }
    Ok(())
}

/// Generate the rust bindings for a contract
///
/// # Arguments
///
/// * `bindings_for` - The contracts to generate bindings for
/// * `project` - The project to compile
///
/// # Returns
///
/// * `eyre::Result<()>`
async fn generate_bindings(
    bindings_for: Vec<&str>,
    project: ProjectCompileOutput,
) -> eyre::Result<()> {
    fs::create_dir_all("src/lib/generated/abi")?;
    fs::create_dir_all("src/lib/generated/bindings")?;
    fs::write("src/lib/generated/mod.rs", "pub mod bindings;")?;

    for contract in &bindings_for {
        compile_contract(contract, project.clone()).await?;
    }

    println!("Writing mod.rs file for generated bindings");
    let mut modules = bindings_for
        .iter()
        .map(|contract| format!("pub mod contract_{};", contract.to_lowercase()))
        .collect::<Vec<String>>();

    modules.sort();

    fs::write("src/lib/generated/bindings/mod.rs", modules.join("\n"))
        .context("Failed to write src/lib/generated/bindings/mod.rs file")?;

    Ok(())
}

/// Compile the contract
///
/// # Arguments
///
/// * `contract_name` - The name of the contract to compile
/// * `project` - The project to compile
///
/// # Returns
///
/// * `eyre::Result<()>`
async fn compile_contract(contract_name: &str, project: ProjectCompileOutput) -> eyre::Result<()> {
    println!("Compiling {} Contract", contract_name);
    let factory = project
        .find_first(contract_name)
        .context(format!("Failed to find contract: {:?}", contract_name))?
        .clone();

    let abi = factory.abi.context("context")?;

    let json_abi = serde_json::to_string(&abi)?;

    let bindings = Abigen::new(contract_name, json_abi)?.generate()?;

    bindings.write_to_file(format!(
        "src/lib/generated/bindings/contract_{}.rs",
        contract_name.to_lowercase()
    ))?;

    Ok(())
}

/// singleton of compiled contracts
///
/// This is a singleton of the compiled contracts.
pub async fn get_compiled_contracts() -> &'static Mutex<ProjectCompileOutput<ConfigurableArtifacts>>
{
    static COMPILED_CONTRACTS: OnceCell<Mutex<ProjectCompileOutput<ConfigurableArtifacts>>> =
        OnceCell::const_new();
    COMPILED_CONTRACTS
        .get_or_init(|| async {
            let proj = compile("contracts/", "node_modules/", "lib/")
                .expect("Failed to compile contracts");
            Mutex::new(proj)
        })
        .await
}

/// Compile the solidity contracts
///
/// # Arguments
///
/// * `root` - The root directory of the project
/// * `node_modules` - The node modules directory of the project
///
/// # Returns
///
/// * `eyre::Result<ProjectCompileOutput<ConfigurableArtifacts>>` - The result of the compilation
fn compile(
    root: &str,
    node_modules: &str,
    additional_libs: &str,
) -> eyre::Result<ProjectCompileOutput<ConfigurableArtifacts>> {
    // Create path from string and check if the path exists
    let root = PathBuf::from(root);
    let node_modules_path = PathBuf::from(node_modules);
    let additional_libs_path = PathBuf::from(additional_libs);
    if !root.exists() {
        return Err(eyre!("Project root {root:?} does not exist!"));
    }
    if !node_modules_path.exists() {
        return Err(eyre!("Node modules path {node_modules:?} does not exist!"));
    }

    // Configure `root` as our project root
    let paths = ProjectPathsConfig::builder()
        .root(&root)
        .sources(&root)
        .libs([node_modules_path, additional_libs_path])
        .build()?;

    let solc_config = SolcConfig::builder().settings(Settings {
        stop_after: None,
        remappings: vec![],
        optimizer: Optimizer {
            enabled: Some(true),
            runs: Some(200),
            details: None,
        },
        model_checker: None,
        metadata: None,
        output_selection: OutputSelection::default_output_selection(),
        evm_version: Some(ethers_solc::EvmVersion::Paris),
        via_ir: Some(true),
        debug: None,
        libraries: Libraries::default(),
    });

    // Create a solc ProjectBuilder instance for compilation

    let project = Project::builder()
        .paths(paths)
        .solc_config(solc_config.build())
        .set_auto_detect(true) // auto detect solc version from solidity source code
        .no_artifacts()
        .build()?;

    // Compile project
    let output = project.compile()?;

    // Check for compilation errors
    if output.has_compiler_errors() {
        Err(eyre!(
            "Compiling solidity project failed: {:?}",
            output.output().errors
        ))
    } else {
        Ok(output)
    }
}

/// Get the ABI and bytecode for a contract in the compiled contracts
///
/// # Arguments
///
/// * `contract_name` - The name of the contract to get the ABI and bytecode for
/// * `project` - The compiled contracts
///
/// # Returns
///
/// * `eyre::Result<(abi::Abi, Bytes)>` - The ABI and bytecode for the contract
async fn get_abi_and_bytecode(
    contract_name: String,
    project: ProjectCompileOutput,
) -> eyre::Result<(abi::Abi, Bytes)> {
    let factory = project
        .find_first(contract_name.clone())
        .context(format!("Failed to find contract: {:?}", contract_name))?
        .clone();

    let (abi, bytecode, _) = factory.into_parts();

    let abi: abi::Abi = abi.context(format!(
        "Failed to get ABI for contract: {:?}",
        contract_name
    ))?;

    let bytecode = bytecode.context(format!(
        "Failed to get bytecode for contract: {:?}",
        contract_name
    ))?;

    Ok((abi, bytecode))
}

/// Get the ABI and bytecode for the supplied contract from the file system
///
/// # Arguments
///
/// * `contract_name` - The name of the contract to get the ABI and bytecode for
///
/// # Returns
///
/// * `eyre::Result<(abi::Abi, Bytes)>` - The ABI and bytecode for the contract
pub async fn abi_and_bytecode_for_contract(contract_name: &str) -> eyre::Result<(abi::Abi, Bytes)> {
    let contract_path = format!(
        "src/lib/generated/abi/contract_{}.json",
        contract_name.to_lowercase()
    );
    let bytecode_path = format!(
        "src/lib/generated/abi/contract_{}.bytecode",
        contract_name.to_lowercase()
    );

    let abi_json = fs::read_to_string(contract_path)?;
    let bytecode = fs::read_to_string(bytecode_path)?;

    let abi = serde_json::from_str::<ethers::abi::Contract>(&abi_json)?;
    let bytecode = Bytes::from_str(bytecode.as_str())?;

    Ok((abi, bytecode))
}
