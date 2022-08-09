// console.* module

wit_bindgen_rust::import!("./console.wit");

pub struct Console;

impl Console {
    pub fn log(msg: &str) {
        console::console_log(msg);
    }

    pub fn error(msg: &str) {
        console::console_error(msg);
    }
}