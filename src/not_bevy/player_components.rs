use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component, Default)]
pub struct Player {
    pub grounded : bool,
    pub left_walled: i32,
    pub right_walled: i32,
    pub bottom_walled: i32,
    pub bounce_timer: i32,
    pub collider_map: HashMap<u32, i8>
}
