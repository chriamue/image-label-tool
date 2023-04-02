use yew::prelude::*;

/// A status bar component that displays the number of images in the image list panel.
pub struct StatusBar;

/// The properties for the `StatusBar` component.
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    /// The number of images in the image list panel.
    pub image_count: usize,
}

impl Component for StatusBar {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="status-bar">
                <p>{ format!("Number of images: {}", ctx.props().image_count) }</p>
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::LocalServerRenderer;

    #[crate::test]
    async fn test_render() {
        let image_count = 5;
        let rendered = LocalServerRenderer::<StatusBar>::with_props(Props { image_count })
            .render()
            .await;

        assert!(rendered.contains("Number of images: 5"));
    }
}
