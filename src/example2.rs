use std::collections::HashMap;

use anyhow::Result;

use plotters::chart::{ChartBuilder, LabelAreaPosition};
use plotters::coord::combinators::{BindKeyPoints, IntoLogRange};
use plotters::coord::Shift;
use plotters::drawing::{DrawingArea, IntoDrawingArea};
use plotters::element::Rectangle;
use plotters::series::LineSeries;
use plotters::style::{
    colors::{BLACK, WHITE},
    Color, Palette, Palette99,
};
use plotters::style::{AsRelative, IntoFont};

use plotters::prelude::SVGBackend;

const OUT_FILE_NAME: &'static str = "logscale-sample.svg";
const FONT: &'static str = "sans-serif";

pub fn run() -> Result<()> {
    let root: DrawingArea<SVGBackend, Shift> =
        SVGBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let (upper, lower) = root.split_vertically(750);

    lower.titled(
        "log-scale sample",
        (FONT, 10).into_font().color(&BLACK.mix(0.5)),
    )?;

    let mut chart = ChartBuilder::on(&upper)
        .caption("FREQUENCY RESPONSE", (FONT, (5).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (8).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
        .margin((1).percent())
        .build_cartesian_2d(
            (1u32..10_000u32)
                .log_scale()
                .with_key_points(vec![1, 10, 100, 1000, 10_000]),
            (0u32..101u32).with_key_points(vec![0, 20, 40, 60, 80, 100]),
        )?;

    chart
        .configure_mesh()
        .x_desc("freq [Hz]")
        .y_desc("gain [dB]")
        .draw()?;

    let data: HashMap<String, ParentData> = stub();

    for (idx, &series) in ["A", "B"].iter().enumerate() {
        let color = Palette99::pick(idx).mix(0.9);
        chart
            .draw_series(LineSeries::new(
                data[series]
                    .data
                    .iter()
                    .map(|&ChildData { freq, gain, .. }| (freq as u32, gain as u32)),
                color.stroke_width(3),
            ))?
            .label(series)
            .legend(move |(x, y)| {
                Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled())
            });
    }

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()?;

    //// To avoid the IO failure being ignored silently, we manually call the present function
    //root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}

fn stub() -> HashMap<String, ParentData> {
    let mut hash = HashMap::new();
    hash.insert(
        "A".to_string(),
        ParentData {
            data: vec![
                ChildData::new(1., 1., 1.),
                ChildData::new(10., 10., 10.),
                ChildData::new(10_000., 100., 100.),
            ],
        },
    );
    hash.insert(
        "B".to_string(),
        ParentData {
            data: vec![
                ChildData::new(1., 1., 1.),
                ChildData::new(20., 20., 20.),
                ChildData::new(10_000., 100., 100.),
            ],
        },
    );

    return hash;
}

struct ParentData {
    data: Vec<ChildData>,
}

struct ChildData {
    freq: f64,
    gain: f64,
    _phase: f64,
}

impl ChildData {
    pub fn new(freq: f64, gain: f64, _phase: f64) -> Self {
        Self { freq, gain, _phase }
    }
}
