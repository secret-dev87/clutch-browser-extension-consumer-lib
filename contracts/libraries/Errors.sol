// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.17;

library Errors {
    error ADDRESS_ALREADY_EXISTS();
    error ADDRESS_NOT_EXISTS();
    error CALLER_MUST_BE_ENTRYPOINT();
    error CALLER_MUST_BE_SELF_OR_MODULE();
    error CALLER_MUST_BE_MODULE();
    error HASH_ALREADY_APPROVED();
    error HASH_ALREADY_REJECTED();
    error INVALID_ADDRESS();
    error INVALID_GUARD_HOOK_DATA();
    error INVALID_SELECTOR();
    error INVALID_SIGNTYPE();
    error MODULE_ADDRESS_EMPTY();
    error MODULE_NOT_SUPPORT_INTERFACE();
    error MODULE_SELECTOR_UNAUTHORIZED();
    error MODULE_SELECTORS_EMPTY();
    error MODULE_EXECUTE_FROM_MODULE_RECURSIVE();
    error NO_OWNER();
    error PLUGIN_ADDRESS_EMPTY();
    error PLUGIN_HOOK_TYPE_ERROR();
    error PLUGIN_INIT_FAILED();
    error PLUGIN_NOT_SUPPORT_INTERFACE();
    error PLUGIN_POST_HOOK_FAILED();
    error PLUGIN_PRE_HOOK_FAILED();
    error PLUGIN_NOT_REGISTERED();
    error SELECTOR_ALREADY_EXISTS();
    error SELECTOR_NOT_EXISTS();
    error UNSUPPORTED_SIGNTYPE();
    error INVALID_LOGIC_ADDRESS();
    error SAME_LOGIC_ADDRESS();
    error UPGRADE_FAILED();
    error NOT_IMPLEMENTED();
    error INVALID_SIGNATURE();
    error ALERADY_INITIALIZED();
    error INVALID_KEY();
    error NOT_INITIALIZED();
    error INVALID_TIME_RANGE();
    error UNAUTHORIZED();
    error INVALID_DATA();
    error GUARDIAN_SIGNATURE_INVALID();
}
