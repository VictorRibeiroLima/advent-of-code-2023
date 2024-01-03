use std::ops::RangeInclusive;

use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: AttrValue,
    pub label: AttrValue,
    pub range: RangeInclusive<usize>,
    pub callback: Callback<String>,
}

#[function_component(Select)]
pub fn text_area(props: &Props) -> Html {
    let callback = props.callback.clone();
    let on_change = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .dyn_into::<web_sys::HtmlSelectElement>()
            .unwrap()
            .value();

        callback.emit(value);
    });
    html! {
    <select id={props.id.clone()} onchange={on_change}>
        {create_options(props)}
    </select>
    }
}

fn create_options(props: &Props) -> Html {
    let range = props.range.clone();
    let id = props.id.clone();
    let label = props.label.clone();

    range.map(|value| {
      html! {
          <option value={format!("{}{}",id.clone(),value)}>{format!("{} {}",label.clone(),value)}</option>
      }
    }).collect::<Html>()
}
