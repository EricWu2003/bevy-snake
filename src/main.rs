use bevy::prelude::*;
use rand::prelude::random;
use bevy::time::FixedTimestep;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x, height: x
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_snake(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: SNAKE_HEAD_COLOR,
            ..default()
        },
        transform: Transform { 
            scale: Vec3::new(10.0, 10.0, 10.0),
            ..default()
        },
        ..default()
    })
    .insert(SnakeHead)
    .insert(Position {x:3, y:3})
    .insert(Size::square(0.8));
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let Some(window) = windows.get_primary() else {
        return
    };
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window/2.) + (tile_size/2.)
    }
    let Some(window) = windows.get_primary() else {
        return
    };    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_HEIGHT as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0
        )
    }
}

fn snake_movement(
    mut head_positions: Query<&mut Position, With<SnakeHead>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for mut position in head_positions.iter_mut () {
        if keyboard_input.pressed(KeyCode::Left) {
            position.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            position.x += 1;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            position.y += 1;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            position.y -= 1;
        }
    }
}

#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
struct Food;


fn food_spawner(mut commands: Commands) {
    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        }
    )
    .insert(Food)
    .insert(Position {
        x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
        y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
    })
    .insert(Size::square(0.8));
}

fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system(snake_movement)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling)
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(food_spawner)
        )
        .add_plugins(DefaultPlugins.set(WindowPlugin{ 
            window: WindowDescriptor{
                title: "Snake".to_string(),
                width: 500.0,
                height: 500.0,
                ..default()
            },
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_system(bevy::window::close_on_esc)
        .run();
}
