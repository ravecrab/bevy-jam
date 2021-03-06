use bevy::{
    prelude::{Plugin as PluginTrait, *},
    window::WindowMode,
};

pub struct Plugin;

// TODO: Maybe move all of this into a serializable file or
// into a resource so it can be changed at runtime?
const GAME_TITLE: &str = "Bevy Jam";
const GAME_WIDTH: f32 = 1024.0;
const GAME_HEIGHT: f32 = 768.0;
const WINDOW_VSYNC: bool = false;
const WINDOW_RESIZABLE: bool = true;
const WINDOW_MODE: WindowMode = WindowMode::Windowed;

impl PluginTrait for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: GAME_TITLE.to_string(),
            width: GAME_WIDTH,
            height: GAME_HEIGHT,
            resizable: WINDOW_RESIZABLE,
            mode: WINDOW_MODE,
            vsync: WINDOW_VSYNC,
            ..Default::default()
        });
    }
}
