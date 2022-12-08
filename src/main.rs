mod components;

use components::{
    consumer::Consumer, producer::Producer, videoChunksProvider::VideoChunksProvider,
};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! (
            <VideoChunksProvider>
        <div class={"grid"}>
            <Producer/>
            <Consumer/>
        </div>
    </VideoChunksProvider>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
