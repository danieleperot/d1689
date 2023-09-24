use crate::cpu::{Cpu, Register};
use crate::instructions::add::Add;
use crate::instructions::{Argument, Instruction};

mod cpu;
mod instructions;

fn main() {
    let mut cpu = Cpu::new();

    cpu.assign_register(Register::A, 18);
    println!("Adding 24 to register A containing 18");
    Add::execute(
        &mut cpu,
        vec![Argument::Register(Register::A), Argument::Byte(24)],
    )
    .unwrap();

    println!("Result: {}", cpu.register(Register::A));
    println!("Flags: {:#010b}", cpu.flags());
    println!("Zero flag: {}", cpu.flag(cpu::Flag::Zero));
    println!("Carry flag: {}", cpu.flag(cpu::Flag::Carry));

    println!("\nAdding 214 to register A containing previous result (42)");
    Add::execute(
        &mut cpu,
        vec![Argument::Register(Register::A), Argument::Byte(214)],
    )
    .unwrap();

    println!("Result: {}", cpu.register(Register::A));
    println!("Flags: {:#010b}", cpu.flags());
    println!("Zero flag: {}", cpu.flag(cpu::Flag::Zero));
    println!("Carry flag: {}", cpu.flag(cpu::Flag::Carry));
}
