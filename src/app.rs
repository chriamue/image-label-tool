use crate::editor::Editor;
use crate::header::Header;
use crate::upload_image::UploadImage;
use crate::Annotation;
use image::DynamicImage;
use yew::prelude::*;

pub struct App {
    label: String,
    labels: Vec<String>,
}

pub enum Msg {
    ImageChanged((String, Vec<u8>)),
    NewAnnotation(Annotation),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            label: "none".to_string(),
            labels: include_str!("../res/labels.txt")
                .split('\n')
                .map(|s| s.to_string())
                .collect(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ImageChanged((_filename, _data)) => true,
            Msg::NewAnnotation(_annotation) => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_image_change = ctx
            .link()
            .callback(|(filename, data): (String, Vec<u8>)| Msg::ImageChanged((filename, data)));
        let on_new_annotation = ctx.link().callback(Msg::NewAnnotation);
        let image = DynamicImage::ImageRgb8(image::ImageBuffer::new(100, 100));
        let annotations = Vec::new();
        html! {
            <>
            <Header />
            <UploadImage onchange={on_image_change}/>
            <Editor label={self.label.clone()} labels={self.labels.clone()} {image} {annotations} onchange={on_new_annotation}/>
            </>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn test_render() {
        let rendered = yew::LocalServerRenderer::<App>::new().render().await;
        assert!(rendered.contains("<h1>Image Label Tool</h1>"));
        assert!(rendered.contains("Upload an image file"));
    }
}
