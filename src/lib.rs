use wasm_bindgen::prelude::*;

mod app;
mod header;
mod upload_image;

#[wasm_bindgen]
pub fn init_label_tool(root: web_sys::Element) {
    yew::Renderer::<app::App>::with_root(root).render();
}

#[cfg(test)]
mod tests {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
}
