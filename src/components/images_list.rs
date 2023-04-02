use crate::utils::image_to_base64_image;
use crate::{annotated_image::AnnotatedImage, stores::ImageStore};
use yew::{html, Callback, Component, Context, Html, Properties};

/// A component representing a list of images.
pub struct ImagesList;

/// Messages for updating the `ImagesList` component.
pub enum Msg {
    /// Add a new image to the list.
    AddImage,
}

/// The properties for the `ImagesList` component.
#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// The `ImageStore` containing the list of images.
    pub image_store: ImageStore,
    /// The index of the currently selected image in the list.
    pub current: usize,
    /// A callback function to handle the addition of a new image.
    pub onaddimage: Callback<()>,
    /// A callback function to handle image selection in the list.
    pub onimageselected: Callback<usize>,
}

impl ImagesList {
    fn create_image_element(
        &self,
        index: usize,
        annotated_image: &AnnotatedImage,
        current: usize,
        onclick: Callback<usize>,
    ) -> Html {
        let border = if index == current {
            "border border-primary"
        } else {
            ""
        };
        html! {<li class="list-group-item">
        <div key={format!("annotations-{}",index)} class={border} onclick={ move |_|{ onclick.emit(index); }}>
        <img src={image_to_base64_image(&annotated_image.get_image())} width={32} height={32} />
        { format!(" annotations: {}",annotated_image.len()) }
        </div></li>}
    }
}

impl Component for ImagesList {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddImage => {
                ctx.props().onaddimage.emit(());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_add_image = ctx.link().callback(|_| Msg::AddImage);
        let current = ctx.props().current;
        html! {
            <div id="images-list" class="card" style="width: 18rem;">
            <ul class="list-group list-group-flush">
            <li class="list-group-item">
                <button type="button" class="btn btn-success" onclick={on_add_image}>
                    { "Add Image" }
                </button>
            </li>
            {
                ctx.props().image_store.images.borrow().iter().enumerate().map(|(i, annotations)| {
                    self.create_image_element(i, annotations, current, ctx.props().onimageselected.clone())
                }).collect::<Html>()
            }
            </ul>
          </div>
        }
    }
}
