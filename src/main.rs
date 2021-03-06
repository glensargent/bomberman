use bevy::{prelude::*, sprite::collide_aabb::collide};

const BOMB_SIZE: f32 = 25.;
const PLAYER_SIZE: f32 = 25.;
const EXPLOSION_LENGTH: f32 = 250.;
struct Name(String);
struct Ammo(i8);
struct Health(i8);
struct Player;
struct Bomb;
struct Explosion;
// PlayerBundle is a struct that bundles together all the components
// needed for a player entity
#[derive(Bundle)]
struct PlayerBundle {
    name: Name,
    ammo: Ammo,
    health: Health,
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
            _p: Player,
        }
    }
}

enum GameState {
    InMenu,
    InGame,
}
struct Materials {
    player_material: Handle<ColorMaterial>,
    bomb_material: Handle<ColorMaterial>,
    explosion_material: Handle<ColorMaterial>,
}

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    // startup system runs once on startup
    .add_startup_system(setup.system())
    .add_startup_stage("game_setup", SystemStage::single(spawn_player.system()))
    // regular systems run every frame
    .add_system(player_controller.system())
    .add_system(explosion.system())
    .add_system(explosion_hit.system())
    .add_system(bomb_timer.system())
    .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials{
        player_material: materials.add(Color::rgb(1., 1., 1.).into()),
        bomb_material: materials.add(Color::rgb(0., 0., 0.).into()),
        explosion_material: materials.add(Color::rgb(1., 0.64, 0.).into()),
    });
}

fn spawn_player(mut commands: Commands, materials: Res<Materials>) {
    // create the player entity
    // commands.spawn_bundle(PlayerBundle::new_player("Glen".to_string()));
    commands.spawn_bundle(SpriteBundle {
        material: materials.player_material.clone(),
        sprite: Sprite::new(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
        ..Default::default()
    })
    .insert_bundle(PlayerBundle::new_player("Glen".to_string()));
}

fn player_controller(mut commands: Commands, materials: Res<Materials>, keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>) {
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.;
        }

        // place a bomb
        if keyboard_input.just_released(KeyCode::Space) {
            commands.spawn_bundle(SpriteBundle {
                material: materials.bomb_material.clone(),
                sprite: Sprite::new(Vec2::new(BOMB_SIZE, BOMB_SIZE)),
                transform: Transform::from_xyz(transform.translation.x, transform.translation.y, -0.1),
                ..Default::default()
            })
            .insert(Bomb)
            .insert(Timer::from_seconds(5., false));
        }
    }
}

fn bomb_timer(mut commands: Commands, materials: Res<Materials>, time: Res<Time>, mut query: Query<(&mut Transform, &mut Timer, Entity), With<Bomb>>) {
    for (trans, mut timer, entity) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let mut spawn_explosion = |x: f32, y: f32| {
                // spawn explosion Y entity
                commands.spawn_bundle(SpriteBundle {
                    material: materials.explosion_material.clone(),
                    transform: Transform::from_xyz(trans.translation.x, trans.translation.y, -0.1),
                    sprite: Sprite::new(Vec2::new(x, y)),
                    ..Default::default()
                })
                .insert(Explosion)
                .insert(Timer::from_seconds(2., false));
            };

            spawn_explosion(BOMB_SIZE, EXPLOSION_LENGTH);
            spawn_explosion(EXPLOSION_LENGTH, BOMB_SIZE);
            commands.entity(entity).despawn(); // despawn the bomb, otherwise this query will keep getting hit..
        }
    }
}

fn explosion(mut commands: Commands, time: Res<Time>, mut query: Query<(&mut Timer, Entity), With<Explosion>>) {
    for (mut timer, entity) in query.iter_mut() {
        // cleanup the explosion after its duration
        timer.tick(time.delta());
        if timer.finished() {
            commands.entity(entity).despawn(); // despawn the bomb, otherwise this query will keep getting hit..
        }
    }
}

fn explosion_hit(player_query: Query<(Entity, &Transform, &Sprite), With<Player>>, explosion_query: Query<(Entity, &Transform, &Sprite), With<Explosion>>) {
    for (_expl_entity, expl_tf, expl_sprite) in explosion_query.iter() {
        for (_player_entity, player_tf, player_sprite) in player_query.iter() {
            // we need to get vec2 from the transform
            let player_scale = Vec2::from(player_tf.scale);
            let explosion_scale = Vec2::from(expl_tf.scale);
            let collision = collide(
                expl_tf.translation,
                expl_sprite.size * explosion_scale,
                player_tf.translation,
                player_sprite.size * player_scale
            );

            if let Some(_) = collision {
                println!("owww");
                // player dies
            }

        }
    }
}