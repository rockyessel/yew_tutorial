use gloo::console::log;
use stylist::yew::styled_component;
use yew::prelude::*;

mod components;

use components::atoms::main_title::{Color, MainHeader};
use components::molecules::custom_form::CustomForm;

#[styled_component(App)]
pub fn app() -> Html {
    log!("Loaded!!!");
    html! {
        <main>
            <MainHeader color={Color::Error} title="Hi There from lib"/>
            <MainHeader color={Color::Ok} title="Hi There from lib"/>
            <MainHeader color={Color::Normal} title="Hi There from lib"/>
            <CustomForm />
        </main>
    }
}
