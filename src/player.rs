use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{assets::PlayerAssets, states::GameState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Playing, create_player_system)
            .add_system(
                switch_players_system
                    .run_in_state(GameState::Playing)
                    .run_if(time_to_switch),
            )
            .add_system(player_movement_system.run_in_state(GameState::Playing));
    }
}

#[derive(Component)]
struct PlayerMarker;

#[derive(Bundle)]
struct PlayerBundle {
    #[bundle]
    sprite: SpriteBundle,

    _marker: PlayerMarker,
}

#[derive(Component)]
struct SelectedPlayer;

impl PlayerBundle {
    fn new(img: Handle<Image>, pos: Vec2) -> Self {
        println!("{:?}", img);
        PlayerBundle {
            sprite: SpriteBundle {
                texture: img,
                transform: Transform::from_translation(pos.extend(0.0)),
                ..default()
            },
            _marker: PlayerMarker,
        }
    }
}

#[derive(Default)]
struct PlayerSwitchCountdown(Timer);

fn create_player_system(mut commands: Commands, assets: Res<PlayerAssets>) {
    commands
        .spawn_bundle(PlayerBundle::new(
            assets.active.clone(),
            Vec2::new(-100.0, -100.0),
        ))
        .insert(SelectedPlayer);
    commands.spawn_bundle(PlayerBundle::new(
        assets.inactive.clone(),
        Vec2::new(100.0, 100.0),
    ));

    // TODO: CHANGE THIS BACK
    commands.insert_resource(PlayerSwitchCountdown(Timer::from_seconds(5.0, true)));
}

fn time_to_switch(time: Res<Time>, mut timer: ResMut<PlayerSwitchCountdown>) -> bool {
    timer.0.tick(time.delta()).just_finished()
}

fn switch_players_system(
    mut commands: Commands,
    assets: Res<PlayerAssets>,
    mut selected_player: Query<
        (Entity, &mut Handle<Image>),
        (With<PlayerMarker>, With<SelectedPlayer>),
    >,
    mut unselected_player: Query<
        (Entity, &mut Handle<Image>),
        (With<PlayerMarker>, Without<SelectedPlayer>),
    >,
) {
    for (player, mut img) in selected_player.iter_mut() {
        commands.entity(player).remove::<SelectedPlayer>();
        *img = assets.inactive.clone();
    }
    for (player, mut img) in unselected_player.iter_mut() {
        commands.entity(player).insert(SelectedPlayer);
        *img = assets.active.clone();
    }
}

fn player_movement_system(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<SelectedPlayer>>,
) {
    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::A) {
        direction.x -= 100.0
    }
    if keyboard.pressed(KeyCode::D) {
        direction.x += 100.0
    }
    if keyboard.pressed(KeyCode::W) {
        direction.y += 100.0
    }
    if keyboard.pressed(KeyCode::S) {
        direction.y -= 100.0
    }
    direction *= time.delta_seconds();

    for mut player_pos in query.iter_mut() {
        player_pos.translation += direction.extend(0.0);
    }
}
