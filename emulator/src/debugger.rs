use crate::state::EmulatorState;


pub trait Debugger {

    /// Function handling the instruction breakpoints.
    fn handle_instruction(&mut self, opcode: u16, state: &mut EmulatorState);
}

pub struct NoOpDebugger;

impl NoOpDebugger {
    pub fn new() -> Self {
        Self {}
    }
}

impl Debugger for NoOpDebugger {
    fn handle_instruction(&mut self, _opcode: u16, _state: &mut EmulatorState) {}
}
