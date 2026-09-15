// src/widgets/slider.rs
//
// A draggable slider that behaves like a native range input:
// click-and-drag the handle (or click anywhere on the track to jump
// there and start dragging), arrow keys to nudge the value by one
// step while focused (Right/Up increase, Left/Down decrease — same
// mapping browsers use for <input type="range"> regardless of
// orientation), Home/End to jump to the min/max, Page Up/Down for a
// bigger jump, and mouse wheel to nudge while hovered.

use macroquad::prelude::*;

use crate::anim::{Lerp, smoothing_factor};
use crate::widgets::widget::{Widget, WidgetState};

// ============================================================
// ORIENTATION
// ============================================================

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SliderOrientation {
    Horizontal,
    Vertical,
}

// ============================================================
// SLIDER STYLE
// ============================================================

#[derive(Clone)]
pub struct SliderStyle {
    pub track_color: Color,
    /// Colors the portion of the track between the start and the handle.
    pub fill_color: Color,

    pub handle_color: Color,
    pub handle_border_color: Color,
    pub handle_border_width: f32,
    pub handle_radius: f32,

    /// Overall bounding box: length along the slide axis by thickness
    /// across it (thickness also caps how tall/wide the track line is
    /// drawn, independent of the handle radius).
    pub size: Vec2,
    pub track_thickness: f32,

    pub scale: Vec2,
    pub rotation: f32,

    /// Reserved for rounded-rect track drawing; not yet applied by
    /// `draw()`, kept for API parity with `ButtonStyle`/`TextInputStyle`.
    pub rounding: f32,
}

impl Default for SliderStyle {
    fn default() -> Self {
        Self {
            track_color: Color::from_rgba(60, 60, 66, 255),
            fill_color: Color::from_rgba(90, 140, 255, 255),

            handle_color: WHITE,
            handle_border_color: Color::from_rgba(90, 140, 255, 255),
            handle_border_width: 0.0,
            handle_radius: 9.0,

            size: vec2(220.0, 24.0),
            track_thickness: 6.0,

            scale: vec2(1.0, 1.0),
            rotation: 0.0,

            rounding: 0.0,
        }
    }
}

impl SliderStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = color;
        self
    }

    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = color;
        self
    }

    pub fn handle_color(mut self, color: Color) -> Self {
        self.handle_color = color;
        self
    }

    pub fn handle_border(mut self, color: Color, width: f32) -> Self {
        self.handle_border_color = color;
        self.handle_border_width = width;
        self
    }

    pub fn handle_radius(mut self, radius: f32) -> Self {
        self.handle_radius = radius;
        self
    }

    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    pub fn track_thickness(mut self, thickness: f32) -> Self {
        self.track_thickness = thickness;
        self
    }

    pub fn scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    pub fn rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn rounding(mut self, rounding: f32) -> Self {
        self.rounding = rounding;
        self
    }
}

impl Lerp for SliderStyle {
    fn lerp_towards(&self, other: &Self, t: f32) -> Self {
        Self {
            track_color: self.track_color.lerp_towards(&other.track_color, t),
            fill_color: self.fill_color.lerp_towards(&other.fill_color, t),

            handle_color: self.handle_color.lerp_towards(&other.handle_color, t),
            handle_border_color: self
                .handle_border_color
                .lerp_towards(&other.handle_border_color, t),
            handle_border_width: self
                .handle_border_width
                .lerp_towards(&other.handle_border_width, t),
            handle_radius: self.handle_radius.lerp_towards(&other.handle_radius, t),

            size: self.size.lerp_towards(&other.size, t),
            track_thickness: self.track_thickness.lerp_towards(&other.track_thickness, t),

            scale: self.scale.lerp_towards(&other.scale, t),
            rotation: self.rotation.lerp_towards(&other.rotation, t),

            rounding: self.rounding.lerp_towards(&other.rounding, t),
        }
    }
}

// ============================================================
// SLIDER
// ============================================================

pub struct Slider {
    position: Vec2,

    pub normal: SliderStyle,
    pub hover: SliderStyle,
    pub pressed: SliderStyle,
    pub focused: SliderStyle,
    pub disabled: SliderStyle,

    current: SliderStyle,

    min: f32,
    max: f32,
    value: f32,
    /// Snaps the value to multiples of `step` away from `min`.
    /// `0.0` means continuous (no snapping).
    step: f32,

    orientation: SliderOrientation,

    dragging: bool,
    is_focused: bool,
    /// True for the frame the value last changed.
    changed: bool,

    speed: f32,
    enabled: bool,
}

impl Slider {
    // ========================================================
    // CONSTRUCTOR
    // ========================================================

    pub fn new(position: Vec2, min: f32, max: f32) -> Self {
        let normal = SliderStyle::default();

        Self {
            position,

            normal: normal.clone(),
            hover: normal.clone(),
            pressed: normal.clone(),
            focused: normal.clone(),
            disabled: normal.clone(),

            current: normal,

            min,
            max,
            value: min,
            step: 0.0,

            orientation: SliderOrientation::Horizontal,

            dragging: false,
            is_focused: false,
            changed: false,

            speed: 20.0,
            enabled: true,
        }
    }

    // ========================================================
    // STYLE
    // ========================================================

    pub fn normal(mut self, style: SliderStyle) -> Self {
        self.normal = style.clone();
        self.current = style;
        self
    }

    pub fn hover(mut self, style: SliderStyle) -> Self {
        self.hover = style;
        self
    }

    pub fn pressed(mut self, style: SliderStyle) -> Self {
        self.pressed = style;
        self
    }

    pub fn focused(mut self, style: SliderStyle) -> Self {
        self.focused = style;
        self
    }

    pub fn disabled(mut self, style: SliderStyle) -> Self {
        self.disabled = style;
        self
    }

    // ========================================================
    // CONFIG
    // ========================================================

    pub fn value(mut self, value: f32) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = step.max(0.0);
        self
    }

    pub fn orientation(mut self, orientation: SliderOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn position(mut self, position: Vec2) -> Self {
        self.position = position;
        self
    }

    // ========================================================
    // QUERIES
    // ========================================================

    pub fn get(&self) -> f32 {
        self.value
    }

    /// True for the single frame the value last changed.
    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    pub fn is_dragging(&self) -> bool {
        self.dragging
    }

    // ========================================================
    // FOCUS / VALUE CONTROL
    // ========================================================

    pub fn focus(&mut self) {
        self.is_focused = true;
    }

    pub fn unfocus(&mut self) {
        self.is_focused = false;
        self.dragging = false;
    }

    pub fn set_value(&mut self, value: f32) {
        let stepped = if self.step > 0.0 {
            self.min + ((value - self.min) / self.step).round() * self.step
        } else {
            value
        };
        let clamped = stepped.clamp(self.min, self.max);

        if (clamped - self.value).abs() > f32::EPSILON {
            self.value = clamped;
            self.changed = true;
        }
    }

    // ========================================================
    // TARGET STYLE
    // ========================================================

    fn target_style(&self) -> &SliderStyle {
        match self.state() {
            WidgetState::Normal => &self.normal,
            WidgetState::Hover => &self.hover,
            WidgetState::Pressed => &self.pressed,
            WidgetState::Focused => &self.focused,
            WidgetState::Disabled => &self.disabled,
        }
    }

    // ========================================================
    // GEOMETRY / VALUE MAPPING
    // ========================================================

    fn normalized(&self) -> f32 {
        if self.max <= self.min {
            0.0
        } else {
            (self.value - self.min) / (self.max - self.min)
        }
    }

    /// Center of the handle in screen space for the current value.
    fn handle_center(&self) -> Vec2 {
        let style = &self.current;
        let size = style.size * style.scale;
        let t = self.normalized();

        match self.orientation {
            SliderOrientation::Horizontal => {
                let start_x = self.position.x - size.x * 0.5 + style.handle_radius;
                let usable = (size.x - style.handle_radius * 2.0).max(1.0);
                vec2(start_x + usable * t, self.position.y)
            }
            SliderOrientation::Vertical => {
                // Top of the track is `max`, bottom is `min` — matches
                // the usual vertical-slider convention (e.g. volume).
                let start_y = self.position.y - size.y * 0.5 + style.handle_radius;
                let usable = (size.y - style.handle_radius * 2.0).max(1.0);
                vec2(self.position.x, start_y + usable * (1.0 - t))
            }
        }
    }

    fn value_from_mouse(&self) -> f32 {
        let style = &self.current;
        let size = style.size * style.scale;
        let (mouse_x, mouse_y) = mouse_position();

        let t = match self.orientation {
            SliderOrientation::Horizontal => {
                let start_x = self.position.x - size.x * 0.5 + style.handle_radius;
                let usable = (size.x - style.handle_radius * 2.0).max(1.0);
                ((mouse_x - start_x) / usable).clamp(0.0, 1.0)
            }
            SliderOrientation::Vertical => {
                let start_y = self.position.y - size.y * 0.5 + style.handle_radius;
                let usable = (size.y - style.handle_radius * 2.0).max(1.0);
                1.0 - ((mouse_y - start_y) / usable).clamp(0.0, 1.0)
            }
        };

        self.min + t * (self.max - self.min)
    }

    fn nudge(&mut self, steps: f32) {
        let unit = if self.step > 0.0 {
            self.step
        } else {
            (self.max - self.min) * 0.01
        };
        self.set_value(self.value + steps * unit);
    }

    // ========================================================
    // INPUT HANDLING
    // ========================================================

    fn handle_pointer(&mut self) {
        if !self.enabled {
            return;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            if self.mouse_inside() {
                self.focus();
                self.dragging = true;
                let v = self.value_from_mouse();
                self.set_value(v);
            } else {
                self.unfocus();
            }
        }

        if self.dragging {
            if is_mouse_button_down(MouseButton::Left) {
                let v = self.value_from_mouse();
                self.set_value(v);
            } else {
                self.dragging = false;
            }
        }
    }

    fn handle_keyboard(&mut self) {
        if !self.is_focused || !self.enabled {
            return;
        }

        // Right/Up increase, Left/Down decrease — the same mapping
        // browsers use for <input type="range">, regardless of
        // whether the slider is drawn horizontally or vertically.
        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::Up) {
            self.nudge(1.0);
        }
        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Down) {
            self.nudge(-1.0);
        }
        if is_key_pressed(KeyCode::PageUp) {
            self.nudge(10.0);
        }
        if is_key_pressed(KeyCode::PageDown) {
            self.nudge(-10.0);
        }
        if is_key_pressed(KeyCode::Home) {
            self.set_value(self.min);
        }
        if is_key_pressed(KeyCode::End) {
            self.set_value(self.max);
        }
    }

    fn handle_wheel(&mut self) {
        if !self.enabled || !self.hovered() {
            return;
        }

        let (_, wheel_y) = mouse_wheel();
        if wheel_y != 0.0 {
            self.nudge(wheel_y.signum());
        }
    }
}

// ============================================================
// WIDGET IMPL
// ============================================================

impl Widget for Slider {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn hitbox(&self) -> Rect {
        let size = self.normal.size;

        Rect::new(
            self.position.x - size.x * 0.5,
            self.position.y - size.y * 0.5,
            size.x,
            size.y,
        )
    }

    fn state(&self) -> WidgetState {
        if !self.enabled {
            WidgetState::Disabled
        } else if self.dragging {
            WidgetState::Pressed
        } else if self.is_focused {
            WidgetState::Focused
        } else if self.hovered() {
            WidgetState::Hover
        } else {
            WidgetState::Normal
        }
    }

    fn update(&mut self) {
        self.changed = false;

        self.handle_pointer();
        self.handle_keyboard();
        self.handle_wheel();

        let dt = get_frame_time();
        let t = smoothing_factor(self.speed, dt);
        let target = self.target_style().clone();
        self.current = self.current.lerp_towards(&target, t);
    }

    fn draw(&self) {
        let style = &self.current;
        let size = style.size * style.scale;
        let handle = self.handle_center();

        match self.orientation {
            SliderOrientation::Horizontal => {
                let track_y = self.position.y;
                let track_left = self.position.x - size.x * 0.5 + style.handle_radius;
                let track_right = self.position.x + size.x * 0.5 - style.handle_radius;

                // Background track.
                draw_rectangle_ex(
                    self.position.x,
                    track_y,
                    track_right - track_left,
                    style.track_thickness,
                    DrawRectangleParams {
                        color: style.track_color,
                        rotation: style.rotation,
                        offset: vec2(0.5, 0.5),
                    },
                );

                // Filled portion up to the handle.
                let fill_width = (handle.x - track_left).max(0.0);
                if fill_width > 0.0 {
                    draw_rectangle_ex(
                        track_left + fill_width * 0.5,
                        track_y,
                        fill_width,
                        style.track_thickness,
                        DrawRectangleParams {
                            color: style.fill_color,
                            rotation: style.rotation,
                            offset: vec2(0.5, 0.5),
                        },
                    );
                }
            }
            SliderOrientation::Vertical => {
                let track_x = self.position.x;
                let track_top = self.position.y - size.y * 0.5 + style.handle_radius;
                let track_bottom = self.position.y + size.y * 0.5 - style.handle_radius;

                draw_rectangle_ex(
                    track_x,
                    self.position.y,
                    style.track_thickness,
                    track_bottom - track_top,
                    DrawRectangleParams {
                        color: style.track_color,
                        rotation: style.rotation,
                        offset: vec2(0.5, 0.5),
                    },
                );

                // Filled portion from the handle down to the bottom
                // (bottom = min, so the fill grows upward as the value
                // increases — matches the top-is-max convention above).
                let fill_height = (track_bottom - handle.y).max(0.0);
                if fill_height > 0.0 {
                    draw_rectangle_ex(
                        track_x,
                        track_bottom - fill_height * 0.5,
                        style.track_thickness,
                        fill_height,
                        DrawRectangleParams {
                            color: style.fill_color,
                            rotation: style.rotation,
                            offset: vec2(0.5, 0.5),
                        },
                    );
                }
            }
        }

        // Handle.
        draw_circle(handle.x, handle.y, style.handle_radius, style.handle_color);
        if style.handle_border_width > 0.0 {
            draw_circle_lines(
                handle.x,
                handle.y,
                style.handle_radius,
                style.handle_border_width,
                style.handle_border_color,
            );
        }
    }
}
