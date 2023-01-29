use stylist::{yew::styled_component,style};
use yew::prelude::*;



#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub color: Color
}

#[derive(PartialEq)]
pub enum Color {
    Normal,
    Ok,
    Error,
 }

 impl Color {
    pub fn to_string(&self) -> String {
        match self {
            Color::Normal => "normal".to_owned(),
            Color::Ok => "pk".to_owned(),
            Color::Error => "error".to_owned(),
        }
    }
 }

#[styled_component(MainHeader)]
pub fn main_header(props: &Props) -> Html {

    let style_sheet = style!(r#"
    .normal {
        color: white;
    }

    .ok{
        color:green;
    }

    .error{
        color:red;
    }
    
    "#).unwrap();

    html! {
        <div class={style_sheet}>
            <h1 class={props.color.to_string()}>{&props.title}</h1>
        </div>
        
    }
}
