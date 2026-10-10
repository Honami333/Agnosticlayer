use wibr::New;

use crate::base_data::*;
use crate::node::Node;


#[derive(Debug, Clone, PartialEq, New, Default)]
pub struct DynamicLable<Id: WidgetId> {
    pub node: Node<Id>,
    pub rotate: Rotate,
    pub text_style: DynamicTextStyle,
    pub color: Color,
    pub visual_mode: VisualMode,
}

impl<Id> DynamicLable<Id>
where 
    Id: WidgetId
{
    pub fn as_text(&self) -> &str { self.text_style.as_text() }
}

#[derive(Debug, Clone, PartialEq, New, Default)]
pub struct DynamicTextStyle {
    pub text: String,
    pub font_size: FontSize,
    pub color: Color,
    pub line_height: LineHeight,
    pub letter_spacing: LetterSpacing,
    pub align: Align,
}

impl DynamicTextStyle { pub fn as_text(&self) -> &str { self.text.as_str() } }
