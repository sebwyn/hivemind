use bevy::prelude::*;

#[derive(Component)]
pub struct Character {
    pub move_speed: f32,
}
#[derive(Component)]
pub struct Health {
    pub health: f32,
    pub max_health: f32,
}
#[derive(Component, PartialEq, Clone, Copy)]
pub enum Team {
    Friend,
    Foe,
}
#[derive(Component)]
pub struct Focus {
    pub position: Vec2,
}
#[derive(Component, Default)]
pub struct Attacking;

#[derive(Component)]
pub struct MoveTo {
    pub position: Vec2
}

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_dead_characters);
        app.add_systems(Update, move_characters);

    }
}

fn despawn_dead_characters(mut commands: Commands, characters: Query<(Entity, &Health)>) {
    for (entity, health) in characters.iter() {
        if health.health <= 0. {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn move_characters(mut commands: Commands, mut characters: Query<(Entity, &mut Transform, &Character, &MoveTo)>) {
    for (entity, mut transform, character, move_to) in characters.iter_mut() {
        let move_vector = move_to.position - transform.translation.xy();
        
        if move_vector.length() < character.move_speed {
            
            transform.translation.x += move_vector.x;
            transform.translation.y += move_vector.y;

            commands.entity(entity).remove::<MoveTo>();
        } else {
            let move_direction = move_vector.normalize();

            transform.translation.x += character.move_speed * move_direction.x;
            transform.translation.y += character.move_speed * move_direction.y;
        }
    }
}
