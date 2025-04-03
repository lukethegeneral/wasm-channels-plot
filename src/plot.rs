use std::collections::HashMap;
extern crate wasm_bindgen;
use std::vec;

use crate::DrawResult;
use js_sys::Error;
use js_sys::{ArrayBuffer, Uint8Array};
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(text: &str) {
    alert(&format!("Wasm plot, {}!", text));
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

//const FILE_NAME: &str = "../input-data/RPM_DATA.bin";

#[wasm_bindgen]
pub fn read_file(bytes: Uint8Array) -> Result<Vec<u16>, Error> {
    // let bytes = std::fs::read(file_name).unwrap();
    // let bytes = include_bytes!("../input-data/RPM_DATA.bin");
    // let bytes = vec![12, 45, 78, 32, 22, 12, 44, 55, 66, 77, 88, 99];

    greet(&starting());
    //greet(&len.to_string());

    let mut bytes_rust = vec![0; bytes.length() as usize];
    bytes.copy_to(&mut bytes_rust[..]);

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
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    let backend = CanvasBackend::with_canvas_object(canvas).unwrap();

    let root = backend.into_drawing_area();
    root.fill(&YELLOW)?;

    let chart_data = read_file(bytes).unwrap();
    //let chart_data = vec![1234, 2345, 5678, 3271, 8822, 1234];
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

    chart.configure_mesh().draw()?;

    // Channels 0..channels
    for (channel, color) in COLORS.iter().enumerate().filter(|x| x.0 < channels) {
        chart
            .draw_series(LineSeries::new(
                chart_data
                    .iter()
                    .skip(channel)
                    .step_by(3)
                    .enumerate()
                    .map(|(i, y)| (i as u32, *y as u32)),
                color,
            ))?
            .label(format!("y = channel {}", channel))
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color.clone()));
    }

    /*
        // Channel 1
        chart
            .draw_series(LineSeries::new(
                chart_data
                    .iter()
                    .skip(1)
                    .step_by(3)
                    .enumerate()
                    .map(|(i, y)| (i as u32, *y as u32)),
                &BLUE,
            ))?
            .label("y = ch1")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        // Channel 2
        chart
            .draw_series(LineSeries::new(
                chart_data
                    .iter()
                    .skip(2)
                    .step_by(3)
                    .enumerate()
                    .map(|(i, y)| (i as u32, *y as u32)),
                &GREEN,
            ))?
            .label("y = ch2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
    */

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    context.stroke();

    return Ok(chart.into_coord_trans());
}

/// Draw power function f(x) = x^power.
pub fn draw_2(power: i32) -> DrawResult<impl Fn((i32, i32)) -> Option<(f32, f32)>> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    ////
    let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .caption(format!("y=x^{}", power), font)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(-1f32..1f32, -1.2f32..1.2f32)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    chart.draw_series(LineSeries::new(
        (-50..=50)
            .map(|x| x as f32 / 50.0)
            .map(|x| (x, x.powf(power as f32))),
        &RED,
    ))?;

    root.present()?;
    return Ok(chart.into_coord_trans());
}
