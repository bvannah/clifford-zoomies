use std::collections::HashMap;

use bevy::{
    camera::ScalingMode,
    image::{ImageAddressMode, ImageLoaderSettings},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::Material2d,
};
use bevy_rapier2d::prelude::*;

use super::{
    animation::{Animation, Animator, default_frame_duration},
    player_components::Player,
};

pub const CANVAS_SIZE: Vec2 = Vec2::new(480. * 1.5, 270. * 1.5);
pub const MAX_SIDE_DIST: f32 = CANVAS_SIZE.x * 1.;
pub const LEVEL1_HEIGHT: f32 = CANVAS_SIZE.y * 3.;
pub const PLAYER_SIZE: f32 = 32.0;
pub const side_player_spawn_dist: f32 = CANVAS_SIZE.x;
pub const below_player_spawn_dist: f32 = PLAYER_SIZE * 2.0;
pub const above_player_spawn_dist: f32 = CANVAS_SIZE.y * 1.0;
pub const pspawn_x: f32 = 0.0;
pub const pspawn_y: f32 = 1.0;
pub const pspawn_z: f32 = 1.0;
pub const PLAYER_SPRITE: &str = "cliff/clifford.png";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BackgroundMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Handle<Image>,
}

impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "background.wgsl".into()
    }
}

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut config_store: ResMut<GizmoConfigStore>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut animations = HashMap::new();

    animations.insert(
        "idle".to_string(),
        Animation {
            start: 2,
            end: 8,
            frame_duration: default_frame_duration,
            looping: true,
        },
    );
    animations.insert(
        "walk".to_string(),
        Animation {
            start: 28,
            end: 38,
            frame_duration: default_frame_duration,
            looping: true,
        },
    );
    animations.insert(
        "jump".to_string(),
        Animation {
            start: 9,
            end: 15,
            frame_duration: default_frame_duration,
            looping: false,
        },
    );
    animations.insert(
        "float_up".to_string(),
        Animation {
            start: 16,
            end: 16,
            frame_duration: default_frame_duration,
            looping: true,
        },
    );
    animations.insert(
        "fall".to_string(),
        Animation {
            start: 17,
            end: 22,
            frame_duration: default_frame_duration,
            looping: false,
        },
    );
    animations.insert(
        "float_down".to_string(),
        Animation {
            start: 23,
            end: 23,
            frame_duration: default_frame_duration,
            looping: true,
        },
    );
    animations.insert(
        "land".to_string(),
        Animation {
            start: 24,
            end: 27,
            frame_duration: default_frame_duration,
            looping: false,
        },
    );

    let frame_count = 38;
    let frame_size = UVec2::new(32, 32);

    let layout = TextureAtlasLayout::from_grid(frame_size, frame_count, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.enabled = true;

    commands.spawn((
        Camera2d,
        Projection::Orthographic((OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: CANVAS_SIZE.x,
                max_height: CANVAS_SIZE.y,
            },
            ..OrthographicProjection::default_2d()
        })),
    ));



    commands.spawn((


        Player {
            grounded: true,
            left_walled: false,
            right_walled: false,
            bounce_timer: 0,
        },
        Transform::from_xyz(pspawn_x, pspawn_y, pspawn_z),
        Velocity {
            linear: Vec2::new(1.0, 2.0),
            angular: 0.0,
        },
        RigidBody::Dynamic,
        Collider::cuboid(PLAYER_SIZE / 2.0, PLAYER_SIZE / 2.0/2.0),
        Restitution::new(0.8),
        Friction::new(0.7),
        ColliderMassProperties::Density(2.0),
        LockedAxes::ROTATION_LOCKED,
        ActiveEvents::COLLISION_EVENTS | ActiveEvents::CONTACT_FORCE_EVENTS,
        ))
        .with_children(|parent| {
            parent.spawn(( //sprite bundle
                Sprite {
                    image: asset_server.load(PLAYER_SPRITE),
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layout,
                        index: 0,
                    }),
                    ..default()
                },
                
                Animator {
                    animation: "idle".to_string(),
                    animations: animations,
                    ..Default::default()
                },
                Transform::from_xyz(0.0, 8.0, 0.0),

            ));
            });
        
    

    spawn_levels(commands, asset_server, config_store, meshes, materials, texture_atlas_layouts);

}


fn spawn_levels(mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut config_store: ResMut<GizmoConfigStore>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,){


    commands.spawn(( // floor
        Sprite {
            custom_size: Some(Vec2::new(MAX_SIDE_DIST * 2.0, 1.0)),
            image_mode: SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::axes(8., 19.),
                center_scale_mode: (SliceScaleMode::Stretch),
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Collider::cuboid(MAX_SIDE_DIST, 1.0 / 2.0),
        super::spawn_sprites::Furn,
    ));

    commands.spawn(( // left wall
        Sprite {
            custom_size: Some(Vec2::new(1.0, LEVEL1_HEIGHT)),
            image_mode: SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::axes(8., 19.),
                center_scale_mode: (SliceScaleMode::Stretch),
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(-MAX_SIDE_DIST, LEVEL1_HEIGHT / 2.0, 1.0),
        Collider::cuboid(1.0 / 2.0, LEVEL1_HEIGHT / 2.0),
        super::spawn_sprites::Furn,
    ));

    commands.spawn(( // right wall
        Sprite {
            custom_size: Some(Vec2::new(1.0, LEVEL1_HEIGHT)),
            image_mode: SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::axes(8., 19.),
                center_scale_mode: (SliceScaleMode::Stretch),
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(MAX_SIDE_DIST, LEVEL1_HEIGHT / 2.0, 1.0),
        Collider::cuboid(1.0 / 2.0, LEVEL1_HEIGHT / 2.0),
        super::spawn_sprites::Furn,
    ));

    commands.spawn((
    //     Mesh2d(meshes.add(Rectangle::new(MAX_SIDE_DIST * 2., CANVAS_SIZE.x * LEVEL1_HEIGHT))),
    //     MeshMaterial2d(materials.add(BackgroundMaterial {
    //         color_texture: asset_server.load(
    //             "Screenshot From 2026-06-10 12-54-36.png"),
    //     }),
    // )

        Sprite {
            custom_size: Some(Vec2::new(MAX_SIDE_DIST*2.0, LEVEL1_HEIGHT)),
            image: asset_server.load_with_settings(
                "second_floor.png",
            |settings: &mut ImageLoaderSettings| {
                        settings
                            .sampler
                            .get_or_init_descriptor();
                            // .set_filter(
                            //     bevy::image::ImageFilterMode::Nearest,
                            // );
                        },
            ),
            ..default()
        },
        Transform::from_xyz(0.0, LEVEL1_HEIGHT / 2.0, 0.0),



));

}
