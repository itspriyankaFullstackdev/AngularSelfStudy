use wasm_bindgen:: prelude::wasm_bindgen;
use web_sys:: console::log_1 as log;
use base64:: {encode, decode};
use image::load_from_memory;
use image::ImageOutputFormat:: Png;


#[wasm_bindgen]

pub fn grayscale(encoded_file: &str) -> String {
    log(&"grayscale called".into());
    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"image encoded".into());
    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"image loaded".into());
    img = img.grayscale();
    log(&"Grayscale effect applied".into());
    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    log(&"Image writte n".into());

    let encoded_Img = encode(&buffer); 
    let data_url = format!("data:image/png;base64,{}",encoded_Img);
    data_url
}
