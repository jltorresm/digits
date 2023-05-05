mod app;
mod input_form;
mod results;
mod cache;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    dioxus_web::launch(app::app);
}
