use super::constants_and_startup::*;
use super::player_components::*;
use super::spawn_sprites::*;
use bevy::{ecs::entity::EntityIndex, prelude::*};
use bevy_rapier2d::dynamics::Velocity;
use bevy_rapier2d::{plugin::ReadRapierContext, rapier::geometry::CollisionEventFlags};

fn resolve_collision_entities<'a>(
    entity1: &'a Entity,
    entity2: &'a Entity,
    player_id: EntityIndex,
) -> Option<(&'a Entity, &'a Entity)> {
    if entity1.index() == player_id {
        Some((entity1, entity2))
    } else if entity2.index() == player_id {
        Some((entity2, entity1))
    } else {
        None
    }
}

fn orient_collision_normal(normal: Vec2, player_entity: &Entity, entity1: &Entity) -> Vec2 {
    if player_entity.index() == entity1.index() {
        normal
    } else {
        -normal
    }
}

pub fn process_bounce_directions(
    entity1: &Entity,
    entity2: &Entity,
    // flags: CollisionEventFlags,
    player: &mut Player,
    player_id: EntityIndex,
    is_start: bool,
    rapier_context: &ReadRapierContext,
    velocity: &Velocity,
    wall_query: Query<&NoBounceWall>,
) {
    let Some((player_entity, other_entity)) =
        resolve_collision_entities(entity1, entity2, player_id)
    else {
        return;
    };

    let other_id = other_entity.index_u32();

    if !is_start {
        if let Some(direction_i8) = player.collider_map.remove(&other_id) {
            if direction_i8 == 0_i8 {
                player.right_walled -= 1;
            }
            if direction_i8 == 1_i8 {
                player.left_walled -= 1;
            }
            if direction_i8 == 2_i8 {
                player.bottom_walled -= 1;
            }
            info!(
                "Player walls: {}, {}, {}",
                player.left_walled, player.right_walled, player.bottom_walled
            );
            info!("{:#?}", player.collider_map);
        }
        return;
    }

    let context = rapier_context.single().unwrap();
    if let Some(contact_pair) = context.contact_pair(*entity1, *entity2) {
        for manifold in contact_pair.manifolds() {
            let normal = orient_collision_normal(manifold.local_n1(), player_entity, entity1);
            info!("normal {:?}", normal);
            if normal.x > 0.0 {
                player.right_walled += 1;
                player.collider_map.insert(other_id, 0_i8);
                player.right_forgiveness = BOUNCE_FORGIVENESS;
            }
            if normal.x < 0.0 {
                player.left_walled += 1;
                player.collider_map.insert(other_id, 1_i8);
                player.left_forgiveness = BOUNCE_FORGIVENESS;
            }
            if normal.x == 0.0 && normal.y < 0.0 {
                if !wall_query.contains(*other_entity) {
                    player.bottom_walled += 1;
                    player.collider_map.insert(other_id, 2_i8);
                    player.bottom_forgiveness = BOUNCE_FORGIVENESS;
                }
            }
        }
        info!(
            "Player walls: {}, {}, {}",
            player.left_walled, player.right_walled, player.bottom_walled
        );
        info!("{:#?}", player.collider_map);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_player_when_it_is_the_second_entity() {
        let mut world = World::new();
        let entity1 = world.spawn_empty().id();
        let entity2 = world.spawn_empty().id();

        let (player_entity, other_entity) =
            resolve_collision_entities(&entity1, &entity2, entity2.index()).unwrap();

        assert_eq!(player_entity, &entity2);
        assert_eq!(other_entity, &entity1);
    }

    #[test]
    fn returns_none_when_player_is_not_in_the_pair() {
        let mut world = World::new();
        let entity1 = world.spawn_empty().id();
        let entity2 = world.spawn_empty().id();
        let entity3 = world.spawn_empty().id();

        let resolved = resolve_collision_entities(&entity1, &entity2, entity3.index());

        assert!(resolved.is_none());
    }

    #[test]
    fn flips_normal_when_player_is_the_second_entity() {
        let mut world = World::new();
        let entity1 = world.spawn_empty().id();
        let entity2 = world.spawn_empty().id();

        let normal = orient_collision_normal(Vec2::new(1.0, 0.0), &entity2, &entity1);

        assert_eq!(normal, Vec2::new(-1.0, 0.0));
    }
}
