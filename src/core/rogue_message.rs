#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum RogueMessage {
    AlreadyInUse,
    WelcomeToLevel(usize),
}