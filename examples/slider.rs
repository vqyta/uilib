// examples/slider.rs
//
// Two sliders: a stepped horizontal volume slider (drag, arrow keys,
// mouse wheel, Home/End) and a continuous vertical one.

use macroquad::prelude::*;

use uilibrary::widgets::slider::{Slider, SliderOrientation, SliderStyle};
use uilibrary::widgets::widget::Widget;

fn window_conf() -> Conf {
    Conf {
        window_title: "uilib — slider".to_owned(),
        window_width: 640,
        window_height: 360,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut volume = Slider::new(vec2(320.0, 120.0), 0.0, 100.0)
        .value(50.0)
        .step(1.0)
        .normal(SliderStyle::new())
        .hover(SliderStyle::new().handle_radius(11.0))
        .pressed(SliderStyle::new().handle_radius(13.0))
        .focused(SliderStyle::new().handle_border(Color::from_rgba(90, 140, 255, 255), 2.0))
        .speed(20.0);

    let mut brightness = Slider::new(vec2(500.0, 200.0), 0.0, 1.0)
        .value(0.75)
        .orientation(SliderOrientation::Vertical)
        .normal(SliderStyle::new().size(vec2(24.0, 200.0)))
        .hover(
            SliderStyle::new()
                .size(vec2(24.0, 200.0))
                .handle_radius(11.0),
        )
        .pressed(
            SliderStyle::new()
                .size(vec2(24.0, 200.0))
                .handle_radius(13.0),
        )
        .speed(20.0);

    loop {
        clear_background(Color::from_rgba(20, 20, 24, 255));

        volume.update();
        brightness.update();

        volume.draw();
        brightness.draw();

        draw_text(
            &format!("Volume: {:.0}", volume.get()),
            220.0,
            60.0,
            24.0,
            WHITE,
        );
        draw_text(
            &format!("Brightness: {:.2}", brightness.get()),
            420.0,
            60.0,
            24.0,
            WHITE,
        );

        draw_text(
            "Drag, arrow keys, wheel, Home/End, Page Up/Down",
            20.0,
            320.0,
            20.0,
            GRAY,
        );

        next_frame().await;
    }
}
