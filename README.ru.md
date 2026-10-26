# agnosticlayer

[English](README.md) | Русский

Плоский слой данных для виджетных UI-деревьев, не привязанный к рендереру.

Крейт хранит данные виджетов (ноды, лейблы, контейнеры, картинки, hit-шейпы) в обычных `HashMap` и ничего не рисует. Ты читаешь мапы и отдаёшь их своему рендеру: wgpu, терминал, что угодно. Типы id, текста, картинок и шейпов — твои.

## Что внутри

- `Node<Id>` — transform, size, scale, необязательный `parent`; основа любого виджета
- `Label<Text, Id>` — текст из твоего типа (`TextItem`)
- `DynamicLabel<Id>` — текст, которым владеет сам виджет (`String`)
- `Container<Id>` — padding и clip
- `Image<ImageHandle, Id>` — твой хендл картинки
- `HitLayer<SVec, SMap, Id>` — таблица «шейп → id» на `Vec` или `HashMap` для своего hit-testing
- `Layer<SVec, SMap, ImageHandle, Text, Id>` — хранит все мапы, методы `add_*`, `remove_*`, `update_*`, `get_*`

## Чего здесь нет

- Сцен-графа: вложенность — одно поле `parent: Option<Id>`, по дереву ходишь сам. Родителем может быть любой виджет.
- Рендера, layout и обработки ввода.
- Привязки к типам: `WidgetId`, `TextItem`, `ImageItem`, `ShapeVec`, `ShapeMap` — трейты. `WidgetId`, `ImageItem`, `ShapeVec` и `ShapeMap` реализованы для всех подходящих типов, `u32` годится сразу.

## Быстрый старт

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

// Свой тип текста 
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Text(&'static str);

impl TextItem for Text {
    fn as_text(&self) -> &str { self.0 }
}

// Layer<SVec, SMap, ImageHandle, Text, Id>
// Ненужные параметры закрываются `Void` или `()`
let mut layer: Layer<Void, Void, Void, Text, u32> = Layer::new();

let root = Container::new(
    Node::new(1u32, Transform::default(), Size::new(400.0, 300.0), Scale::default(), None),
    Rotate::default(),
    ContainerStyle::new(8.0, true),
    Color::from_u32(0x202020FF),
    VisualMode::default(),
);
layer.add_container(root);

// Лейбл с родителем 1
let label = Label::new(
    Node::new(2u32, Transform::default(), Size::new(120.0, 24.0), Scale::default(), Some(1u32)),
    Rotate::default(),
    TextStyle::new(Text("Hello"), FontSize::Default, Color::from_u32(0xFFFFFFFF), LineHeight::Default, LetterSpacing::Default, Align::CenterCenter),
    Color::default(),
    VisualMode::default(),
);
layer.add_label(label);

// Родитель лейбла
let parent = layer.get_container_label(label);

// Чтение для рендера
for label in layer.get_label_map().values() {
    println!("{}", label.as_text());
}
```

Hit-testing в крейте не выполняется: `HitLayer` только хранит таблицу «шейп → id», а проверку попадания пишешь сам. Чтобы пользоваться таблицей, подставь свой тип шейпа вместо `Void` в `SVec` (для `Vec`) или в `SMap` (для `HashMap`, шейп должен быть `Hash + Eq`).

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Rect { x: f32, y: f32, w: f32, h: f32 }

let mut layer: Layer<Rect, Void, Void, Text, u32> = Layer::new();

let button = Rect { x: 0.0, y: 0.0, w: 120.0, h: 24.0 };
layer.add_hit_in_vec(button, 2u32);

let id = layer.get_hit_in_vec(button); // Some(2)
```

Поиск идёт по равенству шейпа. Попадает ли курсор в `Rect`, определяет твой код, а в таблице ты находишь id виджета.