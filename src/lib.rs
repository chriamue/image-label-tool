#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use wasm_bindgen::prelude::*;

use crate::app::App;
use crate::label_tool::LabelTool;

mod annotated_image;
mod app;
mod bbox;
mod editor;
mod header;
mod images_list;
mod label_tool;
mod labels;
mod upload_image;
mod utils;

/// object class id
pub type Class = u32;
/// annotation is an object bounding box in image and class type
pub type Annotation = (bbox::BBox, Class);

#[wasm_bindgen]
/// init label tool and start app on given root html element
pub fn init_label_tool(root: web_sys::Element) -> LabelTool {
    use console_error_panic_hook;
    console_error_panic_hook::set_once();
    let label_tool = LabelTool::default();
    yew::Renderer::<App>::with_root_and_props(
        root,
        app::Props {
            label_tool: label_tool.clone(),
        },
    )
    .render();
    label_tool
}

#[cfg(test)]
mod tests {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
}
