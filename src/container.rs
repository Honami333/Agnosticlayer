use wibr::New;

use crate::base_date::*;
use crate::node::Node;


#[derive(Debug, Clone, Copy, PartialEq, New, Default)]
pub struct Container<Id: WidgetId> {
    pub node: Node<Id>,
    pub rotate: Rotate,
    pub container_style: ContainerStyle,
    pub color: Color,
    pub visual_mode: VisualMode,
}

#[derive(Debug, Clone, Copy, PartialEq, New, Default)]
pub struct ContainerStyle {
    pub padding: f32,
    pub clip: bool
}
