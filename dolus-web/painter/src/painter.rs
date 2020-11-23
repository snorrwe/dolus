use chrono::NaiveDateTime;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::prelude::*;

use crate::CountByUrl;

use std::{collections::HashMap, fmt::Debug};

#[wasm_bindgen]
pub struct DolusChartPainter {
    pub(crate) word: String,
    pub(crate) labels: Vec<NaiveDateTime>,
    pub(crate) count: CountByUrl,
    pub(crate) color_by_url: HashMap<String, ShapeStyle>,
}

impl DolusChartPainter {
    /// return url, timestamp, value tuples
    fn get_closest_values(&self, x_pos: f64) -> impl Iterator<Item = (&str, NaiveDateTime, u64)> {
        let minx = self.labels.first().unwrap().timestamp();
        let maxx = self.labels.last().unwrap().timestamp();
        let dist = maxx - minx;

        let d = dist as f64 * x_pos;
        let p = minx + d as i64;

        let ind = self
            .labels
            .binary_search_by_key(&p, |t| t.timestamp())
            .unwrap_or_else(|i| i);

        let target_time = match self.labels.get(ind) {
            Some(dt) => dt.timestamp(),
            None => 0,
        };

        self.count.by_url.iter().filter_map(move |(url, count)| {
            count
                .iter()
                .min_by_key(|(t, _)| (t.timestamp() - target_time).abs())
                .map(|(t, val)| {
                    let val = *val;
                    let t = *t;
                    (url.as_str(), t, val)
                })
        })
    }
}

#[wasm_bindgen]
impl DolusChartPainter {
    #[wasm_bindgen]
    pub fn word(&self) -> String {
        self.word.clone()
    }

    #[wasm_bindgen(js_name = "numberOfItems")]
    pub fn number_of_items(&self) -> usize {
        self.count.by_url.iter().map(|(_, c)| c.len()).sum()
    }

    /// @param `x_pos` should be normalized, in interval [0, 1]
    /// @return Dictionary of urls and [datetime, value] tuples
    #[wasm_bindgen(js_name = "getClosest")]
    pub fn get_closest(&self, x_pos: f64) -> JsValue {
        if self.labels.len() < 2 {
            return JsValue::NULL;
        }

        let res = self
            .get_closest_values(x_pos)
            .map(|(url, ts, val)| (url, (ts.format("%Y-%m-%d %H:%M").to_string(), val)))
            .collect::<HashMap<_, _>>();

        JsValue::from_serde(&res).unwrap()
    }

    /// @param x_pos should be normalized to the canvas, in interval [0, 1.0]
    #[wasm_bindgen]
    pub fn draw(&mut self, canvas_id: String, x_pos: Option<f32>) -> Result<(), JsValue> {
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
                                    (((i * 3) * 75) & 225).max(20) as u8,
                                    (((i * 7) * 50) & 225).max(20) as u8,
                                    (((i * 13) * 25) & 225).max(20) as u8,
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

#[inline]
fn map_err_to_js<T: Debug + 'static>(prefix: impl AsRef<str>) -> impl FnOnce(T) -> JsValue {
    move |err: T| JsValue::from_str(&format!("{} {:?}", prefix.as_ref(), err))
}
