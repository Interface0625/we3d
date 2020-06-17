use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use super::fetch::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
lazy_static! {
    static ref TOTAL_COUNT: Mutex< usize > = Mutex::new(  0  );
    static ref COUNT: Mutex< usize > = Mutex::new(  0  );
    static ref CONTENT: Mutex< Vec<String> > = Mutex::new(  Vec::new() );
    static ref CONTENT_KEYS: Mutex< Vec<String> > = Mutex::new( Vec::new() );
}
async fn load_file_list(url: &str) -> Vec<String> {
    let json = fetch_json(url).await;
    let file_list: Vec<String> = json.unwrap().into_serde().unwrap();
    file_list
}
const PUBLIC_ROOT: &str = "public"; 
const INDEX_JSON_FILENAME: &str = "index.json";
pub async fn load_all() ->Result<JsValue, JsValue> {
    let file_list = load_file_list(&format!("{}/{}", PUBLIC_ROOT, INDEX_JSON_FILENAME)).await;
    *TOTAL_COUNT.lock().unwrap() = file_list.len(); 
    let futs = file_list.into_iter()
        .map(
            |filename| fetch_and_store_resource( format!("{}/{}", PUBLIC_ROOT, filename) )
        );
    futures::future::join_all(futs).await;
    Ok(JsValue::from_str("ok"))
}

async fn fetch_and_store_resource(url: String) -> Result<JsValue, JsValue> {
    push_resource(url.clone(), fetch_text(&url).await?.as_string().unwrap());
    Ok(JsValue::from_str("ok"))
}

fn push_resource(url: String, buffer: String ){
    let mut data = CONTENT.lock().unwrap();
    let mut count = COUNT.lock().unwrap();
    let mut data_keys = CONTENT_KEYS.lock().unwrap();
    data_keys.push(url);
    data.push(buffer);
    *count = *count + 1;
}
pub fn get_count() -> usize {
    let lock = COUNT.lock().unwrap();
    lock.clone()
}
pub fn get_reources() -> Vec<String> { 
    let lock = CONTENT.lock().unwrap();
    lock.clone()
}
pub fn get_reource_keys() -> Vec<String> { 
    let lock = CONTENT_KEYS.lock().unwrap();
    lock.clone()
}
pub fn get_progress() -> f32 {
    let total = *TOTAL_COUNT.lock().unwrap() as f32;
    if total == 0. { 0. }
    else {
        let count = *COUNT.lock().unwrap() as f32;
        if count == 0. { 0. }
        else{
            total / count
        }
    }
     
} 


