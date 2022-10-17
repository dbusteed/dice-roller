use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

#[derive(Component)]
struct Die;

struct AssetHandles {
    die: Handle<Scene>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(throw_dice)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let asset_handles = AssetHandles {
        die: asset_server.load("die.glb#Scene0"),
    };
    commands.insert_resource(asset_handles);

    commands.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(3.),
                left: Val::Px(5.),
                ..default()
            },
            ..default()
        },
        text: Text {
            sections: vec![TextSection {
                value: "Press SPACE to roll dice".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            }],
            ..default()
        },
        ..default()
    });

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(10.0, 1.0, 10.0))),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: materials.add(StandardMaterial {
                base_color: Color::DARK_GREEN,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(5., 0.5, 5.));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 3.0, 10.0))),
            transform: Transform::from_xyz(-5.5, 1.0, 0.0),
            material: materials.add(StandardMaterial {
                base_color: Color::DARK_GRAY,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 1.5, 5.0));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(11.0, 3.0, 1.0))),
            transform: Transform::from_xyz(-0.5, 1.0, -5.5),
            material: materials.add(StandardMaterial {
                base_color: Color::DARK_GRAY,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(5.5, 1.5, 0.5));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(11.0, 3.0, 1.0))),
            transform: Transform::from_xyz(-0.5, 1.0, 5.5),
            material: materials.add(StandardMaterial {
                base_color: Color::DARK_GRAY,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(5.5, 1.5, 0.5));

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 10.0, 2.0),
        ..default()
    });

    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 15.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn throw_dice(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    asset_handles: Res<AssetHandles>,
    query: Query<Entity, With<Die>>,
) {
    let mut rng = rand::thread_rng();

    let dice = vec![
        Vec3::new(10.0, 4.0, 0.0),
        Vec3::new(10.0, 4.0, 1.0),
        Vec3::new(10.0, 4.0, -1.0),
        Vec3::new(12.0, 4.0, 1.0),
        Vec3::new(12.0, 4.0, -1.0),
    ];

    if keyboard_input.just_released(KeyCode::Space) {
        for ent in query.iter() {
            commands.entity(ent).despawn_recursive();
        }

        for trans in dice.iter() {
            let impulse = Vec3::new(rng.gen_range(-12.0..=-8.0), rng.gen_range(0.0..=2.0), 0.0);

            let torque_impulse = Vec3::new(
                rng.gen_range(-2.0..=2.0),
                rng.gen_range(-1.0..=1.0),
                rng.gen_range(-2.0..=2.0),
            );

            commands
                .spawn_bundle(SceneBundle {
                    scene: asset_handles.die.clone(),
                    transform: Transform {
                        translation: *trans,
                        scale: Vec3::new(0.5, 0.5, 0.5),
                        ..default()
                    },
                    ..default()
                })
                .insert(RigidBody::Dynamic)
                .insert(Collider::cuboid(1.0, 1.0, 1.0))
                .insert(ExternalImpulse {
                    impulse,
                    torque_impulse,
                })
                .insert(Die);
        }
    }
}
