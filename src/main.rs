use bevy::prelude::*;
use bevy_jam::{bootstrap, config, ui};

fn main() {
    App::new()
        .add_plugin(bootstrap::Plugin)
        .add_plugin(config::Plugin)
        .add_plugin(ui::Plugin)
        .add_plugins(DefaultPlugins)
        .run();
}
