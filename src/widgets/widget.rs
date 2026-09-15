// src/widgets/widget.rs

use macroquad::prelude::*;

// ============================================================
// VIRTUAL RESOLUTION
// ============================================================

/// Fixed logical resolution used by the UI.
///
/// Widgets use these coordinates regardless of the actual
/// window size. The UI is then uniformly scaled to fit the
/// available screen while preserving the aspect ratio.
#[derive(Clone, Copy, Debug)]
pub struct VirtualResolution {
    pub width: f32,
    pub height: f32,
}

impl VirtualResolution {
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Default UI design resolution.
    ///
    /// This matches the original 600x360 example exactly.
    pub const fn default() -> Self {
        Self::new(600.0, 360.0)
    }

    /// Scale factor from virtual coordinates to screen coordinates.
    pub fn scale(&self) -> f32 {
        (screen_width() / self.width).min(screen_height() / self.height)
    }

    /// The actual screen area occupied by the virtual resolution.
    ///
    /// If the aspect ratio differs, the unused area becomes
    /// letterboxing around the UI.
    pub fn viewport(&self) -> Rect {
        let scale = self.scale();

        let width = self.width * scale;
        let height = self.height * scale;

        let x = (screen_width() - width) * 0.5;
        let y = (screen_height() - height) * 0.5;

        Rect::new(x, y, width, height)
    }

    /// Convert real mouse coordinates into virtual UI coordinates.
    pub fn mouse_position(&self) -> Vec2 {
        let (mouse_x, mouse_y) = mouse_position();

        let viewport = self.viewport();
        let scale = self.scale();

        vec2(
            (mouse_x - viewport.x) / scale,
            (mouse_y - viewport.y) / scale,
        )
    }
}

// ============================================================
// WIDGET STATE
// ============================================================

/// Interaction state shared across all widget kinds.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WidgetState {
    Normal,
    Hover,
    Pressed,

    /// Widget currently has keyboard focus.
    ///
    /// Buttons never enter this state; it exists so focus-based
    /// widgets can use the same state/style system.
    Focused,

    Disabled,
}

// ============================================================
// WIDGET TRAIT
// ============================================================

pub trait Widget {
    /// Advance the widget's internal animated state by one frame.
    fn update(&mut self);

    /// Render the widget in its current state.
    fn draw(&self);

    /// Axis-aligned bounding box in VIRTUAL coordinates.
    fn hitbox(&self) -> Rect;

    /// Whether the widget currently accepts input.
    fn is_enabled(&self) -> bool;

    /// Virtual resolution used by this widget.
    ///
    /// Override this if a widget needs a different design resolution.
    fn virtual_resolution(&self) -> VirtualResolution {
        VirtualResolution::default()
    }

    /// Current mouse position in virtual coordinates.
    fn virtual_mouse_position(&self) -> Vec2 {
        self.virtual_resolution().mouse_position()
    }

    /// Whether the mouse is inside the widget's virtual hitbox.
    fn mouse_inside(&self) -> bool {
        self.hitbox().contains(self.virtual_mouse_position())
    }

    /// Whether the widget is currently hovered.
    fn hovered(&self) -> bool {
        self.is_enabled() && self.mouse_inside()
    }

    /// Whether the left mouse button is currently held over it.
    fn pressed_now(&self) -> bool {
        self.is_enabled() && is_mouse_button_down(MouseButton::Left) && self.mouse_inside()
    }

    /// Whether the widget was clicked this frame.
    fn clicked(&self) -> bool {
        self.is_enabled() && is_mouse_button_pressed(MouseButton::Left) && self.mouse_inside()
    }

    /// Current interaction state.
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
