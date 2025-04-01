use crate::DrawResult;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

fn read_file(file_name: &str) -> Vec<u16> {
    let bytes = std::fs::read(file_name).unwrap();

    let bytes_converted = bytes
        .chunks_exact(2)
        .map(|w| u16::from_le_bytes([w[0], w[1]]))
        .collect::<Vec<u16>>();

    bytes_converted
}

//pub fn draw(canvas: HtmlCanvasElement) -> DrawResult<impl Fn((i32, i32)) -> Option<(u32, u32)>> {
pub fn draw() -> DrawResult<impl Fn((i32, i32)) -> Option<(u32, u32)>> {
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
    root.fill(&YELLOW)?;

    let chart_data = read_file("input-data/RPM_DATA.bin");
    println!("{:?}", chart_data);
    let chart_data_len = chart_data.len() / 3;
    let chart_data_min = chart_data.iter().min().unwrap();
    let chart_data_max = chart_data.iter().max().unwrap();

    //let root = BitMapBackend::new("output-data/0.png", (640, 480)).into_drawing_area();
    //root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Data channels", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            0u32..chart_data_len as u32,
            *chart_data_min as u32 - 200..*chart_data_max as u32 + 200,
        )?;

    chart.configure_mesh().draw()?;

    // Channel 0
    chart
        .draw_series(LineSeries::new(
            chart_data
                .iter()
                .skip(0)
                .step_by(3)
                .enumerate()
                .map(|(i, y)| (i as u32, *y as u32)),
            &RED,
        ))?
        .label("y = ch0")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

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

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

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
