use bevy::prelude::*;

#[derive(Component)]
pub struct InfoContainer;

#[derive(Component)]
pub struct InfoText;

pub fn create_container(parent: &mut ChildBuilder, font: &Handle<Font>) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(40.), Val::Percent(100.)),
                ..Default::default()
            },
            color: Color::TEAL.into(),
            ..Default::default()
        })
        .insert(InfoContainer)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        max_size: Size::new(Val::Px(450.), Val::Undefined),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font_size: 24.0,
                            color: Color::WHITE,
                            font: font.clone(),
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
