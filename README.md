# agnosticlayer

**A flat, renderer-agnostic data layer for widget-like UI trees.**

Add UI data to your engine without picking a UI framework — one dependency, no rendering opinions, no decisions about what else to pull in. Built as the tool I wanted for myself, now it's yours.

---

## EN

### What it is

`agnosticlayer` is not a UI toolkit — it's a place to put UI *data*. Nodes, labels, containers, images, hit shapes — all stored as flat maps, all generic over the types you already use in your engine. The library never draws a pixel. You read the maps and feed them into wgpu, ash, a terminal renderer, `println!` — whatever you've already got.

The split is deliberate: this crate is the backend. Your renderer is the frontend. They don't know about each other and don't have to.

The point isn't another UI on Rust. The point is to solve a specific low-level problem — where UI data lives, how it's keyed, how you walk it — and leave 100% of control to whoever writes the renderer. Everything the crate contains covers the whole surface of a typical widget tree. Anything past that is your call.

### What it isn't

- Not a scene graph. Nesting is a single `parent: Option<Id>` field on `Node` — hierarchy exists only if *you* walk that field. Nothing is nested by default.
- Not opinionated about IDs, text, images, or hit shapes. `WidgetId`, `TextItem`, `ImageItem`, `ShapeVec`, `ShapeMap` are traits with blanket impls where it's safe to — bring your own `u32`, enum, string wrapper, AABB, whatever.
- Not a renderer, and never will be. Rendering, layout, clicking — all of that lives in your engine. This crate just holds the numbers.
- Not restricted to a single parenting rule. Any widget can act as a parent — the backend never walks relationships automatically, so nothing stops you from treating any id as one. If you want a `Lable` to own children, set their `parent` to it and read the connection back in your renderer. If you want the notion of "parent" spread across several maps, or keyed by a custom enum, do it. The reference example uses `Container` for that role because it does the job cleanly, not because the crate enforces it.

### Features

- `Node<Id>` — transform, size, scale, optional parent; the base every widget sits on
- `Lable<Text, Id>` / `DunamicLable<Id>` — static or owned text, styled independently from layout
- `Container<Id>` — padding + clip; the widget used in the reference nesting example, nothing forced
- `Image<ImageHandle, Id>` — a generic handle wrapped in the same `Node` / `VisualMode` language as everything else
- `HitLayer<SVec, SMap, Id>` — a flat `Vec`- or `HashMap`-backed shape-to-id table for your own hit-testing; the id can point at any widget, so any object in the tree can be made clickable
- `Layer<...>` — the top-level struct owning every map, with a full add / remove / update / get API
- Generic over `Id`, `Text`, `Image`, and `Shape` types — nothing hardcoded, no forced dependency on any of them
- No smart pointers, no rendering code, one dependency (`wibr`)

### Quick Start

**1. Add the dependency**

```toml
[dependencies]
agnosticlayer = "0.1.0"
```

**2. Describe your own types**

The crate has no built-in text, image, or shape types — you plug in your own. `WidgetId` is auto-implemented for anything `Copy + Hash + Eq`, so a plain integer works out of the box.

```rust
use agnosticlayer::base_date::TextItem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Label(&'static str);

impl TextItem for Label {
    fn as_text(&self) -> &str { self.0 }
}

type Id = u32;
```

**About `Void`**

Any generic parameter the crate exposes can be filled with `Void` when you don't need it. `Void` implements `TextItem` (returns its type name) and works as an id, image handle, or shape type too. A tuple like `()` works for everything *except* the text parameter — there, `Void` is the placeholder to reach for.

```rust
use agnosticlayer::base_date::Void;

// image handle, hit shapes, and text all unused:
type MyLayer = Layer<Void, Void, Void, Void, u32>;
```

**3. Create a layer**

`Layer` is the top of the stack. It owns every map: nodes, labels, containers, images, dynamic labels, hit layer.

```rust
use agnosticlayer::layer::Layer;
use agnosticlayer::base_date::Void;

let mut layer: Layer<Void, Void, Void, Label, Id> = Layer::new();
```

**4. Add a container**

```rust
use agnosticlayer::container::{Container, ContainerStyle};
use agnosticlayer::node::Node;
use agnosticlayer::base_date::{Transform, Size, Scale, Rotate, Color, VisualMode};

let root = Container::new(
    Node::new(1u32, Transform::default(), Size::new(400.0, 300.0), Scale::default(), None),
    Rotate::default(),
    ContainerStyle::new(8.0, true),
    Color::from_u32(0x202020FF),
    VisualMode::default(),
);

layer.add_container(root);
```

**5. Add a label as its child**

Parenthood is just a field. The container doesn't know the label exists until you walk the tree.

```rust
use agnosticlayer::lable::{Lable, TextStyle};
use agnosticlayer::base_date::{FontSize, LineHeight, LetterSpacing, Align};

let label = Lable::new(
    Node::new(2u32, Transform::default(), Size::new(120.0, 24.0), Scale::default(), Some(1u32)),
    Rotate::default(),
    TextStyle::new(Label("Hello"), FontSize::Default, Color::from_u32(0xFFFFFFFF), LineHeight::Default, LetterSpacing::Default, Align::CenterCenter),
    Color::default(),
    VisualMode::default(),
);

layer.add_lable(label);
```

**6. Walk the parent relationship**

```rust
let parent = layer.get_container_lable(label);
```

**7. Hit testing**

`SVec` / `SMap` were `Void` above, so hit testing is a no-op in this example. Swap in your own shape type — rect, circle, AABB — to turn it on:

```rust
layer.add_hit_in_map(my_shape, 2u32);
let hit_id = layer.get_hit_in_map(my_shape);
```

**8. Read it back and render it yourself**

```rust
for (_, container) in layer.get_container_map() {
    // container.color / container.container_style, whatever your renderer needs
}

for (_, label) in layer.get_lable_map() {
    println!("{}", label.as_text());
}
```

**The other widgets — same shape, same API**

Every widget follows the same pattern: a struct with a `Node` inside, a map keyed by id, and `add_*` / `remove_*` / `update_*` / `get_*` methods.

```rust
use agnosticlayer::image::Image;
use agnosticlayer::dynamic_lable::{DunamicLable, DunamicTextStyle};
use agnosticlayer::base_date::{FontSize, LineHeight, LetterSpacing, Align, Transform, Size, Scale, Rotate, Color, VisualMode};

// Image: generic handle + Node + VisualMode
let image = Image::new(
    Node::new(3u32, Transform::default(), Size::new(64.0, 64.0), Scale::default(), Some(1u32)),
    my_texture_handle,
    VisualMode::default(),
);
layer.add_image(image);

// Dynamic label: owns its text, TextItem not needed
let dlabel = DunamicLable::new(
    Node::new(4u32, Transform::default(), Size::new(200.0, 24.0), Scale::default(), Some(1u32)),
    Rotate::default(),
    DunamicTextStyle::new("Counter: 0".to_string(), FontSize::Default, Color::default(), LineHeight::Default, LetterSpacing::Default, Align::CenterCenter),
    Color::default(),
    VisualMode::default(),
);
layer.add_dunamic_lable(dlabel);

// Node: standalone, if a widget doesn't need any visual data
layer.add_node(Node::new(5u32, Transform::default(), Size::new(0.0, 0.0), Scale::default(), None));

// HitLayer with both containers:
// - vec for short lists scanned linearly
// - map for direct lookup by shape
layer.add_hit_in_vec(my_shape_a, 2u32);
layer.add_hit_in_map(my_shape_b, 3u32);
```

**Recommended dependencies**

The crate itself pulls in only `wibr`. For the code around it — the parts you write in your engine — this is what I reach for:

- `strum` — if your `WidgetId` is an enum, `EnumIter` gives you `iter()`, `AsRefStr` / `IntoStaticStr` give `&'static str`, and `#[strum(serialize = "...")]` on each variant pins the string key your renderer will look the widget up by.

Example with `strum_macros`:

```rust
use strum_macros::{EnumIter, AsRefStr, IntoStaticStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, AsRefStr, IntoStaticStr)]
enum UiId {
    #[strum(serialize = "root")]
    Root,
    #[strum(serialize = "title")]
    Title,
    #[strum(serialize = "subtitle")]
    Subtitle,
    #[strum(serialize = "play_button")]
    PlayButton,
    #[strum(serialize = "pause_button")]
    PauseButton,
    #[strum(serialize = "settings")]
    Settings,
    #[strum(serialize = "volume_slider")]
    VolumeSlider,
}

// WidgetId is auto-implemented — no extra derive needed.

for id in UiId::iter() {
    let key: &'static str = id.as_ref();
    // renderer.register(key, id);
}
```

**Where the line is**

Everything in this crate is `Copy`, keyed by id, and stored in plain maps. No `Rc`, no `RefCell`, no trait objects inside the data. Serialize it, diff it, snapshot it, send it across threads — nothing stops you, because nothing is hidden behind an abstraction the crate owns. The abstraction is your renderer. This is the data it reads.

---

## RU

### Что это

`agnosticlayer` — не UI-тулкит, а место для хранения UI-*данных*. Ноды, лейблы, контейнеры, картинки, hit-шейпы — всё лежит плоскими мапами, всё дженерик по типам, которые уже есть в твоём движке. Библиотека не рисует ни одного пикселя — ты читаешь мапы и отдаёшь их в wgpu, ash, терминальный рендер, `println!` — что угодно.

Разделение осознанное: этот крейт — бекенд. Твой рендер — фронтенд. Они не знают друг о друге и не должны.

Смысл не в том, чтобы сделать ещё один UI на Rust. Смысл — решить конкретную низкоуровневую проблему: где лежат UI-данные, как они закешированы, как по ним ходить — и оставить 100% контроля тому, кто пишет рендер. Всё, что есть в крейте, закрывает весь типичный набор виджетного дерева. Дальше — твоё дело.

### Чем это не является

- Не сцен-граф. Вложенность — это одно поле `parent: Option<Id>` в `Node`, иерархия существует только если ты сам её строишь, обходя это поле. По умолчанию ничего не вложено.
- Не диктует типы для id, текста, картинок, hit-шейпов. `WidgetId`, `TextItem`, `ImageItem`, `ShapeVec`, `ShapeMap` — трейты с blanket-имплементациями там, где это безопасно: подставляй свой `u32`, enum, обёртку над строкой, AABB — что угодно.
- Не рендерер и не станет им. Рендер, layout, клики — всё это остаётся в твоём движке, крейт хранит только числа.
- Не ограничивает роль родителя одним виджетом. Родителем может быть кто угодно — бекенд просто не обходит связи автоматически, так что ничто не мешает считать родителем любой id. Хочешь, чтобы `Lable` владел детьми — ставь его id в их `parent` и читай связь обратно в рендере. Хочешь раскидать понятие «родитель» по нескольким мапам или завести под это свой enum — делай. В референсном примере эту роль взял на себя `Container`, потому что справляется чисто, а не потому что так предписано.

### Возможности

- `Node<Id>` — transform, size, scale, опциональный parent; база для любого виджета
- `Lable<Text, Id>` / `DunamicLable<Id>` — статичный или владеющий текст, стиль отдельно от layout
- `Container<Id>` — padding + clip; виджет, на котором построен референсный пример вложенности, ничего навязанного
- `Image<ImageHandle, Id>` — дженерик-хендл, обёрнутый в те же `Node` / `VisualMode`, что и всё остальное
- `HitLayer<SVec, SMap, Id>` — плоская таблица шейп → id на `Vec` или `HashMap`, под свой hit-testing; id может указывать на любой виджет, так что кликабельным можно сделать что угодно
- `Layer<...>` — верхнеуровневая структура, владеющая всеми мапами, с полным add / remove / update / get API
- Дженерик по `Id`, `Text`, `Image`, `Shape` — ничего не зашито, ни один из них не тянет обязательную зависимость
- Никаких умных указателей, никакого рендер-кода, одна зависимость (`wibr`)

### Быстрый старт

**1. Добавь зависимость**

```toml
[dependencies]
agnosticlayer = "0.1.0"
```

**2. Опиши свои типы**

В крейте нет встроенных типов для текста, картинок и шейпов — ты подставляешь свои. `WidgetId` авто-реализован для всего `Copy + Hash + Eq`, так что обычное целое число работает сразу.

```rust
use agnosticlayer::base_date::TextItem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Label(&'static str);

impl TextItem for Label {
    fn as_text(&self) -> &str { self.0 }
}

type Id = u32;
```

**Про `Void`**

Любой дженерик-параметр крейта можно закрыть `Void`, если он тебе не нужен. `Void` реализует `TextItem` (возвращает имя своего типа) и работает как id, image-хендл или тип шейпа. Кортеж вроде `()` тоже подойдёт для всего, кроме текстового параметра — там нужен именно `Void`.

```rust
use agnosticlayer::base_date::Void;

// хендл картинки, hit-шейпы и текст не используются:
type MyLayer = Layer<Void, Void, Void, Void, u32>;
```

**3. Создай слой**

`Layer` — вершина стека. Владеет всеми мапами: ноды, лейблы, контейнеры, картинки, динамические лейблы, hit layer.

```rust
use agnosticlayer::layer::Layer;
use agnosticlayer::base_date::Void;

let mut layer: Layer<Void, Void, Void, Label, Id> = Layer::new();
```

**4. Добавь контейнер**

```rust
use agnosticlayer::container::{Container, ContainerStyle};
use agnosticlayer::node::Node;
use agnosticlayer::base_date::{Transform, Size, Scale, Rotate, Color, VisualMode};

let root = Container::new(
    Node::new(1u32, Transform::default(), Size::new(400.0, 300.0), Scale::default(), None),
    Rotate::default(),
    ContainerStyle::new(8.0, true),
    Color::from_u32(0x202020FF),
    VisualMode::default(),
);

layer.add_container(root);
```

**5. Добавь лейбл как его ребёнка**

Родительство — это просто поле. Контейнер не знает о лейбле, пока ты не пройдёшь по дереву.

```rust
use agnosticlayer::lable::{Lable, TextStyle};
use agnosticlayer::base_date::{FontSize, LineHeight, LetterSpacing, Align};

let label = Lable::new(
    Node::new(2u32, Transform::default(), Size::new(120.0, 24.0), Scale::default(), Some(1u32)),
    Rotate::default(),
    TextStyle::new(Label("Hello"), FontSize::Default, Color::from_u32(0xFFFFFFFF), LineHeight::Default, LetterSpacing::Default, Align::CenterCenter),
    Color::default(),
    VisualMode::default(),
);

layer.add_lable(label);
```

**6. Пройди по связи parent**

```rust
let parent = layer.get_container_lable(label);
```

**7. Hit-testing**

`SVec` / `SMap` выше были `Void`, так что hit-testing в этом примере ничего не делает. Подставь свой тип шейпа — прямоугольник, круг, AABB — чтобы включить его:

```rust
layer.add_hit_in_map(my_shape, 2u32);
let hit_id = layer.get_hit_in_map(my_shape);
```

**8. Читай и рендери сам**

```rust
for (_, container) in layer.get_container_map() {
    // container.color / container.container_style — как хочет твой рендер
}

for (_, label) in layer.get_lable_map() {
    println!("{}", label.as_text());
}
```

**Остальные виджеты — та же форма, тот же API**

Каждый виджет устроен одинаково: структура с `Node` внутри, мапа по id и методы `add_*` / `remove_*` / `update_*` / `get_*`.

```rust
use agnosticlayer::image::Image;
use agnosticlayer::dynamic_lable::{DunamicLable, DunamicTextStyle};
use agnosticlayer::base_date::{FontSize, LineHeight, LetterSpacing, Align, Transform, Size, Scale, Rotate, Color, VisualMode};

// Image: дженерик-хендл + Node + VisualMode
let image = Image::new(
    Node::new(3u32, Transform::default(), Size::new(64.0, 64.0), Scale::default(), Some(1u32)),
    my_texture_handle,
    VisualMode::default(),
);
layer.add_image(image);

// Динамический лейбл: владеет своим текстом, TextItem не нужен
let dlabel = DunamicLable::new(
    Node::new(4u32, Transform::default(), Size::new(200.0, 24.0), Scale::default(), Some(1u32)),
    Rotate::default(),
    DunamicTextStyle::new("Counter: 0".to_string(), FontSize::Default, Color::default(), LineHeight::Default, LetterSpacing::Default, Align::CenterCenter),
    Color::default(),
    VisualMode::default(),
);
layer.add_dunamic_lable(dlabel);

// Node: отдельно, если виджету не нужны визуальные данные
layer.add_node(Node::new(5u32, Transform::default(), Size::new(0.0, 0.0), Scale::default(), None));

// HitLayer с двумя контейнерами:
// - vec для коротких списков, сканируемых линейно
// - map для прямого поиска по шейпу
layer.add_hit_in_vec(my_shape_a, 2u32);
layer.add_hit_in_map(my_shape_b, 3u32);
```

**Удобные зависимости**

Сам крейт тянет только `wibr`. Для кода вокруг — того, что ты пишешь в своём движке — я обычно беру:

- `strum` — если твой `WidgetId` это enum, `EnumIter` даёт `iter()`, `AsRefStr` / `IntoStaticStr` дают `&'static str`, а `#[strum(serialize = "...")]` на каждом варианте фиксирует строковый ключ, по которому твой рендер будет искать виджет.

Пример со `strum_macros`:

```rust
use strum_macros::{EnumIter, AsRefStr, IntoStaticStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, AsRefStr, IntoStaticStr)]
enum UiId {
    #[strum(serialize = "root")]
    Root,
    #[strum(serialize = "title")]
    Title,
    #[strum(serialize = "subtitle")]
    Subtitle,
    #[strum(serialize = "play_button")]
    PlayButton,
    #[strum(serialize = "pause_button")]
    PauseButton,
    #[strum(serialize = "settings")]
    Settings,
    #[strum(serialize = "volume_slider")]
    VolumeSlider,
}

// WidgetId реализуется автоматически — отдельный derive не нужен.

for id in UiId::iter() {
    let key: &'static str = id.as_ref();
    // renderer.register(key, id);
}
```

**Где проходит граница**

Всё в этом крейте — `Copy`, закешировано по id и лежит в обычных мапах. Никаких `Rc`, никаких `RefCell`, никаких трейт-объектов внутри данных. Сериализуй, диффай, снимай снапшоты, отправляй между потоками — ничто не мешает, потому что ничто не спрятано за абстракцией, которой владеет крейт. Абстракция — это твой рендер. Здесь только данные, которые он читает.