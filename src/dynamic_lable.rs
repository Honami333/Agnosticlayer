use wibr::New;

use crate::base_date::*;
use crate::node::Node;


#[derive(Debug, Clone, PartialEq, New, Default)]
pub struct DunamicLable<Id: WidgetId> {
    pub node: Node<Id>,
    pub rotate: Rotate,
    pub text_style: DunamicTextStyle,
    pub color: Color,
    pub visual_mode: VisualMode,
}

impl<Id> DunamicLable<Id>
where 
    Id: WidgetId
{
    pub fn as_text(&self) -> &str { self.text_style.as_text() }
}

#[derive(Debug, Clone, PartialEq, New, Default)]
pub struct DunamicTextStyle {
    pub text: String,
    pub font_size: FontSize,
    pub color: Color,
    pub line_height: LineHeight,
    pub letter_spacing: LetterSpacing,
    pub align: Align,
}

impl DunamicTextStyle { pub fn as_text(&self) -> &str { self.text.as_str() } }
