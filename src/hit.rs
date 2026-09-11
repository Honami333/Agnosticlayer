use std::collections::HashMap;

use wibr::Make;

use crate::base_date::*;


#[derive(Debug, Clone, PartialEq, Make, Default)]
pub struct HitLayer<SVec: ShapeVec, SMap: ShapeMap, Id: WidgetId> {
    #[Some(Vec::new())] pub shape_vec: Vec<(SVec, Id)>,
    #[Some(HashMap::new())] pub shape_map: HashMap<SMap, Id>
}

impl<SVec, SMap, Id> HitLayer<SVec, SMap, Id>
where
    SVec: ShapeVec,
    SMap: ShapeMap,
    Id: WidgetId
{
    pub fn find_vec(&self, data: SVec) -> Option<Id> {
        self.shape_vec.iter().find(|(d, _)| *d == data).map(|(_, id)| *id)
    }

    pub fn find_mut_vec(&mut self, data: SVec) -> Option<&mut Id> {
        self.shape_vec.iter_mut().find(|(d, _)| *d == data).map(|(_, id)| id)
    }

    pub fn find_map(&self, data: SMap) -> Option<Id> { self.shape_map.get(&data).copied() }
    pub fn find_mut_map(&mut self, data: SMap) -> Option<&mut Id> { self.shape_map.get_mut(&data) }
}

