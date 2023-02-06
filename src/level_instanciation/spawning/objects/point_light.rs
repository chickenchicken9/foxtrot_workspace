use crate::level_instanciation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use bevy::prelude::*;

pub struct PointLightSpawner;

impl PrimedGameObjectSpawnerImplementor for PointLightSpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a, '_>,
        _object: GameObject,
    ) {
        spawner.commands.spawn((
            PointLightBundle {
                point_light: PointLight {
                    color: Color::WHITE,
                    intensity: 1.0,
                    range: 1.0,
                    radius: 1.0,
                    shadows_enabled: true,
                    ..default()
                },
                ..default()
            },
            Name::new("Light"),
        ));
    }
}
