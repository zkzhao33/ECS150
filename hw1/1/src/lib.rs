

pub enum Operator {
    // `+`
    Add, 
    // `-`
    Sub,
    // `*`
    Mul,
}

pub enum Token {
    Operator(Operator),
    Operand(isize),
}

/// Evaluates the postix expression.
///
/// Input: a postfix expression, where each element contains an operator or operand.
/// Returns: if the postfix expression is valid, returns `Some(value)`;
///     otherwise, returns `None`.
pub fn eval(tokens: &[Token]) -> Option<isize> {
    // TODO
    //let mut tokens = &tokenss;
    let mut vec = Vec::new();
    let mut x = 1;
    match tokens[0] {
       Token :: Operator(ref ope) => return None,
       Token :: Operand(n) => { 
       	if tokens.len() == 1 {
       		return Some(n);
       	}
       	vec.push(n)
       },
    }
    match tokens[1] {
       Token :: Operator(ref ope) => return None,
       Token :: Operand(n) => vec.push(n),
    }
    //let last = tokens.len();
    for i in 2..tokens.len(){
       // vec.push(tokens[i]);
        match tokens[i] {
            Token :: Operator(ref ope) =>{
                match  ope {
                   &Operator :: Add =>{
                   		if vec.len() < 2 {
                   			return None;
                   		}
                        x = vec.pop().unwrap() + vec.pop().unwrap();
                    },
                    
                   &Operator :: Sub => {
                   	if vec.len() < 2 {
                   			return None;
                   		}
                        x = -vec.pop().unwrap() + vec.pop().unwrap();
                    },
                    
                    &Operator :: Mul => {
                    	if vec.len() < 2 {
                   			return None;
                   		}
                        x = vec.pop().unwrap() * vec.pop().unwrap();
                    },
                };
               // vec.pop();
               //vec.pop();
               // vec.pop();
                vec.push(x);
            },
             Token :: Operand(n) => {
             	
                vec.push(n);
            }

        }
    }
    if vec.len() > 1 {
    	return None;
    }
    return Some(x);
   // unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn it_works() {
    let a = Token :: Operand(3);
//let b = Token :: Operand(2);
  //  let c = Token :: Operator(Operator :: Add);
  //  let d = Token :: Operator(Operator :: Add);
   // let e = Token :: Operand(3);
   // let f = Token :: Operand(3);
   // let g = Token :: Operand(3);
   // let h = Token :: Operator(Operator :: Add);
   // let i = Token :: Operator(Operator :: Add);
    let x = [a];
    assert_eq!(Some(3),eval(&x));
    }
}





//fn main() {
   // let mut v: Vec<Token> = Vec::new();
   // v.push(Token::Operand(1));
   // v.push(Token::Operand(2));
   // v.push(Token::Operator(Operator::Add));
    //eval(&v);
//}
