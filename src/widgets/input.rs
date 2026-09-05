// src/widgets/input.rs
//
// A single-line text input that behaves the way a native text field
// does: click to focus and place the caret, drag to select, type to
// insert, Backspace/Delete to remove, arrow keys (+ Shift) to move
// the caret and extend the selection, Home/End to jump to the edges,
// Ctrl+A/C/X/V for select-all/copy/cut/paste, Enter to submit, and a
// blinking caret while focused. Text scrolls horizontally so the
// caret always stays visible once it outgrows the box.
//
// It does not clip overflowing text against the box edges — see the
// note above `draw()` if you need that.

use macroquad::prelude::*;

use crate::anim::{Lerp, smoothing_factor};
use crate::widgets::widget::{Widget, WidgetState};

// ============================================================
// TEXT INPUT STYLE
// ============================================================

#[derive(Clone)]
pub struct TextInputStyle {
    pub color: Color,
    pub text_color: Color,
    pub placeholder_color: Color,
    pub cursor_color: Color,
    pub selection_color: Color,

    pub size: Vec2,
    pub scale: Vec2,
    pub rotation: f32,

    pub font: Option<Font>,
    pub font_size: u16,

    pub border_color: Color,
    pub border_width: f32,

    pub rounding: f32,

    /// Horizontal inset between the box edge and the text.
    pub padding: f32,
}

impl Default for TextInputStyle {
    fn default() -> Self {
        Self {
            color: Color::from_rgba(30, 30, 34, 255),
            text_color: WHITE,
            placeholder_color: Color::from_rgba(130, 130, 135, 255),
            cursor_color: WHITE,
            selection_color: Color::from_rgba(80, 120, 220, 120),

            size: vec2(240.0, 48.0),
            scale: vec2(1.0, 1.0),
            rotation: 0.0,

            font: None,
            font_size: 22,

            border_color: Color::from_rgba(80, 80, 90, 255),
            border_width: 1.0,

            rounding: 0.0,

            padding: 10.0,
        }
    }
}

impl TextInputStyle {
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

    pub fn placeholder_color(mut self, color: Color) -> Self {
        self.placeholder_color = color;
        self
    }

    pub fn cursor_color(mut self, color: Color) -> Self {
        self.cursor_color = color;
        self
    }

    pub fn selection_color(mut self, color: Color) -> Self {
        self.selection_color = color;
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

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }
}

impl Lerp for TextInputStyle {
    fn lerp_towards(&self, other: &Self, t: f32) -> Self {
        Self {
            color: self.color.lerp_towards(&other.color, t),
            text_color: self.text_color.lerp_towards(&other.text_color, t),
            placeholder_color: self
                .placeholder_color
                .lerp_towards(&other.placeholder_color, t),
            cursor_color: self.cursor_color.lerp_towards(&other.cursor_color, t),
            selection_color: self.selection_color.lerp_towards(&other.selection_color, t),

            size: self.size.lerp_towards(&other.size, t),
            scale: self.scale.lerp_towards(&other.scale, t),
            rotation: self.rotation.lerp_towards(&other.rotation, t),

            // Fonts can't be blended — snap once we're past the halfway
            // point, same convention as ButtonStyle.
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
            padding: self.padding.lerp_towards(&other.padding, t),
        }
    }
}

// ============================================================
// TEXT INPUT
// ============================================================

const BLINK_PERIOD: f32 = 1.0; // seconds for a full on/off cycle

pub struct TextInput {
    text: String,
    placeholder: String,
    position: Vec2,

    pub normal: TextInputStyle,
    pub hover: TextInputStyle,
    pub focused: TextInputStyle,
    pub disabled: TextInputStyle,

    current: TextInputStyle,

    /// Caret position, in *characters* (not bytes) from the start of `text`.
    cursor: usize,
    /// Other end of the selection, if any is active. `None` means no selection.
    selection_anchor: Option<usize>,

    is_focused: bool,
    dragging: bool,

    /// True for the frame after the text changed (typed, pasted, deleted...).
    changed: bool,
    /// True for the frame Enter was pressed while focused.
    submitted: bool,

    blink_timer: f32,
    scroll_x: f32,

    max_len: Option<usize>,

    speed: f32,
    enabled: bool,
}

impl TextInput {
    // ========================================================
    // CONSTRUCTOR
    // ========================================================

    pub fn new(position: Vec2) -> Self {
        let normal = TextInputStyle::default();

        Self {
            text: String::new(),
            placeholder: String::new(),
            position,

            normal: normal.clone(),
            hover: normal.clone(),
            focused: normal.clone(),
            disabled: normal.clone(),

            current: normal,

            cursor: 0,
            selection_anchor: None,

            is_focused: false,
            dragging: false,

            changed: false,
            submitted: false,

            blink_timer: 0.0,
            scroll_x: 0.0,

            max_len: None,

            speed: 16.0,
            enabled: true,
        }
    }

    // ========================================================
    // STYLE
    // ========================================================

    pub fn normal(mut self, style: TextInputStyle) -> Self {
        self.normal = style.clone();
        self.current = style;
        self
    }

    pub fn hover(mut self, style: TextInputStyle) -> Self {
        self.hover = style;
        self
    }

    pub fn focused(mut self, style: TextInputStyle) -> Self {
        self.focused = style;
        self
    }

    pub fn disabled(mut self, style: TextInputStyle) -> Self {
        self.disabled = style;
        self
    }

    /// Sets the font on every state style at once (see `Button::font`
    /// for why this exists instead of setting it per-style).
    pub fn font(mut self, font: Font) -> Self {
        self.normal.font = Some(font.clone());
        self.hover.font = Some(font.clone());
        self.focused.font = Some(font.clone());
        self.disabled.font = Some(font.clone());
        self.current.font = Some(font);
        self
    }

    pub fn font_size(mut self, font_size: u16) -> Self {
        self.normal.font_size = font_size;
        self.hover.font_size = font_size;
        self.focused.font_size = font_size;
        self.disabled.font_size = font_size;
        self.current.font_size = font_size;
        self
    }

    // ========================================================
    // CONFIG
    // ========================================================

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Sets the initial text content.
    pub fn value(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self.cursor = self.text.chars().count();
        self
    }

    /// Caps the number of characters that can be typed or pasted in.
    pub fn max_len(mut self, max_len: usize) -> Self {
        self.max_len = Some(max_len);
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

    pub fn text(&self) -> &str {
        &self.text
    }

    /// True for the single frame after the text changed.
    pub fn changed(&self) -> bool {
        self.changed
    }

    /// True for the single frame Enter was pressed while focused.
    pub fn submitted(&self) -> bool {
        self.submitted
    }

    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    // ========================================================
    // FOCUS CONTROL
    // ========================================================

    pub fn focus(&mut self) {
        self.is_focused = true;
        self.blink_timer = 0.0;
    }

    pub fn unfocus(&mut self) {
        self.is_focused = false;
        self.dragging = false;
        self.selection_anchor = None;
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.cursor = self.cursor.min(self.text.chars().count());
        self.selection_anchor = None;
        self.changed = true;
    }

    pub fn clear(&mut self) {
        self.set_text(String::new());
    }

    // ========================================================
    // TARGET STYLE
    // ========================================================

    fn target_style(&self) -> &TextInputStyle {
        match self.state() {
            WidgetState::Normal => &self.normal,
            WidgetState::Hover => &self.hover,
            WidgetState::Focused => &self.focused,
            // TextInput has no "held down" look distinct from focused;
            // fall back to normal so the match stays exhaustive.
            WidgetState::Pressed => &self.normal,
            WidgetState::Disabled => &self.disabled,
        }
    }

    // ========================================================
    // TEXT / CURSOR HELPERS
    // ========================================================

    fn char_count(&self) -> usize {
        self.text.chars().count()
    }

    fn byte_offset(&self, char_idx: usize) -> usize {
        self.text
            .char_indices()
            .nth(char_idx)
            .map(|(b, _)| b)
            .unwrap_or(self.text.len())
    }

    fn selection_range(&self) -> Option<(usize, usize)> {
        self.selection_anchor.map(|anchor| {
            if anchor < self.cursor {
                (anchor, self.cursor)
            } else {
                (self.cursor, anchor)
            }
        })
    }

    fn text_width(&self, up_to_char: usize) -> f32 {
        let byte_idx = self.byte_offset(up_to_char);
        measure_text(
            &self.text[..byte_idx],
            self.current.font.as_ref(),
            self.current.font_size,
            1.0,
        )
        .width
    }

    /// Maps a horizontal offset (relative to the start of the text, in
    /// the same units as `text_width`) to the nearest character index.
    fn char_index_for_x(&self, x: f32) -> usize {
        let len = self.char_count();
        if len == 0 {
            return 0;
        }

        let mut best_idx = 0;
        let mut best_dist = f32::MAX;

        for idx in 0..=len {
            let w = self.text_width(idx);
            let dist = (w - x).abs();
            if dist < best_dist {
                best_dist = dist;
                best_idx = idx;
            }
        }

        best_idx
    }

    fn cursor_index_for_mouse(&self) -> usize {
        let (mouse_x, _) = mouse_position();
        let box_left = self.position.x - self.current.size.x * 0.5;
        let text_start_x = box_left + self.current.padding;
        let local_x = (mouse_x - text_start_x + self.scroll_x).max(0.0);
        self.char_index_for_x(local_x)
    }

    // ------------------------------------------------------------
    // Editing
    // ------------------------------------------------------------

    fn replace_selection_or_cursor_with(&mut self, insert: &str) {
        let (start, end) = self.selection_range().unwrap_or((self.cursor, self.cursor));
        let start_b = self.byte_offset(start);
        let end_b = self.byte_offset(end);

        self.text.replace_range(start_b..end_b, insert);
        self.cursor = start + insert.chars().count();
        self.selection_anchor = None;

        self.changed = true;
        self.blink_timer = 0.0;
    }

    fn insert_char(&mut self, c: char) {
        if let Some(max) = self.max_len {
            let selecting = self.selection_anchor.is_some();
            if self.char_count() >= max && !selecting {
                return;
            }
        }
        self.replace_selection_or_cursor_with(&c.to_string());
    }

    fn backspace(&mut self) {
        if self.selection_anchor.is_some() {
            self.replace_selection_or_cursor_with("");
            return;
        }
        if self.cursor == 0 {
            return;
        }
        let start_b = self.byte_offset(self.cursor - 1);
        let end_b = self.byte_offset(self.cursor);
        self.text.replace_range(start_b..end_b, "");
        self.cursor -= 1;
        self.changed = true;
        self.blink_timer = 0.0;
    }

    fn delete_forward(&mut self) {
        if self.selection_anchor.is_some() {
            self.replace_selection_or_cursor_with("");
            return;
        }
        if self.cursor >= self.char_count() {
            return;
        }
        let start_b = self.byte_offset(self.cursor);
        let end_b = self.byte_offset(self.cursor + 1);
        self.text.replace_range(start_b..end_b, "");
        self.changed = true;
        self.blink_timer = 0.0;
    }

    fn move_cursor(&mut self, delta: isize, shift: bool) {
        if !shift && self.selection_anchor.is_some() {
            let (start, end) = self.selection_range().unwrap();
            self.cursor = if delta < 0 { start } else { end };
            self.selection_anchor = None;
            self.blink_timer = 0.0;
            return;
        }

        if shift && self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.cursor);
        }
        if !shift {
            self.selection_anchor = None;
        }

        let len = self.char_count() as isize;
        let new_cursor = (self.cursor as isize + delta).clamp(0, len);
        self.cursor = new_cursor as usize;
        self.blink_timer = 0.0;
    }

    fn move_cursor_to(&mut self, idx: usize, shift: bool) {
        if shift && self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.cursor);
        } else if !shift {
            self.selection_anchor = None;
        }
        self.cursor = idx.min(self.char_count());
        self.blink_timer = 0.0;
    }

    fn select_all(&mut self) {
        let len = self.char_count();
        if len == 0 {
            return;
        }
        self.selection_anchor = Some(0);
        self.cursor = len;
    }

    fn selected_text(&self) -> Option<String> {
        self.selection_range().map(|(start, end)| {
            let start_b = self.byte_offset(start);
            let end_b = self.byte_offset(end);
            self.text[start_b..end_b].to_string()
        })
    }

    fn copy_selection(&self) {
        if let Some(text) = self.selected_text() {
            miniquad::window::clipboard_set(&text);
        }
    }

    fn cut_selection(&mut self) {
        if self.selected_text().is_some() {
            self.copy_selection();
            self.replace_selection_or_cursor_with("");
        }
    }

    fn paste_clipboard(&mut self) {
        if let Some(mut text) = miniquad::window::clipboard_get() {
            text.retain(|c| !c.is_control() || c == ' ');
            if let Some(max) = self.max_len {
                let remaining = max.saturating_sub(self.char_count());
                let allowed: String = text.chars().take(remaining).collect();
                if !allowed.is_empty() {
                    self.replace_selection_or_cursor_with(&allowed);
                }
            } else if !text.is_empty() {
                self.replace_selection_or_cursor_with(&text);
            }
        }
    }

    // ------------------------------------------------------------
    // Input handling
    // ------------------------------------------------------------

    fn handle_pointer(&mut self) {
        if !self.enabled {
            return;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            if self.mouse_inside() {
                self.focus();
                self.dragging = true;
                let idx = self.cursor_index_for_mouse();
                self.cursor = idx;
                self.selection_anchor = Some(idx);
            } else {
                self.unfocus();
            }
        }

        if self.dragging {
            if is_mouse_button_down(MouseButton::Left) {
                self.cursor = self.cursor_index_for_mouse();
            } else {
                self.dragging = false;
                if self.selection_anchor == Some(self.cursor) {
                    self.selection_anchor = None;
                }
            }
        }
    }

    fn handle_keyboard(&mut self) {
        if !self.is_focused || !self.enabled {
            return;
        }

        let ctrl = is_key_down(KeyCode::LeftControl)
            || is_key_down(KeyCode::RightControl)
            || is_key_down(KeyCode::LeftSuper)
            || is_key_down(KeyCode::RightSuper);
        let shift = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);

        if ctrl && is_key_pressed(KeyCode::A) {
            self.select_all();
        } else if ctrl && is_key_pressed(KeyCode::C) {
            self.copy_selection();
        } else if ctrl && is_key_pressed(KeyCode::X) {
            self.cut_selection();
        } else if ctrl && is_key_pressed(KeyCode::V) {
            self.paste_clipboard();
        }

        // Typed characters. macroquad queues one `char_pressed` event
        // per character per frame; drain it fully in case several
        // arrived (e.g. fast typing, IME input).
        while let Some(c) = get_char_pressed() {
            if ctrl || c.is_control() {
                continue;
            }
            self.insert_char(c);
        }

        if is_key_pressed(KeyCode::Backspace) {
            self.backspace();
        }
        if is_key_pressed(KeyCode::Delete) {
            self.delete_forward();
        }
        if is_key_pressed(KeyCode::Left) {
            self.move_cursor(-1, shift);
        }
        if is_key_pressed(KeyCode::Right) {
            self.move_cursor(1, shift);
        }
        if is_key_pressed(KeyCode::Home) {
            self.move_cursor_to(0, shift);
        }
        if is_key_pressed(KeyCode::End) {
            let len = self.char_count();
            self.move_cursor_to(len, shift);
        }
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
            self.submitted = true;
        }
    }

    fn update_scroll(&mut self) {
        let visible_width = (self.current.size.x - self.current.padding * 2.0).max(0.0);
        let cursor_x = self.text_width(self.cursor);

        if cursor_x - self.scroll_x > visible_width {
            self.scroll_x = cursor_x - visible_width;
        }
        if cursor_x - self.scroll_x < 0.0 {
            self.scroll_x = cursor_x;
        }
        self.scroll_x = self.scroll_x.max(0.0);
    }
}

// ============================================================
// WIDGET IMPL
// ============================================================

impl Widget for TextInput {
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
        self.submitted = false;

        self.handle_pointer();
        self.handle_keyboard();
        self.update_scroll();

        if self.is_focused {
            self.blink_timer = (self.blink_timer + get_frame_time()) % BLINK_PERIOD;
        }

        let dt = get_frame_time();
        let t = smoothing_factor(self.speed, dt);
        let target = self.target_style().clone();
        self.current = self.current.lerp_towards(&target, t);
    }

    fn draw(&self) {
        let style = &self.current;

        let size = style.size * style.scale;
        let center = self.position;

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

        if style.border_width > 0.0 {
            draw_rectangle_lines_ex(
                center.x,
                center.y,
                size.x,
                size.y,
                style.border_width,
                DrawRectangleParams {
                    color: style.border_color,
                    rotation: style.rotation,
                    offset: vec2(0.5, 0.5),
                },
            );
        }

        // ----------------------------------------------------
        // TEXT AREA
        // ----------------------------------------------------
        //
        // NOTE: this does not clip against the box edges — text that
        // outgrows the visible width will scroll (via `scroll_x`) but
        // may still draw a sliver past the border on the frame it
        // catches up. Add a scissor/render-target clip here if that
        // matters for your use case.

        let box_left = center.x - size.x * 0.5;
        let text_start_x = box_left + style.padding - self.scroll_x;

        let line_height = measure_text("Hg", style.font.as_ref(), style.font_size, 1.0).height;
        let text_y = center.y + line_height * 0.5;

        // Selection highlight, drawn behind the text.
        if let Some((start, end)) = self.selection_range() {
            let x0 = text_start_x + self.text_width(start);
            let x1 = text_start_x + self.text_width(end);
            draw_rectangle(
                x0,
                center.y - size.y * 0.5 + style.border_width,
                (x1 - x0).max(1.0),
                size.y - style.border_width * 2.0,
                style.selection_color,
            );
        }

        if self.text.is_empty() && !self.is_focused {
            draw_text_ex(
                &self.placeholder,
                text_start_x,
                text_y,
                TextParams {
                    font: style.font.as_ref(),
                    font_size: style.font_size,
                    font_scale: 1.0,
                    color: style.placeholder_color,
                    rotation: style.rotation,
                    ..Default::default()
                },
            );
        } else {
            draw_text_ex(
                &self.text,
                text_start_x,
                text_y,
                TextParams {
                    font: style.font.as_ref(),
                    font_size: style.font_size,
                    font_scale: 1.0,
                    color: style.text_color,
                    rotation: style.rotation,
                    ..Default::default()
                },
            );
        }

        // Blinking caret.
        if self.is_focused && self.blink_timer < BLINK_PERIOD * 0.5 {
            let caret_x = text_start_x + self.text_width(self.cursor);
            draw_rectangle(
                caret_x,
                center.y - line_height * 0.5,
                1.5,
                line_height,
                style.cursor_color,
            );
        }
    }
}
