use super::instruction::{Instruction, Instruction::Down, Instruction::Forward, Instruction::Up};

pub fn solve(instructions: &[Instruction]) -> u32 {
    let (final_horizontal_position, final_depth) =
        instructions
            .iter()
            .fold(
                (0, 0),
                |(horizontal_position, depth), instruction| match instruction {
                    Forward(amount) => (horizontal_position + amount, depth),
                    Down(amount) => (horizontal_position, depth + amount),
                    Up(amount) => (horizontal_position, depth - amount),
                },
            );

    final_horizontal_position * final_depth
}
