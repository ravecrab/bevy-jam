use bevy::prelude::{Plugin as PluginTrait, *};

pub struct Plugin;

impl PluginTrait for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera)
            // TODO: Implement a context-aware exit on esc to replace this one
            .add_system(bevy::input::system::exit_on_esc_system);
    }
}

pub fn camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
