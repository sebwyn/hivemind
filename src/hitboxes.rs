use super::character::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct HitBox;

#[derive(Component)]
pub struct HurtBox {
    pub damage: f32,
}

pub struct HitboxPlugin;
impl Plugin for HitboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_collisions);
    }
}

fn check_collisions(
    mut commands: Commands,
    hit_boxes: Query<(&Parent, &GlobalTransform), With<HitBox>>,
    hurt_boxes: Query<(&Parent, &GlobalTransform, &HurtBox)>,
    mut hit_parent_q: Query<(&mut Health, &Team)>,
    hurt_parent_q: Query<&Team>,
) {
    let mut the_notebook_from_the_deathnote = Vec::new();
    //let hurt_boxes_list = hurt_boxes.iter().collect();
    for (hit_p, hit_transform) in hit_boxes.iter() {
        let x1 = hit_transform.translation().x - hit_transform.compute_transform().scale.x / 2.;
        let x2 = hit_transform.translation().x + hit_transform.compute_transform().scale.x / 2.;

        let y1 = hit_transform.translation().y - hit_transform.compute_transform().scale.y / 2.;
        let y2 = hit_transform.translation().y + hit_transform.compute_transform().scale.y / 2.;
        for (hurt_p, hurt_transform, HurtBox { damage }) in hurt_boxes.iter() {
            let x1_hurt =
                hurt_transform.translation().x - hurt_transform.compute_transform().scale.x / 2.;
            let x2_hurt =
                hurt_transform.translation().x + hurt_transform.compute_transform().scale.x / 2.;

            let y1_hurt =
                hurt_transform.translation().y - hurt_transform.compute_transform().scale.y / 2.;
            let y2_hurt =
                hurt_transform.translation().y + hurt_transform.compute_transform().scale.y / 2.;

            if (x2_hurt > x1 && x1_hurt < x2) && (y2_hurt > y1 && y1_hurt < y2) {
                let (mut hit_health, hit_team) = hit_parent_q.get_mut(hit_p.get()).unwrap();
                let hurt_team = hurt_parent_q.get(hurt_p.get()).unwrap();
                if hurt_team != hit_team {
                    hit_health.health -= damage;
                    the_notebook_from_the_deathnote.push(hurt_p.get());
                }
            }
        }
    }

    the_notebook_from_the_deathnote.dedup();
    for victim in the_notebook_from_the_deathnote.into_iter() {
        commands.entity(victim).despawn_recursive();
    }
}
