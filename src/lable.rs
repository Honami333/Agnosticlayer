use wibr::New;

use crate::base_date::*;
use crate::node::Node;


#[derive(Debug, Clone, Copy, PartialEq, New, Default)]
pub struct Lable<Text: TextItem, Id: WidgetId> {
    pub node: Node<Id>,
    pub rotate: Rotate,
    pub text_style: TextStyle<Text>,
    pub color: Color,
    pub visual_mode: VisualMode,
}

impl<Text, Id> Lable<Text, Id>
where 
    Text: TextItem,
    Id: WidgetId
{
    pub fn as_text(&self) -> &str { self.text_style.as_text() }
}

#[derive(Debug, Clone, Copy, PartialEq, New, Default)]
pub struct TextStyle<Text: TextItem> {
    pub text: Text,
    pub font_size: FontSize,
    pub color: Color,
    pub line_height: LineHeight,
    pub letter_spacing: LetterSpacing,
    pub align: Align,
}

impl<Text> TextStyle<Text>
where 
    Text: TextItem,
{
    pub fn as_text(&self) -> &str { self.text.as_text() }
}