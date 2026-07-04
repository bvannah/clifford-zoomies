#![allow(unused_variables)]

/*
Clifford Zoomies!
collisions!! when you hit something, you keep 90% of your speed, but going in different direction
you have a small window of time to press the jump button to increase your speed instead
the window of time grows smaller the faster you go
keep track of your speed, get more and more ridiculous the faster you go
sounds get louder and louder

run around on the floor the couch, the walls, out a window and through the roof
sleeping dog animation

each collision obj should:
- at random times:
- spawn in, have extremely small idle animation, and also an animation showing when it will go away
- despawn

it's okay to go off screen in any direction
camera will follow the player instantly

after you break through the atmosphere, you transform into 3d world. more of a traditional platformer.
almost dreamlike. peaceful. a little bit spooky, but mostly mysterious
twinkling stars.
slow music
interesting npcs
    they also jump around unless they see you.
    if they see you, they will stop or say hi
    frogs, people, other dogs, cats, birds
world map! biomes, random spawning, etc.
*/


/*
TODO:
turn off bboxes for release version
make player collision box much shorter -- half the height. this will require separating sprite from collision box
add detailed background texture to make it look like you are moving faster
make furns spawn within radius of player, quite wide, pretty tall, not below though
    if there are too many furns, dont spawn more?
improve movement a LOT -- the balance between vertical and horizontal movement seems wrong
make a background image -- custom, blank room that can be repeated
add custom furniture like desks and stuff that have colliders, random furniture will not spawn there


*/

use bevy::{prelude::*, sprite_render::Material2dPlugin};
use bevy_rapier2d::prelude::*;

mod not_bevy;
use not_bevy::animation::*;
use not_bevy::constants_and_startup::{
    startup, BackgroundMaterial, PLAYER_SIZE, pspawn_x, pspawn_y, pspawn_z,
};
use not_bevy::player_components::Player;
use not_bevy::spawn_sprites::*;


// #[require(Gravity(1000.), Velocity)]
// #[require(Velocity)]


// #[derive(Component)]
// struct Gravity(f32);

// #[derive(Component, Default)]
// struct Velocity(Vec2);

#[derive(Event)]
struct EndGame;

#[derive(Event)]
struct Bounce;

fn main() -> AppExit {
    App::new()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        .add_plugins(DefaultPlugins)
        .add_plugins((FurnPlugin, Material2dPlugin::<BackgroundMaterial>::default(
            ),))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(AtlasAnimationPlugin)
        .add_systems(Startup, startup)
        // .add_systems(FixedUpdate, (gravity, rl_move, check_in_bounds, check_collisions).chain())
        // .add_systems(FixedUpdate, (rl_move, check_in_bounds, check_collisions).chain())
        // .add_systems(FixedUpdate, (check_in_bounds, check_collisions, display_events).chain())
        .add_systems(Update, (display_events, controls))
        .add_systems(PostUpdate, (move_camera, update_player_animations))
        .add_observer(respawn_on_endgame)
        .run()
}

fn controls(
    mut velocity: Single<&mut Velocity, With<Player>>,
    // player_entity: Single<&mut Entity, With<Player>>,
    // mut player: Single<&mut Player>,
    mut player_query : Query<(&mut Player)>,
    mut sprite_query : Query<(&mut Sprite, &mut Animator)>,
    collider_query: Query<&Collider>,
    mut collision_events: MessageReader<CollisionEvent>,
    mut contact_force_events: MessageReader<ContactForceEvent>,
    buttons: Res<ButtonInput<KeyCode>>, 
) {

    if let Ok(mut player) = player_query.single_mut() &&
        let Ok((mut sprite, mut animator)) = SpriteImageMode_query.single_mut(){

        // update grounded status
        // TODO: should really be based on colliders
        if velocity.linear.y == 0.0{
            player.grounded = true;
        }
        else {
            player.grounded = false;
        }

        // direction facing
        if velocity.linear.x < 0. {
            sprite.flip_x = false;
        }
        else if velocity.linear.x > 0.{
            sprite.flip_x = true;
        }


        // update LR collider status for left-right

        // update bounce timer regardless. we will set it to 0 later if we want to
        if player.bounce_timer != 0{
            player.bounce_timer += 1;
        }

        if !player.grounded && (velocity.linear.y - 0.0).abs() < 100.{
            animator.animation = "fall".to_string();
        }
        else if velocity.linear.y > 100. && player.bounce_timer > jump_end_frames * fps_converter{
            // if jump animation is done
            animator.animation = "float_up".to_string();
        }
        else if velocity.linear.y < -100. && player.bounce_timer > jump_end_frames * fps_converter{
            // if arbitrary time duration is done (dont want animation changing a bunch when we are doing small bounces on the ground)
            animator.animation = "float_down".to_string();
        }

        if buttons.just_pressed(KeyCode::KeyW// here, we can replace this first velocity with W, and maybe velocity needs to be a 3d vector to handle the movement. or, y_velocity, x_velocity, z_velocity
        ) {
            if false{ // TODO: LR collider, bounce!
                player.bounce_timer =  1; // reset bounce timer
                // play abbreviated jump animation
                // add y velocity if it's low, multiply y velocity, multiply/bounce x velocity
            }
            else if player.grounded && player.bounce_timer == 0{ // start basic jump
                player.bounce_timer += 1;
                animator.animation = "jump".to_string();
            }
            

        }



        if buttons.pressed(KeyCode::KeyD){
            if velocity.linear.x < 300.{
                velocity.linear.x += 10.;
                // play bound animation
            }
            else if velocity.linear.x < 300.{
                velocity.linear.x = velocity.linear.x.powf(1.05);
                // play run animation
            }
        }
        if buttons.pressed(KeyCode::KeyA){
            if velocity.linear.x > -300.{
                velocity.linear.x += -10.;
                // play bound animation
            }
            else if velocity.linear.x > -300.{
                velocity.linear.x = -((-velocity.linear.x).powf(1.05));
                // play run animation
            }
        }

        if player.grounded{
            if player.bounce_timer == jump_startup_frames * fps_converter{
                velocity.linear.y += 400.;
            }
            if player.bounce_timer > jump_startup_frames * fps_converter{ // this is a stupid way to do this. we want to check collider velocity
                player.bounce_timer = 0;
                animator.animation = "land".to_string();
                // we want to force a pause

            }
            if player.bounce_timer == 0{
                if velocity.linear.x == 0.{
                    animator.animation = "idle".to_string();
                }
                else {
                    animator.animation = "walk".to_string();
                }
            }
            
        }





    }
}


    // // let mut total_player_force = Vec2::ZERO;
    // // let mut is_any_contact: bool = false;
    // // let mut forces_count: i32 = 0;

    // // we want to update player collision state every frame so that we can do animations
    // // if true{
    // // if buttons.pressed(KeyCode::KeyD) || buttons.pressed(KeyCode::KeyA) || buttons.pressed(KeyCode::KeyW){
    //     // if any of our interesting keys are pressed, calc all collisions an


    //     // update grounded status
    //     if velocity.linear.y == 0.0{
    //         player.grounded = true;
    //     }
    //     else {
    //         player.grounded = false;
    //     }
    //     // update non-ground colliders for bouncing





    //         // TODO:
    //         // detect if touching any collider
    //         // need angle, collision status, etc

    //         // https://rapier.rs/docs/user_guides/bevy_plugin/advanced_collision_detection/

    //         // if touching collider, calculate angle
    //         // multiply clifford jump velocity by angle, give extra small boost to each direction
    //         // if vert velocity is 0, do a different calculation? (just add)


    //     // TODO: filter by events that actually touch the player, not just all contact force events
    //     // because sometimes there are cfe that don't?? for some reason?


    //     // for cfe in contact_force_events.read(){
    //     //     // info!("Contact: {:?} <-> {:?}", cfe.collider1, cfe.collider2);
    //     //     // for now, everything should be a match because the player is the only thing that can move
    //     //     if cfe.collider2.index() == player_entity.1.index(){// || cfe.collider2.index() == player_entity.index(){
    //     //         // info!("Match!");
    //     //         // what do these forces acually mean? they are very large integers.
    //     //         // if i time things perfectly, it does seem like the forces can be negative or positive depending on direction

    //     //         total_player_force.y += cfe.total_force.y;
    //     //         total_player_force.x += cfe.total_force.x;
    //     //         is_any_contact = true;
    //     //         forces_count += 1;

    //     //     }
    //     //     else if cfe.collider1.index() == player_entity.1.index(){
    //     //         is_any_contact = true;
    //     //     }
    //     // }
    // // }

    // if buttons.pressed(KeyCode::KeyD){
    //     if velocity.linear.x < 300.{
    //         velocity.linear.x += 10.;
    //         // play bound animation
    //     }
    //     else if velocity.linear.x < 300.{
    //         velocity.linear.x = velocity.linear.x.powf(1.05);
    //         // play run animation
    //     }
    // }
    // if buttons.pressed(KeyCode::KeyA){
    //     if velocity.linear.x > -300.{
    //         velocity.linear.x += -10.;
    //         // play bound animation
    //     }
    //     else if velocity.linear.x > -300.{
    //         velocity.linear.x = -((-velocity.linear.x).powf(1.05));
    //         // play run animation
    //     }
    // }

    // if buttons.just_pressed(KeyCode::KeyW// here, we can replace this first velocity with W, and maybe velocity needs to be a 3d vector to handle the movement. or, y_velocity, x_velocity, z_velocity
    // ) {


    //     // if total_player_force.y == 0.0{
    
    //     //     if is_any_contact{
    //     //         velocity.linear.y += 300.;
    //     //     }

    //     // }
    //     // else{
    //     //     velocity.linear.y += 300.;
    //     //     // velocity.linear.x += total_player_force.x/forces_count as f32;
    //     //     // velocity.linear.y += total_player_force.y/forces_count as f32;
    //     //     // velocity.linear += total_player_force;
    //     //     velocity.linear.x *= 1.2;
    //     //     velocity.linear.y *= 1.2;
    //     //     info!("Total force: {}", total_player_force);
    //     //     info!("forces: {}", forces_count)
    //     // }
    // }

fn update_animation(){


}

fn check_in_bounds(
    player: Single<&Transform, With<Player>>,
    mut commands: Commands,
) {
    if player.translation.y < -0.0 -PLAYER_SIZE{ // || player.translation.y>CANVAS_SIZE.y/2.0+PLAYER_SIZE{
        info!("check_in_bounds");
        commands.trigger(EndGame);
    }
 }

fn respawn_on_endgame(
    _: On<EndGame>,
    mut commands: Commands,
    player: Single<Entity, With<Player>>,
) {
    commands.entity(*player).insert((
        Transform::from_xyz(pspawn_x, pspawn_y, pspawn_z),
        Velocity {
            linear: Vec2::new(1.0, 2.0),
            angular: 0.0,
        },
    ));
}

fn move_camera(
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    time: Res<Time>,
) {
    let lerp_speed = 10.0; // Adjust for smoother/snappier tracking
    let target = Vec3::new(
        player.translation.x,
        player.translation.y,
        camera.translation.z,
    );
    camera.translation = camera.translation.lerp(target, lerp_speed * time.delta_secs());
}   

/* A system that displays the events. */
fn display_events(
    mut collision_events: MessageReader<CollisionEvent>,
    mut contact_force_events: MessageReader<ContactForceEvent>,
) {
    for collision_event in collision_events.read() {
        // println!("Received collision event: {:?}", collision_event);
        info!("Received collision event: {:?}", collision_event);

    }

    for contact_force_event in contact_force_events.read() {
        info!("Received contact force event: {:?}", contact_force_event);
    }
}

fn update_player_collisions(player_entity: Single<Entity, With<Player>>){


}


fn update_player_animations(
    mut player_query : Query<(&mut Player,&mut Sprite,&mut Animator)>,
    velocity: Single<& Velocity, With<Player>>
) {
    if let Ok((player,mut sprite,mut animator)) = player_query.single_mut() {

        if velocity.linear.x < 0. {
            sprite.flip_x = false;
        }
        else if velocity.linear.x > 0.{
            sprite.flip_x = true;
        }

        // // boring low-priority animations first so we can overwrite them later
        // if player.grounded{
        //     if velocity.linear.x == 0.{
        //         animator.animation = "idle".to_string();
        //     }
        //     else {
        //         animator.animation = "walk".to_string();
        //     }
        // }
        
        // if !player.grounded {
        //     // some animation logic based on velocity, bounce timer, etc.
        //     // decide between a bunch of stuff
        //     // animator.animation = "jump".to_string();
        // }
        // else{
        //     ()

        // }
    
    }
}
