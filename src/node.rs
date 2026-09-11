
use wibr::New;

use crate::base_date::*;


#[derive(Debug, Clone, Copy, PartialEq, New, Default)]
pub struct Node<Id: WidgetId> {
    pub id: Id,
    pub transform: Transform,
    pub size: Size,
    pub scale: Scale,
    pub parent: Option<Id>
}
