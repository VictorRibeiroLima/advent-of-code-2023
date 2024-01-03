use crate::worker::ProcessInput;
use components::form::Form;
use worker::Input;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_agent::oneshot::{use_oneshot_runner, OneshotProvider};

mod components;
pub mod worker;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <OneshotProvider<ProcessInput> path="/worker.js">
            <Main />
        </OneshotProvider<ProcessInput>>
    }
}

#[function_component(Main)]
fn main() -> Html {
    let result = use_state(|| "".to_string());

    let process_task = use_oneshot_runner::<ProcessInput>();

    let output = result.clone();

    let input_callback = Callback::from(move |input: Input| {
        let process_agent = process_task.clone();
        let output = output.clone();
        output.set("Processing...".to_string());
        spawn_local(async move {
            // start the worker
            let output_value = process_agent.run(input).await;

            output.set(output_value);
        });
    });

    html! {
            <div class="container">

                        <Form callback={input_callback} />

                        <div class="result-section">
                            <h2>{"Result: "}</h2>
                            <p id="result">{result.clone().to_string()}</p>
                        </div>

            </div>
    }
}
