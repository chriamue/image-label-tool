use yew::{html, Component, Context, Html};

/// A header component for the Image Label Tool.
pub struct Header;

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <header class="header">
                <h1>{"Image Label Tool"}</h1>
            </header>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[crate::test]
    async fn test_render() {
        let rendered = yew::LocalServerRenderer::<Header>::new().render().await;
        assert!(rendered.contains("<h1>Image Label Tool</h1>"));
    }
}
