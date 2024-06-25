pub const MAP_HEIGHT: usize = 30; // MAXLINES - 2
pub const MAP_WIDTH: usize = 80; // MAXCOLS

pub struct GameViewState {
    pub main_view_state: MainViewState,
    pub overlay_view_state: Option<OverlayViewState>,
}

pub struct MainViewState {
    pub message: String,
    pub map: [[char; MAP_WIDTH]; MAP_HEIGHT],
    pub status: String,
}

pub enum OverlayViewState {
    Help,
    Inventory,
    Win,
    Loss,
    Quit,
}