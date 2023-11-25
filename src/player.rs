use super::character::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Controlled;

#[derive(Component)]
pub struct Mind;

#[derive(Component, Default)]
pub struct Roster {
    pub list: Vec<Entity>, 
    pub current: usize
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_player, camera_follows_player));
    }
}

fn move_player(
    keys: Res<Input<KeyCode>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,

    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform, &Character), With<Controlled>>,
    mut mind_query: Query<&mut Roster>,
) {

    let mut roster = mind_query.single_mut();
    let (entity, mut transform, character) = player_query.single_mut();

    let mut move_direction = Vec2::new(0.,0.);

    if keys.pressed(KeyCode::W) {
        move_direction.y += 1.;
    }

    if keys.pressed(KeyCode::A) {
        move_direction.x -= 1.;
    }

    if keys.pressed(KeyCode::S) {
        move_direction.y -= 1.;
    }

    if keys.pressed(KeyCode::D) {
        move_direction.x += 1.;
    }

    if move_direction.length() >= 1. {
        move_direction = move_direction.normalize() * character.move_speed;

        transform.translation.x += move_direction.x;
        transform.translation.y += move_direction.y;
    }

    if keys.just_pressed(KeyCode::Q) {
        commands.entity(entity).remove::<Controlled>();
        roster.current = (roster.current + 1) % roster.list.len();
        commands.entity(roster.list[roster.current]).insert(Controlled);
    }

    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };
    
    if keys.just_pressed(KeyCode::Space) {
        commands
            .entity(entity)
            .insert((Focus { position: point }, Attacking));
    }
}


pub fn camera_follows_player(
    player: Query<&Transform, With<Controlled>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Controlled>)>,
) {
    let player = player.single();
    let mut transform = camera.single_mut();
        let vector = player.translation.xy() - transform.translation.xy();
        let distance = vector.length();

        if distance > 10. {
            let move_vector = vector.normalize() * 5.;

            transform.translation.x += move_vector.x;
            transform.translation.y += move_vector.y;
        }
}
