use crate::component;



pub struct Model {
    pub active_window: component::WindowType,
    pub running_state: RunningState,
}

#[derive(Clone)]
#[derive(Debug, PartialEq, Eq)]
#[derive(Default)]
#[allow(dead_code)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
