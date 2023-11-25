use bevy::prelude::*;
use crate::{player::*, character::MoveTo};

const FOLLOW_RADIUS: f32 = 150.;

pub struct PartnerPlugin;
impl Plugin for PartnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, follow_player);
    }
}

#[derive(Component)]
pub struct Partner;

pub fn follow_player(
    mut commands: Commands,
    player: Query<&Transform, With<Controlled>>,
    parnters: Query<(&Transform, Entity), With<Partner>>,
) {
    let player = player.single();
    for (partner_transform, partner) in parnters.iter() {
        let vector = player.translation.xy() - partner_transform.translation.xy();
        let distance = vector.length();

        if distance > FOLLOW_RADIUS {
            let follow_point = player.translation.xy() - (vector.normalize() * FOLLOW_RADIUS);
            commands.entity(partner).insert(MoveTo {position: follow_point});
        }
    }
}