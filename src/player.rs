use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{assets::PlayerAssets, states::GameState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Playing, create_player_system);
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
}
