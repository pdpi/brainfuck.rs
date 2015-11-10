use memory::Memory;

#[derive(PartialEq,Eq)]
pub enum Instruction {
  Incr, Decr, Add(i16),
  Next, Prev,
  Get, Put,
  Loop(Program)
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
    let instrs = Vec::new();
    let mut stack = Vec::new();
    stack.push(Program { instrs: instrs });
    for c in s.chars() {
      let idx = stack.len() - 1;
      match c {
        '+' => {
          let frame = &mut stack[idx].instrs;
          frame.push(Instruction::Incr);
        }
        '-' => {
          let frame = &mut stack[idx].instrs;
          frame.push(Instruction::Decr);
        }
        '>' => {
          let frame = &mut stack[idx].instrs;
          frame.push(Instruction::Next);
        }
        '<' => {
          let frame = &mut stack[idx].instrs;
          frame.push(Instruction::Prev);
        }
        ',' => {
          let frame = &mut stack[idx].instrs;
          frame.push(Instruction::Get);
        }
        '.' => {
          let frame = &mut stack[idx].instrs;
          frame.push(Instruction::Put);
        }
        '[' => stack.push(Program { instrs: Vec::new() }),
        ']' => {
          let instrs = stack.pop().unwrap();
          let idx = stack.len() - 1;
          let frame = &mut stack[idx].instrs;
          frame.push(Instruction::Loop(instrs))
        }
        _   => ()
      }
    }
    stack.pop().unwrap()
  }

  pub fn execute(&self, mem: &mut Memory) {
    for i in &self.instrs {
      match *i {
        Instruction::Incr => mem.incr(),
        Instruction::Decr => mem.decr(),
        Instruction::Add(n) => mem.add(n),
        Instruction::Next => mem.next(),
        Instruction::Prev => mem.prev(),
        Instruction::Put => print!("{}",mem.read() as char),
        Instruction::Get => panic!("NYI"),
        Instruction::Loop(ref instrs) => {
          while mem.read() != 0 {
            instrs.execute(mem);
          }
        }
      }
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