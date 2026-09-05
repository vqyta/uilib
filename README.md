# uilib

A small, lightweight and highly customizable UI widget toolkit for [Macroquad](https://macroquad.rs/) written in Rust.

`uilib` provides interactive widgets with per-state styling, smooth animations, keyboard and mouse input handling, and a simple builder-style API.

## Features

* Lightweight and simple API
* Built specifically for Macroquad
* Animated widget state transitions
* Per-state styling
* Builder-style configuration
* Mouse interaction
* Keyboard interaction
* Keyboard focus support
* Clipboard support
* Text selection
* Horizontal and vertical sliders
* Custom fonts
* No external UI layout system
* No retained widget tree required

## Widgets

Currently available:

* `Button`
* `TextInput`
* `Slider`

More widgets can be added without changing the core widget interface.

---

## Installation

Add `uilib` to your `Cargo.toml`:

```toml
[dependencies]
macroquad = "0.4"
uilib = { git = "https://github.com/vqyta/uilib" }
```



---

# Basic Usage

Import the widgets directly from the crate root:

```rust
use macroquad::prelude::*;
use uilib::{Button, Widget};

#[macroquad::main("uilib")]
async fn main() {
    let mut button = Button::new("Click me", vec2(400.0, 300.0));

    loop {
        clear_background(BLACK);

        button.update();
        button.draw();

        next_frame().await;
    }
}
```

For interactive widgets, call:

```rust
widget.update();
widget.draw();
```

once per frame.

`update()` handles input and animation, while `draw()` renders the current widget state.

---

# Button

`Button` is a simple clickable widget with four style states:

* Normal
* Hover
* Pressed
* Disabled

## Creating a Button

```rust
let mut button = Button::new(
    "Play",
    vec2(400.0, 300.0),
);
```

## Checking Clicks

Because `Button` implements `Widget`, you can use:

```rust
if button.clicked() {
    println!("Button clicked!");
}
```

Example:

```rust
loop {
    clear_background(BLACK);

    button.update();

    if button.clicked() {
        println!("Clicked!");
    }

    button.draw();

    next_frame().await;
}
```

---

# Button Styling

`ButtonStyle` controls the visual appearance of a button.

```rust
let style = ButtonStyle::new()
    .color(DARKGRAY)
    .text_color(WHITE)
    .size(vec2(220.0, 60.0))
    .scale(vec2(1.0, 1.0))
    .rotation(0.0)
    .font_size(24)
    .border(WHITE, 2.0)
    .rounding(8.0);
```

Apply it to a state:

```rust
let mut button = Button::new("Play", vec2(400.0, 300.0))
    .normal(style);
```

You can define completely different styles for each state:

```rust
let mut button = Button::new("Play", vec2(400.0, 300.0))
    .normal(
        ButtonStyle::new()
            .color(DARKGRAY)
    )
    .hover(
        ButtonStyle::new()
            .color(GRAY)
            .scale(vec2(1.05, 1.05))
    )
    .pressed(
        ButtonStyle::new()
            .color(WHITE)
            .text_color(BLACK)
            .scale(vec2(0.95, 0.95))
    )
    .disabled(
        ButtonStyle::new()
            .color(Color::from_rgba(40, 40, 40, 255))
    );
```

The widget smoothly interpolates between these styles.

---

# Button Configuration

### Position

```rust
Button::new("Hello", vec2(400.0, 300.0))
    .position(vec2(500.0, 300.0));
```

### Animation Speed

```rust
Button::new("Hello", vec2(400.0, 300.0))
    .speed(12.0);
```

Higher values produce faster transitions.

A value of `0.0` or less makes the transition effectively instantaneous.

### Enable / Disable

```rust
Button::new("Hello", vec2(400.0, 300.0))
    .enabled(false);
```

### Font

```rust
let font = load_ttf_font("assets/font.ttf")
    .await
    .unwrap();

let mut button = Button::new("Hello", vec2(400.0, 300.0))
    .font(font)
    .font_size(28);
```

`.font()` and `.font_size()` apply the setting to every state style.

---

# TextInput

`TextInput` provides a single-line text field with native-style editing behavior.

Features include:

* Text input
* Cursor
* Blinking caret
* Mouse focus
* Mouse text selection
* Drag selection
* Arrow navigation
* Shift selection
* Home / End
* Backspace
* Delete
* Ctrl+A
* Ctrl+C
* Ctrl+X
* Ctrl+V
* Enter submission
* Clipboard integration
* Maximum character length
* Horizontal scrolling

## Creating a TextInput

```rust
let input = TextInput::new(vec2(400.0, 300.0));
```

## Placeholder

```rust
let input = TextInput::new(vec2(400.0, 300.0))
    .placeholder("Enter your name...");
```

## Initial Value

```rust
let input = TextInput::new(vec2(400.0, 300.0))
    .value("Hello");
```

## Reading Text

```rust
println!("{}", input.text());
```

The returned value is a `&str`.

---

# TextInput Styling

`TextInputStyle` supports:

```rust
let style = TextInputStyle::new()
    .color(Color::from_rgba(30, 30, 34, 255))
    .text_color(WHITE)
    .placeholder_color(GRAY)
    .cursor_color(WHITE)
    .selection_color(Color::from_rgba(80, 120, 220, 120))
    .size(vec2(300.0, 50.0))
    .scale(vec2(1.0, 1.0))
    .rotation(0.0)
    .font_size(22)
    .border(WHITE, 1.0)
    .rounding(4.0)
    .padding(10.0);
```

Text inputs have four visual states:

* Normal
* Hover
* Focused
* Disabled

Example:

```rust
let input = TextInput::new(vec2(400.0, 300.0))
    .normal(
        TextInputStyle::new()
            .color(DARKGRAY)
    )
    .hover(
        TextInputStyle::new()
            .color(GRAY)
    )
    .focused(
        TextInputStyle::new()
            .color(Color::from_rgba(35, 35, 45, 255))
            .border(BLUE, 2.0)
    );
```

---

# TextInput Events

## Detecting Changes

```rust
if input.changed() {
    println!("Text: {}", input.text());
}
```

`changed()` is `true` for the frame in which the text changes.

## Detecting Submission

```rust
if input.submitted() {
    println!("Submitted: {}", input.text());
}
```

Submission occurs when Enter is pressed while the input is focused.

## Focus

```rust
input.focus();
```

Remove focus:

```rust
input.unfocus();
```

Check focus:

```rust
if input.is_focused() {
    // ...
}
```

## Programmatically Changing Text

```rust
input.set_text("New value");
```

Clear the input:

```rust
input.clear();
```

---

# Maximum Length

Limit the number of characters:

```rust
let input = TextInput::new(vec2(400.0, 300.0))
    .max_len(32);
```

The limit applies to both typed and pasted text.

---

# Slider

`Slider` is an interactive range widget supporting mouse dragging, keyboard controls and mouse-wheel input.

It supports:

* Horizontal sliders
* Vertical sliders
* Minimum / maximum values
* Step snapping
* Mouse dragging
* Keyboard control
* Mouse wheel
* Focus
* Animated styling
* Per-state styles

## Creating a Slider

```rust
let slider = Slider::new(
    vec2(400.0, 300.0),
    0.0,
    100.0,
);
```

The initial value is the minimum value.

## Setting a Value

```rust
let slider = Slider::new(vec2(400.0, 300.0), 0.0, 100.0)
    .value(50.0);
```

Read the value:

```rust
let value = slider.get();
```

Programmatically change it:

```rust
slider.set_value(75.0);
```

---

# Slider Steps

For discrete values, use `.step()`:

```rust
let slider = Slider::new(vec2(400.0, 300.0), 0.0, 100.0)
    .step(10.0);
```

This produces values such as:

```text
0
10
20
30
...
100
```

A step of `0.0` enables continuous values.

---

# Slider Orientation

Horizontal is the default:

```rust
Slider::new(vec2(400.0, 300.0), 0.0, 100.0)
```

Vertical:

```rust
Slider::new(vec2(400.0, 300.0), 0.0, 100.0)
    .orientation(SliderOrientation::Vertical);
```

For vertical sliders:

* Top = maximum
* Bottom = minimum

---

# Slider Styling

`SliderStyle` provides control over the track, fill and handle.

```rust
let style = SliderStyle::new()
    .track_color(DARKGRAY)
    .fill_color(BLUE)
    .handle_color(WHITE)
    .handle_border(BLUE, 2.0)
    .handle_radius(10.0)
    .size(vec2(300.0, 24.0))
    .track_thickness(6.0)
    .scale(vec2(1.0, 1.0))
    .rotation(0.0);
```

Slider states:

* Normal
* Hover
* Pressed
* Focused
* Disabled

Example:

```rust
let slider = Slider::new(vec2(400.0, 300.0), 0.0, 100.0)
    .normal(
        SliderStyle::new()
            .track_color(DARKGRAY)
            .fill_color(BLUE)
    )
    .hover(
        SliderStyle::new()
            .handle_radius(11.0)
    )
    .pressed(
        SliderStyle::new()
            .handle_radius(13.0)
    );
```

---

# Slider Input

### Mouse

Clicking the track changes the value and starts dragging.

### Keyboard

When focused:

| Key         | Action               |
| ----------- | -------------------- |
| Right / Up  | Increase             |
| Left / Down | Decrease             |
| Page Up     | Increase by 10 steps |
| Page Down   | Decrease by 10 steps |
| Home        | Minimum              |
| End         | Maximum              |

### Mouse Wheel

When hovered, the mouse wheel changes the value.

---

# Slider Events

Check whether the value changed:

```rust
if slider.changed() {
    println!("Value: {}", slider.get());
}
```

Check focus:

```rust
if slider.is_focused() {
    // ...
}
```

Check dragging:

```rust
if slider.is_dragging() {
    // ...
}
```

---

# Widget Trait

All widgets implement the common `Widget` trait.

```rust
pub trait Widget {
    fn update(&mut self);
    fn draw(&self);
    fn hitbox(&self) -> Rect;
    fn is_enabled(&self) -> bool;
}
```

The trait also provides shared behavior.

## Mouse Hover

```rust
widget.hovered()
```

## Mouse Press

```rust
widget.pressed_now()
```

## Click

```rust
widget.clicked()
```

## State

```rust
widget.state()
```

Possible states:

```rust
WidgetState::Normal
WidgetState::Hover
WidgetState::Pressed
WidgetState::Focused
WidgetState::Disabled
```

This allows different widgets to share the same interaction model.

---

# Animation

`uilib` contains a small generic interpolation system in the `anim` module.

The main trait is:

```rust
pub trait Lerp {
    fn lerp_towards(&self, other: &Self, t: f32) -> Self;
}
```

It is implemented for:

* `f32`
* `Color`
* `Vec2`
* Widget style structures

Widget styles are interpolated every frame using a frame-rate-independent smoothing factor.

```rust
let t = smoothing_factor(speed, dt);
```

This allows transitions such as:

```text
Normal
   ↓
Hover
   ↓
Pressed
```

to animate smoothly instead of switching instantly.

---

# Complete Example

```rust
use macroquad::prelude::*;
use uilib::{
    Button,
    ButtonStyle,
    Slider,
    SliderStyle,
    TextInput,
    TextInputStyle,
    Widget,
};

#[macroquad::main("uilib Example")]
async fn main() {
    let mut button = Button::new(
        "Click me",
        vec2(400.0, 150.0),
    )
    .normal(
        ButtonStyle::new()
            .color(DARKGRAY)
            .text_color(WHITE)
            .size(vec2(220.0, 60.0))
            .border(WHITE, 1.0),
    )
    .hover(
        ButtonStyle::new()
            .color(GRAY)
            .scale(vec2(1.05, 1.05)),
    )
    .pressed(
        ButtonStyle::new()
            .color(WHITE)
            .text_color(BLACK)
            .scale(vec2(0.95, 0.95)),
    );

    let mut input = TextInput::new(vec2(400.0, 250.0))
        .placeholder("Type something...")
        .max_len(64);

    let mut slider = Slider::new(
        vec2(400.0, 350.0),
        0.0,
        100.0,
    )
    .value(50.0)
    .step(1.0)
    .normal(
        SliderStyle::new()
            .track_color(DARKGRAY)
            .fill_color(BLUE)
            .handle_color(WHITE),
    );

    loop {
        clear_background(Color::from_rgba(20, 20, 24, 255));

        button.update();
        input.update();
        slider.update();

        if button.clicked() {
            println!("Button clicked");
        }

        if input.submitted() {
            println!("Input: {}", input.text());
        }

        if slider.changed() {
            println!("Slider: {}", slider.get());
        }

        button.draw();
        input.draw();
        slider.draw();

        next_frame().await;
    }
}
```

---

# Architecture

The library is intentionally small.

```text
uilib
├── anim
│   ├── Lerp
│   └── smoothing_factor
│
└── widgets
    ├── widget
    │   ├── Widget
    │   └── WidgetState
    │
    ├── button
    │   ├── Button
    │   └── ButtonStyle
    │
    ├── input
    │   ├── TextInput
    │   └── TextInputStyle
    │
    └── slider
        ├── Slider
        ├── SliderStyle
        └── SliderOrientation
```

The widgets use a simple update/draw model:

```text
Input
  ↓
update()
  ↓
State detection
  ↓
Style interpolation
  ↓
draw()
```

There is no global UI manager required.

---

# Design Goals

`uilib` aims to be:

### Small

The library provides widgets rather than a complete application framework.

### Customizable

Styles are regular Rust structs, making them easy to configure and extend.

### Animated

State changes smoothly interpolate instead of requiring every widget to implement its own animation system.

### Macroquad-native

The library works directly with Macroquad types such as:

```rust
Color
Vec2
Rect
Font
```

### Simple

The intended usage is:

```rust
widget.update();
widget.draw();
```

without a complex UI tree or layout engine.

---

# Roadmap

Potential future widgets include:

* [ ] Checkbox
* [ ] Toggle
* [ ] Label
* [ ] Progress Bar
* [ ] Image


The widget architecture is designed so new widgets can reuse the existing `Widget`, `WidgetState`, styling and animation systems.

---

# License

Add your project's license here.

For example:

```text
MIT License
```

or:

```text
Apache-2.0
```

---

# Contributing

Contributions are welcome.

When adding a new widget, try to follow the existing architecture:

1. Create a widget-specific style struct.
2. Implement `Lerp` for the style.
3. Implement `Widget`.
4. Keep input handling inside `update()`.
5. Keep rendering inside `draw()`.
6. Expose the widget from `src/lib.rs`.
7. Add an example demonstrating the widget.

---

# Status

`uilib` is currently an early-stage UI toolkit for Macroquad.

The API may change as more widgets functionality are added.
