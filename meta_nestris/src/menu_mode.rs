#[derive(Clone, Copy, Eq, PartialEq)]
pub enum MenuMode {
    CopyrightScreen,
    TitleScreen,
    GameTypeSelect,
    LevelSelect,
    InitializingGame,
}
