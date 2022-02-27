use bevy::prelude::{Plugin as PluginTrait, *};

use crate::state::GameState;

pub struct Plugin;

impl PluginTrait for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera)
            // TODO: Implement a context-aware exit on esc to replace this one
            .add_system(bevy::input::system::exit_on_esc_system)
            // TODO: Switch to `Intro` state for a release build or
            // another state for debug builds. Also, this could be in
            // a config file, maybe?
            .add_state(GameState::Battle);
    }
}

pub fn camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
