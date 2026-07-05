use bevy::{prelude::*, ecs::entity::EntityIndex};
use bevy_rapier2d::dynamics::Velocity;
use bevy_rapier2d::{plugin::ReadRapierContext, rapier::geometry::CollisionEventFlags};
use super::player_components::*;
use super::constants_and_startup::*;

pub fn process_bounce_directions(
    entity1: &Entity,
    entity2: &Entity,
    // flags: CollisionEventFlags,
    mut player: &mut Player,
    player_id: EntityIndex,
    is_start: bool,
    rapier_context: &ReadRapierContext,
    velocity: & Velocity,

){

            if entity1.index() == player_id{
                if let Some(direction_i8) = player.collider_map.remove(&entity2.index_u32()){
                    if direction_i8 == 0_i8{
                        player.right_walled -= 1;
                    }
                    if direction_i8 == 1_i8{
                        player.left_walled -= 1;
                    }
                    if direction_i8 == 2_i8{
                        player.bottom_walled -= 1;
                    }
                info!("Player walls: {}, {}, {}", player.left_walled, player.right_walled, player.bottom_walled);
                return // early exit because this collision is a stop, not a start
                }



                // Find the contact points for this pair of colliders
                let context = rapier_context.single().unwrap();
                if let Some(contact_pair) = context.contact_pair(*entity1, *entity2) {
                    // Find the deepest contact between the two manifolds
                    for manifold in contact_pair.manifolds() {
                        let normal = manifold.local_n1(); // The surface normal                                
                        info!("normal {:?}", normal);
                        if normal.x > 0.0{
                            if is_start{
                                player.right_walled += 1;
                                player.collider_map.insert(entity2.index_u32(), 0_i8);
                                player.right_forgiveness = BOUNCE_FORGIVENESS;

                            }
                        }
                        if normal.x < 0.0{
                            if is_start{
                                player.left_walled += 1;
                                player.collider_map.insert(entity2.index_u32(), 1_i8);
                                player.bottom_forgiveness = BOUNCE_FORGIVENESS;


                            }
                        }
                        // info!("velocity y is {}", velocity.linear.y);
                        if normal.x == 0.0 && normal.y < 0.0{// && velocity.linear.y > 0.{
                            if is_start{
                                player.bottom_walled += 1;
                                player.collider_map.insert(entity2.index_u32(), 2_i8);
                                player.bottom_forgiveness = BOUNCE_FORGIVENESS;

                            }
                        }
                    }
                    info!("Player walls: {}, {}, {}", player.left_walled, player.right_walled, player.bottom_walled);
                }
                // if let Ok((en2_collider, en2_transform)) = collider_query.get(*entity2){
                //     let (is_left_bounce, is_right_bounce, is_bottom_bounce) = get_bounce_directions(player_position, en2_transform.translation);
                    
                // }    
            }


}
