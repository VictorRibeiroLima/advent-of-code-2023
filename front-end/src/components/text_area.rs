use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: AttrValue,
    pub placeholder: AttrValue,
    pub callback: Callback<String>,
}

#[function_component(TextArea)]
pub fn text_area(props: &Props) -> Html {
    let callback = props.callback.clone();
    let on_change = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .dyn_into::<web_sys::HtmlTextAreaElement>()
            .unwrap()
            .value();
        callback.emit(value);
    });
    html! {
      <textarea id={props.id.clone()} placeholder={props.placeholder.clone()} onchange={on_change}></textarea>
    }
}
