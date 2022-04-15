use std::ops::{Add, Div, Mul, Sub};

#[allow(non_camel_case_types)]
enum ByteCode {
    LOAD_VAL(isize),
    WRITE_VAR(char),
    READ_VAR(char),
    ADD,
    MULTIPLY,
    SUBTRACT,
    DIVIDE,
    RETURN_VALUE,
    TO(isize),
    GOTO(usize),
    PRINT(char),
    PRINTLN,
}

#[derive(Debug, Clone)]
struct Data {
    var: char,
    val: isize,
}

impl Add for Data {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            val: self.val + rhs.val,
            var: '-',
        }
    }
}

impl Mul for Data {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            val: self.val * rhs.val,
            var: '-',
        }
    }
}

impl Sub for Data {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            val: self.val - rhs.val,
            var: '-',
        }
    }
}

impl Div for Data {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            val: self.val / rhs.val,
            var: '-',
        }
    }
}

#[derive(Debug)]
struct Stack(Vec<Data>);

impl Stack {
    fn push(&mut self, value: Data) {
        self.0.push(value);
    }

    fn pop(&mut self) -> Data {
        self.0.pop().expect("poped an empty stack")
    }

    fn peek(&self) -> Data {
        self.0.last().expect("peeked an empty stack").to_owned()
    }

    fn move_top(&mut self, var: char) {
        for (i, data) in self.0.iter().enumerate() {
            if var == data.var {
                let item = data.to_owned();

                self.0.remove(i);
                self.push(item);

                break;
            }
        }
    }
}

fn interpret(bytecodes: &mut Vec<ByteCode>) -> isize {
    use ByteCode::*;

    let mut stack = Stack(Vec::new());
    let mut index = 0;
    let mut to = 0;

    while let Some(instruction) = bytecodes.get(index) {
        index += 1;

        match instruction {
            LOAD_VAL(val) => {
                let data = Data {
                    var: ' ',
                    val: *val,
                };
                stack.push(data);
            }

            WRITE_VAR(var) => {
                let data = stack.pop();
                stack.push(Data {
                    var: *var,
                    val: data.val,
                });
            }

            READ_VAR(var) => {
                stack.move_top(*var);
            }

            ADD => {
                let (item1, item2) = (stack.pop(), stack.pop());
                let sum = item1 + item2;

                stack.push(sum);
            }

            MULTIPLY => {
                let (item1, item2) = (stack.pop(), stack.pop());
                let mul = item1 * item2;

                stack.push(mul);
            }

            SUBTRACT => {
                let (item1, item2) = (stack.pop(), stack.pop());
                let sub = item2 - item1;

                stack.push(sub);
            }

            DIVIDE => {
                let (item1, item2) = (stack.pop(), stack.pop());
                let div = item2 / item1;

                stack.push(div);
            }

            RETURN_VALUE => {
                return stack.pop().val;
            }

            TO(val) => to = *val,

            GOTO(p) => {
                if stack.peek().val != to {
                    index = *p;

                    let item = stack.peek();
                    bytecodes[index] = LOAD_VAL(item.val);
                }
            }

            PRINT(ch) => {
                print!("{} {} ", stack.peek().val, ch);
            }

            PRINTLN => println!("{}", stack.peek().val),
        }
    }

    stack.pop().val
}

#[cfg(test)]
mod tests {
    use super::*;
    use ByteCode::*;

    #[test]
    fn test_1() {
        let mut bytecodes = vec![
            LOAD_VAL(1),
            WRITE_VAR('x'),
            LOAD_VAL(2),
            WRITE_VAR('y'),
            READ_VAR('x'),
            LOAD_VAL(1),
            ADD,
            READ_VAR('y'),
            MULTIPLY,
            RETURN_VALUE,
        ];

        let result = interpret(&mut bytecodes);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2() {
        let mut bytecodes = vec![
            LOAD_VAL(1),
            WRITE_VAR('x'),
            LOAD_VAL(2),
            WRITE_VAR('y'),
            READ_VAR('x'),
            LOAD_VAL(1),
            ADD,
            READ_VAR('y'),
            DIVIDE,
            RETURN_VALUE,
        ];

        let result = interpret(&mut bytecodes);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_3() {
        let mut bytecodes = vec![
            LOAD_VAL(1),
            WRITE_VAR('x'),
            LOAD_VAL(2),
            WRITE_VAR('y'),
            READ_VAR('x'),
            LOAD_VAL(1),
            ADD,
            READ_VAR('y'),
            SUBTRACT,
            RETURN_VALUE,
        ];

        let result = interpret(&mut bytecodes);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_4() {
        let mut bytecodes = vec![LOAD_VAL(5), LOAD_VAL(2), ADD, RETURN_VALUE];

        let result = interpret(&mut bytecodes);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_5() {
        let to: isize = 6;
        let mut bytecodes = vec![
            TO(to),
            LOAD_VAL(10),
            PRINT('-'),
            LOAD_VAL(1),
            PRINT('='),
            SUBTRACT,
            PRINTLN,
            GOTO(1),
            RETURN_VALUE,
        ];

        let result = interpret(&mut bytecodes);
        assert_eq!(result, to);
    }

    #[test]
    fn test_6() {
        let to: isize = 10;
        let mut bytecodes = vec![
            TO(to),
            LOAD_VAL(0),
            PRINT('+'),
            LOAD_VAL(1),
            PRINT('='),
            ADD,
            PRINTLN,
            GOTO(1),
            RETURN_VALUE,
        ];

        let result = interpret(&mut bytecodes);
        assert_eq!(result, to);
    }
}
