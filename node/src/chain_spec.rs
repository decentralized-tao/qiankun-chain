use runtime::{BalancesConfig, SudoConfig, WASM_BINARY};
use sc_service::{ChainType, Properties};
use serde_json::{json, Value};
use sp_keyring::AccountKeyring;

pub type ChainSpec = sc_service::GenericChainSpec<()>;

fn props() -> Properties {
    let mut properties = Properties::new();
    properties.insert("tokenDecimals".to_string(), 0.into());
    properties.insert("tokenSymbol".to_string(), "QKUN".into());
    properties
}

pub fn developement_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.expect("Development wasm not available"),
        Default::default(),
    )
    .with_name("Development")
    .with_id("dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_patch(testnet_genesis())
    .with_properties(props())
    .build())
}

fn testnet_genesis() -> Value {
    use bounded_collections::Get;
    use runtime::interface::{Balance, MinimumBalance};

    let endowment = <MinimumBalance as Get<Balance>>::get().max(1) * 1000;
    let balances = AccountKeyring::iter()
        .map(|a| (a.to_account_id(), endowment))
        .collect::<Vec<_>>();

    json!({
        "balances": BalancesConfig { balances},
        "sudo": SudoConfig { key: Some(AccountKeyring::Alice.to_account_id())},
    })
}
