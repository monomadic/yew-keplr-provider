use crate::base_currency;
use web3::transports::eip_1193::Chain;

// Chain id reference https://github.com/cosmos/chain-registry/blob/master/cosmoshub/chain.json

/*
    // cosmoshub
    "chain_name": "cosmoshub",
    "chain_id": "cosmoshub-4",
    "rpc_url": https://cosmos-rpc.quickapi.com:443"
    "native_currency": "ATOM"

    // osmosis
    "chain_name": "osmosis"
    "chain_id": "osmosis-1"
    "rpc-url": "https://osmosis-rpc.quickapi.com:443"
    "native_currency": "OSMO"

*/
pub fn cosmoshub() -> Chain {
    Chain {
        chain_id: "cosmoshub-4".into(),
        chain_name: "COSMOS HUB".into(),
        rpc_urls: [String::from("https://cosmos-rpc.quickapi.com:443")],
        native_currency: base_currency::atom(),
        block_explorer_urls: Some([String::from("https://api.avax-test.network/ext/bc/C/rpc")]),
    }
}

pub fn osmosis() -> Chain {
    Chain {
        chain_id: String::from("osmosis"),
        chain_name: String::from("OSMOSIS"),
        rpc_urls: [String::from("https://osmosis-rpc.quickapi.com:443")],
        native_currency: base_currency::osmo(),
        block_explorer_urls: Some([String::from("https://api.avax-test.network/ext/bc/C/rpc")]),
    }
}
