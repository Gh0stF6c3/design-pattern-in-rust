pub mod copy;
pub mod cut;
pub mod paste;

pub trait Command {
    fn execute(&mut self,app: &mut cursive::Cursive) -> bool;
    fn undo(&mut self,app: &mut cursive::Cursive);
}