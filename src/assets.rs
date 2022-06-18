use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::states::GameState;

#[derive(AssetCollection)]
pub struct PlayerAssets {
    #[asset(path = "images/player/active.png")]
    pub active: Handle<Image>,
    #[asset(path = "images/player/inactive.png")]
    pub inactive: Handle<Image>,
}

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
                .with_collection::<PlayerAssets>(),
        );
    }
}
