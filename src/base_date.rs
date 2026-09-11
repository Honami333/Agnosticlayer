use std::hash::Hash;

use wibr::New;


pub trait WidgetId: Copy + Hash + Eq {}
impl<T: Copy + Hash + Eq> WidgetId for T {}

pub trait TextItem: Copy { fn as_text(&self) -> &str; }

pub trait ImageItem: Copy {}
impl<T: Copy> ImageItem for T {}

pub trait ShapeVec: Copy + PartialEq {}
impl<T: Copy + PartialEq> ShapeVec for T {}
pub trait ShapeMap: Copy + Hash + Eq {}
impl<T: Copy + Hash + Eq> ShapeMap for T {}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Void;

impl TextItem for Void { fn as_text(&self) -> &str { std::any::type_name::<Void>() }}

pub enum WidgetType {
    Node,
    Lable,
    DunamicLable,
    Container,
    Image
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FontSize {
    #[default] Default,
    New(usize)
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum LineHeight {
    #[default] Default,
    New(f32)
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum LetterSpacing {
    #[default] Default,
    New(f32)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Align {
    #[default] TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    CenterCenter,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Anchor {
    #[default] TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    CenterCenter,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight
}

#[derive(Debug, Clone, Copy, PartialEq, New, Default)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Debug, Clone, Copy, PartialEq, New, Default)]
pub struct Size {
    pub width: f32,
    pub height: f32
}

#[derive(Debug, Clone, Copy, PartialEq, New)]
pub struct Scale {
    pub x: f32,
    pub y: f32
}

impl Default for Scale { fn default() -> Self { Scale { x: 1.0, y: 1.0 } }}

#[derive(Debug, Clone, Copy, PartialEq, New, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Color {
    pub fn from_u32(rgba: u32) -> Self {
        Self { r: (rgba >> 24) as u8, g: (rgba >> 16) as u8, b: (rgba >> 8) as u8, a: rgba as u8 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, New)]
pub struct VisualMode {
    pub visible: bool,
    pub opacity: u8
}

impl Default for VisualMode { fn default() -> Self { Self { visible: true, opacity: 255 }}}

#[derive(Debug, Clone, Copy, PartialEq, New, Default)]
pub struct Rotate {
    pub rotate: f32,
    pub anchor: Anchor
}