use std::collections::HashMap;

use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::prelude::*;

#[derive(serde::Deserialize)]
pub struct HistogramEntry {
    pub counts: Vec<(String, Option<i64>)>,
    pub total: i64,
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
        vec.sort_by(|a, b| self.data[b].total.cmp(&self.data[a].total));
        JsValue::from_serde(&vec).unwrap()
    }

    #[wasm_bindgen]
    pub fn draw(&mut self, canvas_id: String, word: &str) -> Result<(), JsValue> {
        let backend = CanvasBackend::new(canvas_id.as_str())
            .ok_or_else(|| JsValue::from_str("cannot find canvas"))?;

        let root = backend.into_drawing_area();
        let font: FontDesc = ("sans-serif", 25.0).into();

        root.fill(&WHITE).map_err(map_err_to_js("fill"))?;

        let entry = &self.data[word];
        let urls: Vec<_> = entry
            .counts
            .iter()
            .filter_map(|(url, c)| c.and_then(|x| if x > 0 { Some(url) } else { None }))
            .cloned()
            .collect();

        let max = entry
            .counts
            .iter()
            .filter_map(|(_, v)| v.clone())
            .max()
            .unwrap_or(0);

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .margin(5)
            .caption(format!("{} ({})", word, entry.total), font)
            .build_cartesian_2d(urls.into_segmented(), 0..max)
            .map_err(map_err_to_js("build chart"))?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .bold_line_style(&BLACK)
            .y_desc("Darab")
            .x_desc("URL")
            .axis_desc_style(("sans-serif", 15))
            .draw()
            .map_err(map_err_to_js("configure mesh"))?;

        chart
            .draw_series(
                Histogram::vertical(&chart)
                    .style(RED.mix(0.8).filled()) // TODO: color for each url
                    .data(entry.counts.iter().filter_map(|(k, v)| v.map(|x| (k, x)))),
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
