pub mod add;
pub mod load;

use crate::cpu::{Cpu, Register};

const ERR_MIS_REGISTER: &str = "Missing register";
const ERR_NOT_REGISTER: &str = "Argument is not a register";
const ERR_MIS_VALUE: &str = "Missing value";
const ERR_NOT_VALUE: &str = "Argument is not a byte";

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

fn err_arg_missing(position: u8, message: &str) -> InstructionError {
    InstructionError::MissingArgument(position, message.to_string())
}

fn err_arg_mismatched(position: u8, message: &str) -> InstructionError {
    InstructionError::MismatchedArgument(position, message.to_string())
}

fn register_first_arg(arguments: &[Argument]) -> Result<Register, InstructionError> {
    match arguments.first() {
        None => Err(err_arg_missing(1, ERR_MIS_REGISTER)),
        Some(first_arg) => match first_arg {
            Argument::Register(register) => Ok(*register),
            _ => Err(err_arg_mismatched(1, ERR_NOT_REGISTER)),
        },
    }
}
