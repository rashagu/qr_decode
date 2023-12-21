// 你可以使用 `wasm-bindgen` 库将 Rust 代码编译成 WebAssembly 模块，然后在浏览器中运行
// 你可以使用 `wasm-pack` 工具来构建和打包你的 Rust 项目

// mod media_recorder;

use js_sys::{Uint8Array};
use image;
use wasm_bindgen::prelude::*;
use web_sys::*;
use quircs;

#[wasm_bindgen]
pub async fn start_camera(a: JsValue) -> String {
    let array_buffer = Uint8Array::new(&a);
    let img = image::load_from_memory(&*array_buffer.to_vec()).unwrap();


    // convert to gray scale
    let img_gray = img.into_luma8();

    // create a decoder
    let mut decoder = quircs::Quirc::default();

    // identify all qr codes
    let codes = decoder.identify(img_gray.width() as usize, img_gray.height() as usize, &img_gray);

    let mut str:Vec<String> = Vec::new();
    for code in codes {
        let code = code.expect("failed to extract qr code");
        let decoded = code.decode().expect("failed to decode qr code");
        str.push(std::str::from_utf8(&decoded.payload).unwrap().parse().unwrap());
        // println!("qrcode: {}", std::str::from_utf8(&decoded.payload).unwrap());
    }
    str.join(",")
}

