use bevy::prelude::*;

#[allow(dead_code)]
pub fn create_ui(parent: &mut ChildBuilder) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(40.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(50.), Val::Percent(100.)),
                    ..Default::default()
                },
                color: Color::GRAY.into(),
                ..Default::default()
            });

            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(50.), Val::Percent(100.)),
                    ..Default::default()
                },
                color: Color::WHITE.into(),
                ..Default::default()
            });
        });
}
