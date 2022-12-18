use crate::editor::Editor;
use crate::header::Header;
use crate::label_tool::LabelTool;
use crate::upload_image::UploadImage;
use crate::Annotation;
use image::DynamicImage;
use yew::prelude::*;

pub struct App {
    current: usize,
    label: String,
    labels: Vec<String>,
}

pub enum Msg {
    ImageChanged((String, Vec<u8>)),
    NewAnnotation(Annotation),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label_tool: LabelTool,
}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            current: 0,
            label: "none".to_string(),
            labels: include_str!("../res/labels.txt")
                .split('\n')
                .map(|s| s.to_string())
                .collect(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ImageChanged((filename, data)) => {
                let img = image::load_from_memory(&data).unwrap();
                println!("loaded {}", filename);
                {
                    let images = ctx.props().label_tool.annotated_images();
                    let mut annotated_images = images.lock().unwrap();
                    annotated_images
                        .get_mut(self.current)
                        .unwrap()
                        .set_image(img);
                    annotated_images.get_mut(self.current).unwrap().clear()
                };
                true
            }
            Msg::NewAnnotation(_annotation) => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_image_change = ctx
            .link()
            .callback(|(filename, data): (String, Vec<u8>)| Msg::ImageChanged((filename, data)));
        let on_new_annotation = ctx.link().callback(Msg::NewAnnotation);
        let image = match ctx.props().label_tool.get_annotated_image(self.current) {
            Some(annotated_image) => annotated_image.get_image(),
            None => DynamicImage::ImageRgb8(image::ImageBuffer::new(100, 100)),
        };
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
        let label_tool = LabelTool::new();
        let rendered = yew::LocalServerRenderer::<App>::with_props(Props { label_tool })
            .render()
            .await;
        assert!(rendered.contains("<h1>Image Label Tool</h1>"));
        assert!(rendered.contains("Upload an image file"));
    }
}
