use stylist::{style, yew::styled_component, Style};
use yew::prelude::*;

mod components;

use components::atoms::main_title::{Color, MainHeader};

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <MainHeader color={Color::Ok} title="Hi There from lib"/>
    </main>
    }
}
//
