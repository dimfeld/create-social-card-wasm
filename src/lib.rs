mod utils;
use ab_glyph::FontRef;
use create_social_card::{overlay_text, OverlayOptions, Rect, Shadow};
use js_sys::Error;
use serde::Deserialize;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Deserialize)]
pub struct Options {
    text: String,
    text_rect: Rect,
    min_size: f32,
    max_size: f32,
    color: String,
    shadow: Option<Shadow>,
}

fn wrap_error<T: std::string::ToString>(e: T) -> Error {
    let e = Error::new(&e.to_string());
    e.set_name("SocialCardError");
    e
}

#[wasm_bindgen]
pub fn create_card(
    background: &[u8],
    font: &[u8],
    options: &JsValue,
) -> Result<Box<[u8]>, JsValue> {
    let input: Options = options
        .into_serde()
        .map_err(|e| wrap_error(format!("{}: {:?}", e.to_string(), options)))?;

    let input_image = image::load_from_memory(background).map_err(wrap_error)?;
    let font = FontRef::try_from_slice(font).map_err(wrap_error)?;

    let options = OverlayOptions {
        text: &input.text,
        background: input_image,
        font: &font,
        text_rect: &input.text_rect,
        min_size: input.min_size,
        max_size: input.max_size,
        color: &input.color,
        shadow: input.shadow.as_ref(),
    };

    let result_image = overlay_text(&options).map_err(wrap_error)?;

    let mut output_buf = Vec::new();
    let encoder = image::png::PngEncoder::new(&mut output_buf);
    encoder
        .encode(
            result_image.as_raw(),
            result_image.width(),
            result_image.height(),
            image::ColorType::Rgba8,
        )
        .map_err(wrap_error)?;
    Ok(output_buf.into_boxed_slice())
}
