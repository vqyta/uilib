// src/widgets/widget.rs
//
// Common interface every interactive UI widget implements
// (Button, and future widgets like Checkbox / Slider / Toggle).

use macroquad::prelude::*;

// ============================================================
// WIDGET STATE
// ============================================================

/// Interaction state shared across all widget kinds.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WidgetState {
    Normal,
    Hover,
    Pressed,
    /// Widget currently has keyboard focus (text inputs and anything
    /// else that accepts keyboard input). Buttons never enter this
    /// state; it exists so focus-based widgets can plug into the same
    /// `WidgetState` / style-per-state pattern as everything else.
    Focused,
    Disabled,
}

// ============================================================
// WIDGET TRAIT
// ============================================================

pub trait Widget {
    /// Advance the widget's internal (usually animated) state by one frame.
    fn update(&mut self);

    /// Render the widget in its current state.
    fn draw(&self);

    /// Axis-aligned bounding box used for hit-testing.
    fn hitbox(&self) -> Rect;

    /// Whether the widget currently accepts input.
    fn is_enabled(&self) -> bool;

    // --------------------------------------------------------
    // Shared, derived behaviour — widgets get these for free
    // as long as they implement `hitbox` and `is_enabled`.
    // --------------------------------------------------------

    /// Is the mouse cursor currently inside the widget's hitbox.
    fn mouse_inside(&self) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        self.hitbox().contains(vec2(mouse_x, mouse_y))
    }

    /// True while the pointer is over an enabled widget.
    fn hovered(&self) -> bool {
        self.is_enabled() && self.mouse_inside()
    }

    /// True while the primary mouse button is held down over the widget.
    fn pressed_now(&self) -> bool {
        self.is_enabled() && is_mouse_button_down(MouseButton::Left) && self.mouse_inside()
    }

    /// True on the single frame the widget was clicked.
    fn clicked(&self) -> bool {
        self.is_enabled() && is_mouse_button_pressed(MouseButton::Left) && self.mouse_inside()
    }

    /// Current interaction state, derived from the above.
    fn state(&self) -> WidgetState {
        if !self.is_enabled() {
            WidgetState::Disabled
        } else if self.pressed_now() {
            WidgetState::Pressed
        } else if self.hovered() {
            WidgetState::Hover
        } else {
            WidgetState::Normal
        }
    }
}
