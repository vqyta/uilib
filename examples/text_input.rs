// examples/text_input.rs
//
// A single-line text input with a placeholder, plus submit-on-Enter
// handling. Click to focus, type, select with Shift+arrows or by
// dragging, copy/cut/paste with Ctrl+C/X/V.

use macroquad::prelude::*;

use uilib::widgets::input::{TextInput, TextInputStyle};
use uilib::widgets::widget::Widget;

fn window_conf() -> Conf {
    Conf {
        window_title: "uilib — text input".to_owned(),
        window_width: 640,
        window_height: 360,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut input = TextInput::new(vec2(320.0, 160.0))
        .placeholder("Type your name...")
        .max_len(40)
        .normal(TextInputStyle::new().color(Color::from_rgba(30, 30, 34, 255)))
        .hover(TextInputStyle::new().color(Color::from_rgba(38, 38, 44, 255)))
        .focused(
            TextInputStyle::new()
                .color(Color::from_rgba(30, 30, 34, 255))
                .border(Color::from_rgba(90, 140, 255, 255), 2.0),
        )
        .speed(18.0);

    let mut last_submitted = String::new();

    loop {
        clear_background(Color::from_rgba(20, 20, 24, 255));

        input.update();

        if input.submitted() {
            last_submitted = input.text().to_string();
        }

        input.draw();

        draw_text(
            "Click the box, type, press Enter to submit",
            20.0,
            30.0,
            22.0,
            GRAY,
        );

        if !last_submitted.is_empty() {
            draw_text(
                &format!("Last submitted: {last_submitted}"),
                20.0,
                250.0,
                22.0,
                WHITE,
            );
        }

        next_frame().await;
    }
}
