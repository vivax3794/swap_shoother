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
            );
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    #[bundle]
    sprite: SpriteBundle,
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
    commands.insert_resource(PlayerSwitchCountdown(Timer::from_seconds(5.0, true)));
}

fn time_to_switch(time: Res<Time>, mut timer: ResMut<PlayerSwitchCountdown>) -> bool {
    timer.0.tick(time.delta()).just_finished()
}

fn switch_players_system(
    mut commands: Commands,
    assets: Res<PlayerAssets>,
    mut selected_player: Query<(Entity, &mut Handle<Image>), With<SelectedPlayer>>,
    mut unselected_player: Query<(Entity, &mut Handle<Image>), Without<SelectedPlayer>>,
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
