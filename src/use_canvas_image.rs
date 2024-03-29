use image::{DynamicImage, ImageOutputFormat, RgbaImage};
use std::io::Cursor;
use wasm_bindgen::JsCast;
use web_sys::ImageData;
use yew::{html, Callback, Component, Context, Html, Properties};

fn convert_image_data_to_png_data(image: &ImageData) -> Vec<u8> {
    let tmp = RgbaImage::from_raw(image.width(), image.height(), image.data().to_vec()).unwrap();
    let dyn_image = DynamicImage::ImageRgba8(tmp);
    let mut image_png: Vec<u8> = Vec::new();
    dyn_image
        .write_to(&mut Cursor::new(&mut image_png), ImageOutputFormat::Png)
        .unwrap();
    image_png
}

pub enum Msg {
    GetCanvasImage,
}

pub struct UseCanvasImage {}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub element_id: String,
    pub onchange: Callback<(String, Vec<u8>)>,
}

impl Component for UseCanvasImage {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetCanvasImage => {
                let document = web_sys::window().unwrap().document().unwrap();
                let canvas = document.get_element_by_id(&ctx.props().element_id).unwrap();
                let canvas: web_sys::HtmlCanvasElement = canvas
                    .dyn_into::<web_sys::HtmlCanvasElement>()
                    .map_err(|_| ())
                    .unwrap();
                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .unwrap();
                let img = context
                    .get_image_data(
                        0.0_f64,
                        0.0_f64,
                        canvas.width() as f64,
                        canvas.height() as f64,
                    )
                    .unwrap();
                let image_data = convert_image_data_to_png_data(&img);

                ctx.props()
                    .onchange
                    .emit(("canvas".to_string(), image_data));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::GetCanvasImage);

        html! {
            <div id="use-canvas-image">
                <button type="button" class="btn btn-success" {onclick}>
                    { "Get Image" }
                </button>
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[crate::test]
    fn test_convert_image_data_to_png_data() {
        use web_sys::ImageData;
        let image_data = ImageData::new_with_sw(100, 100).unwrap();
        let image_png = convert_image_data_to_png_data(&image_data);
        let img = image::load_from_memory(&image_png);
        assert!(img.is_ok());
        let img = img.unwrap();
        assert_eq!(100, img.width());
        assert_eq!(100, img.height());
    }
}
