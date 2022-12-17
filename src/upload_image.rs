use js_sys::{ArrayBuffer, Uint8Array};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{html, Callback, Component, Context, Html, NodeRef, Properties};

pub struct UploadImage {
    input_ref: NodeRef,
}

pub enum Msg {
    OnClick(),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onchange: Callback<(String, Vec<u8>)>,
}

impl Component for UploadImage {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnClick() => {
                let input_element = self
                    .input_ref
                    .get()
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap();
                let onchange = ctx.props().onchange.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if let Some(files) = input_element.files() {
                        if let Some(file) = files.item(0) {
                            let file_array_buffer_promise = file.array_buffer();
                            let file_array_buffer: ArrayBuffer =
                                wasm_bindgen_futures::JsFuture::from(file_array_buffer_promise)
                                    .await
                                    .expect("Should be able to get array buffer from uploaded file")
                                    .dyn_into()
                                    .unwrap();
                            let file_array_buffer = Uint8Array::new(file_array_buffer.as_ref());
                            let file_bytes = file_array_buffer.to_vec();
                            onchange.emit((file.name(), file_bytes));
                        }
                    }
                });
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let handle_change = ctx.link().callback(|_| Msg::OnClick());

        html! {
            <div id="upload-image" class="form-group">
                <label for="upload-image">{"Upload an image file"}</label>
                <input id="upload-image" class="btn btn-success"
                    aria-description="Upload an image file"
                    type="file"
                    accept="image/*"
                    onchange={handle_change}
                    ref={self.input_ref.clone()}
                    disabled={false}
                />
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn test_render() {
        let rendered = yew::LocalServerRenderer::<UploadImage>::with_props(Props {
            onchange: Callback::default(),
        })
        .render()
        .await;
        assert!(rendered.contains("Upload an image file"));
    }
}
