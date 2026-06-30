use bevy::prelude::*;


#[derive(Component, Default)]
pub struct Player {
    pub grounded : bool,
    pub left_walled: bool,
    pub right_walled: bool,
    pub bounce_timer: i32
}
