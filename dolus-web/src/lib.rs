use std::{collections::HashMap, fmt::Debug};

use chrono::NaiveDateTime;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
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
struct DolusCountResponse {
    url: String,
    payload: Vec<(String, u64)>,
}

#[wasm_bindgen]
pub struct DolusChartPainter {
    word: String,
    labels: Vec<NaiveDateTime>,
    count: CountByUrl,
    color_by_url: HashMap<String, ShapeStyle>,
}

struct CountByUrl {
    max_value: u64,
    by_url: HashMap<String, Vec<(NaiveDateTime, u64)>>,
}

fn map_err_to_js<T: Debug + 'static>(prefix: impl AsRef<str>) -> impl FnOnce(T) -> JsValue {
    move |err: T| JsValue::from_str(&format!("{} {:?}", prefix.as_ref(), err))
}

#[wasm_bindgen]
impl DolusChartPainter {
    #[wasm_bindgen]
    pub fn word(&self) -> String {
        self.word.clone()
    }

    #[wasm_bindgen]
    pub fn draw(&mut self, canvas_id: String) -> Result<(), JsValue> {
        if self.labels.len() < 2 {
            return Err(map_err_to_js("need at least 2 entries")(()));
        }

        let backend = CanvasBackend::new(canvas_id.as_str())
            .ok_or_else(|| JsValue::from_str("cannot find canvas"))?;
        let root = backend.into_drawing_area();
        let font: FontDesc = ("sans-serif", 25.0).into();

        root.fill(&WHITE).map_err(map_err_to_js("fill"))?;

        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .caption(self.word.clone(), font)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                self.labels[0].timestamp()..self.labels.last().unwrap().timestamp(),
                0..self.count.max_value,
            )
            .map_err(map_err_to_js("build"))?;

        chart
            .configure_mesh()
            .disable_y_mesh()
            .x_labels(5)
            .y_labels((self.count.max_value as usize / 3).max(3))
            .x_label_formatter(&|x| {
                NaiveDateTime::from_timestamp(*x, 0)
                    .format("%Y-%m-%d %H:%M")
                    .to_string()
            })
            .draw()
            .map_err(map_err_to_js("configure mesh"))?;

        for (i, (url, count)) in self.count.by_url.iter().enumerate() {
            chart
                .draw_series(
                    LineSeries::new(
                        count.iter().map(|(x, y)| (x.timestamp(), *y)),
                        self.color_by_url
                            .entry(url.clone())
                            .or_insert_with(|| {
                                plotters::style::RGBColor(
                                    (((i * 3) * 75) & 225) as u8,
                                    (((i * 7) * 50) & 225) as u8,
                                    (((i * 13) * 25) & 225) as u8,
                                )
                                .filled()
                            })
                            .stroke_width(3),
                    )
                    .point_size(3),
                )
                .map_err(map_err_to_js("draw series"))?
                .label(url.clone())
                .legend({
                    let url = url.clone();
                    let color = self.color_by_url[url.as_str()].stroke_width(50);
                    move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color.clone())
                });
        }

        let font: FontDesc = ("sans-serif", 15.0).into();

        chart
            .configure_series_labels()
            .margin(5)
            .label_font(font)
            .background_style(&WHITE.mix(0.9))
            .border_style(&BLACK)
            .draw()
            .map_err(map_err_to_js("failed to draw legend"))?;

        root.present().map_err(map_err_to_js("present"))?;

        Ok(())
    }
}

#[wasm_bindgen]
pub async fn fetch_data(word: String, url: String) -> Result<DolusChartPainter, JsValue> {
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

    let data: Vec<DolusCountResponse> = json.into_serde().expect("Failed to parse response");

    let mut res = DolusChartPainter {
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

    Ok(res)
}
