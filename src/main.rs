use bevy::prelude::*;

struct Name(String);
struct Ammo(i8);
struct Health(i8);
struct Player;
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
}

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    // startup system runs once on startup
    .add_startup_system(setup.system())
    .add_startup_stage("game_setup", SystemStage::single(spawn_player.system()))
    // regular systems run every frame
    .add_system(greet_player.system())
    .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials{
        player_material: materials.add(Color::rgb(1., 1., 1.).into())
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

fn greet_player(query: Query<(&Name, &Position, &Health, &Ammo, &Player)>) {
    for (name, pos, hp, ammo, controller) in query.iter() { // every entity that matches the query
        println!("Hello, world! {}", name.0);
    }
}
