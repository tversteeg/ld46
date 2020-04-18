#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Phase {
    Menu,
    Initialize,
    Setup,
    Play,
    GameOver,
}
