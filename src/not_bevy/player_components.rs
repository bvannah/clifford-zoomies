use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component, Default)]
pub struct Player {
    pub grounded: bool,
    pub left_walled: i32,
    pub right_walled: i32,
    pub bottom_walled: i32,
    pub bounce_timer: i32,
    pub collider_map: HashMap<u32, i8>, // i8 0 = right, 1 = left, 2 = bottom
    pub left_forgiveness: i8,
    pub right_forgiveness: i8,
    pub bottom_forgiveness: i8,
}
