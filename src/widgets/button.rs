use macroquad::prelude::*;

use crate::anim::{Lerp, smoothing_factor};
use crate::widgets::widget::{VirtualResolution, Widget, WidgetState};

// ============================================================
// BUTTON STYLE
// ============================================================

#[derive(Clone)]
pub struct ButtonStyle {
    pub color: Color,
    pub text_color: Color,

    pub size: Vec2,
    pub scale: Vec2,

    pub rotation: f32,

    pub font: Option<Font>,
    pub font_size: u16,

    pub border_color: Color,
    pub border_width: f32,

    pub rounding: f32,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            color: Color::from_rgba(40, 40, 45, 255),
            text_color: WHITE,

            size: vec2(200.0, 60.0),
            scale: vec2(1.0, 1.0),

            rotation: 0.0,

            font: None,
            font_size: 24,

            border_color: Color::from_rgba(80, 80, 90, 255),
            border_width: 0.0,

            rounding: 0.0,
        }
    }
}

impl ButtonStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
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

    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn font_size(mut self, font_size: u16) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn border(mut self, color: Color, width: f32) -> Self {
        self.border_color = color;
        self.border_width = width;
        self
    }

    pub fn rounding(mut self, rounding: f32) -> Self {
        self.rounding = rounding;
        self
    }
}

impl Lerp for ButtonStyle {
    fn lerp_towards(&self, other: &Self, t: f32) -> Self {
        Self {
            color: self.color.lerp_towards(&other.color, t),
            text_color: self.text_color.lerp_towards(&other.text_color, t),

            size: self.size.lerp_towards(&other.size, t),
            scale: self.scale.lerp_towards(&other.scale, t),

            rotation: self.rotation.lerp_towards(&other.rotation, t),

            font: if t > 0.5 {
                other.font.clone()
            } else {
                self.font.clone()
            },

            font_size: if t > 0.5 {
                other.font_size
            } else {
                self.font_size
            },

            border_color: self.border_color.lerp_towards(&other.border_color, t),

            border_width: self.border_width.lerp_towards(&other.border_width, t),

            rounding: self.rounding.lerp_towards(&other.rounding, t),
        }
    }
}

// ============================================================
// BUTTON
// ============================================================

pub struct Button {
    pub text: String,
    pub position: Vec2,

    pub normal: ButtonStyle,
    pub hover: ButtonStyle,
    pub pressed: ButtonStyle,
    pub disabled: ButtonStyle,

    current: ButtonStyle,

    speed: f32,
    enabled: bool,
}

impl Button {
    pub fn new(text: impl Into<String>, position: Vec2) -> Self {
        let normal = ButtonStyle::default();

        Self {
            text: text.into(),
            position,

            normal: normal.clone(),
            hover: normal.clone(),
            pressed: normal.clone(),
            disabled: normal.clone(),

            current: normal,

            speed: 12.0,
            enabled: true,
        }
    }

    // ========================================================
    // STYLE
    // ========================================================

    pub fn normal(mut self, style: ButtonStyle) -> Self {
        self.normal = style.clone();
        self.current = style;
        self
    }

    pub fn hover(mut self, style: ButtonStyle) -> Self {
        self.hover = style;
        self
    }

    pub fn pressed(mut self, style: ButtonStyle) -> Self {
        self.pressed = style;
        self
    }

    pub fn disabled(mut self, style: ButtonStyle) -> Self {
        self.disabled = style;
        self
    }

    // ========================================================
    // CONFIG
    // ========================================================

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
    // FONT
    // ========================================================

    pub fn font(mut self, font: Font) -> Self {
        self.normal.font = Some(font.clone());
        self.hover.font = Some(font.clone());
        self.pressed.font = Some(font.clone());
        self.disabled.font = Some(font.clone());
        self.current.font = Some(font);

        self
    }

    pub fn font_size(mut self, font_size: u16) -> Self {
        self.normal.font_size = font_size;
        self.hover.font_size = font_size;
        self.pressed.font_size = font_size;
        self.disabled.font_size = font_size;
        self.current.font_size = font_size;

        self
    }

    // ========================================================
    // TARGET STYLE
    // ========================================================

    fn target_style(&self) -> &ButtonStyle {
        match self.state() {
            WidgetState::Normal => &self.normal,
            WidgetState::Hover => &self.hover,
            WidgetState::Pressed => &self.pressed,
            WidgetState::Disabled => &self.disabled,
            WidgetState::Focused => &self.normal,
        }
    }
}

// ============================================================
// WIDGET
// ============================================================

impl Widget for Button {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// All button coordinates live in virtual space.
    fn virtual_resolution(&self) -> VirtualResolution {
        VirtualResolution::default()
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

    fn update(&mut self) {
        let dt = get_frame_time();
        let t = smoothing_factor(self.speed, dt);

        let target = self.target_style().clone();

        self.current = self.current.lerp_towards(&target, t);
    }

    fn draw(&self) {
        let style = &self.current;

        let virtual_resolution = self.virtual_resolution();

        let scale = virtual_resolution.scale();

        let viewport = virtual_resolution.viewport();

        // Convert virtual coordinates to real screen coordinates.
        let center = vec2(
            viewport.x + self.position.x * scale,
            viewport.y + self.position.y * scale,
        );

        let size = style.size * style.scale * scale;

        // ----------------------------------------------------
        // BACKGROUND
        // ----------------------------------------------------

        draw_rectangle_ex(
            center.x,
            center.y,
            size.x,
            size.y,
            DrawRectangleParams {
                color: style.color,
                rotation: style.rotation,
                offset: vec2(0.5, 0.5),
            },
        );

        // ----------------------------------------------------
        // BORDER
        // ----------------------------------------------------

        if style.border_width > 0.0 {
            draw_rectangle_lines_ex(
                center.x,
                center.y,
                size.x,
                size.y,
                style.border_width * scale,
                DrawRectangleParams {
                    color: style.border_color,
                    rotation: style.rotation,
                    offset: vec2(0.5, 0.5),
                },
            );
        }

        // ----------------------------------------------------
        // TEXT
        // ----------------------------------------------------

        let font_size = (style.font_size as f32 * scale).round().max(1.0) as u16;

        let dimensions = match &style.font {
            Some(font) => measure_text(&self.text, Some(font), font_size, 1.0),

            None => measure_text(&self.text, None, font_size, 1.0),
        };

        let text_x = center.x - dimensions.width * 0.5;
        let text_y = center.y + dimensions.height * 0.5;

        draw_text_ex(
            &self.text,
            text_x,
            text_y,
            TextParams {
                font: style.font.as_ref(),
                font_size,
                font_scale: 1.0,
                color: style.text_color,
                rotation: style.rotation,
                ..Default::default()
            },
        );
    }
}
