use bevy::prelude::*;
use bevy_jam::{bootstrap, cards, config, ui};

fn main() {
    let app = App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(cards::CardPlugin)
        .add_plugin(bootstrap::Plugin)
        .add_plugin(config::Plugin)
        .add_plugin(ui::Plugin)
        .run();
}
