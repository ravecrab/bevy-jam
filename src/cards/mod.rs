use bevy::prelude::*;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Speed(u32);

#[derive(Component)]
struct Attack(u32);

#[derive(Component)]
struct Hitpoints(u32);

#[derive(Component)]
struct Deck;

#[derive(Component)]
struct Hand;

#[derive(Component)]
struct Card;

#[derive(Component)]
struct InPlay;
