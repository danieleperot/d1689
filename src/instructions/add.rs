use super::ERR_MIS_VALUE;
use super::{err_arg_missing, register_first_arg};
use super::{Argument, Instruction, InstructionError};
use crate::cpu::{Cpu, Flag, Register};

pub struct Add {}

impl Instruction for Add {
    fn execute(cpu: &mut Cpu, arguments: Vec<Argument>) -> Result<(), InstructionError> {
        let register = register_first_arg(&arguments)?;

        match arguments.get(1) {
            None => Err(err_arg_missing(2, ERR_MIS_VALUE)),
            Some(value) => match value {
                Argument::Byte(src) => add_directly(cpu, register, *src),
                Argument::Register(src) => add_registers(cpu, register, *src),
            },
        }
    }
}

fn add_directly(cpu: &mut Cpu, register: Register, value: u8) -> Result<(), InstructionError> {
    let register_value = cpu.register(register) as u16;
    let sum: u16 = register_value + (value as u16);
    let sum_as_byte = (sum & 0xFF) as u8;

    assing_to_cpu(cpu, register, sum_as_byte, sum)
}

fn add_registers(cpu: &mut Cpu, dest: Register, source: Register) -> Result<(), InstructionError> {
    let dest_value = cpu.register(dest) as u16;
    let source_value = cpu.register(source) as u16;
    let sum: u16 = source_value + dest_value;
    let sum_as_byte = (sum & 0xFF) as u8;

    assing_to_cpu(cpu, dest, sum_as_byte, sum)
}

fn assing_to_cpu(
    cpu: &mut Cpu,
    register: Register,
    sum_as_byte: u8,
    sum: u16,
) -> Result<(), InstructionError> {
    cpu.assign_register(register, sum_as_byte);

    cpu.assign_flag(Flag::Carry, sum > 0xFF);
    cpu.assign_flag(Flag::Zero, sum_as_byte == 0);

    Ok(())
}

#[test]
fn requires_register_to_be_provided_as_first_argument() {
    let mut cpu = Cpu::new();
    let result = Add::execute(&mut cpu, vec![]);

    let message = super::ERR_MIS_REGISTER.to_string();
    let expected = InstructionError::MissingArgument(1, message);

    // TODO: Would be cool to have a function to get a checksum of the current
    // CPU state and verify that it wasn't changed

    assert!(result.is_err_and(|x| x == expected));
}

#[test]
fn requires_register_of_register_type_to_be_provided_as_first_argument() {
    let mut cpu = Cpu::new();
    let result = Add::execute(&mut cpu, vec![Argument::Byte(123)]);

    let message = super::ERR_NOT_REGISTER.to_string();
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

#[test]
fn adds_source_register_to_the_destination_register() {
    let mut cpu = Cpu::new();
    cpu.assign_register(Register::A, 4);
    cpu.assign_register(Register::B, 38);
    let result = Add::execute(
        &mut cpu,
        vec![
            Argument::Register(Register::A),
            Argument::Register(Register::B),
        ],
    );

    assert!(result.is_ok());
    assert_eq!(42, cpu.register(Register::A));
    assert_eq!(38, cpu.register(Register::B));
    assert_eq!(false, cpu.flag(Flag::Carry));
    assert_eq!(false, cpu.flag(Flag::Zero));
}

#[test]
fn carry_and_zero_flags_are_reset_before_sum_of_registers() {
    let mut cpu = Cpu::new();
    cpu.assign_flag(Flag::Carry, true);
    cpu.assign_flag(Flag::Zero, true);
    cpu.assign_register(Register::B, 0xDE);

    let result = Add::execute(
        &mut cpu,
        vec![
            Argument::Register(Register::A),
            Argument::Register(Register::B),
        ],
    );

    assert!(result.is_ok());
    assert_eq!(0xDE, cpu.register(Register::A));
    assert_eq!(false, cpu.flag(Flag::Carry));
    assert_eq!(false, cpu.flag(Flag::Zero));
}

#[test]
fn when_sum_is_zero_then_zero_flag_is_set_also_when_adding_registers() {
    let mut cpu = Cpu::new();
    let result = Add::execute(
        &mut cpu,
        vec![
            Argument::Register(Register::A),
            Argument::Register(Register::B),
        ],
    );

    assert!(result.is_ok());
    assert_eq!(0x0, cpu.register(Register::A));
    assert_eq!(false, cpu.flag(Flag::Carry));
    assert_eq!(true, cpu.flag(Flag::Zero));
}

#[test]
fn when_sum_is_larger_than_0xff_then_carry_flag_is_set_also_when_adding_registers() {
    let mut cpu = Cpu::new();
    cpu.assign_register(Register::A, 100);
    cpu.assign_register(Register::B, 157);

    let result = Add::execute(
        &mut cpu,
        vec![
            Argument::Register(Register::A),
            Argument::Register(Register::B),
        ],
    );

    assert!(result.is_ok());
    assert_eq!(1, cpu.register(Register::A));
    assert_eq!(true, cpu.flag(Flag::Carry));
    assert_eq!(false, cpu.flag(Flag::Zero));
}

#[test]
fn when_sum_is_larger_than_0xff_and_lsb_is_zero_then_both_flags_are_set_also_when_adding_registers()
{
    let mut cpu = Cpu::new();
    cpu.assign_register(Register::A, 156);
    cpu.assign_register(Register::B, 100);

    let result = Add::execute(
        &mut cpu,
        vec![
            Argument::Register(Register::B),
            Argument::Register(Register::A),
        ],
    );

    assert!(result.is_ok());
    assert_eq!(0, cpu.register(Register::B));
    assert_eq!(true, cpu.flag(Flag::Carry));
    assert_eq!(true, cpu.flag(Flag::Zero));
}
