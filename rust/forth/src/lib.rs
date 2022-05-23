pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Debug)]
struct Definition {
    name: String,
    body: String,
    env_end: usize, // Forth.env[0..env_end] denotes the env for the definition
}

#[derive(Debug)]
struct ExecTerm {
    term: String,
    env_end: usize, // Forth.env[0..env_end] denotes the current env
}

#[derive(Debug)]
pub struct Forth {
    env: Vec<Definition>,
    valst: Vec<Value>,     // value stack
    execst: Vec<ExecTerm>, // execution stack
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
            env: Vec::new(),
            valst: Vec::new(),
            execst: Vec::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.valst.as_slice()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let term_strings = self.parse(input)?;
        for ts in term_strings {
            self.execst.push(ExecTerm {
                term: ts.to_ascii_uppercase(),
                env_end: self.env.len(),
            });
            while !self.execst.is_empty() {
                let ts = self.execst.pop().unwrap();
                self.eval_execterm(&ts)?;
            }
        }

        Ok(())
    }

    /// Parse the input to term string
    fn parse<'a>(&self, input: &'a str) -> std::result::Result<Vec<&'a str>, Error> {
        const NOT_STARTED: i32 = 0;
        const BEGIN_WORD: i32 = 1;
        const BEGIN_DEF: i32 = 2;
        let mut beg = usize::MAX;
        let mut end;
        let mut state = NOT_STARTED;
        let mut ans: Vec<&str> = Vec::new();
        for (i, ch) in input.char_indices() {
            if state == NOT_STARTED {
                if !ch.is_ascii_whitespace() {
                    if ch == ':' {
                        state = BEGIN_DEF;
                        beg = i;
                    } else if ch == ';' {
                        return Err(Error::InvalidWord);
                    } else {
                        state = BEGIN_WORD;
                        beg = i;
                    }
                }
            } else if state == BEGIN_WORD {
                if ch.is_ascii_whitespace() {
                    end = i;
                    ans.push(&input[beg..end]);
                    state = NOT_STARTED;
                }
            } else if state == BEGIN_DEF {
                if ch == ';' {
                    end = i;
                    ans.push(&input[beg..end]);
                    state = NOT_STARTED;
                } else if ch == ':' {
                    panic!("invalid input, nested def is not supported");
                }
            } else {
                panic!("program error in parse()");
            }
        }
        if state == BEGIN_WORD {
            end = input.len();
            ans.push(&input[beg..end]);
        } else if state == BEGIN_DEF {
            return Err(Error::InvalidWord);
        }
        Ok(ans)
    }

    // ts is an ASCII uppercase string
    fn eval_execterm(&mut self, execterm: &ExecTerm) -> Result {
        let ts = &execterm.term;
        let env_end = execterm.env_end;
        let bytes: &[u8] = ts.as_bytes();
        if bytes.len() >= 1 && bytes[0] as char == ':' {
            // add word definition to self.env
            // the definition encloses its environment: self.env[0..env_end]
            self.parse_def(ts, env_end)?;
        } else if is_num(ts) {
            self.valst.push(ts.parse::<i32>().unwrap());
        } else if let Some(index) = self.env_get(ts, env_end) {
            let def = &self.env[index];
            let term_strings = self.parse(&def.body)?;
            for ts in term_strings.into_iter().rev() {
                self.execst.push(ExecTerm {
                    term: ts.to_string(),
                    env_end: def.env_end,
                });
            }
        } else if ts == "+" {
            self.arith(|a, b| Ok(a + b))?
        } else if ts == "-" {
            self.arith(|a, b| Ok(a - b))?
        } else if ts == "*" {
            self.arith(|a, b| Ok(a * b))?
        } else if ts == "/" {
            self.arith(|a, b| {
                if b == 0 {
                    Err(Error::DivisionByZero)
                } else {
                    Ok(a / b)
                }
            })?
        } else if ts == "DROP" {
            self.pop()?;
        } else if ts == "DUP" {
            if self.valst.len() < 1 {
                return Err(Error::StackUnderflow);
            }
            let a = self.valst[self.valst.len() - 1];
            self.valst.push(a);
        } else if ts == "OVER" {
            if self.valst.len() < 2 {
                return Err(Error::StackUnderflow);
            }
            self.valst.push(self.valst[self.valst.len() - 2]);
        } else if ts == "SWAP" {
            let top = self.pop()?;
            let bottom = self.pop()?;
            self.valst.push(top);
            self.valst.push(bottom);
        } else {
            return Err(Error::UnknownWord);
        }
        Ok(())
    }

    fn parse_def(&mut self, term_string: &str, env_end: usize) -> std::result::Result<(), Error> {
        let tokens = term_string.split_ascii_whitespace().collect::<Vec<_>>();
        if tokens.len() < 3 {
            // ";" is not included
            return Err(Error::InvalidWord);
        }
        if is_num(tokens[1]) {
            return Err(Error::InvalidWord);
        }
        self.env.push(Definition {
            name: tokens[1].to_ascii_uppercase(),
            body: tokens[2..].join(" "),
            env_end,
        });
        Ok(())
    }

    // pre: word is ASCII uppercase
    fn env_get(&self, word: &str, env_end: usize) -> Option<usize> {
        assert!(env_end <= self.env.len());
        for i in (0..env_end).rev() {
            if self.env[i].name == word {
                return Some(i);
            }
        }
        None
    }

    fn pop(&mut self) -> std::result::Result<Value, Error> {
        match self.valst.pop() {
            Some(v) => Ok(v),
            None => Err(Error::StackUnderflow),
        }
    }

    fn arith<F>(&mut self, op: F) -> Result
    where
        F: FnOnce(Value, Value) -> std::result::Result<Value, Error>,
    {
        let rhs = self.pop()?;
        let lhs = self.pop()?;
        self.valst.push(op(lhs, rhs)?);
        Ok(())
    }
}

fn is_num(token: &str) -> bool {
    token.chars().all(|c| c.is_digit(10))
}
