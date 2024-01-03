use front_end::worker::ProcessInput;
use yew_agent::Registrable;

fn main() {
    ProcessInput::registrar().register();
}
