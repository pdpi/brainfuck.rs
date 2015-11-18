use memory::Memory;

#[derive(PartialEq,Eq)]
pub enum Instruction {
  Incr, Decr, Add(i16),
  Next, Prev,
  Get, Put,
  Jump(usize),
  End
}

#[derive(PartialEq,Eq)]
pub struct Program {
  instrs: Vec<Instruction>
}

impl Program {
  pub fn new(instrs: Vec<Instruction>) -> Program {
    Program {
      instrs: instrs
    }
  }
  pub fn parse(s: &String) -> Program {
    let mut stack = Vec::new();
    let instrs = Vec::new();
    let mut program = Program { instrs: instrs };

    for c in s.chars() {
      match c {
        '+' => {
          program.instrs.push(Instruction::Incr);
        }
        '-' => {
          program.instrs.push(Instruction::Decr);
        }
        '>' => {
          program.instrs.push(Instruction::Next);
        }
        '<' => {
          program.instrs.push(Instruction::Prev);
        }
        ',' => {
          program.instrs.push(Instruction::Get);
        }
        '.' => {
          program.instrs.push(Instruction::Put);
        }
        '[' => { stack.push(program.instrs.len()); },
        ']' => {
          let idx = stack.pop().unwrap();
          program.instrs.push(Instruction::Jump(idx));
        }
        _   => ()
      }
    }
    program.instrs.push(Instruction::End);
    program
  }

  pub fn execute(&self, mem: &mut Memory) {
    let mut counter = 0;
    while self.instrs[counter] != Instruction::End {
      match self.instrs[counter] {
        Instruction::Incr => mem.incr(),
        Instruction::Decr => mem.decr(),
        Instruction::Add(n) => mem.add(n),
        Instruction::Next => mem.next(),
        Instruction::Prev => mem.prev(),
        Instruction::Put => print!("{}",mem.read() as char),
        Instruction::Get => (), // panic!("NYI"),
        Instruction::Jump(idx) => if mem.read() != 0 { counter = idx - 1; },
        _ => ()
      }
      counter += 1;
    }
  }
}

#[test]
fn it_runs(){
  let p = Program::new(vec![Instruction::Incr,Instruction::Incr,Instruction::Decr]);
  let mut m = Memory::new();
  p.execute(&mut m);
  assert!(m.read() == 1);
}

#[test]
fn it_parses(){
  let p = Program::parse(&"+>[-]".to_string());
  assert!(p.instrs[0] == Instruction::Incr);
  assert!(p.instrs[1] == Instruction::Next);
  assert!(p.instrs[2] == Instruction::Loop(Program::new(vec![Instruction::Decr])));
}