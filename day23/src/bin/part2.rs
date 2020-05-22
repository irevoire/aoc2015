use day23::*;

fn main() {
    let instructions: Vec<Instr> = aoc::parser::lines_from_args(1)
        .map(|line| line.parse().unwrap())
        .collect();

    let mut cpu = Cpu::default().with_instructions(instructions);
    cpu.a = 1;

    while !cpu.finished() {
        cpu.cycle();
    }

    println!("register B: {}", cpu.b);
}
