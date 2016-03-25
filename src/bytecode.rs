//! Bytecode Pattern
//! http://gameprogrammingpatterns.com/bytecode.html

use num::FromPrimitive;

pub fn set_health(wizard: f32, value: f32) {
    println!("Changing wizard nr {} health to: {}", wizard, value);
    // ...
}
pub fn set_wisdom(wizard: f32, value: f32) {
    println!("Changing wizard nr {} wisdom to: {}", wizard, value);
    // ...
}
pub fn set_agility(wizard: f32, value: f32) {
    println!("Changing wizard nr {} agility to: {}", wizard, value);
    // ...
}
pub fn get_health(wizard: f32) -> f32 {
    println!("Getting health of wizard nr {}", wizard);
    // ...
    5.0 // Let's just assume it was 5
}
pub fn get_wisdom(wizard: f32) -> f32 {
    println!("Getting wisdom of wizard nr {}", wizard);
    // ...
    12.0
}
pub fn get_agility(wizard: f32) -> f32 {
    println!("Getting agility of wizard nr {}", wizard);
    // ...
    8.0
}
pub fn play_sound(id: f32) {
    println!("Playing sound with id: {}", id);
    // ...
}
pub fn spawn_particles(id: f32) {
    println!("Spawning particle system with id: {}", id);
    // ...
}

enum_from_primitive!{
    #[derive(Debug)]
    pub enum Instruction {
        Literal        = 1,
        SetHealth      = 2,
        SetAgility     = 3,
        SetWisdom      = 4,
        PlaySound      = 5,
        SpawnParticles = 6,
        GetHealth      = 7,
        GetAgility     = 8,
        GetWisdom      = 9,
        Add            = 10,
        Divide         = 11,
    }
}

#[derive(Debug, Default)]
pub struct VM {
    pub bytecode: Vec<u8>,
    pub stack: Vec<f32>,
}

impl VM {
    pub fn new(bytecode: Vec<u8>) -> VM {
        VM {
            stack: Vec::with_capacity(128),
            bytecode: bytecode,
        }
    }

    pub fn push(&mut self, val: f32) {
        self.stack.push(val);
    }

    /// # Panics
    /// * If stack is empty
    pub fn pop(&mut self) -> f32 {
        self.stack.pop().unwrap()
    }

    pub fn interpret(&mut self) {
        let mut push = false;
        for byte in self.bytecode.clone() {
            if push {
                self.push(byte as f32);
                push = false;
            } else {
                // Check if interpreted instruction results in push of next value on to the stack.
                push = self.interpret_instruction(byte);
            }
        }
    }

    /// Interpret instruction and call associated function. Returns True if next byte needs to be
    /// pushed on to the stack.
    fn interpret_instruction(&mut self, instruction: u8) -> bool {
        match Instruction::from_u8(instruction).unwrap() {
            Instruction::SetHealth => {
                let amount = self.pop();
                let wizard = self.pop();
                set_health(wizard, amount);
            }
            Instruction::SetAgility => {
                let amount = self.pop();
                let wizard = self.pop();
                set_agility(wizard, amount);
            }
            Instruction::SetWisdom => {
                let amount = self.pop();
                let wizard = self.pop();
                set_wisdom(wizard, amount);
            }
            Instruction::PlaySound => {
                play_sound(self.pop());
            }
            Instruction::SpawnParticles => {
                spawn_particles(self.pop());
            }
            Instruction::GetHealth => {
                let wizard = self.pop();
                self.push(get_health(wizard));
            }
            Instruction::GetAgility => {
                let wizard = self.pop();
                self.push(get_agility(wizard));
            }
            Instruction::GetWisdom => {
                let wizard = self.pop();
                self.push(get_wisdom(wizard));
            }
            Instruction::Add => {
                let b = self.pop();
                let a = self.pop();
                println!("Adding {} + {}", a, b);
                self.push(a + b);
            }
            Instruction::Divide => {
                let b = self.pop();
                let a = self.pop();
                println!("Dividing {} / {}", a, b);
                self.push(a / b);
            }
            Instruction::Literal => return true,
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytecode() {
        let mut bytecode: Vec<u8> = Vec::new();
        // set_health(0, get_health(0) + (get_agility(0) + get_wisdom(0)) / 2);
        bytecode.push(1);  // Literal                       []
        bytecode.push(0);  // Wizard index                  [0]
        bytecode.push(1);  // Literal
        bytecode.push(0);  // Wizard index                  [0, 0]
        bytecode.push(7);  // Get health                    [0, 5]
        bytecode.push(1);  // Literal
        bytecode.push(0);  // Wizard index                  [0, 5, 0]
        bytecode.push(8);  // Get Agility                   [0, 5, 8]
        bytecode.push(1);  // Literal
        bytecode.push(0);  // Wizard index                  [0, 5, 8, 0]
        bytecode.push(9);  // Get Wisdom                    [0, 5, 8, 12]
        bytecode.push(10); // Add Agility and wisdom        [0, 5, 20]
        bytecode.push(1);  // Literal
        bytecode.push(2);  // Divisor                       [0, 5, 20, 2]
        bytecode.push(11); // Divide                        [0, 5, 10]
        bytecode.push(10); // Add average to current health [0, 15]
        bytecode.push(2);  // Set Health                    []
        let mut vm = VM::new(bytecode);
        vm.interpret();

        bytecode = Vec::new();
        bytecode.push(1); // Literal
        bytecode.push(0); // Index
        bytecode.push(7); // GetHealth
        vm = VM::new(bytecode);
        vm.interpret();
        assert!((vm.stack[0]).round() as i32 == 5, "Should be get_health value");
    }
}
