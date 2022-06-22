#[derive(Debug, PartialEq)]
pub enum Token {
    Right,
    Left,
    Plus,
    Minus,
    Out,
    In,
    StartLoop,
    EndLoop,
}

pub fn tokenize(chars: &[char]) -> Vec<Token> {
    let mut result = Vec::new();

    for c in chars {
        match c {
            '>' => result.push(Token::Right),
            '<' => result.push(Token::Left),
            '+' => result.push(Token::Plus),
            '-' => result.push(Token::Minus),
            '.' => result.push(Token::Out),
            ',' => result.push(Token::In),
            '[' => result.push(Token::StartLoop),
            ']' => result.push(Token::EndLoop),
            _ => (),
        }   
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_shift_test() {
        let input = vec!['>'];
        
        assert_eq!(tokenize(&input)[0], Token::Right);
    }

    #[test]
    fn left_shift_test() {
        let input = vec!['<'];
        
        assert_eq!(tokenize(&input)[0], Token::Left);
    }
    
    #[test]
    fn plus_test() {
        let input = vec!['+'];
        
        assert_eq!(tokenize(&input)[0], Token::Plus);
    }

    #[test]
    fn minus_test() {
        let input = vec!['-'];
        
        assert_eq!(tokenize(&input)[0], Token::Minus);
    }

    #[test]
    fn out_test() {
        let input = vec!['.'];
        
        assert_eq!(tokenize(&input)[0], Token::Out);
    }

    #[test]
    fn in_test() {
        let input = vec![','];
        
        assert_eq!(tokenize(&input)[0], Token::In);
    }

    #[test]
    fn startloop_test() {
        let input = vec!['['];
        
        assert_eq!(tokenize(&input)[0], Token::StartLoop);
    }

    #[test]
    fn endloop_test() {
        let input = vec![']'];
        
        assert_eq!(tokenize(&input)[0], Token::EndLoop);
    }

    #[test]
    fn ignores_other_chars() {
        let bf_chars = vec!['>', '<', '+', '-', '.', ',', '[', ']'];
        let mut input = Vec::new();

        for i in 0..u8::MAX {
            if !bf_chars.contains(&(i as char)) {
                input.push(i as char);
            }
        }

        let tokens = tokenize(&input);

        assert_eq!(0, tokens.len());
    }
}