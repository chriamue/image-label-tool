use yew::prelude::*;
use crate::header::Header;

pub struct App {
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <Header />
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
        let rendered = yew::LocalServerRenderer::<App>::new()
        .render().await;
        assert!(rendered.contains("<h1>Image Label Tool</h1>"));
    }
}
