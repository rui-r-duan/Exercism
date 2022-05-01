use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    env: HashMap<String, Vec<String>>,
    stack: Vec<Value>,
    def_buffer: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            env: HashMap::new(),
            stack: Vec::new(),
            def_buffer: Vec::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        for s in input.split(' ') {
            self.eval_cmd(s)?;
        }
        if !self.def_buffer.is_empty() {
            return Err(Error::InvalidWord);
        }
        Ok(())
    }

    fn eval_cmd(&mut self, cmd: &str) -> Result {
        let cmd = cmd.to_ascii_lowercase();
        println!("{:?}", self.def_buffer);
        if self.is_collecting_word_def() {
            if cmd != ";" {
                self.def_buffer.push(cmd);
            } else {
                match self.add_word() {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => {
                        eprintln!("{:?}: {:?}", e, self.def_buffer);
                        self.def_buffer.clear();
                        return Err(e);
                    }
                }
            }
        } else {
            if cmd == ":" {
                self.def_buffer.push(cmd);
            } else if is_num(&cmd) {
                self.stack.push(cmd.parse().unwrap());
            } else {
                let mut word_cmds: Vec<String> = Vec::new();
                let word_cmds_ref: Option<&Vec<String>> = self.env.get(&cmd);
                match word_cmds_ref {
                    Some(word_def) => {
                        for word_cmd in word_def {
                            word_cmds.push(word_cmd.to_owned());
                        }
                    }
                    None => {
                        if cmd == "drop" {
                            match self.stack.pop() {
                                Some(_) => (),
                                None => {
                                    return Err(Error::StackUnderflow);
                                }
                            }
                        } else if cmd == "swap" {
                            if self.stack.len() < 2 {
                                return Err(Error::StackUnderflow);
                            }
                            let top = self.stack.pop().unwrap();
                            let second = self.stack.pop().unwrap();
                            self.stack.push(top);
                            self.stack.push(second);
                        } else if cmd == "over" {
                            if self.stack.len() < 2 {
                                return Err(Error::StackUnderflow);
                            }
                            let n = self.stack.len();
                            let second = self.stack[n - 2];
                            self.stack.push(second);
                        } else if cmd == "dup" {
                            let n = self.stack.len();
                            if n < 1 {
                                return Err(Error::StackUnderflow);
                            }
                            self.stack.push(self.stack[n - 1]);
                        } else if cmd == "+" {
                            let n = self.stack.len();
                            if n < 2 {
                                return Err(Error::StackUnderflow);
                            }
                            let b = self.stack.pop().unwrap();
                            let a = self.stack.pop().unwrap();
                            self.stack.push(a + b);
                        } else if cmd == "-" {
                            let n = self.stack.len();
                            if n < 2 {
                                return Err(Error::StackUnderflow);
                            }
                            let b = self.stack.pop().unwrap();
                            let a = self.stack.pop().unwrap();
                            self.stack.push(a - b);
                        } else if cmd == "*" {
                            let n = self.stack.len();
                            if n < 2 {
                                return Err(Error::StackUnderflow);
                            }
                            let b = self.stack.pop().unwrap();
                            let a = self.stack.pop().unwrap();
                            self.stack.push(a * b);
                        } else if cmd == "/" {
                            let n = self.stack.len();
                            if n < 2 {
                                return Err(Error::StackUnderflow);
                            }
                            let b = self.stack.pop().unwrap();
                            if b == 0 {
                                return Err(Error::DivisionByZero);
                            }
                            let a = self.stack.pop().unwrap();
                            self.stack.push(a / b);
                        } else {
                            return Err(Error::UnknownWord);
                        }
                    }
                }
                for token in word_cmds {
                    self.eval_cmd(&token)?;
                }
            }
        }
        Ok(())
    }

    fn is_collecting_word_def(&self) -> bool {
        self.def_buffer.len() > 0
    }

    fn add_word(&mut self) -> Result {
        let buf = &self.def_buffer;
        if buf.len() < 3 || buf[0] != ":" {
            return Err(Error::InvalidWord);
        }
        let word_name = buf[1].clone();
        if !is_valid_word_name(&word_name) {
            return Err(Error::InvalidWord);
        }
        let word_def = buf[2..].iter().cloned().collect::<Vec<_>>();
        self.env.insert(word_name, word_def);
        self.def_buffer.clear();
        Ok(())
    }
}

fn is_num(token: &str) -> bool {
    token.chars().all(|c| c.is_digit(10))
}

fn is_valid_word_name(name: &str) -> bool {
    !is_num(name)
}
