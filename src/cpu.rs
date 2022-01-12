#[derive(Debug)]
pub struct Cpu {
    instruction: u8
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu{ instruction: 5 }
    }
}
