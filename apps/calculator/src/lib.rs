#![no_std]


#[derive(Clone, Copy)]
pub struct Calculator {
    accumulator: i64,
    pending: Option<Operation>,
    input: i64,
    has_input: bool,
    error: bool,
}

#[derive(Clone, Copy)]
enum Operation { Add, Subtract, Multiply, Divide }

impl Calculator {
    pub const fn new() -> Self {
        Self { accumulator: 0, pending: None, input: 0, has_input: false, error: false }
    }

    pub fn clear(&mut self) {
        self.accumulator = 0;
        self.pending = None;
        self.input = 0;
        self.has_input = false;
        self.error = false;
    }

    pub fn push_digit(&mut self, digit: u8) {
        if self.error || digit > 9 { return; }
        if !self.has_input {
            self.input = digit as i64;
            self.has_input = true;
            return;
        }
        match self.input.checked_mul(10).and_then(|v| v.checked_add(digit as i64)) {
            Some(value) => self.input = value,
            None => self.error = true,
        }
    }

    pub fn add(&mut self) { self.select_operation(Operation::Add); }
    pub fn subtract(&mut self) { self.select_operation(Operation::Subtract); }
    pub fn multiply(&mut self) { self.select_operation(Operation::Multiply); }
    pub fn divide(&mut self) { self.select_operation(Operation::Divide); }

    pub fn equals(&mut self) {
        if self.error { return; }
        if self.pending.is_some() && self.has_input {
            let rhs = self.input;
            match self.apply(self.accumulator, rhs) {
                Some(value) => {
                    self.accumulator = value;
                    self.input = value;
                    self.has_input = true;
                    self.pending = None;
                }
                None => self.error = true,
            }
        }
    }

    pub fn value(&self) -> Option<i64> {
        if self.error { None } else if self.has_input { Some(self.input) } else { Some(self.accumulator) }
    }

    pub fn is_error(&self) -> bool { self.error }

    fn select_operation(&mut self, op: Operation) {
        if self.error { return; }
        if self.pending.is_some() && self.has_input {
            match self.apply(self.accumulator, self.input) {
                Some(value) => self.accumulator = value,
                None => { self.error = true; return; }
            }
        } else if self.has_input {
            self.accumulator = self.input;
        }
        self.pending = Some(op);
        self.input = 0;
        self.has_input = false;
    }

    fn apply(&self, lhs: i64, rhs: i64) -> Option<i64> {
        match self.pending {
            Some(Operation::Add) => lhs.checked_add(rhs),
            Some(Operation::Subtract) => lhs.checked_sub(rhs),
            Some(Operation::Multiply) => lhs.checked_mul(rhs),
            Some(Operation::Divide) => if rhs == 0 { None } else { lhs.checked_div(rhs) },
            None => Some(rhs),
        }
    }
}
