use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture};
use web_sys::{Request, RequestInit, RequestMode, Response};


pub async fn fetch_text(url: &str) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init( url, &opts, ).unwrap();
    request
        .headers()
        .set("Accept", "*")
        .unwrap();

    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);
    let resp_value = JsFuture::from(request_promise).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    JsFuture::from(resp.text()?).await
}



/*
use web_sys::{Request, RequestInit, Response};
let mut opts = RequestInit::new();
opts.method("POST");
let form_data = web_sys::FormData::new()?; // web-sys should really require mut here...
form_data.append_with_str("secret", body.secret)?;
form_data.append_with_str("response", &body.response)?;
opts.body(Some(&form_data));
let request = Request::new_with_str_and_init(
    "https://www.google.com/recaptcha/api/siteverify",
    &opts,
)?;

request.headers().set("User-Agent", "sortasecret")?;

let global = worker_global_scope().ok_or(VerifyError::NoGlobal)?;
let resp_value = JsFuture::from(global.fetch_with_request(&request)).await?;
let resp: Response = resp_value.dyn_into()?;
let json = JsFuture::from(resp.json()?).await?;
let verres: VerifyResponse = json.into_serde()?;

Ok(verres)
 */






pub async fn fetch_json(url: &str) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(url, &opts)?;
    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    JsFuture::from(resp.json()?).await
}





