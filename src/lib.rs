#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use wasm_bindgen::prelude::*;

mod annotated_image;
mod app;
mod bbox;
mod editor;
mod header;
mod label_tool;
mod upload_image;
mod utils;

/// object class id
pub type Class = u32;
/// annotation is an object bounding box in image and class type
pub type Annotation = (bbox::BBox, Class);

#[wasm_bindgen]
/// init label tool and start app on given root html element
pub fn init_label_tool(root: web_sys::Element) {
    use console_error_panic_hook;
    console_error_panic_hook::set_once();
    yew::Renderer::<app::App>::with_root(root).render();
}

#[cfg(test)]
mod tests {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
}
