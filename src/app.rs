use crate::annotated_image::AnnotatedImage;
use crate::bbox::BBox;
use crate::components::{ImagesList, StatusBar};
use crate::download::download_bytes;
use crate::editor::Editor;
use crate::header::Header;
use crate::label_tool::LabelTool;
use crate::labels::Labels;
use crate::upload_annotations::UploadAnnotations;
use crate::upload_image::UploadImage;
use crate::use_canvas_image::UseCanvasImage;
use crate::Annotation;
use image::DynamicImage;
use yew::prelude::*;

/// The web app
pub struct App {
    current: usize,
    label: String,
    labels: Vec<String>,
}

pub enum Msg {
    AnnotationsChanged((String, Vec<u8>)),
    LabelChanged(String),
    ImageChanged((String, Vec<u8>)),
    NewAnnotation(Annotation),
    AddImage(),
    ImageSelected(usize),
    DownloadAnnotations,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label_tool: LabelTool,
    pub canvas_element_id: Option<String>,
}

pub fn format_annotation(annotation: &Annotation, labels: &Vec<String>) -> String {
    let (bbox, class) = annotation;
    format!(
        "{} {} {} {} {}",
        labels[*class as usize], bbox.x as i32, bbox.y as i32, bbox.w as i32, bbox.h as i32
    )
}

pub fn format_annotations(annotations: &Vec<Annotation>, labels: &Vec<String>) -> Vec<String> {
    annotations
        .iter()
        .map(|annotation| format_annotation(annotation, labels))
        .collect::<Vec<String>>()
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
            Msg::AnnotationsChanged((_, data)) => {
                ctx.props()
                    .label_tool
                    .annotated_images()
                    .lock()
                    .unwrap()
                    .images
                    .borrow_mut()
                    .get_mut(self.current)
                    .unwrap()
                    .clear();
                let data = std::str::from_utf8(&data).unwrap().to_string();
                for line in data.split('\n').collect::<Vec<&str>>() {
                    let mut l = line.split(' ');
                    let label = l.next().unwrap();
                    let class = match self.labels.iter().position(|x| x == label) {
                        Some(class) => class as u32,
                        None => 0,
                    };
                    let x: u32 = l.next().unwrap().parse().unwrap();
                    let y: u32 = l.next().unwrap().parse().unwrap();
                    let w: u32 = match l.next() {
                        Some(w) => w.parse().unwrap(),
                        None => 32,
                    };
                    let h: u32 = match l.next() {
                        Some(h) => h.parse().unwrap(),
                        None => 32,
                    };
                    let annotation = (
                        BBox {
                            x: x as f32,
                            y: y as f32,
                            w: w as f32,
                            h: h as f32,
                        },
                        class,
                    );
                    ctx.props()
                        .label_tool
                        .add_annotation(self.current, annotation);
                }
                true
            }
            Msg::LabelChanged(label) => {
                self.label = label;
                true
            }
            Msg::ImageChanged((filename, data)) => {
                let img = image::load_from_memory(&data).unwrap();
                println!("loaded {}", filename);
                {
                    ctx.props()
                        .label_tool
                        .annotated_images()
                        .lock()
                        .unwrap()
                        .images
                        .borrow_mut()
                        .get_mut(self.current)
                        .unwrap()
                        .set_image(img);

                    ctx.props()
                        .label_tool
                        .annotated_images()
                        .lock()
                        .unwrap()
                        .images
                        .borrow_mut()
                        .get_mut(self.current)
                        .unwrap()
                        .clear();
                };
                true
            }
            Msg::NewAnnotation(annotation) => {
                ctx.props()
                    .label_tool
                    .add_annotation(self.current, annotation);
                true
            }
            Msg::AddImage() => {
                let img = image::DynamicImage::new_rgb8(1, 1);
                let annotations = AnnotatedImage::new();
                annotations.set_image(img);
                ctx.props().label_tool.push(annotations);
                true
            }
            Msg::ImageSelected(index) => {
                self.current = index;
                true
            }
            Msg::DownloadAnnotations => {
                let annotated_images = ctx
                    .props()
                    .label_tool
                    .annotated_images()
                    .lock()
                    .unwrap()
                    .clone();
                let annotations = annotated_images
                    .images
                    .borrow()
                    .get(self.current)
                    .unwrap()
                    .get_annotations();
                let file_data = format_annotations(&annotations, &self.labels).join("\n");
                download_bytes(file_data.as_bytes(), &format!("{}.txt", "annotations"));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_image_change = ctx
            .link()
            .callback(|(filename, data): (String, Vec<u8>)| Msg::ImageChanged((filename, data)));
        let on_new_annotation = ctx.link().callback(Msg::NewAnnotation);
        let on_annotations_change = ctx.link().callback(|(filename, data): (String, Vec<u8>)| {
            Msg::AnnotationsChanged((filename, data))
        });
        let on_label_change = ctx.link().callback(Msg::LabelChanged);
        let on_add_image = ctx.link().callback(|()| Msg::AddImage());
        let on_image_selected = ctx.link().callback(Msg::ImageSelected);
        let on_download_annotations = ctx.link().callback(|_| Msg::DownloadAnnotations);

        let annotated_images = ctx
            .props()
            .label_tool
            .annotated_images()
            .lock()
            .unwrap()
            .clone();
        let annotations = annotated_images
            .images
            .borrow()
            .get(self.current)
            .unwrap()
            .get_annotations();
        let image = match ctx.props().label_tool.get_annotated_image(self.current) {
            Some(annotated_image) => annotated_image.get_image(),
            None => DynamicImage::ImageRgb8(image::ImageBuffer::new(100, 100)),
        };

        let fetch_image = match &ctx.props().canvas_element_id {
            Some(element_id) => {
                html!(<UseCanvasImage element_id={element_id.to_string()} onchange={on_image_change.clone()} />)
            }
            None => html!(),
        };
        let image_count = annotated_images.len();

        html! {
            <>
            <Header />
            <div id="data-sources">
                {fetch_image}
                <UploadImage onchange={on_image_change}/>
                <UploadAnnotations onchange={on_annotations_change}/>
            </div>
            <Labels onchange={on_label_change} label={self.label.clone()} labels={self.labels.clone()} />
            <ImagesList image_store={annotated_images} onaddimage={on_add_image} current={self.current} onimageselected={on_image_selected}/>
            <Editor label={self.label.clone()} labels={self.labels.clone()} {image} {annotations} onchange={on_new_annotation}/>
            <button type="button" class="btn btn-success" onclick={on_download_annotations}>
                { "Download Annotations" }
            </button>
            <StatusBar {image_count} />
            </>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[crate::test]
    async fn test_render() {
        let label_tool = LabelTool::default();
        let rendered = yew::LocalServerRenderer::<App>::with_props(Props {
            label_tool,
            canvas_element_id: None,
        })
        .render()
        .await;
        assert!(rendered.contains("<h1>Image Label Tool</h1>"));
        assert!(rendered.contains("Upload an image file"));
        assert!(rendered.contains("none"));
        assert!(rendered.contains("object1"));
        assert!(rendered.contains("Add Image"));
        assert!(rendered.contains("Download Annotations"));
    }

    #[crate::test]
    async fn test_render_with_canvas_element_id() {
        let label_tool = LabelTool::default();
        let rendered = yew::LocalServerRenderer::<App>::with_props(Props {
            label_tool,
            canvas_element_id: Some("canvas".to_string()),
        })
        .render()
        .await;
        assert!(rendered.contains("Get Image"));
    }
}
