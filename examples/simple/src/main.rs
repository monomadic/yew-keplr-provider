use yew::prelude::*;
use yew_keplr_provider::{
    chain, AccountLabel, ConnectButton, KeplrContextProvider, SwitchNetworkButton
};



#[function_component]
pub fn App() -> Html {
    // this is optional

    let disconnected = html! {
        <button>{"Disconnect"}</button>
    };

    
    html! {
        <div class="p-5">
            <KeplrContextProvider>
                <ConnectButton {disconnected}>
                    <button class="btn btn-success mb-2">{"Connect"}</button>
                </ConnectButton>
                <AccountLabel />
                <SwitchNetworkButton chain={chain::cosmoshub()}/>
                <SwitchNetworkButton chain={chain::osmosis()}/>
            </KeplrContextProvider>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

