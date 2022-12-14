use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

// for detecting when velocity is "zero"
const VEC_EPSILON: f32 = 0.01;

// normals of the 3D model
const NORMALS: [Vec3; 6] = [
    Vec3::X,     // 1
    Vec3::NEG_Y, // 2
    Vec3::Z,     // 3
    Vec3::NEG_Z, // 4
    Vec3::Y,     // 5
    Vec3::NEG_X, // 6
];

#[derive(Component)]
struct Die {
    value: Option<usize>,
}

impl Default for Die {
    fn default() -> Self {
        Die { value: None }
    }
}

#[derive(Component)]
struct DiceText;

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
        .add_system(find_rolled_value)
        .add_system(update_text)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(AssetHandles {
        die: asset_server.load("die.glb#Scene0"),
    });

    commands.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(3.),
                left: Val::Px(6.),
                ..default()
            },
            ..default()
        },
        text: Text {
            sections: vec![TextSection {
                value: "Click anywhere to roll dice".to_string(),
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
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(30.),
                    left: Val::Px(6.),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "? + ? + ? + ? + ? = ?".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                }],
                ..default()
            },
            ..default()
        })
        .insert(DiceText);

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

fn update_text(mut text: Query<&mut Text, With<DiceText>>, dice: Query<&Die>) {
    let mut t = text.get_single_mut().unwrap();
    let values = dice
        .iter()
        .map(|die| {
            if let Some(val) = die.value {
                val.to_string()
            } else {
                "?".to_string()
            }
        })
        .collect::<Vec<String>>();
    t.sections[0].value = values.join(" ");
}

fn throw_dice(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    asset_handles: Res<AssetHandles>,
    query: Query<Entity, With<Die>>,
) {
    let mut rng = rand::thread_rng();

    // spawn positions
    let dice = vec![
        Vec3::new(10.0, 4.0, 0.0),
        Vec3::new(10.0, 4.0, 1.0),
        Vec3::new(10.0, 4.0, -1.0),
        Vec3::new(12.0, 4.0, 1.0),
        Vec3::new(12.0, 4.0, -1.0),
    ];

    if buttons.just_released(MouseButton::Left) {
        for ent in query.iter() {
            commands.entity(ent).despawn_recursive();
        }

        for trans in dice.iter() {
            let impulse = Vec3::new(rng.gen_range(-12.0..=-8.0), rng.gen_range(1.0..=3.0), 0.0);

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
                .insert(Velocity::default())
                .insert(Collider::cuboid(1.0, 1.0, 1.0))
                .insert(ExternalImpulse {
                    impulse,
                    torque_impulse,
                })
                .insert(Die::default());
        }
    }
}

fn find_rolled_value(mut query: Query<(&Transform, &Velocity, &mut Die), With<Die>>) {
    for (trans, vel, mut die) in query.iter_mut() {
        if vel.linvel.abs().max_element() < VEC_EPSILON {
            let mut rolled_num: usize = 1;
            let mut max_distance: f32 = -10.0;

            for (i, dir) in NORMALS.iter().enumerate() {
                let val = trans.rotation.mul_vec3(*dir).y;
                if val > max_distance {
                    rolled_num = i + 1;
                    max_distance = val;
                }
            }

            die.value = Some(rolled_num);
        }
    }
}
