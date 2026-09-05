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
                if matches!(operator_stack.pop(), Some(Token::LParen)) {
                    // Successfully discarded '('
                } else {
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
    if operand_stack.len() == 1 {
        Ok(operand_stack.pop().unwrap())
    } else {
        Err("Malformed expression: multiple root nodes generated")
    }
}

fn main() {
    // Represents: 3 + 4 * 2
    let tokens = vec![
        Token::Number(3.0),
        Token::Op('+'),
        Token::Number(4.0),
        Token::Op('*'),
        Token::Number(2.0),
    ];

    match build_ast(tokens) {
        Ok(root) => {
            println!("Successfully built AST!");
            println!("{:#?}", root);
        }
        Err(e) => println!("Error: {}", e),
    }
}
