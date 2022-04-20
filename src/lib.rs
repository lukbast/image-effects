use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{decode, encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use std::io::Cursor;

#[wasm_bindgen]
pub fn greyscale(encoded_file: &str) -> String{
    log(&"Grayscalle called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    img = img.grayscale();
    log(&"Grayscale effect applied".into());

    let mut buffer = vec![];
    img.write_to(&mut Cursor::new(&mut buffer), Png).unwrap();
    log(&"New image written".into());

    let encoden_img = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encoden_img);
    log(&"New image encoded".into());

    data_url
}