pub mod add;

use crate::cpu::{Cpu, Register};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum InstructionError {
    MissingArgument(u8, String),
    MismatchedArgument(u8, String),
}

pub enum Argument {
    Register(Register),
    Byte(u8),
}

pub trait Instruction {
    fn execute(cpu: &mut Cpu, arguments: Vec<Argument>) -> Result<(), InstructionError>;
}
