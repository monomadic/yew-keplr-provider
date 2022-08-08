//use wasm_bindgen::JsValue;
use js_sys::JsString;
use wasm_bindgen::{prelude::*, JsValue};

use web3::transports::eip_1193::Provider;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace=window, js_name=keplr)]
    pub fn keplr_exist() -> Result<JsValue, JsString>;

    #[wasm_bindgen(catch, js_namespace=["window", "keplr"], js_name=enable)]
    pub async fn keplr_enable(s: &str) -> Result<JsValue, JsString>;

    #[wasm_bindgen(catch, js_namespace=["window", "keplr"], js_name=getKey)]
    pub async fn keplr_getKey(s: &str) -> Result<JsValue, JsString>;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &JsValue);

    #[wasm_bindgen(js_namespace = console, js_name=log)]
    fn log_string(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name=log)]
    fn log_number(s: usize);

    #[wasm_bindgen(js_namespace= window)]
    fn alert(s: &str);

    #[wasm_bindgen(catch, js_namespace= ["window", "Object"])]
    pub async fn keys(s: &JsValue) -> Result<JsValue, JsString>;

    #[wasm_bindgen(js_namespace= JSON, js_name=stringify)]
    pub fn json_stringify(s: &JsValue) -> String;
}

/*
#[derive(Serialize, Deserialize)]
pub struct KeyPromise {
    pub name: String,
    pub algo: String,
    pub pubKey: Vec<u8>,
    pub address: Vec<u8>,
    pub bech32Address: String,
    pub isNanoLedger: bool
}
*/

#[derive(Clone, Debug)]
pub struct UseKeplrHandle {
    pub provider: Provider,
    connected: UseStateHandle<bool>,
    accounts: UseStateHandle<String>,
    chain_id: UseStateHandle<Option<String>>,
}

impl PartialEq for UseKeplrHandle {
    fn eq(&self, other: &Self) -> bool {
        self.connected == other.connected
            && self.accounts == other.accounts
            && self.chain_id == other.chain_id
    }
}

impl UseKeplrHandle {
    pub async fn connect(&self) -> Result<(), String> {
        log::info!("connect()");

        if let Ok(_return_value) = keplr_enable("osmosis-1").await {
            if let Ok(return_value) = keplr_getKey("osmosis-1").await {
                // get the account address
                let res_str = json_stringify(&return_value);

                let v: Vec<_> = res_str.match_indices("bech32Address").collect();
                let v1: Vec<_> = res_str.match_indices("isNanoLedger").collect();

                log_number(v[0].0);
                log_number(v1[0].0);

                let (_first, target) = res_str.split_at(v[0].0 + 16);

                let (account_address, _last) = target.split_at(v1[0].0 - v[0].0 - 19);

                log_string(account_address);

                self.connected.set(true);
                self.accounts.set(String::from(account_address));
            }
        } else {
            alert("Please install keplr extension");
        };

        Ok(())
    }
    pub fn disconnect(&self) {
        log::info!("disconnect()");
        self.connected.set(false);
    }

    pub fn connected(&self) -> bool {
        *self.connected
    }

    pub fn display_address(&self) -> String {
        //self.accounts.as_ref();
        String::from("Ok")
    }
}

#[hook]
pub fn use_keplr(default: Option<Provider>) -> UseKeplrHandle {
    let connected = use_state(move || false);
    let accounts = use_state(move || String::from(""));
    let chain_id = use_state(move || None as Option<String>);

    UseKeplrHandle {
        provider: default.unwrap_or_else(|| Provider::default().unwrap().unwrap()),
        connected,
        accounts,
        chain_id,
    }
}
