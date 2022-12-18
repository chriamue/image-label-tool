use crate::bbox::BBox;
use crate::utils::image_to_base64_image;
use crate::{Annotation, Class};
use image::DynamicImage;
use yew::{
    events::{DragEvent, MouseEvent},
    html, Callback, Component, Context, Html, Properties,
};

pub enum Msg {
    Dropped(String),
    MouseDown(i32, i32),
    MouseUp(i32, i32),
    Nothing,
}

pub struct Editor {
    start_pos: (i32, i32),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub labels: Vec<String>,
    pub image: DynamicImage,
    pub annotations: Vec<Annotation>,
    pub onchange: Callback<Annotation>,
}

pub fn format_annotation(annotation: &Annotation, labels: &Vec<String>) -> String {
    let (bbox, class) = annotation;
    format!(
        "{} {} {} {} {}",
        labels[*class as usize], bbox.x as i32, bbox.y as i32, bbox.w as i32, bbox.h as i32
    )
}

impl Editor {
    fn create_annotation_description(
        &self,
        img: &DynamicImage,
        annotation: &Annotation,
        labels: &Vec<String>,
    ) -> Html {
        let img = img.crop_imm(
            annotation.0.x as u32,
            annotation.0.y as u32,
            annotation.0.w as u32,
            annotation.0.h as u32,
        );
        html!(
            <div>
            <img src={image_to_base64_image(&img)} width={24} height={24} />
            {format_annotation(annotation, labels)}
            </div>
        )
    }
}

impl Component for Editor {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { start_pos: (0, 0) }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Dropped(_data) => true,
            Msg::MouseDown(x1, y1) => {
                self.start_pos = (x1, y1);
                true
            }
            Msg::MouseUp(x2, y2) => {
                let (x1, y1) = self.start_pos;
                let x = x1.min(x2);
                let y = y1.min(y2);
                let w = ((x2 - x1).abs()).max(1);
                let h = ((y2 - y1).abs()).max(1);
                let class = ctx
                    .props()
                    .labels
                    .iter()
                    .position(|x| x == &ctx.props().label)
                    .unwrap() as Class;
                ctx.props().onchange.emit((
                    BBox {
                        x: x as f32,
                        y: y as f32,
                        w: w as f32,
                        h: h as f32,
                    },
                    class,
                ));
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ondrop = ctx
            .link()
            .callback(|_: DragEvent| Msg::Dropped("dropped".to_string()));
        let onmousedown = ctx
            .link()
            .callback(|e: MouseEvent| Msg::MouseDown(e.offset_x(), e.offset_y()));
        let onmousemove = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::Nothing
        });
        let onmouseup = ctx
            .link()
            .callback(|e: MouseEvent| Msg::MouseUp(e.offset_x(), e.offset_y()));
        let img_url = image_to_base64_image(&ctx.props().image);

        html! {
            <div id="editor" class="flex w-screen bg-gray-100" { ondrop }>
            <img src={img_url} {onmousedown} {onmousemove} {onmouseup} />
            <p>
            { ctx.props().annotations.iter().map(|annotation| {
                    self.create_annotation_description(&ctx.props().image, annotation, &ctx.props().labels)
            }).collect::<Html>() }
            </p>
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[test]
    fn test_format_annotations() {
        let det1 = (
            BBox {
                x: 0.5,
                y: 0.5,
                w: 1.0,
                h: 1.0,
            },
            0,
        );
        let det2 = (
            BBox {
                x: 0.6,
                y: 0.6,
                w: 1.0,
                h: 1.0,
            },
            0,
        );
        let det3 = (
            BBox {
                x: 1.5,
                y: 1.5,
                w: 1.0,
                h: 1.0,
            },
            1,
        );
        let labels = vec!["other".to_string(), "one".to_string()];
        let annotations: Vec<Annotation> = vec![det1, det2, det3];
        let formatted = format_annotation(&annotations[0], &labels);
        assert_eq!("other 0 0 1 1".to_string(), formatted);
    }

    #[wasm_bindgen_test]
    async fn test_render() {
        let det1 = (
            BBox {
                x: 0.5,
                y: 0.5,
                w: 1.0,
                h: 1.0,
            },
            0,
        );
        let det2 = (
            BBox {
                x: 0.6,
                y: 0.6,
                w: 1.0,
                h: 1.0,
            },
            1,
        );
        let label = "other".to_string();
        let labels = vec!["other".to_string(), "one".to_string()];
        let annotations: Vec<Annotation> = vec![det1, det2];
        let image = DynamicImage::ImageRgb8(image::ImageBuffer::new(100, 100));

        let rendered = yew::LocalServerRenderer::<Editor>::with_props(Props {
            label,
            labels,
            image,
            annotations,
            onchange: Callback::default(),
        })
        .render()
        .await;
        assert!(rendered.contains("other 0 0 1 1"));
        assert!(rendered.contains("one 0 0 1 1"));
    }
}
