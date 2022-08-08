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
pub fn kelpr() -> Chain {
    Chain {
        chain_id: "0x1".into(),
        chain_name: "Ethereum".into(),
        rpc_urls: [String::from("https://api.avax-test.network/ext/bc/C/rpc")],
        native_currency: base_currency::eth(),
        block_explorer_urls: Some([String::from("https://api.avax-test.network/ext/bc/C/rpc")]),
    }
}

pub fn avalanche_testnet() -> Chain {
    Chain {
        chain_name: String::from("Avalanche Fuji Testnet"),
        chain_id: String::from("0xA869"),
        rpc_urls: [String::from("https://api.avax-test.network/ext/bc/C/rpc")],
        native_currency: base_currency::avax(),
        block_explorer_urls: Some([String::from("https://api.avax-test.network/ext/bc/C/rpc")]),
    }
}
