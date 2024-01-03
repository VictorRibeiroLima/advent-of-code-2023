use crate::{
    components::{select::Select, text_area::TextArea},
    worker::Input,
};

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub callback: Callback<Input>,
}

#[function_component(Form)]
pub fn text_area(props: &Props) -> Html {
    let callback = props.callback.clone();
    let part_range = 1..=2;
    let day_range = 1..=25;

    let input = use_state(|| "".to_string());
    let part = use_state(|| "part2".to_string());
    let day = use_state(|| "day25".to_string());

    let callback_input = input.clone();
    let callback_part = part.clone();
    let callback_day = day.clone();

    let callback_input = Callback::from(move |value: String| {
        callback_input.set(value);
    });

    let callback_part = Callback::from(move |value: String| {
        callback_part.set(value);
    });

    let callback_day = Callback::from(move |value: String| {
        callback_day.set(value);
    });

    let on_submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let input = input.clone().to_string();
        let part = part.clone().to_string();
        let day = day.clone().to_string();

        let input = Input { input, part, day };

        callback.emit(input);
    });

    html! {
      <div class="input-section" onsubmit={on_submit}>
        <form>
          <Select id="day" label="Day" range={day_range} callback={callback_day} />
          <Select id="part" label="Part" range={part_range} callback={callback_part} />
          <TextArea id="input" placeholder="Enter your input here" callback={callback_input} />
          <button id="submit">{"Submit"}</button>
        </form >
      </div>
    }
}
