use bevy::math::vec3;
use bevy::prelude::*;

mod menu;

use menu::Background;

// Assets
const PLAYER_RED_SPRITE: &str = "player_red.png";
const PLAYER_BLUE_SPRITE: &str = "player_blue.png";
const BALL_SPRITE: &str = "ball.png";
const PITCH1_SPRITE: &str = "pitch1.png";
const PITCH2_SPRITE: &str = "pitch2.png";
const PITCH3_SPRITE: &str = "pitch3.png";
const FONT: &str = "fonts/FiraSans-Regular.ttf";

// Constants
const MAX_SPEED: f32 = 3.0;
const PLAYER_RADIUS: f32 = 25.0;
const BALL_RADIUS: f32 = 10.0;
const CORNER_RADIUS: f32 = 10.0;
const RED_INITIAL_X: f32 = -200.0;
const BLUE_INITIAL_X: f32 = 200.0;
const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 768.0;
const CORNER_UP_HEIGHT: f32 = 100.0;
const CORNER_DOWN_HEIGHT: f32 = -100.0;

// Components
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
struct PlayerRed;

#[derive(Component)]
struct PlayerBlue;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Radius(f32);

#[derive(Component)]
pub struct Score {
    pub red: i32,
    pub blue: i32,
}

#[derive(Component)]
struct ScoreText(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    InMenu,
    InGame,
}

fn main() {
    App::new()
        // First, we initialize the menu.
        .add_state(GameState::InMenu)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "RustBall".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(menu::Menu)
        .add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .with_system(init_game_system)
                .with_system(spawn_players_system)
                .with_system(spawn_ball_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(player_red_keyboard_system)
                .with_system(player_blue_keyboard_system)
                .with_system(movement_system)
                .with_system(collision_system_red)
                .with_system(collision_system_blue)
                .with_system(players_collision_system)
                .with_system(control_ball_velocity)
                .with_system(edge_collision_system)
                .with_system(corner_collision_system)
                .with_system(goal_system),
        )
        .run();
}

// Initialize the game state.
fn init_game_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    background_query: Query<(Entity, &Background)>,
) {
    // Init camera.
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let font = asset_server.load(FONT);
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    // Show score on the screen (on the bottom left corner)
    let score_text = String::from("Score: 0–0");
    commands.spawn_bundle(Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value: score_text,
                style: text_style,
            }],
            alignment: text_alignment,
        },
        transform: Transform::from_translation(vec3(
            -WINDOW_WIDTH / 2. + 125.,
            -WINDOW_HEIGHT / 2. + 50.,
            2.0,
        )),
        global_transform: Default::default(),
        text_2d_size: Default::default(),
        text_2d_bounds: Default::default(),
        visibility: Visibility { is_visible: true },
    });
    let (_, background_type) = background_query.iter().next().unwrap();

    // Set pitch as selected in the menu.
    let pitch = match background_type {
        Background::Pitch1 => PITCH1_SPRITE,
        Background::Pitch2 => PITCH2_SPRITE,
        Background::Pitch3 => PITCH3_SPRITE,
    };
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(pitch),
        transform: Transform {
            translation: vec3(0.0, 0.0, 1.0),
            ..Default::default()
        },

        ..Default::default()
    });

    // Score as a resource.
    commands.insert_resource(Score { red: 0, blue: 0 });
}

// Spawns the players.
fn spawn_players_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn red circle that'll be representing first player.
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(PLAYER_RED_SPRITE),
            //Move the player to the left side.
            transform: Transform::from_translation(Vec3::new(RED_INITIAL_X, 0.0, 5.0)),
            ..Default::default()
        })
        .insert(PlayerRed)
        .insert(Velocity { x: 0.0, y: 0.0 })
        .insert(Radius(PLAYER_RADIUS));

    // Spawn blue circle that'll be representing second player.
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(PLAYER_BLUE_SPRITE),
            //Move the player to the right side.
            transform: Transform::from_translation(Vec3::new(BLUE_INITIAL_X, 0.0, 5.0)),
            ..Default::default()
        })
        .insert(PlayerBlue)
        .insert(Velocity { x: 0.0, y: 0.0 })
        .insert(Radius(PLAYER_RADIUS));
}

fn spawn_ball_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn ball.
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(BALL_SPRITE),
            //Move the ball to the center of the screen.
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
            ..Default::default()
        })
        .insert(Ball)
        .insert(Velocity { x: 0.0, y: 0.0 })
        .insert(Radius(BALL_RADIUS));
}

// Slows down the ball.
fn control_ball_velocity(mut query: Query<&mut Velocity, With<Ball>>) {
    // Get ball velocity.
    let mut velocity = query.iter_mut().next().unwrap();

    if velocity.x > 0. {
        velocity.x -= 0.05;
        velocity.x = velocity.x.max(-MAX_SPEED);
    } else if velocity.x < 0. {
        velocity.x += 0.05;
        velocity.x = velocity.x.min(MAX_SPEED);
    };

    if velocity.y > 0. {
        velocity.y -= 0.05;
        velocity.y = velocity.y.max(-MAX_SPEED);
    } else if velocity.y < 0. {
        velocity.y += 0.05;
        velocity.y = velocity.y.min(MAX_SPEED);
    };
}

// Parses keyboard input and changes velocity of the red player.
fn player_red_keyboard_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<PlayerRed>>,
) {
    for mut velocity in query.iter_mut() {
        if kb.pressed(KeyCode::W) {
            velocity.y += 0.1;
            velocity.y = velocity.y.min(MAX_SPEED);
        } else if kb.pressed(KeyCode::S) {
            velocity.y -= 0.1;
            velocity.y = velocity.y.max(-MAX_SPEED);
        } else if velocity.y > 0. {
            velocity.y -= 0.05;
            velocity.y = velocity.y.max(-MAX_SPEED);
        } else if velocity.y < 0. {
            velocity.y += 0.05;
            velocity.y = velocity.y.min(MAX_SPEED);
        }

        if kb.pressed(KeyCode::A) {
            velocity.x -= 0.1;
            velocity.x = velocity.x.max(-MAX_SPEED);
        } else if kb.pressed(KeyCode::D) {
            velocity.x += 0.1;
            velocity.x = velocity.x.min(MAX_SPEED);
        } else if velocity.x > 0. {
            velocity.x -= 0.05;
            velocity.x = velocity.x.max(-MAX_SPEED);
        } else if velocity.x < 0. {
            velocity.x += 0.05;
            velocity.x = velocity.x.min(MAX_SPEED);
        }
    }
}

// Parses keyboard input and changes velocity of the blue player.
fn player_blue_keyboard_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<PlayerBlue>>,
) {
    for mut velocity in query.iter_mut() {
        if kb.pressed(KeyCode::Up) {
            velocity.y += 0.1;
            velocity.y = velocity.y.min(MAX_SPEED);
        } else if kb.pressed(KeyCode::Down) {
            velocity.y -= 0.1;
            velocity.y = velocity.y.max(-MAX_SPEED);
        } else if velocity.y > 0. {
            velocity.y -= 0.05;
            velocity.y = velocity.y.max(-MAX_SPEED);
        } else if velocity.y < 0. {
            velocity.y += 0.05;
            velocity.y = velocity.y.min(MAX_SPEED);
        }

        if kb.pressed(KeyCode::Left) {
            velocity.x -= 0.1;
            velocity.x = velocity.x.max(-MAX_SPEED);
        } else if kb.pressed(KeyCode::Right) {
            velocity.x += 0.1;
            velocity.x = velocity.x.min(MAX_SPEED);
        } else if velocity.x > 0. {
            velocity.x -= 0.05;
            velocity.x = velocity.x.max(-MAX_SPEED);
        } else if velocity.x < 0. {
            velocity.x += 0.05;
            velocity.x = velocity.x.min(MAX_SPEED);
        }
    }
}

// Changes the position of the entities, based on their velocity.
fn movement_system(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x;
        translation.y += velocity.y;
    }
}

// Calculates new velocity vectors after collision.
// Using some math formulas from the internet.
// Inspired with: https://stackoverflow.com/questions/345838/ball-to-ball-collision-detection-and-handling
fn handle_collision(
    velocity1: &mut Velocity,
    velocity2: &mut Velocity,
    transform_red: &mut Transform,
    transform_blue: &mut Transform,
    radius1: f32,
    radius2: f32,
) {
    let delta = (transform_red.translation - transform_blue.translation).truncate();
    let players_distance = transform_red
        .translation
        .distance(transform_blue.translation);
    let d = players_distance;
    let multiplier = (-d + radius1 + radius2) / d;
    let delta_x = delta.x * multiplier;
    let delta_y = delta.y * multiplier;
    let mtd = Vec2::new(delta_x, delta_y);

    let im1 = 1.;
    let im2 = 1.;

    transform_red.translation.x += mtd[0] * (im1 / (im1 + im2));
    transform_red.translation.y += mtd[1] * (im1 / (im1 + im2));

    transform_blue.translation.x -= mtd[0] * (im2 / (im1 + im2));
    transform_blue.translation.y -= mtd[1] * (im2 / (im1 + im2));

    let v = Vec2::new(velocity1.x - velocity2.x, velocity1.y - velocity2.y);
    let vn = v.dot(mtd.normalize());

    if vn > 0.0 {
        return;
    }

    let i = (-1.5 * vn) / (im1 + im2);
    let impulse = mtd.normalize() * i;

    velocity1.x += impulse[0] * im1;
    velocity1.y += impulse[1] * im1;

    velocity2.x -= impulse[0] * im2;
    velocity2.y -= impulse[1] * im2;
}

// Detects collision between red player and the ball.
fn collision_system_red(
    mut query_red: Query<(&mut Velocity, &mut Transform, &PlayerRed), Without<Ball>>,
    mut query_ball: Query<(&mut Velocity, &mut Transform, &Ball, Without<PlayerRed>)>,
    kb: Res<Input<KeyCode>>,
) {
    let (mut velocity_red, mut transform_red, _) = query_red.iter_mut().next().unwrap();
    let (mut velocity_ball, mut transform_ball, _, _) = query_ball.iter_mut().next().unwrap();

    let player_ball_distance = transform_red
        .translation
        .distance(transform_ball.translation);
    if player_ball_distance < PLAYER_RADIUS + BALL_RADIUS {
        // If space pressed, shoot the ball
        if kb.pressed(KeyCode::Space) {
            let diff_x = transform_red.translation.x - transform_ball.translation.x;
            let diff_y = transform_red.translation.y - transform_ball.translation.y;
            let angle = diff_y.atan2(diff_x);
            velocity_ball.y += -5.0 * angle.sin();
            velocity_ball.x += -5.0 * angle.cos();
        }
        handle_collision(
            &mut velocity_red,
            &mut velocity_ball,
            &mut transform_red,
            &mut transform_ball,
            PLAYER_RADIUS,
            BALL_RADIUS,
        );
    }
}

// Detects collision between blue player and the ball.
fn collision_system_blue(
    mut query_blue: Query<(&mut Velocity, &mut Transform, &PlayerBlue), Without<Ball>>,
    mut query_ball: Query<(&mut Velocity, &mut Transform, &Ball, Without<PlayerBlue>)>,
    kb: Res<Input<KeyCode>>,
) {
    let (mut velocity_blue, mut transform_blue, _) = query_blue.iter_mut().next().unwrap();
    let (mut velocity_ball, mut transform_ball, _, _) = query_ball.iter_mut().next().unwrap();

    let player_ball_distance = transform_blue
        .translation
        .distance(transform_ball.translation);
    if player_ball_distance < PLAYER_RADIUS + BALL_RADIUS {
        // If right control pressed, shoot the ball.
        if kb.pressed(KeyCode::RControl) {
            let diff_x = transform_blue.translation.x - transform_ball.translation.x;
            let diff_y = transform_blue.translation.y - transform_ball.translation.y;
            let angle = diff_y.atan2(diff_x);
            velocity_ball.y += -5.0 * angle.sin();
            velocity_ball.x += -5.0 * angle.cos();
        }

        handle_collision(
            &mut velocity_blue,
            &mut velocity_ball,
            &mut transform_blue,
            &mut transform_ball,
            PLAYER_RADIUS,
            BALL_RADIUS,
        );
    }
}

// Handles collision between the players.
fn players_collision_system(
    mut query_red: Query<(&mut Velocity, &mut Transform, &PlayerRed), Without<PlayerBlue>>,
    mut query_blue: Query<(&mut Velocity, &mut Transform, &PlayerBlue), Without<PlayerRed>>,
) {
    let (mut velocity_red, mut transform_red, _) = query_red.iter_mut().next().unwrap();
    let (mut velocity_blue, mut transform_blue, _) = query_blue.iter_mut().next().unwrap();

    // If player red and player blue collide
    let players_distance = transform_red
        .translation
        .distance(transform_blue.translation);
    if players_distance < PLAYER_RADIUS * 2.0 {
        handle_collision(
            &mut velocity_red,
            &mut velocity_blue,
            &mut transform_red,
            &mut transform_blue,
            PLAYER_RADIUS,
            PLAYER_RADIUS,
        );
    };
}

// Handles collision between the players and corners of the goal.
fn corner_collision_system(mut query: Query<(&mut Velocity, &Transform, &Radius)>) {
    let corner1 = Vec3::new(-WINDOW_WIDTH / 2., CORNER_UP_HEIGHT, 5.);
    let corner2 = Vec3::new(WINDOW_WIDTH / 2., CORNER_UP_HEIGHT, 5.);
    let corner3 = Vec3::new(WINDOW_WIDTH / 2., CORNER_DOWN_HEIGHT, 5.);
    let corner4 = Vec3::new(-WINDOW_WIDTH / 2., CORNER_DOWN_HEIGHT, 5.);

    for (mut velocity, transform, radius) in query.iter_mut() {
        let radius = radius.0;

        let d1 = transform.translation.distance(corner1);
        let d2 = transform.translation.distance(corner2);
        let d3 = transform.translation.distance(corner3);
        let d4 = transform.translation.distance(corner4);

        if d1 <= radius + CORNER_RADIUS {
            velocity.x = -velocity.x;
            velocity.y = -velocity.y;
        }
        if d2 <= radius + CORNER_RADIUS {
            velocity.x = -velocity.x;
            velocity.y = -velocity.y;
        }
        if d3 <= radius + CORNER_RADIUS {
            velocity.x = -velocity.x;
            velocity.y = -velocity.y;
        }
        if d4 <= radius + CORNER_RADIUS {
            velocity.x = -velocity.x;
            velocity.y = -velocity.y;
        }
    }
}

// Handles collision between players and edges of the pitch.
fn edge_collision_system(mut query: Query<(&mut Velocity, &Transform, &Radius)>) {
    for (mut velocity, transform, radius) in query.iter_mut() {
        let translation = transform.translation;
        let radius = radius.0;

        if (translation.x + radius >= WINDOW_WIDTH / 2.
            || translation.x - radius <= -WINDOW_WIDTH / 2.)
            && ((translation.y >= CORNER_UP_HEIGHT || translation.y <= CORNER_DOWN_HEIGHT)
                || radius == PLAYER_RADIUS)
        {
            velocity.x = -velocity.x;
        }

        if (translation.y + radius >= WINDOW_HEIGHT / 2.
            || translation.y - radius <= -WINDOW_HEIGHT / 2.)
            && ((translation.y >= CORNER_UP_HEIGHT || translation.y <= CORNER_DOWN_HEIGHT)
                || radius == PLAYER_RADIUS)
        {
            velocity.y = -velocity.y;
        }
    }
}

// Check if there was a goal.
// If there was, update the score.
fn goal_system(
    mut query_ball: Query<(&mut Velocity, &mut Transform, &Ball)>,
    mut query_players: Query<(&mut Velocity, &mut Transform), Without<Ball>>,
    mut score: ResMut<Score>,
    mut score_text: Query<&mut Text>,
) {
    // Get tuple from query
    let (mut velocity_ball, mut transform_ball, _) = query_ball.iter_mut().next().unwrap();

    // Get text from score_text
    let mut text = score_text.iter_mut().next().unwrap();

    if transform_ball.translation.x >= WINDOW_WIDTH / 2.
        || transform_ball.translation.x <= -WINDOW_WIDTH / 2.
    {
        if transform_ball.translation.x >= WINDOW_WIDTH / 2. {
            score.red += 1;
        } else {
            score.blue += 1;
        }
        text.sections[0].value = format!("Score: {}–{}", score.red, score.blue);
        transform_ball.translation.x = 0.;
        transform_ball.translation.y = 0.;

        velocity_ball.x = 0.;
        velocity_ball.y = 0.;
        let mut i = RED_INITIAL_X;
        for (mut velocity, mut transform) in query_players.iter_mut() {
            velocity.x = 0.;
            velocity.y = 0.;
            transform.translation.x = i;
            i += 2.0 * BLUE_INITIAL_X;
            transform.translation.y = 0.;
        }
    }

    if score.red == 3 {
        score.red = 0;
        score.blue = 0;
        text.sections[0].value = "Red Wins!".to_string();
    } else if score.blue == 3 {
        score.red = 0;
        score.blue = 0;
        text.sections[0].value = "Blue Wins!".to_string();
    }
}
