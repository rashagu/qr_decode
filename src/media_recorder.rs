// 你可以使用 `wasm-bindgen` 库将 Rust 代码编译成 WebAssembly 模块，然后在浏览器中运行
// 以下是一个简单的示例，使用 `js-sys` 库调用浏览器的 `navigator.mediaDevices.getUserMedia` 方法来访问摄像头
// 你可以使用 `wasm-pack` 工具来构建和打包你的 Rust 项目

use js_sys::Uint8Array;
// mod media_recorder;
//
// use js_sys::Math::log;
// use js_sys::Promise;
use wasm_bindgen::prelude::*;
use web_sys::*;
use qrcode::QrCode;

#[wasm_bindgen]
pub async fn start_camera() -> MediaStream {
    let window = window().unwrap();
    let navigator = window.navigator();
    let media_devices = navigator.media_devices().unwrap();

    let mut constraints = MediaStreamConstraints::new();
    constraints.video(&JsValue::from("{ audio: true, video: { width: 1280, height: 720 } }"));
    let promise = media_devices.get_user_media_with_constraints(&constraints).unwrap();
    let result = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();

    let stream = MediaStream::from(result);

    let video = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("video")
        .unwrap()
        .dyn_into::<HtmlVideoElement>()
        .unwrap();

    let body = web_sys::window()
        .unwrap()
        .document()
        .unwrap().query_selector("body").unwrap().unwrap();
    body.append_child(&video).expect("TODO: panic message");

    let c = MediaStream::from(stream);
    video.set_attribute("id","test_video").expect("TODO: panic message");
    video.set_attribute("style","width: 800px;height:600px;").expect("TODO: panic message");
    video.set_src_object(Option::from(&c));
    let _ = video.play().expect("TODO: panic message");

    // let media_recorder = MediaRecorder::new(&stream)?;
    let media_recorder = MediaRecorder::new_with_media_stream(&c).unwrap();
    // 定义处理录制数据的事件处理程序
    let ondataavailable = Closure::wrap(Box::new(move |event: web_sys::BlobEvent| {
        let blob = event.data().unwrap();
        // 处理录制的数据
        // let promise = wasm_bindgen_futures::JsFuture::from(blob.array_buffer()).await.unwrap();
        // let array_buffer = Uint8Array::new(&promise);
        //
        // let result = QrCode::new(array_buffer.as_ref()).unwrap();
        //
        // console::log_1(&JsValue::from(result.width()).into());
        // ...
    }) as Box<dyn FnMut(_)>);



    // media_recorder.add_event_listener_with_callback("ondataavailable", ondataavailable.as_ref().unchecked_ref())
    //     .expect("TODO: panic message");
    // let op = media_recorder.ondataavailable();
    media_recorder.set_ondataavailable(Some(ondataavailable.as_ref().unchecked_ref()));
    // media_recorder.start().expect("TODO: panic message");
    media_recorder.start_with_time_slice(1000).expect("TODO: panic message");
    ondataavailable.forget();

    console::log_1(&Option::from(&c).into());

    return c



    // let get_stream2 = Closure::once_into_js(|stream: JsValue| {
    //     // 在这里处理获取到的 MediaStream 数据
    //     // 例如将其赋值给视频元素的 srcObject 属性
    //     let video = web_sys::window()
    //         .unwrap()
    //         .document()
    //         .unwrap()
    //         .create_element("video")
    //         .unwrap()
    //         .dyn_into::<HtmlVideoElement>()
    //         .unwrap();
    //     let c = MediaStream::from(stream);
    //     log(c.get_video_tracks().length() as f64);
    //     video.set_src_object(Option::from(&c));
    //     video.oncanplay();
    //     // return JsValue::from(video);
    // });
    //
    //
    // let a = promise.then(&get_stream2);
    // return a;
    // return promise.map_or_else(|js_value| Promise::from(js_value), |promise| promise);
}
