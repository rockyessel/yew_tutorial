use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    name: String,
    btn_type: String,
}

#[function_component(CustomButton)]
pub fn custom_button(props: &Props) -> Html {
    html! {
        <button type={props.btn_type} >{&props.name}</button>
    }
}
