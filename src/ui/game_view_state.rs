use crate::core::{dungeon::{MAP_HEIGHT, MAP_WIDTH}, status::Status};

pub struct GameViewState {
    pub main_view_state: MainViewState,
    pub overlay_view_state: Option<OverlayViewState>,
}

pub struct MainViewState {
    pub message: String,
    pub map: [[char; MAP_WIDTH]; MAP_HEIGHT],
    pub status: Status,
}

pub enum OverlayViewState {
    Help,
    Inventory,
    Win,
    Loss,
    Quit { score: usize },
}