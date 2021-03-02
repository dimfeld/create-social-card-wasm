mod utils;
use ab_glyph::FontRef;
use create_social_card::{overlay_text, OverlayOptions};
use js_sys::Error;
use serde::Deserialize;
use std::collections::HashMap;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Deserialize)]
pub struct Options<'a> {
    background: Box<[u8]>,
    fonts: HashMap<String, Box<[u8]>>,
    blocks: Vec<create_social_card::Block<'a>>,
}

fn wrap_error<T: std::string::ToString>(e: T) -> Error {
    let e = Error::new(&e.to_string());
    e.set_name("SocialCardError");
    e
}

#[wasm_bindgen]
pub fn create_card(options: JsValue) -> Result<Box<[u8]>, JsValue> {
    let input: Options =
        serde_wasm_bindgen::from_value(options).map_err(|e| wrap_error(e.to_string()))?;

    let input_image = image::load_from_memory(&input.background).map_err(wrap_error)?;
    let fonts = input
        .fonts
        .iter()
        .map(|(name, font)| {
            Ok(create_social_card::FontDef {
                name: std::borrow::Cow::from(name),
                font: FontRef::try_from_slice(font).map_err(wrap_error)?,
            })
        })
        .collect::<Result<Vec<_>, JsValue>>()?;

    let options = OverlayOptions {
        background: input_image,
        fonts: &fonts,
        blocks: &input.blocks,
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
