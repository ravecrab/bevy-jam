use bevy::prelude::{Plugin as PluginTrait, *};

pub struct Plugin;

impl PluginTrait for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}
