use std::collections::HashMap;

use wibr::Make;

use crate::base_date::*;
use crate::container::Container;
use crate::dynamic_lable::DunamicLable;
use crate::hit::HitLayer;
use crate::image::Image;
use crate::lable::Lable;
use crate::node::Node;


#[derive(Debug, Clone, PartialEq, Make, Default)]
pub struct Layer<SVec: ShapeVec = Void, SMap: ShapeMap = Void, ImageHandle: ImageItem = Void, Text: TextItem = Void, Id: WidgetId = Void> {
    #[Some(WidgetMaps::new())] pub widget: WidgetMaps<ImageHandle, Text, Id>,
    #[Some(HitLayer::new())] pub hit_layer: HitLayer<SVec, SMap, Id>,
}

#[derive(Debug, Clone, PartialEq, Make, Default)]
pub struct WidgetMaps<ImageHandle: ImageItem, Text: TextItem, Id: WidgetId> {
    #[Some(HashMap::new())] pub node_map: HashMap<Id, Node<Id>>,
    #[Some(HashMap::new())] pub lable_map: HashMap<Id, Lable<Text, Id>>,
    #[Some(HashMap::new())] pub dunamic_lable_map: HashMap<Id, DunamicLable<Id>>,
    #[Some(HashMap::new())] pub container_map: HashMap<Id, Container<Id>>,
    #[Some(HashMap::new())] pub image_map: HashMap<Id, Image<ImageHandle, Id>>
}

impl<SVec, SMap, ImageHandle, Text, Id> Layer<SVec, SMap, ImageHandle, Text, Id> 
where
    SVec: ShapeVec,
    SMap: ShapeMap,
    ImageHandle: ImageItem,
    Text: TextItem,
    Id: WidgetId
{
    pub fn get_container_id(&self, id: Id) -> Option<Container<Id>> {
        self.get_container_map().get(&id).copied()
    }
}

// Node
impl<SVec, SMap, ImageHandle, Text, Id> Layer<SVec, SMap, ImageHandle, Text, Id> 
where
    SVec: ShapeVec,
    SMap: ShapeMap,
    ImageHandle: ImageItem,
    Text: TextItem,
    Id: WidgetId
{
    pub fn add_node(&mut self, node: Node<Id>) {
        self.get_mut_node_map().insert(node.id, node);
    }

    pub fn remove_node(&mut self, node: Node<Id>) -> Option<Node<Id>> {
        self.get_mut_node_map().remove(&node.id)
    }

    pub fn update_node(&mut self, node: Node<Id>) {
        if let Some(last_node) = self.get_mut_node(node.id) {
            *last_node = node;
        };
    }

    pub fn get_node(&self, id: Id) -> Option<Node<Id>> {
        self.get_node_map().get(&id).copied()
    }

    pub fn get_mut_node(&mut self, id: Id) -> Option<&mut Node<Id>>{
        self.get_mut_node_map().get_mut(&id)
    }

    pub fn get_container_node(&self, node: Node<Id>) -> Option<Container<Id>> {
        self.get_container_id(node.parent?)
    }
    
    pub fn get_node_map(&self) -> &HashMap<Id, Node<Id>> {
        &self.widget.node_map
    }

    pub fn get_mut_node_map(&mut self) -> &mut HashMap<Id, Node<Id>> {
        &mut self.widget.node_map
    }
}

// Label
impl<SVec, SMap, ImageHandle, Text, Id> Layer<SVec, SMap, ImageHandle, Text, Id> 
where
    SVec: ShapeVec,
    SMap: ShapeMap,
    ImageHandle: ImageItem,
    Text: TextItem,
    Id: WidgetId
{
    pub fn add_lable(&mut self, lable: Lable<Text, Id>) {
        self.get_mut_lable_map().insert(lable.node.id, lable);
    }

    pub fn remove_lable(&mut self, lable: Lable<Text, Id>) -> Option<Lable<Text, Id>> {
        self.get_mut_lable_map().remove(&lable.node.id)
    }

    pub fn update_lable(&mut self, lable: Lable<Text, Id>) {
        if let Some(last_lable) = self.get_mut_lable(lable.node.id) {
            *last_lable = lable;
        };
    }

    pub fn get_lable(&self, id: Id) -> Option<Lable<Text, Id>> {
        self.get_lable_map().get(&id).copied()
    }

    pub fn get_mut_lable(&mut self, id: Id) -> Option<&mut Lable<Text, Id>>{
        self.get_mut_lable_map().get_mut(&id)
    }

    pub fn get_container_lable(&self, lable: Lable<Text, Id>) -> Option<Container<Id>> {
        self.get_container_node(lable.node)
    }

    pub fn get_lable_map(&self) -> &HashMap<Id, Lable<Text, Id>> {
        &self.widget.lable_map
    }

    pub fn get_mut_lable_map(&mut self) -> &mut HashMap<Id, Lable<Text, Id>> {
        &mut self.widget.lable_map
    }
}

// Container
impl<SVec, SMap, ImageHandle, Text, Id> Layer<SVec, SMap, ImageHandle, Text, Id> 
where
    SVec: ShapeVec,
    SMap: ShapeMap,
    ImageHandle: ImageItem,
    Text: TextItem,
    Id: WidgetId
{
    pub fn add_container(&mut self, container: Container<Id>) {
        self.get_mut_container_map().insert(container.node.id, container);
    }

    pub fn remove_container(&mut self, container: Container<Id>) -> Option<Container<Id>> {
        self.get_mut_container_map().remove(&container.node.id)
    }

    pub fn update_container(&mut self, container: Container<Id>) {
        if let Some(last_container) = self.get_mut_container(container.node.id) {
            *last_container = container;
        };
    }

    pub fn get_container(&self, id: Id) -> Option<Container<Id>> {
        self.get_container_map().get(&id).copied()
    }

    pub fn get_mut_container(&mut self, id: Id) -> Option<&mut Container<Id>>{
        self.get_mut_container_map().get_mut(&id)
    }

    pub fn get_container_container(&self, container: Container<Id>) -> Option<Container<Id>> {
        self.get_container_node(container.node)
    }

    pub fn get_container_map(&self) -> &HashMap<Id, Container<Id>> {
        &self.widget.container_map
    }

    pub fn get_mut_container_map(&mut self) -> &mut HashMap<Id, Container<Id>> {
        &mut self.widget.container_map
    }
}

// Image
impl<SVec, SMap, ImageHandle, Text, Id> Layer<SVec, SMap, ImageHandle, Text, Id> 
where
    SVec: ShapeVec,
    SMap: ShapeMap,
    ImageHandle: ImageItem,
    Text: TextItem,
    Id: WidgetId
{
    pub fn add_image(&mut self, image_handle: Image<ImageHandle, Id>) {
        self.get_mut_image_map().insert(image_handle.node.id, image_handle);
    }

    pub fn remove_image(&mut self, image_handle: Image<ImageHandle, Id>) -> Option<Image<ImageHandle, Id>> {
        self.get_mut_image_map().remove(&image_handle.node.id)
    }

    pub fn update_image(&mut self, image_handle: Image<ImageHandle, Id>) {
        if let Some(last_image_handle) = self.get_mut_image(image_handle.node.id) {
            *last_image_handle = image_handle;
        };
    }

    pub fn get_image(&self, id: Id) -> Option<Image<ImageHandle, Id>> {
        self.get_image_map().get(&id).copied()
    }

    pub fn get_mut_image(&mut self, id: Id) -> Option<&mut Image<ImageHandle, Id>>{
        self.get_mut_image_map().get_mut(&id)
    }

    pub fn get_image_map(&self) -> &HashMap<Id, Image<ImageHandle, Id>> {
        &self.widget.image_map
    }

    pub fn get_mut_image_map(&mut self) -> &mut HashMap<Id, Image<ImageHandle, Id>> {
        &mut self.widget.image_map
    }
}

// Dynamic Label
impl<SVec, SMap, ImageHandle, Text, Id> Layer<SVec, SMap, ImageHandle, Text, Id> 
where
    SVec: ShapeVec,
    SMap: ShapeMap,
    ImageHandle: ImageItem,
    Text: TextItem,
    Id: WidgetId
{
    pub fn add_dunamic_lable(&mut self, dunamic_lable: DunamicLable<Id>) {
        self.get_mut_dunamic_lable_map().insert(dunamic_lable.node.id, dunamic_lable);
    }

    pub fn remove_dunamic_lable(&mut self, id: Id) -> Option<DunamicLable<Id>> {
        self.get_mut_dunamic_lable_map().remove(&id)
    }

    pub fn update_dunamic_lable(&mut self, dunamic_lable: DunamicLable<Id>) {
        if let Some(last_dunamic_lable) = self.get_mut_dunamic_lable(dunamic_lable.node.id) {
            *last_dunamic_lable = dunamic_lable;
        };
    }

    pub fn get_dunamic_lable(&self, id: Id) -> Option<&DunamicLable<Id>> {
        self.get_dunamic_lable_map().get(&id)
    }

    pub fn get_mut_dunamic_lable(&mut self, id: Id) -> Option<&mut DunamicLable<Id>>{
        self.get_mut_dunamic_lable_map().get_mut(&id)
    }

    pub fn get_container_dunamic_lable(&self, dunamic_lable: &DunamicLable<Id>) -> Option<Container<Id>> {
        self.get_container_node(dunamic_lable.node)
    }

    pub fn get_dunamic_lable_map(&self) -> &HashMap<Id, DunamicLable<Id>> {
        &self.widget.dunamic_lable_map
    }

    pub fn get_mut_dunamic_lable_map(&mut self) -> &mut HashMap<Id, DunamicLable<Id>> {
        &mut self.widget.dunamic_lable_map
    }
}

// Hit Layer
impl<SVec, SMap, ImageHandle, Text, Id> Layer<SVec, SMap, ImageHandle, Text, Id> 
where
    SVec: ShapeVec,
    SMap: ShapeMap,
    ImageHandle: ImageItem,
    Text: TextItem,
    Id: WidgetId
{
    pub fn add_hit_in_vec(&mut self, data: SVec, id: Id) {
        self.get_mut_hit_vec().push((data, id));
    }

    pub fn add_hit_in_map(&mut self, data: SMap, id: Id) {
        self.get_mut_hit_map().insert(data, id);
    }

    pub fn remove_hit_in_vec(&mut self, data: SVec) -> Option<(SVec, Id)> {
        let index = self.get_hit_vec().iter().position(|(d, _)| * d == data)?;
        Some(self.get_mut_hit_vec().swap_remove(index))
    }

    pub fn remove_hit_in_map(&mut self, data: SMap) -> Option<Id> {
        self.get_mut_hit_map().remove(&data)
    }

    pub fn update_hit_in_vec(&mut self, data: SVec, id: Id) {
        if let Some(last_hit) = self.get_mut_hit_in_vec(data) {
            *last_hit = id;
        };
    }

    pub fn update_hit_in_map(&mut self, data: SMap, id: Id) {
        if let Some(last_hit) = self.get_mut_hit_in_map(data) {
            *last_hit = id;
        };
    }

    pub fn get_hit_in_vec(&self, data: SVec) -> Option<Id> {
        self.hit_layer.find_vec(data)
    }

    pub fn get_mut_hit_in_vec(&mut self, data: SVec) -> Option<&mut Id>{
        self.hit_layer.find_mut_vec(data)
    }

    pub fn get_hit_in_map(&self, data: SMap) -> Option<Id> {
        self.get_hit_map().get(&data).copied()
    }

    pub fn get_mut_hit_in_map(&mut self, data: SMap) -> Option<&mut Id>{
        self.get_mut_hit_map().get_mut(&data)
    }

    pub fn get_hit_vec(&self) -> &Vec<(SVec, Id)> {
        &self.hit_layer.shape_vec
    }

    pub fn get_mut_hit_vec(&mut self) -> &mut Vec<(SVec, Id)> {
        &mut self.hit_layer.shape_vec
    }

    pub fn get_hit_map(&self) -> &HashMap<SMap, Id> {
        &self.hit_layer.shape_map
    }

    pub fn get_mut_hit_map(&mut self) -> &mut HashMap<SMap, Id> {
        &mut self.hit_layer.shape_map
    }
}
