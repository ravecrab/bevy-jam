#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Loading,
    Intro,
    CardPicking,
    Battle,
    Victory,
    Store,
}
