mod stack;

use std::io;

fn main() {
    let mut expression = String::new();
    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read line");

    println!("answer: {}", calc(&expression))
}

fn calc(reverse_polish_notation: &str) -> i32 {
    let mut s = SimpleStack::initialize();

    for v in reverse_polish_notation.split_whitespace() {
        match v {
            "+" => {
                let (a, b) = pop_two_elements(&mut s);
                s.push(a + b);
            }
            "-" => {
                let (a, b) = pop_two_elements(&mut s);
                s.push(b - a);
            }
            "*" => {
                let (a, b) = pop_two_elements(&mut s);
                s.push(a * b);
            }
            _ => s.push(v.parse().unwrap()),
        }
    }
    s.pop()
}

fn pop_two_elements(s: &mut SimpleStack) -> (i32, i32) {
    let a = s.pop();
    let b = s.pop();
    (a, b)
}

pub struct SimpleStack {
    s: Vec<i32>,
    top: usize,
}

impl SimpleStack {
    pub fn initialize() -> Self {
        Self {
            s: vec![0; 5],
            top: 0,
        }
    }

    pub fn push(&mut self, num: i32) {
        if self.is_full() {
            panic!("overflow");
        }
        self.top += 1;
        self.s[self.top] = num;
    }

    fn is_full(&self) -> bool {
        self.top >= self.s.capacity()
    }

    pub fn pop(&mut self) -> i32 {
        if self.is_empty() {
            panic!("underflow");
        }
        self.top -= 1;
        self.s[self.top + 1]
    }

    fn is_empty(&self) -> bool {
        self.top == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn calculate_reverse_polish_notation() {
        let expression = "1 2 + 3 4 - *";
        assert_eq!(calc(expression), -3);
    }
}
