use clutch_wallet_lib::build::smart_contract_code_generation;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    smart_contract_code_generation().await?;

    Ok(())
}
