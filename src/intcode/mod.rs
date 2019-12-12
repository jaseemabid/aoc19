pub struct Machine {
    ops: Vec<usize>,
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add { a: usize, b: usize, dest: usize },
    Mul { a: usize, b: usize, dest: usize },
    Halt,
}

impl Machine {
    pub fn new(ops: &[usize]) -> Self {
        Machine {
            ops: Vec::from(ops),
        }
    }

    pub fn run(&mut self) -> usize {
        self.step(0)
    }

    fn step(&mut self, pc: usize) -> usize {
        match self.exec(pc) {
            None => self.step(pc + 4),
            Some(result) => result,
        }
    }

    fn exec(&mut self, pc: usize) -> Option<usize> {
        match self.op(pc) {
            Op::Add { a, b, dest } => {
                self.write(dest, a + b);
                None
            }
            Op::Mul { a, b, dest } => {
                self.write(dest, a * b);
                None
            }
            Op::Halt => Some(self.read(0)),
        }
    }

    fn op(&self, pc: usize) -> Op {
        match self.read(pc) {
            1 => Op::Add {
                a: self.read(self.read(pc + 1)),
                b: self.read(self.read(pc + 2)),
                dest: self.read(pc + 3),
            },
            2 => Op::Mul {
                a: self.read(self.read(pc + 1)),
                b: self.read(self.read(pc + 2)),
                dest: self.read(pc + 3),
            },
            99 => Op::Halt,
            op => panic!("Unknown OP code {} at {}", op, pc),
        }
    }

    fn read(&self, address: usize) -> usize {
        self.ops
            .get(address)
            .expect(&format!("Reading invalid address: {}", address))
            .clone()
    }

    fn write(&mut self, dest: usize, value: usize) {
        self.ops[dest] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let m = Machine::new(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);

        assert_eq!(
            m.op(0),
            Op::Add {
                a: 30,
                b: 40,
                dest: 3
            }
        );
    }

    #[test]
    fn test_exec() {
        let mut m = Machine::new(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);

        m.exec(0);
        assert_eq!(vec![1, 9, 10, 30 + 40, 2, 3, 11, 0, 99, 30, 40, 50], m.ops);

        m.exec(4);
        assert_eq!(vec![70 * 50, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], m.ops);
    }

    #[test]
    fn test_run() {
        let mut m = Machine::new(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);

        assert_eq!(3500, m.run())
    }

    #[test]
    fn test_one() {
        let mut m = Machine::new(&[
            1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 2, 9, 19, 23, 2, 23, 10,
            27, 1, 6, 27, 31, 1, 31, 6, 35, 2, 35, 10, 39, 1, 39, 5, 43, 2, 6, 43, 47, 2, 47, 10,
            51, 1, 51, 6, 55, 1, 55, 6, 59, 1, 9, 59, 63, 1, 63, 9, 67, 1, 67, 6, 71, 2, 71, 13,
            75, 1, 75, 5, 79, 1, 79, 9, 83, 2, 6, 83, 87, 1, 87, 5, 91, 2, 6, 91, 95, 1, 95, 9, 99,
            2, 6, 99, 103, 1, 5, 103, 107, 1, 6, 107, 111, 1, 111, 10, 115, 2, 115, 13, 119, 1,
            119, 6, 123, 1, 123, 2, 127, 1, 127, 5, 0, 99, 2, 14, 0, 0,
        ]);

        // Once you have a working computer, the first step is to restore the
        // gravity assist program (your puzzle input) to the "1202 program
        // alarm" state it had just before the last computer caught fire. To do
        // this, before running the program, replace position 1 with the value
        // 12 and replace position 2 with the value 2. What value is left at
        // position 0 after the program halts?

        m.ops[1] = 12;
        m.ops[2] = 2;

        assert_eq!(3716250, m.run())
    }
}