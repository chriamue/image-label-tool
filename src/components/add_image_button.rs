use yew::{html, Callback, Component, Context, Html, Properties};

/// A component representing an "Add Image" button.
pub struct AddImageButton;

/// Messages for updating the `AddImageButton` component.
pub enum Msg {
    /// Triggered when the "Add Image" button is clicked.
    AddImage,
}

/// The properties for the `AddImageButton` component.
#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// A callback function to handle the addition of a new image when the button is clicked.
    pub onaddimage: Callback<()>,
}

impl Component for AddImageButton {
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
        html! {
            <button type="button" class="btn btn-success add-image-btn" onclick={on_add_image}>
                { "Add Image" }
            </button>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, rc::Rc};

    #[crate::test]
    async fn test_render_and_click() {
        let on_add_image_called = Rc::new(RefCell::new(false));
        let on_add_image_called_clone = on_add_image_called.clone();
        let on_add_image = Callback::from(move |_| {
            *on_add_image_called_clone.borrow_mut() = true;
        });

        let renderer = yew::LocalServerRenderer::<AddImageButton>::with_props(Props {
            onaddimage: on_add_image,
        });
        let rendered = renderer.render().await;

        assert!(rendered.contains("Add Image"));
    }
}
