use stack;

struct Mem {
    stack: Stack,
    segments: Vec<Segment>,
}

impl Mem {
    pub fn new(stacksz: u64) -> Mem {
        Mem {
            stack: stack::new(stacksz),
            segments: Vec::new()
        }
    }
}