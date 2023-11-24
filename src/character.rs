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
    FRIEND,
    FOE,
}
#[derive(Component)]
pub struct Focus {
    pub position: Vec2,
}
#[derive(Component)]
pub struct Attacking;

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_dead_characters);
    }
}

fn despawn_dead_characters(mut commands: Commands, characters: Query<(Entity, &Health)>) {
    for (entity, health) in characters.iter() {
        if health.health <= 0. {
            commands.entity(entity).despawn_recursive();
        }
    }
}
