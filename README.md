# bevy_transform64
A 64-bit version of Bevy Transform that enhances precision and minimizes floating-point errors in large-scale projects. This custom implementation substitutes Transform with DTransform and GlobalTransform with DGlobalTransform. The original GlobalTransform is retained for rendering purposes.

# Usage
To integrate Bevy Transform64 into your project, follow these steps:

1. Add DTransformPlugin to your Bevy app:

``` rust
App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(DTransformPlugin)
    .run();
```

2. Define the WorldOrigin resource and set it to the camera position or camera entity:

``` rust
let camera = commands.spawn(Camera3dBundle {
    ..Default::default()
}).insert(DTransform::from_xyz(5.0, 0.0, 5.0).looking_at(DVec3::ZERO, DVec3::Y)).id();
commands.insert_resource(WorldOrigin::Entity(camera));
```

3. Use DTransform for transformations. For example, to create a cube with a DTransform:


``` rust
let base_cube = commands.spawn(PbrBundle {
    mesh: mesh.clone(),
    material: material.clone(),
    visibility : Visibility::Visible,
    ..Default::default()
}).insert(DTransform::from_xyz(0.0, 0.0, 0.0)).id();
```

# Example
Here's an example demonstrating how to use Bevy Transform64:

``` rust

use bevy::{prelude::*, math::DVec3};
use bevy_transform64::{prelude::*, WorldOrigin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DTransformPlugin)
        .add_systems(Startup, setup) 
        .add_systems(Update, camera_orbit)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let mesh = meshes.add(Cuboid::new(0.5, 0.5, 0.5));
    let material = materials.add(Color::WHITE);

    let base_cube = commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(material.clone()),
        DTransform::from_xyz(0.0, 0.0, 0.0),
    )).id();

    let base_cube_2 = commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(material.clone()),
        DTransform::from_xyz(0.0, 1.0, 0.0),
    )).id();

    
    let base_cube_3 = commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        DTransform::from_xyz(0.0, 1.0, 0.0),
    )).id();

    commands.entity(base_cube).add_child(base_cube_2);
    commands.entity(base_cube_2).add_child(base_cube_3);

    let camera = commands.spawn(Camera3d {
        ..Default::default()
    }).insert(DTransform::from_xyz(5.0, 0.0, 5.0).looking_at(DVec3::ZERO, DVec3::Y)).id();

    commands.insert_resource(WorldOrigin::Entity(camera));
}

fn camera_orbit(
    mut dtransforms : Query<&mut DTransform, With<Camera>>,
    time : Res<Time>
) {
    let speed = 1.0;
    for mut dtransform in dtransforms.iter_mut() {
        let angle:f32 = speed * time.elapsed_secs();
        let x:f32 = angle.cos() * 5.0;
        let z:f32 = angle.sin() * 5.0;
        dtransform.translation.x = x as f64;
        dtransform.translation.z = z as f64;
        let target = DVec3::new(0.0, dtransform.translation.y, 0.0);
        dtransform.look_at(target, DVec3::Y);
    }
}
```
