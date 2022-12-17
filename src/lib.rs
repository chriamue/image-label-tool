use wasm_bindgen::prelude::*;

mod app;
mod bbox;
mod editor;
mod header;
mod upload_image;
mod utils;

/// object class id
pub type Class = u32;
/// annotation is an object bounding box in image and class type
pub type Annotation = (bbox::BBox, Class);

#[wasm_bindgen]
pub fn init_label_tool(root: web_sys::Element) {
    yew::Renderer::<app::App>::with_root(root).render();
}

#[cfg(test)]
mod tests {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
}
