use super::character::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_player, camera_follows_player));
    }
}

fn move_player(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(Entity, &mut Transform, &Character), With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let (entity, mut transform, character) = player_query.single_mut();

    if keys.pressed(KeyCode::W) {
        transform.translation.y += character.move_speed;
    }

    if keys.pressed(KeyCode::A) {
        transform.translation.x -= character.move_speed;
    }

    if keys.pressed(KeyCode::S) {
        transform.translation.y -= character.move_speed;
    }

    if keys.pressed(KeyCode::D) {
        transform.translation.x += character.move_speed;
    }

    if keys.just_pressed(KeyCode::Space) {
        let (camera, camera_transform) = camera_query.single();

        let Some(cursor_position) = windows.single().cursor_position() else {
            return;
        };
        let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
            return;
        };

        commands
            .entity(entity)
            .insert((Focus { position: point }, Attacking));
    }
}

fn camera_follows_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
