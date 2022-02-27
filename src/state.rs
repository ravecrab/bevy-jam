#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Intro,
    Battle,
    Victory,
    Store,
}
