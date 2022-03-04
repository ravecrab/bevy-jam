use bevy::prelude::*;

#[derive(Component)]
pub struct InfoContainer;

#[derive(Component)]
pub struct InfoText;

pub fn create_container(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/slkscr.ttf");

    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Px(229.), Val::Px(229.)),
                padding: Rect::all(Val::Px(20.)),
                ..Default::default()
            },
            color: Color::rgba_u8(0, 0, 0, 0).into(),
            ..Default::default()
        })
        .insert(InfoContainer)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        max_size: Size::new(Val::Px(189.), Val::Undefined),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            font: font,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(InfoText);
        });
}
