use ui::app::App;

fn main() {
    tracing_wasm::set_as_global_default();
    yew::Renderer::<App>::new().render();
}
