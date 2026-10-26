# agnosticlayer

English | [Русский](README.ru.md)

A flat, renderer-agnostic data layer for widget-style UI trees.

The crate stores widget data (nodes, labels, containers, images, hit shapes) in plain `HashMap`s and draws nothing. You read the maps and pass them to your own renderer: wgpu, a terminal, anything. The types for ids, text, images and shapes are yours.

## What's inside

- `Node<Id>`: transform, size, scale, optional `parent`; the base of every widget
- `Label<Text, Id>`: text from your own type (`TextItem`)
- `DynamicLabel<Id>`: text owned by the widget itself (`String`)
- `Container<Id>`: padding and clip
- `Image<ImageHandle, Id>`: your own image handle
- `HitLayer<SVec, SMap, Id>`: a shape-to-id table backed by a `Vec` or a `HashMap`, for your own hit testing
- `Layer<SVec, SMap, ImageHandle, Text, Id>`: owns all the maps; methods `add_*`, `remove_*`, `update_*`, `get_*`

## What it isn't

- A scene graph: nesting is a single `parent: Option<Id>` field, and you walk the tree yourself. Any widget can be a parent.
- A renderer, layout engine or input handler.
- Tied to specific types: `WidgetId`, `TextItem`, `ImageItem`, `ShapeVec` and `ShapeMap` are traits. `WidgetId`, `ImageItem`, `ShapeVec` and `ShapeMap` are implemented for every suitable type, so `u32` works as is.

## Quick start

```toml
[dependencies]
agnosticlayer = "0.2.0"
```

```rust
use agnosticlayer::base_data::*;
use agnosticlayer::container::{Container, ContainerStyle};
use agnosticlayer::label::{Label, TextStyle};
use agnosticlayer::layer::Layer;
use agnosticlayer::node::Node;

// Your own text type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Text(&'static str);

impl TextItem for Text {
    fn as_text(&self) -> &str { self.0 }
}

// Layer<SVec, SMap, ImageHandle, Text, Id>
// Unused parameters can be set to `Void` or `()`
let mut layer: Layer<Void, Void, Void, Text, u32> = Layer::new();

let root = Container::new(
    Node::new(1u32, Transform::default(), Size::new(400.0, 300.0), Scale::default(), None),
    Rotate::default(),
    ContainerStyle::new(8.0, true),
    Color::from_u32(0x202020FF),
    VisualMode::default(),
);
layer.add_container(root);

// A label whose parent is 1
let label = Label::new(
    Node::new(2u32, Transform::default(), Size::new(120.0, 24.0), Scale::default(), Some(1u32)),
    Rotate::default(),
    TextStyle::new(Text("Hello"), FontSize::Default, Color::from_u32(0xFFFFFFFF), LineHeight::Default, LetterSpacing::Default, Align::CenterCenter),
    Color::default(),
    VisualMode::default(),
);
layer.add_label(label);

// The label's parent
let parent = layer.get_container_label(label);

// Reading for rendering
for label in layer.get_label_map().values() {
    println!("{}", label.as_text());
}
```

The crate does not perform hit testing: `HitLayer` only stores a shape-to-id table, and the actual hit check is yours to write. To use the table, put your own shape type in place of `Void` in `SVec` (for the `Vec`) or in `SMap` (for the `HashMap`; the shape must be `Hash + Eq`).

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Rect { x: f32, y: f32, w: f32, h: f32 }

let mut layer: Layer<Rect, Void, Void, Text, u32> = Layer::new();

let button = Rect { x: 0.0, y: 0.0, w: 120.0, h: 24.0 };
layer.add_hit_in_vec(button, 2u32);

let id = layer.get_hit_in_vec(button); // Some(2)
```

Lookup is by shape equality. Whether a cursor falls inside a `Rect` is up to your code; the table gives you the widget id.
