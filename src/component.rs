#[allow(dead_code)]
#[allow(unused_variables)]
use super::*;
pub trait Component {
    fn init(&mut self) -> std::io::Result<()>{
        Ok(())
    }
    fn handle_events(&mut self, event: Option<Event>) -> Action {
        match event {
          Some(Event::Stop) => Action::Stop,
          Some(Event::Tick) => Action::Tick,
          Some(Event::Key(key_code)) => self.handle_key_events(key_code),
          Some(Event::Resize(x, y)) => Action::Resize(x, y),
          Some(_) => Action::Noop,
          None => Action::Noop,
        }
      }
      fn handle_key_events(&mut self, key: KeyCode) -> Action {
        Action::Noop
      }
      fn update(&mut self, action: Action) -> Action {
        Action::Noop
      }
}
pub enum Event {
    Start,
    Stop,
    Tick,
    Key(KeyCode),
    Resize(i32,i32),
}
pub enum Action {
    Start,
    Stop,
    Tick,
    Resize(i32,i32),
    Noop,
}