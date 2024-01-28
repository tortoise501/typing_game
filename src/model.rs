use crate::game::Game;


#[derive(Clone)]
#[derive(Debug)]
pub struct Model {
    pub game: Game,
    pub active_view: View,
    pub running_state: RunningState,
}

#[derive(Clone)]
#[derive(Debug, PartialEq, Eq)]
#[derive(Default)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
pub enum View {
    Menu,
    Game,
    Statistics,
}