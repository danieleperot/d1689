mod add;

use crate::cpu::{Cpu, Register};

#[derive(Clone, PartialEq, Eq, Debug)]
enum InstructionError {
    MissingArgument(u8, String),
    MismatchedArgument(u8, String),
}

enum Argument {
    Register(Register),
    Byte(u8),
}

trait Instruction {
    fn execute(cpu: &mut Cpu, arguments: Vec<Argument>) -> Result<(), InstructionError>;
}
