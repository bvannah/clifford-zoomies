use bevy::{image::ImageLoaderSettings, prelude::*};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use super::{
    constants_and_startup::{
        above_player_spawn_dist, below_player_spawn_dist, LEVEL1_HEIGHT, MAX_SIDE_DIST,
        PLAYER_SIZE, side_player_spawn_dist,
    },
    player_components::*,
};

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
                spawn_furns,
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
            // info!("Spawning furns");
            // info!("{}", rng.random_range(50.0..100.0));

            let min_x: f32 = -MAX_SIDE_DIST.max(player.translation.x - side_player_spawn_dist);
            let min_y: f32 = 0.0_f32.max(player.translation.y-below_player_spawn_dist);
            let max_x: f32 = MAX_SIDE_DIST.min(player.translation.x + side_player_spawn_dist);
            let max_y: f32 = LEVEL1_HEIGHT.min(player.translation.y + above_player_spawn_dist);

            // TODO: count the number of furns in this space, dont spawn too many
            // TODO: it's possible, albeit extremely unlikely, that a furn spawns exactly where the player is. they'd get stuck or have momentum halted
            // furns should spawn wherever the player is. limited by out of bounds, and dont spawn more than a bit below player



            spawn_furn(commands, asset_server,
            rng.random_range(50.0..100.0),
            rng.random_range(20.0..50.0),


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
        Collider::cuboid(x_size/2.0, y_size/2.0),
        Furn    
    ));

}

fn despawn_furns(){}

fn animate_furns(){} 