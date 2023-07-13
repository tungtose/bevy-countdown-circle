use bevy::{math::f32, prelude::*};
use bevy_prototype_lyon::prelude::*;

const TIME_OUT: f32 = 15.;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup_system)
        .add_systems(Update, animate_system)
        .run();
}

#[derive(Component)]
pub struct ArcShape;

#[derive(Component)]
pub struct Angle(f32);

fn animate_system(mut query: Query<(&mut Path, &mut Angle), With<ArcShape>>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for (mut path, mut ang) in query.iter_mut() {
        let mut path_builder = PathBuilder::new();

        ang.0 = f32::to_radians(f32::to_degrees(ang.0) - (360. / TIME_OUT) * delta);

        if ang.0 < f32::to_radians(-360.) || ang.0 > f32::to_radians(360.) {
            ang.0 = f32::to_radians(0.);
        }

        path_builder.move_to(Vec2::new(0., 0.));
        path_builder.line_to(Vec2::new(0., 100.));
        path_builder.arc(Vec2::new(0., 0.), Vec2::new(100., 100.), ang.0, 0.);

        path_builder.close();
        let new_path = path_builder.build();

        *path = new_path;
    }
}

fn setup_system(mut commands: Commands) {
    let ang = Angle(f32::to_radians(0.0));
    let mut path_builder = PathBuilder::new();

    path_builder.line_to(Vec2::new(0., 100.));
    path_builder.arc(Vec2::new(0., 0.), Vec2::new(100., 100.), ang.0, 0.);

    path_builder.close();
    let path = path_builder.build();

    let color = Color::with_a(Color::GRAY, 0.07);

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        ang,
        ArcShape,
        ShapeBundle {
            path,
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Stroke::new(Color::YELLOW, 0.),
        Fill::color(color),
    ));
}
