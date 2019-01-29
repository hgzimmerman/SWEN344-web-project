use futures::{future, Future};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_sys::console::log;
use js_sys::Array;


enum Method<T> {
    Get,
    Delete,
    Put(T),
    Post(T)
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Login {
    oauth_token: String
}


#[wasm_bindgen]
impl Login {
    #[wasm_bindgen(constructor)]
    pub fn new(oauth_token: String) -> Login {
        Login {
            oauth_token
        }
    }

    #[wasm_bindgen]
    pub fn fetch(self) -> Promise {
        get_string_from_request_promise(request_promise("http://127.0.0.1:8080/api/auth/login", Method::Post(self), None))
    }
}

#[wasm_bindgen]
pub fn fetch_login(login: Login) -> Promise {
    generic_fetch::<String, Login>("http://127.0.0.1:8080/api/auth/login", Method::Post(login), None)
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    uuid: Uuid,
    client_id: String
}


#[wasm_bindgen]
impl User {
    #[wasm_bindgen]
    pub fn fetch(auth: String) -> Promise {
        generic_fetch::<User, ()>("http://127.0.0.1:8080/api/auth/user", Method::Get, Some(auth))
    }
}



/// This is probably inefficient as hell, but it allows me to define requests in Rust and export them to JS via WASM
fn generic_fetch<T, U>(url: &str, method: Method<U>, auth: Option<String>) -> Promise
where
    T: for <'de> Deserialize<'de> + Serialize + Default,
    U: Serialize
{

    let request_promise = request_promise(url, method, auth);

    let future = JsFuture::from(request_promise)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into().unwrap();
            resp.json()
        })
        .and_then(|json_value: Promise| {
            // Convert this other `Promise` into a rust `Future`.
            JsFuture::from(json_value)
        })
        .and_then(|json| {
            // Use serde to parse the JSON into a struct.
            let t: T = json
                .into_serde()
//                .unwrap();
                .map_err(|e| {
                    log(&Array::from(&JsValue::from_str(&e.to_string())));
                    e
                })
                .unwrap_or_else(|_| T::default());

            future::ok(JsValue::from_serde(&t).unwrap())
        });

    // Convert this Rust `Future` back into a JS `Promise`.
    future_to_promise(future)
}


fn request_promise<T>(url: &str, method: Method<T>, auth: Option<String>) -> Promise
    where T: Serialize
{
    let mut opts = RequestInit::new();

    let method_str = match method {
        Method::Get => "GET",
        Method::Delete => "DELETE",
        Method::Put(_) => "PUT",
        Method::Post(_) => "POST"
    };

    opts.method(method_str);
    opts.mode(RequestMode::Cors);
    match &method {
        Method::Post(t) | Method::Put(t) => {
            let s = serde_json::to_string(t)
                .ok()
                .map(|s| JsValue::from_str(&s));
            opts.body(s.as_ref());
        },
        _ => {}
    };

    let request = Request::new_with_str_and_init(
        url,
        &opts,
    )
        .unwrap();

    request
        .headers()
        .set("Access-Control-Allow-Origin", "*")
        .unwrap();

    match &method {
        Method::Post(_) | Method::Put(_) => {
            request
                .headers()
                .set("content-type", "application/json")
                .unwrap();
        }
        _ => {}
    };

    if let Some(auth) = auth {
        request
            .headers()
//                  Authorization
            .set("Authorization", &format!("bearer {}", auth))
            .unwrap();
    }

    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);
    request_promise
}

fn get_string_from_request_promise(request_promise: Promise) -> Promise {
    let future = JsFuture::from(request_promise)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into().unwrap();
            resp.text()
        })
            .and_then(|text_value: Promise| {
            // Convert this other `Promise` into a rust `Future`.
            JsFuture::from(text_value)
        })
        .and_then(|text| {
            future::ok(text)
        });
    future_to_promise(future)
}