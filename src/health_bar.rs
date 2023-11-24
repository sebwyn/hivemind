use super::character::*;
use bevy::{prelude::*, sprite::Anchor};

#[derive(Component)]
pub struct HealthBar;

pub struct HealthBarPlugin;
impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_health_bars);
    }
}

fn update_health_bars(
    health_bar: Query<(&Parent, &Children), With<HealthBar>>,
    character_health: Query<&Health>,
    mut background_transform: Query<(&mut Transform, &Name)>,
) {
    for (character, children) in health_bar.iter() {
        let health = character_health.get(character.get()).unwrap();
        for &child in children.iter() {
            let (mut transform, name) = background_transform.get_mut(child).unwrap();
            if name.as_str() == "fill" {
                transform.scale.x = health.health / health.max_health;
            }
        }
    }
}

pub fn spawn_health_bar(commands: &mut Commands) -> Entity {
    let parent = commands.spawn((SpatialBundle::default(), HealthBar)).id();

    let background = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(30.0, 5.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 15.0, 0.)),
                ..default()
            },
            Name::new("background"),
        ))
        .id();

    let fill = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 1.0, 0.0),
                    custom_size: Some(Vec2::new(30.0, 5.0)),
                    anchor: Anchor::CenterLeft,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(-15.0, 15.0, 0.)),
                ..default()
            },
            Name::new("fill"),
        ))
        .id();

    commands.entity(parent).push_children(&[background, fill]);

    parent
}
