use super::*;

#[derive(Debug)]
pub struct StatComp {
    pub game: Game,
}
#[allow(unused_variables)]
#[allow(dead_code)]
impl Component for StatComp {
    fn handle_message(&mut self, msg: Message) -> Message {
        todo!()
    }

    fn view(&mut self, f: &mut Frame) {
        todo!()
    }
}
