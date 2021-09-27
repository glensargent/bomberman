use bevy::{input::keyboard, prelude::*, tasks::CountdownEvent};

struct Name(String);
struct Ammo(i8);
struct Health(i8);
struct Player;
struct Bomb {
    timestamp: f64,
}
struct Position {
    x: f32,
    y: f32,
}
// PlayerBundle is a struct that bundles together all the components
// needed for a player entity
#[derive(Bundle)]
struct PlayerBundle {
    name: Name,
    ammo: Ammo,
    health: Health,
    position: Position,
    _p: Player,
    // #[bundle]
    // sprite: SpriteSheetBundle,
}

impl PlayerBundle {
    fn new_player(name: String) -> PlayerBundle {
        PlayerBundle{
            name: Name(name),
            ammo: Ammo(1),
            health: Health(10),
            position: Position{x: 0., y: 0.},
            _p: Player,
        }
    }
}

struct Materials {
    player_material: Handle<ColorMaterial>,
    bomb_material: Handle<ColorMaterial>,
}

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    // startup system runs once on startup
    .add_startup_system(setup.system())
    .add_startup_stage("game_setup", SystemStage::single(spawn_player.system()))
    // regular systems run every frame
    .add_system(position.system())
    .add_system(player_controller.system())
    .add_system(bomb_timer.system())
    .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials{
        player_material: materials.add(Color::rgb(1., 1., 1.).into()),
        bomb_material: materials.add(Color::rgb(0., 0., 0.).into()),
    });
}

fn spawn_player(mut commands: Commands, materials: Res<Materials>) {
    // create the player entity
    // commands.spawn_bundle(PlayerBundle::new_player("Glen".to_string()));
    commands.spawn_bundle(SpriteBundle {
        material: materials.player_material.clone(),
        sprite: Sprite::new(Vec2::new(10., 10.)),
        ..Default::default()
    })
    .insert_bundle(PlayerBundle::new_player("Glen".to_string()));
}

fn player_controller(mut commands: Commands, materials: Res<Materials>, time: Res<Time>, keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Position, With<Player>>) {
    for mut pos in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            pos.x -= 2.;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            pos.x += 2.;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            pos.y += 2.;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            pos.y -= 2.;
        }

        // place a bomb
        if keyboard_input.just_released(KeyCode::Space) {
            commands.spawn_bundle(SpriteBundle {
                material: materials.bomb_material.clone(),
                sprite: Sprite::new(Vec2::new(10., 10.)),
                transform: Transform::from_xyz(pos.x, pos.y, -0.1),
                ..Default::default()
            })
            .insert(Bomb{
                timestamp: time.seconds_since_startup(),
            })
            // make the bomb position the same position as the player
            .insert(Position{ x: pos.x, y: pos.y });
        }
    }
}

fn bomb_timer(mut commands: Commands, time: Res<Time>, mut query: Query<(&Bomb, Entity)>) {
    let current_time = time.seconds_since_startup();
    for (bomb, entity) in query.iter() {
        if bomb.timestamp < current_time - 5. {
            println!("kachow");
            commands.entity(entity).despawn();
        }
    }
}

fn position(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut trans) in query.iter_mut() {
        trans.translation.x = pos.x;
        trans.translation.y = pos.y;
    }
}
