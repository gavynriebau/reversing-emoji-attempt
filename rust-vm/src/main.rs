use std::process::exit;
use std::fs::File;
use std::io::prelude::*;


struct VM {
    rom: Vec<String>,
    accumulator1: i64,
    accumulator2: i64,
    instruction_pointer: usize,
    stack: Vec<i64>,
    debug: bool
}

impl VM {

    fn new() -> VM {
        VM {
            rom: Vec::new(),
            accumulator1: 0,
            accumulator2: 0,
            instruction_pointer: 1,
            stack: Vec::new(),
            debug: false
        }
    }

    fn current_word(&self) -> String {
        let x = &self.rom[self.instruction_pointer];
        return String::from(x);
    }

    fn current_instruction(&self) -> char {
        let word = self.current_word();
        let cur_ins = word.chars().next().unwrap();

        return cur_ins;
    }

    fn step(&mut self) {
        let cur_ins = self.current_instruction();
        self.instruction_pointer += 1;

        if self.debug {
            println!("Processing instruction '{}'", cur_ins);
        }

        match cur_ins {
            '🖋' => {},
            '🍡' => {
                self.add();
            },
            '🤡' => {
                self.clone();
            },
            '📐' => {
                self.divide();
            },
            '😲' => {
                self.if_zero();
            },
            '😄' => {
                self.if_not_zero();
            },
            '🏀' => {
                self.jump_to();
            },
            '🚛' => {
                self.load();
            },
            '📬' => {
                self.modulo();
            },
            '⭐' => {
                self.multiply();
            },
            '🍿' => {
                self.pop();
            },
            '📤' => {
                self.pop_out();
            },
            '🎤' => {
                self.print_top();
            },
            '📥' => {
                self.push();
            },
            '🔪' => {
                self.sub();
            },
            '🌓' => {
                self.xor();
            },
            '⛰' => {
                self.jump_top();
            },
            '⌛' => {
                self.exit();
            }
            _ => println!("unknown instruction: '{}'", cur_ins)
        }
    }

    fn add(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(a + b);
    }

    fn sub(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(b - a);
    }

    fn if_zero(&mut self) {
        let last = self.stack.last().unwrap();
        if *last == 0 {
            if self.debug {
                println!("if_zero was zero");
            }
            while self.current_instruction() != '😐' {
                if self.current_instruction() == '🏀' {
                    break;
                }
                if self.current_instruction() == '⛰' {
                    break;
                }
                self.step();
            }
        } else {
            if self.debug {
                println!("if_zero was not zero '{}'", last);
            }
            self.find_first_endif();
            self.instruction_pointer += 1;
        }
    }

    fn if_not_zero(&mut self) {
        let last = self.stack.last().unwrap();
        if *last != 0 {
            while self.current_instruction() != '😐' {
                if self.current_instruction() == '🏀' {
                    break;
                }
                if self.current_instruction() == '⛰' {
                    break;
                }
                self.step();
            }
        } else {
            self.find_first_endif();
            self.instruction_pointer += 1;
        }
    }

    fn find_first_endif(&mut self) {
        while self.current_instruction() != '😐' {
            self.instruction_pointer += 1;
        }
    }

    fn jump_to(&mut self) {
        let marker = self.current_word();

        if marker.chars().next().unwrap() != '💰' {
            panic!("invalid instruction");
        }

        let mut replaced : Vec<char> = marker.chars().collect();
        replaced[0] = '🖋';

        let replaced_str : String = replaced.into_iter().collect();

        let idx = self.rom.iter().position(|s| s == &replaced_str).unwrap();
        self.instruction_pointer = idx + 1;
    }

    fn jump_top(&mut self) {
        let next = self.stack.pop().unwrap();
        if self.debug {
            println!("Jump top to '{0}'", next);
        }
        self.instruction_pointer = next as usize;
    }

    fn exit(&mut self) {
        exit(0);
    }

    fn print_top(&mut self) {
        let item = self.stack.pop().unwrap();
        println!("item is: '{}'", (item as u8) as char);
        std::io::stdout().flush().unwrap();
    }

    fn push(&mut self) {
        if self.current_instruction() == '🥇' {
            self.stack.push(self.accumulator1);
        } else if self.current_instruction() == '🥈' {
            self.stack.push(self.accumulator2);
        } else {
            panic!("unknown instruction");
        }
        self.instruction_pointer += 1;
    }

    fn pop(&mut self) {
        if self.current_instruction() == '🥇' {
            let item = self.stack.pop().unwrap();
            self.accumulator1 = item;
        } else if self.current_instruction() == '🥈' {
            let item = self.stack.pop().unwrap();
            self.accumulator2 = item;
        } else {
            panic!("unknown instruction");
        }
        self.instruction_pointer += 1;
    }

    fn pop_out(&mut self) {
        self.stack.pop();
    }

    fn load(&mut self) {
        let mut num = 0_i64;
        let acc: i64;

        if self.current_instruction() == '🥇' {
            acc = 1;
        } else if self.current_instruction() == '🥈' {
            acc = 2;
        } else {
            panic!("Unknown instruction");
        }

        self.instruction_pointer += 1;

        while self.current_instruction() != '✋' {
            num = num * 10 + self.current_instruction().to_digit(10).unwrap() as i64;
            self.instruction_pointer += 1;
        }

        if acc == 1 {
            self.accumulator1 = num;
        } else {
            self.accumulator2 = num;
        }

        self.instruction_pointer += 1;
    
    }

    fn clone(&mut self) {
        let last = self.stack.pop().unwrap();
        self.stack.push(last);
        self.stack.push(last);
    }

    fn multiply(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(b * a);
    }

    fn divide(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(b / a);
    }

    fn modulo(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(b % a);
    }

    fn xor(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        if self.debug {
            println!("xor {0} with prime {1}", b, a);
        }
        self.stack.push(b ^ a);
    }

}

fn main() -> std::io::Result<()> {
    println!("Starting VM");

    let mut file = File::open("program")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut vm = VM::new();

    vm.rom = contents
        .split_whitespace()
        .map(|s| String::from(s))
        .collect();

    // Done so that the number of items in "rom" match the number of items in
    // the python version of the app.
    vm.rom.insert(0, '_'.to_string());

    loop {
        vm.step();
    }
}
