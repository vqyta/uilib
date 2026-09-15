// src/lib.rs
//
// uilib — a small immediate-ish, retained-style widget toolkit for
// macroquad. Widgets keep their own animated style state between
// frames; you call `update()` then `draw()` on them each frame.

pub mod anim;

pub mod widgets {
    pub mod button;
    pub mod input;
    pub mod slider;
    pub mod widget;
}

// ----------------------------------------------------------------
// Convenience re-exports.
//
// Lets consumers write:
//   use uilib::{Button, ButtonStyle, TextInput, TextInputStyle, Slider, SliderStyle, Widget};
// instead of reaching into the `widgets::` submodules directly.
// The submodule paths keep working too, for anyone who prefers them.
// ----------------------------------------------------------------

pub use anim::Lerp;
pub use widgets::button::{Button, ButtonStyle};
pub use widgets::input::{TextInput, TextInputStyle};
pub use widgets::slider::{Slider, SliderOrientation, SliderStyle};
pub use widgets::widget::{Widget, WidgetState};
