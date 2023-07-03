use crate::instruction;

pub fn parse_instructions(text: &str) -> Result<Vec<instruction::Instruction>, serde_json::Error> {
    serde_json::from_str(text)
}
