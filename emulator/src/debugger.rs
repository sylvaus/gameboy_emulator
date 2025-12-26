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

impl Default for NoOpDebugger {
    fn default() -> Self {
        Self::new()
    }
}

impl Debugger for NoOpDebugger {
    fn handle_instruction(&mut self, _opcode: u16, _state: &mut EmulatorState) {}
}

struct InstructionHandler<'a> {
    func: Box<dyn FnMut(u16, &mut EmulatorState) -> () + 'a>,
}

impl<'a> InstructionHandler<'a> {
    fn new(func: impl FnMut(u16, &mut EmulatorState) -> () + 'a) -> Self {
        Self { func: Box::new(func) }
    }
}

pub struct ConfigurableDebugger<'a> {
    handlers: [Option<InstructionHandler<'a>>; 512],
}

impl<'a> ConfigurableDebugger<'a> {
    pub fn new() -> Self {
        Self {
            handlers: [const { None }; 512],
        }
    }

    pub fn add_handler(
        &mut self,
        opcode: u16,
        handler: impl FnMut(u16, &mut EmulatorState) -> () + 'a,
    ) {
        self.handlers[opcode as usize] = Some(InstructionHandler::new(handler));
    }

    pub fn remove_handler(&mut self, opcode: u16) {
        self.handlers[opcode as usize] = None;
    }
}

impl Debugger for ConfigurableDebugger<'_> {
    fn handle_instruction(&mut self, opcode: u16, state: &mut EmulatorState) {
        if let Some(handler) = &mut self.handlers[opcode as usize] {
            (handler.func)(opcode, state);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::create_valid_cartridge;
    use super::*;

    #[test]
    fn configurable_debugger_add_handler() {
        let mut value = 0u16;
        {
            let borrowed_value = &mut value;
            let mut config = ConfigurableDebugger::new();
            let mut state = EmulatorState::new(create_valid_cartridge());

            config.add_handler(123, |opcode, _| {*borrowed_value = opcode});


            config.handle_instruction(123, &mut state);
        }

        assert_eq!(123, value);
    }
}
