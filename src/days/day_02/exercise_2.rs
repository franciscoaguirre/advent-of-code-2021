use super::instruction::{Instruction, Instruction::Down, Instruction::Forward, Instruction::Up};

pub fn solve(instructions: &[Instruction]) -> u32 {
    let (final_horizontal_position, final_depth, _) = instructions.iter().fold(
        (0, 0, 0),
        |(horizontal_position, depth, aim), instruction| match instruction {
            Forward(amount) => (horizontal_position + amount, depth + aim * amount, aim),
            Down(amount) => (horizontal_position, depth, aim + amount),
            Up(amount) => (horizontal_position, depth, aim - amount),
        },
    );

    final_horizontal_position * final_depth
}
