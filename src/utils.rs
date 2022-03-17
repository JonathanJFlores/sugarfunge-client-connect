use web_sys::{HtmlInputElement, window};
use wasm_bindgen::JsCast;

pub fn get_value_input_by_id(id: &str) -> String {
    let document = window().and_then(|win| win.document()).expect("Could not access document");
    let input_ele = document.get_element_by_id(id).expect("no input element").dyn_into::<HtmlInputElement>().expect("input no found");
    input_ele.value()
}