use super::{err_arg_mismatched, err_arg_missing};
use super::{Argument, Instruction, InstructionError};
use super::{ERR_MIS_REGISTER, ERR_MIS_VALUE, ERR_NOT_REGISTER, ERR_NOT_VALUE};
use crate::cpu::{Cpu, Register};

pub struct Load {}

impl Instruction for Load {
    fn execute(cpu: &mut Cpu, arguments: Vec<Argument>) -> Result<(), InstructionError> {
        match arguments.first() {
            None => Err(err_arg_missing(1, ERR_MIS_REGISTER)),
            Some(register) => match register {
                Argument::Register(register) => match arguments.get(1) {
                    None => Err(err_arg_missing(2, ERR_MIS_VALUE)),
                    Some(value) => match value {
                        Argument::Byte(value) => load_immediate(cpu, *register, *value),
                        _ => Err(err_arg_mismatched(2, ERR_NOT_VALUE)),
                    },
                },
                _ => Err(err_arg_mismatched(1, ERR_NOT_REGISTER)),
            },
        }
    }
}

fn load_immediate(cpu: &mut Cpu, register: Register, value: u8) -> Result<(), InstructionError> {
    cpu.assign_register(register, value);
    Ok(())
}

#[test]
fn requires_register_to_be_provided_as_first_argument() {
    let mut cpu = Cpu::new();
    let result = Load::execute(&mut cpu, vec![]);

    let message = ERR_MIS_REGISTER.to_string();
    let expected = InstructionError::MissingArgument(1, message);

    assert!(result.is_err_and(|x| x == expected));
}

#[test]
fn requires_register_of_register_type_to_be_provided_as_first_argument() {
    let mut cpu = Cpu::new();
    let result = Load::execute(&mut cpu, vec![Argument::Byte(123)]);

    let message = ERR_NOT_REGISTER.to_string();
    let expected = InstructionError::MismatchedArgument(1, message);

    assert!(result.is_err_and(|x| x == expected));
}

#[test]
fn requires_a_second_parameter_to_be_provided() {
    let mut cpu = Cpu::new();
    let result = Load::execute(&mut cpu, vec![Argument::Register(Register::A)]);

    let message = ERR_MIS_VALUE.to_string();
    let expected = InstructionError::MissingArgument(2, message);

    assert!(result.is_err_and(|x| x == expected));
}

#[test]
fn requires_u8_to_be_provided_as_second_parameter() {
    let mut cpu = Cpu::new();
    let result = Load::execute(
        &mut cpu,
        vec![
            Argument::Register(Register::A),
            Argument::Register(Register::A),
        ],
    );

    let message = ERR_NOT_VALUE.to_string();
    let expected = InstructionError::MismatchedArgument(2, message);

    assert!(result.is_err_and(|x| x == expected));
}

#[test]
fn loads_provided_value_to_register() {
    let mut cpu = Cpu::new();
    let result = Load::execute(
        &mut cpu,
        vec![Argument::Register(Register::A), Argument::Byte(0x42)],
    );

    assert!(result.is_ok());
    assert_eq!(0x42, cpu.register(Register::A));
}
