use crate::level_instanciation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;

pub struct OrbSpawner;

impl PrimedGameObjectSpawnerImplementor for OrbSpawner {
    fn create_mesh(&self, mesh_assets: &mut ResMut<Assets<Mesh>>) -> Option<Handle<Mesh>> {
        Some(mesh_assets.add(Mesh::from(shape::UVSphere {
            radius: 1.0,
            ..default()
        })))
    }
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a, '_>,
        object: GameObject,
    ) {
        spawner
            .commands
            .spawn((
                MaterialMeshBundle {
                    mesh: spawner.outer_spawner.meshes[&object].clone(),
                    material: spawner.materials.glowy.clone(),
                    transform: Transform::from_translation((0., 1.5, 0.).into()),
                    ..default()
                },
                Name::new("Orb"),
                NotShadowCaster,
            ))
            .with_children(|parent| {
                parent.spawn((PointLightBundle {
                    point_light: PointLight {
                        intensity: 10_000.,
                        radius: 1.,
                        color: Color::rgb(0.5, 0.1, 0.),
                        shadows_enabled: true,
                        ..default()
                    },
                    ..default()
                },));
            });
    }
}
