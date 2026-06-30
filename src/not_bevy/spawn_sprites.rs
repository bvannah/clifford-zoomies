use bevy::{image::ImageLoaderSettings, pbr::MAX_DIRECTIONAL_LIGHTS, prelude::*, time::common_conditions::on_timer};
use bevy_rapier2d::prelude::*;
use std::time::Duration;
use rand::prelude::*;

use super::player_components::*;

pub const CANVAS_SIZE: Vec2 = Vec2::new(480.*1.0, 270.*1.0);
pub const MAX_SIDE_DIST: f32 = CANVAS_SIZE.x * 1.;
pub const LEVEL1_HEIGHT: f32 = CANVAS_SIZE.y * 64.;
pub const PLAYER_SIZE: f32 = 32.0;
pub const side_player_spawn_dist: f32 = CANVAS_SIZE.x;
pub const below_player_spawn_dist: f32 = PLAYER_SIZE*2.0;
pub const above_player_spawn_dist: f32 = CANVAS_SIZE.y * 1.0;
pub const pspawn_x: f32 = 0.0; //CANVAS_SIZE.x/3.0;
pub const pspawn_y: f32 = 1.0;// CANVAS_SIZE.y/3.0;
pub const pspawn_z: f32 = 1.0;

// pub const PLAYER_SPRITE: &str = "30-303953_clifford-the-big-red-dog-clifford-hd-png.png";
pub const PLAYER_SPRITE: &str = "cliff/clifford.png";

#[derive(Component)]
pub struct Furn;


pub struct FurnPlugin;

impl Plugin for FurnPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(
            FixedUpdate,
            (
                despawn_furns,
                animate_furns,
                spawn_furns, // in the future, we want to have them spawn in random places at random times
            ),
        );
    }
}

fn spawn_furns(
    mut commands: Commands,
    player: Single<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>) {


        // if time matches whatever random number, spawn a random furn
        // let rng: bool = rand::random();
        let mut rng = rand::rng();

        if rng.random_bool(0.5/60.0){
            info!("Spawning furns");
            info!("{}", rng.random_range(50.0..100.0));

            let min_x: f32 = -MAX_SIDE_DIST.max(player.translation.x - side_player_spawn_dist);
            let min_y: f32 = 0.0_f32.max(player.translation.y-below_player_spawn_dist);
            let max_x: f32 = MAX_SIDE_DIST.min(player.translation.x + side_player_spawn_dist);
            let max_y: f32 = LEVEL1_HEIGHT.min(player.translation.y + above_player_spawn_dist);

            spawn_furn(commands, asset_server,
            rng.random_range(50.0..100.0),
            rng.random_range(20.0..50.0),


            // TODO: it's possible, albeit extremely unlikely, that a furn spawns exactly where the player is. they'd get stuck or have momentum halted
            // furns should spawn wherever the player is. limited by out of bounds, and dont spawn more than a bit below player

            rng.random_range(min_x..max_x),
            rng.random_range(min_y..max_y),
            0.0,
            )


        }






    }



fn spawn_furn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    x_size: f32, y_size: f32, x_location: f32, y_location: f32, rotation360: f32){
    // ){
    // let x_size: f32 = 400.0;
    // let y_size: f32 = 400.0;
    // let y_location: f32 = 200.0;
    // let x_location: f32 = 200.0;
    let image = asset_server.load_with_settings(
        "pipe.png",
    |settings: &mut ImageLoaderSettings| {
                settings
                    .sampler
                    .get_or_init_descriptor()
                    .set_filter(
                        bevy::image::ImageFilterMode::Nearest,
                    );
                },
    );
    
    // let furn_transform = transform_helper
    //     .compute_global_transform(entity)?;
    // let furn_collider = Aabb2d::new(
    //     furn_transform.translation().xy(),
    //     sprite.custom_size.unwrap() / 2.,
    // );


    commands.spawn((
        Sprite{
            image,
            custom_size: Some(Vec2::new(x_size, y_size)),   
            image_mode: SpriteImageMode::Sliced(
                TextureSlicer { 
                    border: BorderRect::axes(8., 19.), 
                    center_scale_mode: (SliceScaleMode::Stretch), 
                    ..default()
                },
            ),
            ..default()
        },
        Transform::from_xyz(x_location, y_location, 1.0),
        // Collider::cuboid(furn_transform.translation().xy(),
        // sprite.custom_size.unwrap() / 2.,),
        // Collider::cuboid(25.0, 10.0),
        Collider::cuboid(x_size/2.0, y_size/2.0),
        Furn    
    ));

}

fn despawn_furns(){}

fn animate_furns(){} 