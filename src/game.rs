use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::states::GameState;

/// main game plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::Loading);
        app.add_plugin(crate::AssetPlugin)
            .add_plugin(crate::PlayerPlugin);
    }
}
