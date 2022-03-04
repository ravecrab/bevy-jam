#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Loading,
    Intro,
    CardPicking,
    OpponentCardPick,
    Battle,
    Victory,
    Store,
}
