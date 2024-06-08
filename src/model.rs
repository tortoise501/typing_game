use crate::component;

pub struct Model {
    pub active_window: component::WindowType,
    pub running_state: RunningState,
    ///text used to generate markov chain
    pub gen_text: String
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[allow(dead_code)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
