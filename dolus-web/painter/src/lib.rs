pub mod painter;

use std::{collections::HashMap, fmt::Debug};

use chrono::NaiveDateTime;
use painter::{charts::ChartPainter, histogram::HistogramPainter};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(serde::Deserialize, Debug)]
struct CountResponse {
    url: String,
    payload: Vec<(String, u64)>,
}

struct CountByUrl {
    max_value: u64,
    by_url: HashMap<String, Vec<(NaiveDateTime, u64)>>,
}

#[wasm_bindgen(js_name=fetchHistogram)]
pub async fn fetch_histo(url: String) -> Result<HistogramPainter, JsValue> {
    console_error_panic_hook::set_once();

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url.as_str(), &opts)?;

    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    let data: painter::histogram::HistogramResponse =
        json.into_serde().expect("Failed to parse response");

    let res = HistogramPainter { data };
    Ok(res)
}

#[wasm_bindgen(js_name=fetchData)]
pub async fn fetch_data(word: String, url: String) -> Result<ChartPainter, JsValue> {
    console_error_panic_hook::set_once();

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url.as_str(), &opts)?;

    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    let data: Vec<CountResponse> = json.into_serde().expect("Failed to parse response");

    let mut res = ChartPainter {
        word,
        labels: Vec::with_capacity(data.len()),
        count: CountByUrl {
            max_value: 0,
            by_url: HashMap::with_capacity(16),
        },
        color_by_url: HashMap::with_capacity(16),
    };

    for datum in data {
        let url = &datum.url;
        for (created, count) in datum.payload {
            let created = NaiveDateTime::parse_from_str(created.as_str(), "%Y-%m-%dT%H:%M:%S.%fZ")
                .expect("Failed to parse `created`");
            res.labels.push(created);
            if res.count.max_value < count {
                res.count.max_value = count;
            }
            res.count
                .by_url
                .entry(url.clone())
                .or_insert_with(|| Vec::with_capacity(1024))
                .push((created, count));
        }
    }

    res.labels.sort();

    Ok(res)
}
