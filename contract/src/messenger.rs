#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    CLType, CLValue, ContractPackageHash,
};

mod dao;

#[no_mangle]
pub extern "C" fn get_message() {
    runtime::ret(CLValue::from_t("v1".to_string()).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, access_token) = storage::create_contract_package_at_hash();
    deploy_messenger(contract_package_hash);
    dao::deploy_dao(contract_package_hash, access_token);
}

fn deploy_messenger(contract_package_hash: ContractPackageHash) {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "get_message",
        vec![],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let (_stored_contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, NamedKeys::new());
    runtime::put_key(
        "messenger_package_hash",
        casper_types::Key::URef(storage::new_uref(contract_package_hash)),
    );
}
