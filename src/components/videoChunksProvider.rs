use yew::prelude::*;
use crate::components::producer::EncodedVideoChunkWrapper;

#[derive(Properties, Debug, PartialEq)]
pub struct VideoChunksProviderProps {
    #[prop_or_default]
    pub children: Children
}

#[function_component(VideoChunksProvider)]
pub fn videoChunksProvider(props: &VideoChunksProviderProps) -> Html {
    let msg = use_reducer(|| EncodedVideoChunkWrapper {
        chunk: None
    });
    html! {
        <div></div>
    }
}