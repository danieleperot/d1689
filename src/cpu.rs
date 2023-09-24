#![allow(dead_code)]

use std::collections::HashMap;

#[repr(u8)]
#[derive(Copy, Clone)]
enum Flag {
    CARRY = 0,
    ZERO = 1,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Register {
    A,
    B,
}

struct Cpu {
    registers: HashMap<Register, u8>,
    flags: u8,
}

impl Cpu {
    fn new() -> Self {
        let mut cpu = Self {
            registers: HashMap::new(),
            flags: 0,
        };
        cpu.reset();

        cpu
    }

    fn reset(&mut self) {
        self.registers.insert(Register::A, 0);
        self.registers.insert(Register::B, 0);
        self.flags = 0;
    }

    fn register(&mut self, register: Register) -> u8 {
        if let Some(content) = self.registers.get(&register) {
            content.clone()
        } else {
            self.registers.insert(register, 0);
            0
        }
    }

    fn assign_register(&mut self, register: Register, value: u8) {
        self.registers.insert(register, value);
    }

    fn flags(&mut self) -> u8 {
        self.flags
    }

    fn flag(&mut self, position: Flag) -> bool {
        let position = position as u8;
        let bit = self.flags >> position & 1;

        bit > 0
    }

    fn assign_flag(&mut self, position: Flag, is_set: bool) {
        let position = position as u8;

        // 0. get the correct mask
        let mask = (1 << position) ^ 0b11111111;
        // 1. reset the flag
        self.flags &= mask;
        // 2. set the flag to the desired value
        self.flags |= (is_set as u8) << position;
    }
}

#[test]
fn cpu_is_initialized() -> () {
    let cpu = Cpu::new();

    assert_eq!(0, cpu.registers.get(&Register::A).unwrap().clone());
    assert_eq!(0, cpu.registers.get(&Register::B).unwrap().clone());
    assert_eq!(0, cpu.flags);
}

#[test]
fn cpu_can_be_reset() -> () {
    let mut cpu = Cpu::new();

    cpu.registers.insert(Register::A, 0x1F);
    cpu.registers.insert(Register::B, 0xD4);
    cpu.flags = 0xAC;

    cpu.reset();

    assert_eq!(0, cpu.registers.get(&Register::A).unwrap().clone());
    assert_eq!(0, cpu.registers.get(&Register::B).unwrap().clone());
    assert_eq!(0, cpu.flags);
}

#[test]
fn value_can_be_added_to_a_register() -> () {
    let mut cpu = Cpu::new();

    cpu.assign_register(Register::A, 0x12);
    assert_eq!(0x12, cpu.register(Register::A));
    assert_eq!(0x12, cpu.registers.get(&Register::A).unwrap().clone());
}

#[test]
fn value_can_be_added_to_b_register() -> () {
    let mut cpu = Cpu::new();

    cpu.assign_register(Register::B, 0x12);
    assert_eq!(0x12, cpu.register(Register::B));
    assert_eq!(0x12, cpu.registers.get(&Register::B).unwrap().clone());
}

#[test]
fn attempting_to_read_unitialized_register_causes_register_to_be_initialized_to_zero() {
    let mut cpu = Cpu::new();

    cpu.registers.remove(&Register::A);
    assert_eq!(0, cpu.register(Register::A));
}

#[test]
fn all_flags_can_be_fetched() -> () {
    let mut cpu = Cpu::new();

    cpu.flags = 0b01001111;
    assert_eq!(0b01001111, cpu.flags());
}

#[test]
fn carry_flag_can_be_fetched() -> () {
    let mut cpu = Cpu::new();

    cpu.flags = 0b00110101;
    assert_eq!(true, cpu.flag(Flag::CARRY));

    cpu.flags = 0b00110100;
    assert_eq!(false, cpu.flag(Flag::CARRY));
}

#[test]
fn carry_flag_can_be_set() -> () {
    let mut cpu = Cpu::new();

    cpu.flags = 0b10111000;
    cpu.assign_flag(Flag::CARRY, true);
    assert_eq!(0b10111001, cpu.flags);

    cpu.flags = 0b10111001;
    cpu.assign_flag(Flag::CARRY, false);
    assert_eq!(0b10111000, cpu.flags);

    cpu.flags = 0b10111001;
    cpu.assign_flag(Flag::CARRY, true);
    assert_eq!(0b10111001, cpu.flags);

    cpu.flags = 0b10111000;
    cpu.assign_flag(Flag::CARRY, false);
    assert_eq!(0b10111000, cpu.flags);
}

#[test]
fn zero_flag_can_be_fetched() -> () {
    let mut cpu = Cpu::new();

    cpu.flags = 0b00110111;
    assert_eq!(true, cpu.flag(Flag::ZERO));

    cpu.flags = 0b00110101;
    assert_eq!(false, cpu.flag(Flag::ZERO));
}

#[test]
fn zero_flag_can_be_set() -> () {
    let mut cpu = Cpu::new();

    cpu.flags = 0b10111000;
    cpu.assign_flag(Flag::ZERO, true);
    assert_eq!(0b10111010, cpu.flags);

    cpu.flags = 0b10111010;
    cpu.assign_flag(Flag::ZERO, false);
    assert_eq!(0b10111000, cpu.flags);

    cpu.flags = 0b10111010;
    cpu.assign_flag(Flag::ZERO, true);
    assert_eq!(0b10111010, cpu.flags);

    cpu.flags = 0b10111000;
    cpu.assign_flag(Flag::ZERO, false);
    assert_eq!(0b10111000, cpu.flags);
}
