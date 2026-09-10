// examples/button.rs
//
// Minimal runnable example: two buttons, animated hover/press states,
// and a click counter — showing the `Widget` trait in action.

use macroquad::prelude::*;

use uilib::widgets::button::{Button, ButtonStyle};
use uilib::widgets::widget::Widget;

fn window_conf() -> Conf {
    Conf {
        window_title: "uilib — basic button".to_owned(),
        window_width: 600,
        window_height: 360,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let font = load_ttf_font_from_bytes(include_bytes!("../font.ttf"));
    // A "primary" button: blue-ish, brightens on hover, darkens on press.
    let mut primary = Button::new("Click me", vec2(320.0, 140.0))
        .normal(
            ButtonStyle::new()
                .color(Color::from_rgba(90, 90, 220, 255))
                .border(Color::from_rgba(90, 120, 255, 255), 2.0)
                .rounding(8.0)
                .font(font.expect("idk")),
        )
        .hover(
            ButtonStyle::new()
                .color(Color::from_rgba(80, 110, 240, 255))
                .border(Color::from_rgba(120, 150, 255, 255), 2.0)
                .rounding(8.0),
        )
        .pressed(
            ButtonStyle::new()
                .color(Color::from_rgba(40, 60, 160, 255))
                .scale(vec2(0.5, 0.5))
                .rounding(8.0),
        )
        .speed(14.0);

    // A disabled button, styled to look obviously inert.
    let disabled = Button::new("Disabled", vec2(320.0, 220.0))
        .normal(ButtonStyle::new().color(Color::from_rgba(50, 50, 55, 255)))
        .disabled(
            ButtonStyle::new()
                .color(Color::from_rgba(35, 35, 38, 255))
                .text_color(Color::from_rgba(120, 120, 120, 255)),
        )
        .enabled(false);

    let mut click_count = 0u32;

    loop {
        clear_background(Color::from_rgba(20, 20, 24, 255));

        // ------------------------------------------------------
        // UPDATE
        // ------------------------------------------------------

        primary.update();

        if primary.clicked() {
            click_count += 1;
        }

        // ------------------------------------------------------
        // DRAW
        // ------------------------------------------------------

        primary.draw();
        disabled.draw();

        draw_text(&format!("Clicks: {click_count}"), 20.0, 30.0, 24.0, WHITE);

        if primary.hovered() {
            draw_text("hovering!", 20.0, 60.0, 20.0, GRAY);
        }

        next_frame().await;
    }
}
