#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuMode {
    CopyrightScreen,
    TitleScreen,
    GameTypeSelect,
    LevelSelect,
    InitializingGame,
}
