use bevy::prelude::*;
use bevy_jam::{battle, bootstrap, cards, config, ui};

fn main() {
    App::new()
        // This one is on top to override the `WindowDescriptor`
        // in the `DefaultPlugins`
        .add_plugin(config::Plugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(bootstrap::Plugin)
        .add_plugin(ui::Plugin)
        .add_plugin(cards::CardPlugin)
        .add_plugin(battle::Plugin)
        .run();
}
