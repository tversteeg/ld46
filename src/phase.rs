#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Phase {
    Menu,
    Initialize,
    Setup,
    Play,
    WaitingForLastEnemy,
    GameOver,
    SwitchTo(Box<Phase>),
}

impl Default for Phase {
    fn default() -> Self {
        Phase::Menu
    }
}
