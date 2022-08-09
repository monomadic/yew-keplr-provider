use web3::transports::eip_1193::BaseCurrency;

pub fn atom() -> BaseCurrency {
    BaseCurrency {
        name: String::from("ATOM"),
        symbol: String::from("ATOM"),
        decimals: 18,
    }
}

pub fn osmo() -> BaseCurrency {
    BaseCurrency {
        name: String::from("OSMO"),
        symbol: String::from("OSMO"),
        decimals: 18,
    }
}
