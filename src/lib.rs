use stylist::{style, yew::styled_component, Style};
use yew::prelude::*;

mod components;

use components::atoms::main_title::{Color, MainHeader};
use components::molecules::custom_form::CustomForm;

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <MainHeader color={Color::Error} title="Hi There from lib"/>
            <CustomForm />
        </main>
    }
}
