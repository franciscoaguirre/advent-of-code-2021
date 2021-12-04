pub enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl Instruction {
    pub fn from_string(string: &str) -> Self {
        let (instruction, amount) = string.split_once(' ').unwrap();
        let parsed_amount: u32 = amount.parse().unwrap();
        match instruction {
            "forward" => Instruction::Forward(parsed_amount),
            "down" => Instruction::Down(parsed_amount),
            "up" => Instruction::Up(parsed_amount),
            _ => panic!("Badly formatted string!"),
        }
    }
}
