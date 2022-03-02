#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Intro,
    CardPicking,
    Battle,
    Victory,
    Store,
}
