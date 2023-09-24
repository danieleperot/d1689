use crate::cpu::{Cpu, Register};
use crate::instructions::add::Add;
use crate::instructions::load::Load;
use crate::instructions::{Argument, Instruction};

mod cpu;
mod instructions;

fn main() {
    let mut cpu = Cpu::new();

    println!("Load value 18 to register A");
    Load::execute(
        &mut cpu,
        vec![Argument::Register(Register::A), Argument::Byte(18)],
    )
    .unwrap();
    debug_cpu(&mut cpu);

    println!("Adding 24 to register A containing 18");
    Add::execute(
        &mut cpu,
        vec![Argument::Register(Register::A), Argument::Byte(24)],
    )
    .unwrap();
    debug_cpu(&mut cpu);

    println!("\nAdding 214 to register A containing previous result (42)");
    Add::execute(
        &mut cpu,
        vec![Argument::Register(Register::A), Argument::Byte(214)],
    )
    .unwrap();
    debug_cpu(&mut cpu);
}

fn debug_cpu(cpu: &mut Cpu) {
    println!("Registers:");

    let register_a = cpu.register(Register::A);
    println!("\tA: {register_a}, [{register_a:#04x}]");

    let register_b = cpu.register(Register::B);
    println!("\tA: {register_b}, [{register_b:#04x}]");

    println!("Flags: [{:#010b}]", cpu.flags());
    println!("\tZero flag: {}", cpu.flag(cpu::Flag::Zero));
    println!("\tCarry flag: {}", cpu.flag(cpu::Flag::Carry));
}
