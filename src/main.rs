use bevy::prelude::*;
use bevy_jam::{battle, bootstrap, cards, config, ui};

fn main() {
    App::new()
        // This one is on top to override the `WindowDescriptor`
        // in the `DefaultPlugins`
        // Ok! I think DefaultPlugin just has to go before CardPlugin (at least)
        // because it creates AssetServer. -- inkoate
        .add_plugin(config::Plugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(bootstrap::Plugin)
        .add_plugin(ui::Plugin)
        .add_plugin(cards::Plugin)
        .add_plugin(battle::Plugin)
        .run();
}
