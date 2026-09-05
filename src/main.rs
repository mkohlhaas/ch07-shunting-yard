#![allow(dead_code)]

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Op(char),
    LParen,
    RParen,
}

#[derive(Debug)]
struct ASTNode {
    value: Token,
    left: Option<Box<ASTNode>>,
    right: Option<Box<ASTNode>>,
}

impl ASTNode {
    fn leaf(val: f64) -> Self {
        ASTNode {
            value: Token::Number(val),
            left: None,
            right: None,
        }
    }

    fn branch(op: char, left: ASTNode, right: ASTNode) -> Self {
        ASTNode {
            value: Token::Op(op),
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

// Whether the character is a supported binary operator
fn is_binary_op(op: char) -> bool {
    matches!(op, '+' | '-' | '*' | '/')
}

// Get operator precedence weight
fn precedence(op: char) -> i32 {
    match op {
        '*' | '/' => 2,
        '+' | '-' => 1,
        _ => 0,
    }
}

fn build_ast(tokens: Vec<Token>) -> Result<ASTNode, &'static str> {
    let mut operator_stack: Vec<Token> = Vec::new();
    let mut operand_stack: Vec<ASTNode> = Vec::new();

    let build_tree_step = |operator_stack: &mut Vec<Token>,
                           operand_stack: &mut Vec<ASTNode>|
     -> Result<(), &'static str> {
        if let Some(Token::Op(op)) = operator_stack.pop() {
            // Right child is popped first due to LIFO behavior
            let right = operand_stack
                .pop()
                .ok_or("Malformed expression: missing operand")?;
            let left = operand_stack
                .pop()
                .ok_or("Malformed expression: missing operand")?;

            let parent_node = ASTNode::branch(op, left, right);
            operand_stack.push(parent_node);
            Ok(())
        } else {
            Err("Expected operator on stack")
        }
    };

    for token in tokens {
        match token {
            Token::Number(val) => {
                // Rule 1: Operands become leaves
                operand_stack.push(ASTNode::leaf(val));
            }
            Token::LParen => {
                // Rule 2: Open parenthesis
                operator_stack.push(token);
            }
            Token::Op(op1) => {
                // Rule 3: Operators handling priority
                if !is_binary_op(op1) {
                    return Err("Unsupported operator");
                }
                while let Some(Token::Op(op2)) = operator_stack.last() {
                    if precedence(*op2) >= precedence(op1) {
                        build_tree_step(&mut operator_stack, &mut operand_stack)?;
                    } else {
                        break;
                    }
                }
                operator_stack.push(Token::Op(op1));
            }
            Token::RParen => {
                // Rule 4: Close parenthesis unwinding
                while let Some(top_token) = operator_stack.last() {
                    if matches!(top_token, Token::LParen) {
                        break;
                    }
                    build_tree_step(&mut operator_stack, &mut operand_stack)?;
                }
                // Pop and discard the matching left parenthesis
                if !matches!(operator_stack.pop(), Some(Token::LParen)) {
                    return Err("Mismatched parentheses");
                }
            }
        }
    }

    // Rule 5: Flush out any remaining operations
    while !operator_stack.is_empty() {
        if matches!(operator_stack.last(), Some(Token::LParen)) {
            return Err("Mismatched parentheses");
        }
        build_tree_step(&mut operator_stack, &mut operand_stack)?;
    }

    // The single remaining item is our root node
    match operand_stack.len() {
        1 => operand_stack.pop().ok_or("Malformed expression: no result"),
        0 => Err("Empty expression"),
        _ => Err("Malformed expression: multiple root nodes generated"),
    }
}

// Convert an infix expression string into tokens.
fn tokenize(expr: &str) -> Result<Vec<Token>, &'static str> {
    let mut tokens = Vec::new();
    let mut chars = expr.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }
        if c.is_ascii_digit() {
            let mut number = String::new();
            while let Some(&d) = chars.peek() {
                if d.is_ascii_digit() {
                    number.push(d);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::Number(
                number.parse().expect("digits form a valid f64"),
            ));
            continue;
        }
        chars.next();
        match c {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            op if is_binary_op(op) => tokens.push(Token::Op(op)),
            _ => return Err("Unexpected character in expression"),
        }
    }
    Ok(tokens)
}

fn main() {
    let expr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "3 + 4 * 2".to_string());

    match tokenize(&expr).and_then(build_ast) {
        Ok(root) => {
            println!("Expression: {}", expr);
            println!("{:#?}", root);
        }
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(expr: &str) -> Result<ASTNode, &'static str> {
        build_ast(tokenize(expr)?)
    }

    fn eval(node: &ASTNode) -> f64 {
        match &node.value {
            Token::Number(v) => *v,
            Token::Op(op) => {
                let left = eval(node.left.as_ref().unwrap());
                let right = eval(node.right.as_ref().unwrap());
                match op {
                    '+' => left + right,
                    '-' => left - right,
                    '*' => left * right,
                    '/' => left / right,
                    _ => unreachable!("unexpected operator"),
                }
            }
            _ => unreachable!("branch node value is an operator"),
        }
    }

    #[test]
    fn precedence_weights() {
        assert_eq!(precedence('*'), 2);
        assert_eq!(precedence('/'), 2);
        assert_eq!(precedence('+'), 1);
        assert_eq!(precedence('-'), 1);
        assert_eq!(precedence('@'), 0);
    }

    #[test]
    fn single_number_leaf() {
        let root = parse("5").unwrap();
        assert!(matches!(root.value, Token::Number(5.0)));
        assert!(root.left.is_none());
        assert!(root.right.is_none());
    }

    #[test]
    fn simple_addition() {
        let root = parse("3+4").unwrap();
        assert_eq!(eval(&root), 7.0);
    }

    #[test]
    fn respects_operator_precedence() {
        let root = parse("3+4*2").unwrap();
        assert_eq!(eval(&root), 11.0);
    }

    #[test]
    fn left_associativity() {
        let root = parse("8-3-2").unwrap();
        assert_eq!(eval(&root), 3.0);
    }

    #[test]
    fn parentheses_override_precedence() {
        let root = parse("(3+4)*2").unwrap();
        assert_eq!(eval(&root), 14.0);
    }

    #[test]
    fn nested_parentheses() {
        let root = parse("2*(3+(4*5))").unwrap();
        assert_eq!(eval(&root), 46.0);
    }

    #[test]
    fn division_and_multiplication() {
        let root = parse("20/4*5").unwrap();
        assert_eq!(eval(&root), 25.0);
    }

    #[test]
    fn missing_operand_rejected() {
        let err = parse("3+").unwrap_err();
        assert!(err.contains("missing operand"));
    }

    #[test]
    fn unbalanced_open_paren_rejected() {
        let err = parse("(3+4").unwrap_err();
        assert!(err.contains("Mismatched parentheses"));
    }

    #[test]
    fn unbalanced_close_paren_rejected() {
        let err = parse("3+4)").unwrap_err();
        assert!(err.contains("Mismatched parentheses"));
    }

    #[test]
    fn multiple_root_nodes_rejected() {
        let err = parse("3 4").unwrap_err();
        assert!(err.contains("multiple root nodes"));
    }

    #[test]
    fn multi_digit_numbers() {
        let root = parse("12+8").unwrap();
        assert_eq!(eval(&root), 20.0);
    }

    #[test]
    fn unexpected_character_rejected() {
        let err = parse("3@4").unwrap_err();
        assert!(err.contains("Unexpected character"));
    }

    #[test]
    fn unsupported_operator_rejected() {
        let tokens = vec![Token::Number(3.0), Token::Op('^'), Token::Number(4.0)];
        let err = build_ast(tokens).unwrap_err();
        assert!(err.contains("Unsupported operator"));
    }

    #[test]
    fn empty_expression_rejected() {
        let err = parse("").unwrap_err();
        assert!(err.contains("Empty expression"));
    }
}
