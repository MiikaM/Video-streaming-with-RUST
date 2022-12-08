
use crate::components::producer::EncodedVideoChunkWrapper;
use js_sys::{JsString, Reflect, Uint8Array};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::*;
use yew::prelude::*;

static VIDEO_CODEC: &str = "vp09.00.10.08";

#[function_component(Consumer)]
pub unsafe fn consumer() -> Html {
    let video_ctx = use_context::<UseReducerHandle<EncodedVideoChunkWrapper>>().unwrap();
    let video_decoder: UseStateHandle<Option<VideoDecoder>> = use_state(|| None);
    if (*video_decoder).is_none() {
        let error_video = Closure::wrap(Box::new(move |e: JsValue| {
            console::log_1(&e);
        }) as Box<dyn FnMut(JsValue)>);

        let output = Closure::wrap(Box::new(move |original_chunk: JsValue| {
            let chunk = Box::new(original_chunk);
            let video_chunk = chunk.clone().unchecked_into::<HtmlImageElement>();
            let width = Reflect::get(&chunk.clone(), &JsString::from("codedWidth"))
                .unwrap()
                .as_f64()
                .unwrap();
            let height = Reflect::get(&chunk.clone(), &JsString::from("codedHeight"))
                .unwrap()
                .as_f64()
                .unwrap();
            let render_canvas = window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("render")
                .unwrap()
                .unchecked_into::<HtmlCanvasElement>();
            render_canvas.set_width(width as u32);
            render_canvas.set_height(height as u32);
            let ctx = render_canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .unchecked_into::<CanvasRenderingContext2d>();
            ctx.draw_image_with_html_image_element(&video_chunk, 0.0, 0.0);
            video_chunk.unchecked_into::<VideoFrame>().close();
        }) as Box<dyn FnMut(JsValue)>);

        let local_video_decoder = VideoDecoder::new(&VideoDecoderInit::new(
            error_video.as_ref().unchecked_ref(),
            output.as_ref().unchecked_ref(),
        ))
        .unwrap();
        error_video.forget();
        output.forget();
        local_video_decoder.configure(&VideoDecoderConfig::new(&VIDEO_CODEC));
        video_decoder.set(Some(local_video_decoder));
    } else if !(*video_ctx).chunk.is_none() {
        let chunk = (*video_ctx).chunk.as_ref().unwrap();
        let mut video_vector = vec![0u8; chunk.byte_length() as usize];
        let video_message = video_vector.as_mut();
        chunk.copy_to_with_u8_array(video_message);
        let decoder: VideoDecoder = (*video_decoder).to_owned().unwrap();
        let data = Uint8Array::from(video_message.as_ref());
        let encoded_video_chunk = EncodedVideoChunk::new(&EncodedVideoChunkInit::new(
            &data,
            chunk.timestamp(),
            chunk.type_(),
        ))
        .unwrap();
        decoder.decode(&encoded_video_chunk);
    }
    html!(
        <div class="consumer">
            <h3>{"Consumer"}</h3>
            <canvas id="render"></canvas>
        </div>
    )
}
