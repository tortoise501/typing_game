use crate::game::Game;

use crate::component;



pub struct Model {
    pub active_view: component::ViewType,
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
