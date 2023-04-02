#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use wasm_bindgen::prelude::*;

use crate::app::App;
use crate::label_tool::LabelTool;

mod annotated_image;
mod app;
mod bbox;
mod components;
mod download;
mod editor;
mod header;
mod images_list;
mod label_tool;
mod labels;
mod upload_annotations;
mod upload_image;
mod use_canvas_image;
mod utils;

/// object class id
pub type Class = u32;
/// annotation is an object bounding box in image and class type
pub type Annotation = (bbox::BBox, Class);

#[wasm_bindgen]
/// init label tool and start app on given root html element
pub fn init_label_tool(
    root: web_sys::Element,
    label_tool: Option<LabelTool>,
    canvas_element_id: Option<String>,
) -> LabelTool {
    use console_error_panic_hook;
    console_error_panic_hook::set_once();
    let label_tool = label_tool.unwrap_or_default();
    yew::Renderer::<App>::with_root_and_props(
        root,
        app::Props {
            label_tool: label_tool.clone(),
            canvas_element_id,
        },
    )
    .render();
    label_tool
}

/// the image label tool prelude
pub mod prelude {
    pub use crate::annotated_image::AnnotatedImage;
    pub use crate::app::App;
    pub use crate::bbox::BBox;
    pub use crate::components::*;
    pub use crate::init_label_tool;
    pub use crate::label_tool::LabelTool;
    pub use crate::Annotation;
    pub use crate::Class;
}

#[cfg(test)]
mod tests {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
}
