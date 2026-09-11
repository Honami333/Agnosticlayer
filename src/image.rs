use wibr::New;

use crate::base_date::*;
use crate::node::Node;


#[derive(Debug, Clone, Copy, PartialEq, New, Default)]
pub struct Image<ImageHandle: ImageItem, Id: WidgetId> {
    pub node: Node<Id>,
    pub image_handle: ImageHandle,
    pub visual_mode: VisualMode,
}