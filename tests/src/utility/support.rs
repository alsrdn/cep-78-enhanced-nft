use casper_engine_test_support::{
    InMemoryWasmTestBuilder, WasmTestBuilder, DEFAULT_RUN_GENESIS_REQUEST,
};
use casper_execution_engine::{
    core::{engine_state::Error as EngineStateError, execution},
    storage::global_state::in_memory::InMemoryGlobalState,
};
use casper_types::{bytesrepr::FromBytes, ApiError, CLTyped, Key, PublicKey, SecretKey};

use super::installer_request_builder::InstallerRequestBuilder;

pub(crate) fn get_dictionary_value_from_key<T: CLTyped + FromBytes>(
    builder: &WasmTestBuilder<InMemoryGlobalState>,
    nft_contract_key: &Key,
    dictionary_name: &str,
    dictionary_key: &str,
) -> T {
    let seed_uref = *builder
        .query(None, *nft_contract_key, &[])
        .expect("must have nft contract")
        .as_contract()
        .expect("must convert contract")
        .named_keys()
        .get(dictionary_name)
        .expect("must have key")
        .as_uref()
        .expect("must convert to seed uref");

    builder
        .query_dictionary_item(None, seed_uref, dictionary_key)
        .expect("should have dictionary value")
        .as_cl_value()
        .expect("T should be CLValue")
        .to_owned()
        .into_t()
        .unwrap()
}

pub(crate) fn create_dummy_key_pair(account_string: [u8; 32]) -> (SecretKey, PublicKey) {
    let secrete_key =
        SecretKey::ed25519_from_bytes(account_string).expect("failed to create secret key");
    let public_key = PublicKey::from(&secrete_key);
    (secrete_key, public_key)
}

pub(crate) fn assert_expected_invalid_installer_request(
    install_request_builder: InstallerRequestBuilder,
    expected_error_code: u16,
) {
    let mut builder = InMemoryWasmTestBuilder::default();

    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();
    builder
        .exec(install_request_builder.build())
        .expect_failure(); // Should test against expected error

    let error = builder.get_error().expect("should have an error");
    assert_expected_error(error, expected_error_code);
}

pub(crate) fn assert_expected_error(actual_error: EngineStateError, error_code: u16) {
    let actual = format!("{:?}", actual_error);
    let expected = format!(
        "{:?}",
        EngineStateError::Exec(execution::Error::Revert(ApiError::User(error_code)))
    );

    assert_eq!(actual, expected, "Error should match {}", error_code)
}

pub(crate) fn query_stored_value<T: CLTyped + FromBytes>(
    builder: &mut InMemoryWasmTestBuilder,
    nft_contract_key: Key,
    path: Vec<String>,
) -> T {
    builder
        .query(None, nft_contract_key, &path)
        .expect("must have stored value")
        .as_cl_value()
        .cloned()
        .expect("must have cl value")
        .into_t::<T>()
        .expect("must get value")
}