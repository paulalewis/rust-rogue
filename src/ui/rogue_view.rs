pub struct RogueView {
    rogue_view_model: RogueViewModel,
    screen: Screen,
}

impl RogueView {
    pub fn new(rogue_state: RogueState) -> RogueView {
        RogueView {
            rogue_view_model: RogueViewModel::new(rogue_state),
            screen: ConsoleScreen::new(),
        }
    }
}
