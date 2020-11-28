use std::collections::{HashMap, HashSet};

use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::prelude::*;

#[derive(serde::Deserialize)]
pub struct HistogramEntry {
    counts: Vec<(String, Option<i64>)>,
    total: i64,
}
pub type HistogramResponse = HashMap<String, HistogramEntry>;

#[wasm_bindgen]
pub struct HistogramPainter {
    /// word: [url, count][]
    pub(crate) data: HistogramResponse,
}
#[wasm_bindgen]
impl HistogramPainter {
    #[wasm_bindgen]
    pub fn words(&self) -> JsValue {
        let mut vec: Vec<_> = self
            .data
            .iter()
            .filter_map(|(k, v)| if v.total > 0 { Some(k) } else { None })
            .cloned()
            .collect();
        vec.sort();
        JsValue::from_serde(&vec).unwrap()
    }

    #[wasm_bindgen]
    pub fn draw(&mut self, canvas_id: String, word: &str) -> Result<(), JsValue> {
        let backend = CanvasBackend::new(canvas_id.as_str())
            .ok_or_else(|| JsValue::from_str("cannot find canvas"))?;

        let root = backend.into_drawing_area();
        let font: FontDesc = ("sans-serif", 25.0).into();

        root.fill(&WHITE).map_err(map_err_to_js("fill"))?;

        let urls: HashSet<_> = self
            .data
            .values()
            .flat_map(|v| v.counts.iter().map(|(url, _)| url.clone()))
            .collect();
        let urls: Vec<_> = urls.into_iter().collect();

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .margin(5)
            .caption(word, font)
            .build_cartesian_2d(urls.into_segmented(), 0..20i64)
            .map_err(map_err_to_js("build chart"))?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(&BLACK)
            .y_desc("Count")
            .x_desc("URL")
            .axis_desc_style(("sans-serif", 15))
            .draw()
            .map_err(map_err_to_js("configure mesh"))?;

        chart
            .draw_series(
                Histogram::vertical(&chart)
                    .style(RED.mix(0.8).filled()) // TODO: color for each url
                    .data(
                        self.data[word]
                            .counts
                            .iter()
                            .filter_map(move |(url, v)| v.map(|v| (url, v))),
                    ),
            )
            .map_err(map_err_to_js("draw"))?;

        Ok(())
    }
}

#[inline]
fn map_err_to_js<T: std::fmt::Debug + 'static>(
    prefix: impl AsRef<str>,
) -> impl FnOnce(T) -> JsValue {
    move |err: T| JsValue::from_str(&format!("{} {:?}", prefix.as_ref(), err))
}
