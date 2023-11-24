use bevy::prelude::*;

#[derive(Component)]
pub struct Fuse(pub Timer);

#[derive(Component)]
pub struct Velocity(pub Vec2);

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_projectiles, tick_projectiles));
    }
}

fn move_projectiles(mut moving_objects: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in moving_objects.iter_mut() {
        transform.translation.x += velocity.0.x;
        transform.translation.y += velocity.0.y;
    }
}

fn tick_projectiles(
    mut commands: Commands,
    mut moving_objects: Query<(&mut Fuse, Entity)>,
    time: Res<Time>,
) {
    for (mut fuse, entity) in moving_objects.iter_mut() {
        fuse.0.tick(time.delta());

        if fuse.0.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
