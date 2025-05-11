use std::str::from_utf8;
use speedy2d::font::Font;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys, wasm_bindgen::JsCast, Request, RequestInit, Response};

pub struct Assets {
    pub font: Option<Font>,
    pub site_url: String,
}
impl Assets {
    pub fn new() -> Assets {
        let win = web_sys::window().unwrap();
        let url = win.document().unwrap().url().unwrap();
        Assets {
            font: None,
            site_url: url,
        }
    }
    pub async fn load(&mut self) {
        let resp = self.load_var("/static/assets/fonts/arial.ttf").await;
        match resp {
            Some(x) => {
                self.font = match Font::new(&x) {
                    Ok(x) => Some(x),
                    Err(e) => {
                        log::info!("error: {}", e);
                        None
                    }
                };
            }
            None => {
                log::info!("server didn't respond")
            }
        }
    }
    pub async fn ws_url(&self) -> String {
        let url = self.load_var(&(self.site_url.clone() + "/ws")).await;
        match url {
            Some(x) => from_utf8(&x).unwrap().trim().to_string(),
            None => panic!("server_url not found"),
        }
    }
    async fn load_var(&self, url: &str) -> Option<Vec<u8>> {
        // Create a new RequestInit object
        let opts = RequestInit::new();
        opts.set_method("GET");

        // Create a new Request object
        let request = Request::new_with_str_and_init(url, &opts).unwrap();

        // Use the fetch API to make the request
        let window = web_sys::window().unwrap();
        let response_promise = window.fetch_with_request(&request);

        // Await the response using JsFuture
        let response_value = JsFuture::from(response_promise).await.unwrap();
        let response: Response = response_value.dyn_into().unwrap();

        // Check if the response is OK
        if response.ok() {
            // Get the response body as a Uint8Array
            let promise = response.array_buffer();
            let array_buffer = JsFuture::from(promise.unwrap()).await.unwrap();
            let bytes = js_sys::Uint8Array::new(&array_buffer);
            let mut vec = vec![0; bytes.length() as usize];
            bytes.copy_to(&mut vec[..]);
            Some(vec)
        } else {
            None
        }
    }
}