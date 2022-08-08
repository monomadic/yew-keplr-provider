use crate::hooks::{use_keplr, UseKeplrHandle};
use yew::{function_component, html, Children, ContextProvider, Html, Properties};

#[derive(Clone, PartialEq)]
pub struct KeplrProviderState {
    pub keplr: UseKeplrHandle,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn KeplrContextProvider(props: &Props) -> Html {
    let keplr = use_keplr(None);

    html! {
        <ContextProvider<UseKeplrHandle> context={keplr}>
            {for props.children.iter()}
        </ContextProvider<UseKeplrHandle>>
    }
}
