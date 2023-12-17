use crate::*;
use alloy_chains::Chain;
use foundry_block_explorers::verify::{CodeFormat, VerifyContract};
use foundry_compilers::{Project, ProjectPathsConfig};
use serial_test::serial;
use std::path::Path;

#[tokio::test]
#[serial]
#[ignore]
async fn can_flatten_and_verify_contract() {
    let root = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../tests/testdata/uniswap"));
    let paths = ProjectPathsConfig::builder()
        .sources(root)
        .build()
        .expect("failed to resolve project paths");
    let project = Project::builder().paths(paths).build().expect("failed to build the project");

    let address = "0x9e744c9115b74834c0f33f4097f40c02a9ac5c33".parse().unwrap();
    let compiler_version = "v0.5.17+commit.d19bba13";
    let constructor_args = "0x000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000005f5e1000000000000000000000000000000000000000000000000000000000000000007596179537761700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000035941590000000000000000000000000000000000000000000000000000000000";
    let contract =
        project.flatten(&root.join("UniswapExchange.sol")).expect("failed to flatten contract");
    let contract_name = "UniswapExchange".to_owned();
    let contract =
        VerifyContract::new(address, contract_name, contract, compiler_version.to_string())
            .constructor_arguments(Some(constructor_args))
            .optimization(true)
            .runs(200);

    run_with_client(Chain::mainnet(), |client| async move {
        let resp = client
            .submit_contract_verification(&contract)
            .await
            .expect("failed to send the request");
        assert_eq!(resp.result,  "Contract source code already verified", "{resp:?}");
        assert_eq!(resp.message, "NOTOK", "{resp:?}");
    })
    .await
}


#[tokio::test]
#[serial]
async fn can_verify_single_file_contract_on_etherscan() {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/test-data/WETH.sol"));
    let source = std::fs::read_to_string(path).expect("failed to read source code");
    let verification_request = VerifyContract {
            address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse().unwrap(),
            code_format: CodeFormat::SingleFile,
            contract_name: "WETH9".to_owned(),
            compiler_version:  "v0.4.19+commit.c4cbbb05".to_owned(),
            runs: Some("200".to_owned()),
            optimization_used: Some("0".to_owned()),
            constructor_arguments: Some("".to_owned()),
            blockscout_constructor_arguments: Some("".to_owned()),
            evm_version: Some("Default".to_owned()),
            source,
            other: std::collections::HashMap::new(),
        };


    // Get environment variable BLOCKSCOUT_API_KEY
    let api_key = std::env::var("ETHERSCAN_API_KEY").expect("ETHERSCAN_API_KEY not set");
    let client = Client::builder().with_url("https://etherscan.io").unwrap().with_api_url("https://api.etherscan.io/api").unwrap().with_api_key(api_key).build().unwrap();
    let resp = client
        .submit_contract_verification(&verification_request)
        .await
        .expect("failed to send the request");
    assert_eq!(resp.result,  "Contract source code already verified", "{resp:?}");
    assert_eq!(resp.message, "NOTOK", "{resp:?}");
}
#[tokio::test]
#[serial]
async fn can_verify_single_file_contract_on_blockscout() {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/test-data/WETH.sol"));
    let source = std::fs::read_to_string(path).expect("failed to read source code");
    let verification_request = VerifyContract {
            address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse().unwrap(),
            code_format: CodeFormat::SingleFile,
            contract_name: "WETH9".to_owned(),
            compiler_version:  "v0.4.19+commit.c4cbbb05".to_owned(),
            runs: Some("200".to_owned()),
            optimization_used: Some("0".to_owned()),
            constructor_arguments: Some("".to_owned()),
            blockscout_constructor_arguments: Some("".to_owned()),
            evm_version: Some("Default".to_owned()),
            source,
            other: std::collections::HashMap::new(),
        };


    // Get environment variable BLOCKSCOUT_API_KEY
    let api_key = std::env::var("BLOCKSCOUT_API_KEY").expect("BLOCKSCOUT_API_KEY not set");
    let client = Client::builder().with_url("https://eth.blockscout.com").unwrap().with_api_url("https://eth.blockscout.com/api").unwrap().with_api_key(api_key).build().unwrap();
    let resp = client
        .submit_contract_verification(&verification_request)
        .await
        .expect("failed to send the request");
    // `Error!` result means that request was malformatted
    assert_ne!(resp.result, "Error!", "{resp:?}");
    assert_ne!(resp.message, "NOTOK", "{resp:?}");
}
