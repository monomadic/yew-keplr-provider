use yew::prelude::*;
use yew_keplr_provider::{
    chain, AccountLabel, ConnectButton, KeplrContextProvider, SwitchNetworkButton,
};



#[function_component]
pub fn App() -> Html {
    // this is optional

    let disconnected = html! {
        <button>{"Disconnect"}</button>
    };

    
    html! {
        <div>
            <KeplrContextProvider>
                <ConnectButton {disconnected}>
                    <button>{"Connect"}</button>
                </ConnectButton>
                <AccountLabel />
            </KeplrContextProvider>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

