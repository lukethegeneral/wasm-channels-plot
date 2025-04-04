extern crate wasm_bindgen;
use std::vec;

use crate::DrawResult;
use js_sys::Error;
use js_sys::Uint8Array;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(text: &str) {
    alert(&format!("Channels plot, {}", text));
}

#[wasm_bindgen(module = "/src/lib.js")]
extern "C" {
    fn starting() -> String;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

const COLORS: [RGBColor; 5] = [RED, GREEN, BLUE, CYAN, MAGENTA];

#[wasm_bindgen]
pub fn read_file(bytes: Uint8Array) -> Result<Vec<u16>, Error> {
    // let bytes = include_bytes!("../input-data/RPM_DATA.bin");
    // let bytes = vec![12, 45, 78, 32, 22, 12, 44, 55, 66, 77, 88, 99];

    greet(&starting());

    let mut bytes_rust = vec![0; bytes.length() as usize];
    bytes.copy_to(&mut bytes_rust[..]);

    // Convert bytes from little endian
    let bytes_converted = bytes_rust
        .chunks_exact(2)
        .map(|w| u16::from_le_bytes([w[0], w[1]]))
        .collect::<Vec<u16>>();

    Ok(bytes_converted)
}

pub fn draw(
    canvas: HtmlCanvasElement,
    bytes: Uint8Array,
    channels: usize,
) -> DrawResult<impl Fn((i32, i32)) -> Option<(u32, u32)>> {
    let backend = CanvasBackend::with_canvas_object(canvas).unwrap();

    let root = backend.into_drawing_area();
    root.fill(&YELLOW)?;

    // Read converted data from file
    let chart_data = read_file(bytes).unwrap();
    //let chart_data = vec![1234, 2345, 5678, 3271, 8822, 1234, 2762, 1765, 1490];
    let chart_data_len = chart_data.len() / channels;
    let chart_data_min = chart_data.iter().min().unwrap();
    let chart_data_max = chart_data.iter().max().unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Data channels", ("sans-serif", 20).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            0u32..chart_data_len as u32,
            *chart_data_min as u32 - 200..*chart_data_max as u32 + 200,
        )?;

    chart
        .configure_mesh()
        .x_desc("x = sample time")
        .y_desc("y = channel value")
        .draw()?;

    // Channels 0..channels
    for (channel, color) in COLORS.iter().enumerate().filter(|x| x.0 < channels) {
        chart
            .draw_series(LineSeries::new(
                chart_data
                    .iter()
                    .skip(channel)
                    .step_by(channels)
                    .enumerate()
                    .map(|(i, y)| (i as u32, *y as u32)),
                color,
            ))?
            .label(format!("y = channel {}", channel))
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color.clone()));
    }

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    return Ok(chart.into_coord_trans());
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    // Run test with: wasm-pack test --firefox
    #[test]
    #[wasm_bindgen_test]
    fn test_read_file_wasm() {
        let bytes = include_bytes!("../input-data/RPM_DATA.bin");
        //First 3 bytes in the file: 44 09 F3 06 D1 05
        let bytes_js = Uint8Array::new_with_length(bytes.len() as u32);
        bytes_js.copy_from(&bytes[..]);
        let result = read_file(bytes_js).unwrap();
        assert_eq!(result[..3], vec![0x944, 0x6f3, 0x5d1]);
    }
}
