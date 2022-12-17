use crate::header::Header;
use crate::upload_image::UploadImage;
use yew::prelude::*;

pub struct App {}

pub enum Msg {
    ImageChanged((String, Vec<u8>)),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ImageChanged((_filename, _data)) => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_image_change = ctx
            .link()
            .callback(|(filename, data): (String, Vec<u8>)| Msg::ImageChanged((filename, data)));
        html! {
            <>
            <Header />
            <UploadImage onchange={on_image_change}/>
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
