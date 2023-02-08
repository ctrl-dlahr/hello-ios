use bevy::{prelude::{*, shape::Cube}, window::WindowId, winit::WinitWindows};
use image;
use std::{io::Cursor, f32::consts::{PI}};
use winit::window::Icon;

fn main() {
    info!("Loading iOS");
    App::new()
        .add_startup_system(set_window_icon)
        .add_plugins(DefaultPlugins)
        .add_startup_system(draw_cube)
        .add_system(rotate_cube)
        .run();    
}

fn set_window_icon(windows: NonSend<WinitWindows>) {
    let window = windows.get_window(WindowId::primary()).expect("no window");
    let (icon_rgba, icon_width, icon_height) = {
        let icon_buf = Cursor::new(include_bytes!("../static/appicon.png"));
        let rgba = image::load(icon_buf, image::ImageFormat::Png)
            .expect("Failed to open icon path")
            .into_rgba8();

        let (width, height) = rgba.dimensions();
        let icon_raw = rgba.into_raw();
        (icon_raw, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height)
        .expect("error making icon");
    window.set_window_icon(Some(icon));
}

fn draw_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 800.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 4.0, 4.0),
        ..default()
    });
    
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(1.0, 1.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cube { size: 0.5 }.into()),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
       Rotatable { speed: 0.5 },
    ));
}

#[derive(Component)]
struct Rotatable {
    speed: f32,
}

fn rotate_cube(mut cubes: Query<(&mut Transform, &Rotatable)>, timer: Res<Time>) {
    for (mut transform, cube) in &mut cubes {
        transform.rotate_x(cube.speed * PI * timer.delta_seconds());
        if transform.translation.x > 2.0 {
            transform.translation.x -= 5.0;
            transform.translation.x -= timer.delta_seconds();
        }
        
        if transform.translation.x < 2.0 {
            transform.translation.x += timer.delta_seconds();
        }
    }
}