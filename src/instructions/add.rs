use super::{err_arg_mismatched, err_arg_missing};
use super::{Argument, Instruction, InstructionError};
use super::{ERR_MIS_REGISTER, ERR_MIS_VALUE, ERR_NOT_REGISTER, ERR_NOT_VALUE};
use crate::cpu::{Cpu, Flag, Register};

pub struct Add {}

impl Instruction for Add {
    fn execute(cpu: &mut Cpu, arguments: Vec<Argument>) -> Result<(), InstructionError> {
        match arguments.first() {
            None => Err(err_arg_missing(1, ERR_MIS_REGISTER)),
            Some(register) => match register {
                Argument::Register(register) => match arguments.get(1) {
                    None => Err(err_arg_missing(2, ERR_MIS_VALUE)),
                    Some(value) => match value {
                        Argument::Byte(value) => add_directly(cpu, *register, *value),
                        _ => Err(err_arg_mismatched(2, ERR_NOT_VALUE)),
                    },
                },
                _ => Err(err_arg_mismatched(1, ERR_NOT_REGISTER)),
            },
        }
    }
}

fn add_directly(cpu: &mut Cpu, register: Register, value: u8) -> Result<(), InstructionError> {
    cpu.assign_flag(Flag::Carry, false);

    let register_value = cpu.register(register) as u16;
    let sum: u16 = register_value + (value as u16);
    let sum_as_byte = (sum & 0xFF) as u8;

    cpu.assign_register(register, sum_as_byte);

    cpu.assign_flag(Flag::Carry, sum > 0xFF);
    cpu.assign_flag(Flag::Zero, sum_as_byte == 0);

    Ok(())
}

#[test]
fn requires_register_to_be_provided_as_first_argument() {
    let mut cpu = Cpu::new();
    let result = Add::execute(&mut cpu, vec![]);

    let message = ERR_MIS_REGISTER.to_string();
    let expected = InstructionError::MissingArgument(1, message);

    // TODO: Would be cool to have a function to get a checksum of the current
    // CPU state and verify that it wasn't changed

    assert!(result.is_err_and(|x| x == expected));
}

#[test]
fn requires_register_of_register_type_to_be_provided_as_first_argument() {
    let mut cpu = Cpu::new();
    let result = Add::execute(&mut cpu, vec![Argument::Byte(123)]);

    let message = ERR_NOT_REGISTER.to_string();
    let expected = InstructionError::MismatchedArgument(1, message);

    assert!(result.is_err_and(|x| x == expected));
}

#[test]
fn requires_a_second_parameter_to_be_provided() {
    let mut cpu = Cpu::new();
    let result = Add::execute(&mut cpu, vec![Argument::Register(Register::A)]);

    let message = ERR_MIS_VALUE.to_string();
    let expected = InstructionError::MissingArgument(2, message);

    assert!(result.is_err_and(|x| x == expected));
}

#[test]
fn requires_u8_to_be_provided_as_second_parameter() {
    let mut cpu = Cpu::new();
    let result = Add::execute(
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
fn adds_value_to_the_specified_cpu_register_a() {
    let mut cpu = Cpu::new();
    let result = Add::execute(
        &mut cpu,
        vec![Argument::Register(Register::A), Argument::Byte(0xDE)],
    );

    assert!(result.is_ok());
    assert_eq!(0xDE, cpu.register(Register::A));
    assert_eq!(false, cpu.flag(Flag::Carry));
    assert_eq!(false, cpu.flag(Flag::Zero));
}

#[test]
fn carry_and_zero_flags_are_reset_before_sum() {
    let mut cpu = Cpu::new();
    cpu.assign_flag(Flag::Carry, true);
    cpu.assign_flag(Flag::Zero, true);

    let result = Add::execute(
        &mut cpu,
        vec![Argument::Register(Register::A), Argument::Byte(0xDE)],
    );

    assert!(result.is_ok());
    assert_eq!(0xDE, cpu.register(Register::A));
    assert_eq!(false, cpu.flag(Flag::Carry));
    assert_eq!(false, cpu.flag(Flag::Zero));
}

#[test]
fn when_addition_returns_zero_then_zero_flag_is_set() {
    let mut cpu = Cpu::new();
    let result = Add::execute(
        &mut cpu,
        vec![Argument::Register(Register::A), Argument::Byte(0x0)],
    );

    assert!(result.is_ok());
    assert_eq!(0x0, cpu.register(Register::A));
    assert_eq!(false, cpu.flag(Flag::Carry));
    assert_eq!(true, cpu.flag(Flag::Zero));
}

#[test]
fn when_addition_returns_value_larger_than_0xff_then_carry_flag_is_set() {
    let mut cpu = Cpu::new();
    cpu.assign_register(Register::A, 100);

    let result = Add::execute(
        &mut cpu,
        vec![Argument::Register(Register::A), Argument::Byte(157)],
    );

    assert!(result.is_ok());
    assert_eq!(1, cpu.register(Register::A));
    assert_eq!(true, cpu.flag(Flag::Carry));
    assert_eq!(false, cpu.flag(Flag::Zero));
}

#[test]
fn when_addition_returns_value_larger_than_0xff_and_lsb_is_zero_then_both_flags_are_set() {
    let mut cpu = Cpu::new();
    cpu.assign_register(Register::B, 100);

    let result = Add::execute(
        &mut cpu,
        vec![Argument::Register(Register::B), Argument::Byte(156)],
    );

    assert!(result.is_ok());
    assert_eq!(0, cpu.register(Register::B));
    assert_eq!(true, cpu.flag(Flag::Carry));
    assert_eq!(true, cpu.flag(Flag::Zero));
}
