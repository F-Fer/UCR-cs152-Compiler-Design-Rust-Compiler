use std::{collections::VecDeque, env, fmt::format, fs};

mod interpreter;

fn main() {
    // get commandline arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide an input file.");
        return;
    }

    if args.len() > 2 {
        println!("Too many commandline arguments.");
        return;
    }

    // read the entire file.
    let filename = &args[1];
    let result = fs::read_to_string(filename);
    let code = match result {
        Err(error) => {
            println!("**Error. File \"{}\": {}", filename, error);
            return;
        }

        Ok(code) => {
            code
        }
    };

    let tokens = match lex(&code) {
        // If lex returns an Err, this block of code will execute.
        Err(error_message) => {
            println!("**Error**");
            println!("----------------------");
            println!("{}", error_message);
            println!("----------------------");
            return;
        },
        
        // lex returns an Ok
        Ok(data) => data,
    };

    // Parse tokens
    let mut index: usize = 0;
    match parse_program(&tokens, &mut index) {

        Ok(generated_code) => {
        println!("Generated Code:");
        println!("-------------------------------");
        println!("{generated_code}");
        println!("-------------------------------");
        interpreter::execute_ir(&generated_code);
        }

        Err(message) => {
            println!("**Error**");
            println!("----------------------");
            if tokens.len() == 0 {
                println!("No code has been provided.");
            } else {
                println!("Error: {message}");
                println!("----------------------");
            }
        }

    }
}



/*
----------------------------------------------------------------
SCANNER
*/



#[derive(Debug, Clone)]
enum Token {
    NotToken,
    Plus,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Assign,
    Num(i32),
    Ident(String),
    If,
    While,
    Read, 
    Func,
    Return,
    Int,

    Print,
    Else,
    Break,
    Continue,
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equality,
    NotEqual,
}


// Returns a List of Tokens or an Error String.
fn lex(mut code: &str) -> Result<Vec<Token>, String> {
    // List of Tokens.
    let mut tokens: Vec<Token> = vec![];
    while code.len() > 0 {

        // Number
        let (success, token, rest) = lex_number(code);
        if success {
            code = rest; 
            tokens.push(token);
            continue;
        } 
    
        // Remove leading whitespaces.
        let (success, rest) = lex_space(code);
        if success {
            code = rest;
            continue;
        }
    
        if code.starts_with("+") {
            code = &code[1..];
            tokens.push(Token::Plus);
            continue;
        }
    
        if code.starts_with("-") {
            code = &code[1..];
            tokens.push(Token::Subtract);
            continue;
        }
    
        if code.starts_with("*") {
            code = &code[1..];
            tokens.push(Token::Multiply);
            continue;
        }
    
        if code.starts_with("/") {
            code = &code[1..];
            tokens.push(Token::Divide);
            continue;
        }
    
        if code.starts_with("%") {
            code = &code[1..];
            tokens.push(Token::Modulus);
            continue;
        }

        if code.starts_with("==") {
            code = &code[2..];
            tokens.push(Token::Equality);
            continue;
        }
    
        if code.starts_with("=") {
            code = &code[1..];
            tokens.push(Token::Assign);
            continue;
        }

        if code.starts_with("(") {
            code = &code[1..];
            tokens.push(Token::LeftParen);
            continue;
        }

        if code.starts_with(")") {
            code = &code[1..];
            tokens.push(Token::RightParen);
            continue;
        }

        if code.starts_with("{") {
            code = &code[1..];
            tokens.push(Token::LeftCurly);
            continue;
        }

        if code.starts_with("}") {
            code = &code[1..];
            tokens.push(Token::RightCurly);
            continue;
        }

        if code.starts_with("[") {
            code = &code[1..];
            tokens.push(Token::LeftBracket);
            continue;
        }

        if code.starts_with("]") {
            code = &code[1..];
            tokens.push(Token::RightBracket);
            continue;
        }

        if code.starts_with(",") {
            code = &code[1..];
            tokens.push(Token::Comma);
            continue;
        }

        if code.starts_with(";") {
            code = &code[1..];
            tokens.push(Token::Semicolon);
            continue;
        }

        if code.starts_with("(") {
            code = &code[1..];
            tokens.push(Token::LeftParen);
            continue;
        }

        if code.starts_with("<=") {
            code = &code[2..];
            tokens.push(Token::LessEqual);
            continue;
        }

        if code.starts_with("<") {
            code = &code[1..];
            tokens.push(Token::Less);
            continue;
        }

        if code.starts_with(">=") {
            code = &code[2..];
            tokens.push(Token::GreaterEqual);
            continue;
        }

        if code.starts_with(">") {
            code = &code[1..];
            tokens.push(Token::Greater);
            continue;
        }

        if code.starts_with("!=") {
            code = &code[2..];
            tokens.push(Token::NotEqual);
            continue;
        }

        // Comment
        // Skip single-line comments
        if code.starts_with("#") {
            if let Some(newline_index) = code.find('\n') {
                // Move past the newline character to continue lexing from the next line.
                // This also safely handles the case where the comment is at the end of the file.
                code = &code[newline_index + 1..];
                continue;
            } else {
                // If no newline is found, the rest of the string is a comment.
                // We can break out of the lexing loop or continue as appropriate.
                break; // or code = ""; if you're continuing to parse further.
            }
        }
        // if code.starts_with("#") {
        //     loop {
        //         code = &code[1..];
        //         if code.starts_with("\n"){
        //             code = &code[2..];
        //             break;
        //         }
        //     }
        // }
    
        // Identifier
        let (success, token, rest) = lex_identifier(code);
        if success {
            code = rest;
            tokens.push(token);
            continue;
        }

        let symbol = unrecognized_symbol(code);
        return Err(format!("Unidentified symbol {symbol}"));
  
    }
  
    return Ok(tokens);
  }



  fn lex_number(code: &str) -> (bool, Token, &str) {
    enum StateMachine {
      Start,
      Number,
    }
  
    let mut success = false;
    let mut state = StateMachine::Start;
    let mut index = 0;
    for letter in code.chars() {
        match state {
            StateMachine::Start => {
                if letter >= '0' && letter <= '9' {
                state = StateMachine::Number;
                success = true;
                index += 1;
                } else {
                    return (false, Token::NotToken, "");
                }
            }
        
            StateMachine::Number => {
                if letter >= '0' && letter <= '9' {
                state = StateMachine::Number;
                success = true;
                index += 1;
                //} else if(letter != ' ') {
                //    return (false, Token::NotToken, "");
                } else {
                    let num = code[..index].parse::<i32>().unwrap();
                    return (true, Token::Num(num), &code[index..]);
                }
            }
    
        }
    }

    if success == true {
        let num: i32 = code.parse::<i32>().unwrap();
        return (true, Token::Num(num), "");
    } else {
        return (false, Token::NotToken, "");
    }
}


// Removes whitespaces.
fn lex_space(code: &str) -> (bool, &str) {
    for letter in code.chars() {
        if letter.is_whitespace() {
            return (true, &code[1..]);
        } else {
            return (false, code);
        }
    }
    return (false, code);
}



fn lex_identifier(code: &str) -> (bool, Token, &str) {
    enum StateMachine {
        Start,
        Ident,
    }
  
    let mut success = false;
    let mut state = StateMachine::Start;
    let mut index = 0;
    for letter in code.chars() {
        match state {
            StateMachine::Start => {
                if (letter >= 'a' && letter <= 'z') || (letter >= 'A' && letter <= 'Z'){
                    state = StateMachine::Ident;
                    success = true;
                    index += 1;
                } else {
                    return (false, Token::NotToken, "");
                }
            }
        
            StateMachine::Ident => {
                if (letter >= 'A' && letter <= 'Z') || (letter >= 'a' && letter <= 'z') || (letter >= '0' && letter <= '9') || letter == '_' {
                    state = StateMachine::Ident;
                    success = true;
                    index += 1;
                } else {
                    let token = &code[..index];
                    return (true, create_identifier(token), &code[index..]);
                }
            }
    
        }
    }
  
    if success == true {
        return (true, create_identifier(code), "");
    } else {
        return (false, Token::NotToken, "");
    }
  }



fn unrecognized_symbol(code: &str) -> &str {
    enum StateMachine {
      Start,
      Symbol,
    }
  
    let mut state_machine = StateMachine::Start;
    let mut index = 0;
    for letter in code.chars() {
        match state_machine {
            StateMachine::Start => {
                state_machine = StateMachine::Symbol;
                index += 1;
            } 
            
            StateMachine::Symbol => {
                if letter.is_whitespace() {
                    return &code[..index];
                } else {
                    index += 1;
                }
            }
        }
    }
    return &code[..index];
} 



fn create_identifier(code: &str) -> Token {
    match code {
        "func" => Token::Func,
        "return" => Token::Return,
        "int" => Token::Int,
    
        // todo: implement all keywords...
        // ... all keywords...
        "print" => Token::Print,
        "else" => Token::Else,
        "break" => Token::Break,
        "continue" => Token::Continue,
    
        "read" => Token::Read,
        "while" => Token::While,
        "if" => Token::If,
        _ => Token::Ident(String::from(code)),
    }
  }



/*
  ----------------------------------------------------------------
  PARSER
*/


struct Expression {
  code: String,
  name: String,
}

static mut VAR_NUM: i64 = 0;

fn create_temp() -> String {
    unsafe {
        VAR_NUM += 1;
        format!("_temp{}", VAR_NUM)
    }
}

static mut LOOP_COUNTER: i64 = 0;

fn get_unique_loop_labels() -> (String, String) {
    unsafe {
        LOOP_COUNTER += 1;
        let iftrue = format!(":beginloop{}", LOOP_COUNTER);
        let end_label = format!(":endloop{}", LOOP_COUNTER);
        (iftrue, end_label)
    }
}

static mut IF_COUNTER: i64 = 0;

fn get_if_labels() -> (String, String, String) {
    unsafe {
        IF_COUNTER += 1;
        let iftrue = format!(":iftrue{}", IF_COUNTER);
        let elselabel = format!(":else{}", IF_COUNTER);
        let endif = format!(":endif{}", IF_COUNTER);
        (iftrue, elselabel, endif)
    }
}

fn peek<'a>(tokens: &'a Vec<Token>, index: usize) -> Option<&'a Token> {
    if index < tokens.len() {
        return Some(&tokens[index])
    } else {
        return None
    }
}

fn peek_result<'a>(tokens: &'a Vec<Token>, index: usize) -> Result<&'a Token, String> {
    if index < tokens.len() {
        return Ok(&tokens[index])
    } else {
        return Err(String::from("expected a token, but got nothing"))
    }
}

fn next<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Option<&'a Token> {
    if *index < tokens.len() {
        let ret = *index;
        *index += 1;
        return Some(&tokens[ret])
    } else {
        return None
    }
}

fn next_result<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<&'a Token, String> {
    if *index < tokens.len() {
        let ret = *index;
        *index += 1;
        return Ok(&tokens[ret])
    } else {
        return Err(String::from("expected a token, but got nothing"))
    }
}

// parsing a statement such as:
// int a;
// a = a + b;
// a = a % b;
// print(a)
// read(a)
// returns epsilon if '}'
fn parse_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<String>, function_table: &mut Vec<String>, arr_table: &mut Vec<String>, label_table: &mut VecDeque<(String, String)>) -> Result<Option<String>, String> {
    match peek(tokens, *index) {
    None => {
        return Ok(None);
    }

    Some(token) => {
        let mut codenode: Option<String> = None;
        match token {

            Token::RightCurly => {
                return Ok(None);
            }
            
            // Array or int declaration
            Token::Int => {
                let code = parse_declaration(tokens, index, symbol_table, arr_table)?;
                if !matches!(next_result(tokens, index)?, Token::Semicolon) {
                    return Err(String::from("expect ';' closing statement"));
                }
                codenode = Some(code);
            }

            // ident = expression or arr[expression] = expression
            Token::Ident(ident) => {
                *index += 1;
                let code: String;

                // Assign expression to array element 
                if matches!(peek_result(tokens, *index)?, Token::LeftBracket) {
                    // %mov [array + i], src1
                    if !find_symbol(&arr_table, ident){
                        return Err(format!("Array '{ident}' not declared"));
                    }
                    *index += 1;
                    let expr2 = parse_expression(tokens, index, symbol_table, function_table, arr_table)?;
                    if !matches!(next_result(tokens, index)?, Token::RightBracket) {
                        return Err(String::from("expected ']'"));
                    }
                    if !matches!(next_result(tokens, index)?, Token::Assign) {
                        return Err(String::from("expected '=' assignment operator"));
                    }
                    let expr = parse_expression(tokens, index, symbol_table, function_table, arr_table)?;
                    code = format!("{}{}%mov [{} + {}], {}\n", expr.code, expr2.code, ident, expr2.name, expr.name);
                } else {
                    if !find_symbol(&symbol_table, ident) {
                        return Err(format!("Variable not declared: {ident}"));
                    }
                    if !matches!(next_result(tokens, index)?, Token::Assign) {
                        return Err(String::from("expected '=' assignment operator"));
                    }
                    let expr = parse_expression(tokens, index, symbol_table, function_table, arr_table)?;
                    code = format!("{}%mov {}, {}\n", expr.code, ident, expr.name);
                }    
                if !matches!(next_result(tokens, index)?, Token::Semicolon) {
                    return Err(String::from("expect ';' closing statement"));
                }
                codenode = Some(code);
            }

            // return expression
            Token::Return => {
                *index += 1;
                let expr = parse_expression(tokens, index, symbol_table, function_table, arr_table)?;
                let code = format!("{}%ret {}\n", expr.code, expr.name);
                if !matches!(next_result(tokens, index)?, Token::Semicolon) {
                    return Err(String::from("expect ';' closing statement"));
                }
                codenode = Some(code);
            }

            // print(expression)
            Token::Print => {
                *index += 1;
                if !matches!(next_result(tokens, index)?, Token::LeftParen) {
                    return Err(String::from("expect '(' opening statement"));
                }
                let expr = parse_expression(tokens, index, symbol_table, function_table, arr_table)?;
                let code = format!("{}%out {}\n", expr.code, expr.name);
                if !matches!(next_result(tokens, index)?, Token::RightParen) {
                    return Err(String::from("expect ')' closing statement"));
                }
                if !matches!(next_result(tokens, index)?, Token::Semicolon) {
                    return Err(String::from("expect ';' closing statement"));
                }
                codenode = Some(code);
            }
            
            // read(expression)
            Token::Read => {
                *index += 1;
                if !matches!(next_result(tokens, index)?, Token::LeftParen) {
                    return Err(String::from("expect '(' opening statement"));
                }
                let expr = parse_expression(tokens, index, symbol_table, function_table, arr_table)?;
                let code = format!("{}%input {}\n", expr.code, expr.name);
                if !matches!(next_result(tokens, index)?, Token::RightParen) {
                    return Err(String::from("expect ')' closing statement"));
                }
                if !matches!(next_result(tokens, index)?, Token::Semicolon) {
                    return Err(String::from("expect ';' closing statement"));
                }
                codenode = Some(code);
            }

            // ----------------------------------------------------------------

            // Parses the control flow logic (i.e. 'if' or 'while' statements)
            // Works with nested control flow 

            Token::While => {
                // while term [bool operator] term { statement* }
                *index += 1;
                let (iftrue, end_label) = get_unique_loop_labels();
                label_table.push_back((iftrue.clone(), end_label.clone()));
                let expr = parse_bool_operation(tokens, index, symbol_table, function_table, arr_table)?;
                let mut code = format!("{}\n", iftrue);
                code += &expr.code;
                code += &format!("%branch_ifn {}, {}\n", expr.name, end_label);
                if !matches!(next_result(tokens, index)?, Token::LeftCurly) {
                    return Err(String::from("expected '{'"));
                }

                // Loop statements
                loop {
                    match parse_statement(tokens, index, symbol_table, function_table, arr_table, label_table)? {
                        None => {
                            break;
                        }
                        Some(statements) => {
                            code += &statements;
                        }
                    }
                }

                if !matches!(next_result(tokens, index)?, Token::RightCurly) {
                    return Err(String::from("expected '}' after while"));
                }

                code += &format!("%jmp {}\n", iftrue);
                code += &format!("{}\n", end_label);

                label_table.pop_back();
                codenode = Some(code);
            }
            Token::If => {
                /*
                    %branch_if bool_expr, :iftrue1
                    %jmp :else1
                    :iftrue1
                    # statements for if case
                    %jmp :endif1
                    :else1
                    # statements for else case
                    :endif1

                */
                // if term [bool operator] term { statement* }
                *index += 1;
                let (iftrue, elselabel, endif) = get_if_labels();
                let temp = create_temp();
                let mut code = format!("%int {}\n", temp);
                let bool_expr = parse_bool_operation(tokens, index, symbol_table, function_table, arr_table)?;
                code += &bool_expr.code;
                code += &format!("%branch_if {}, {}\n", bool_expr.name, iftrue);
                code += &format!("%jmp {}\n", elselabel);
                code += &format!("{}\n", iftrue);

                if !matches!(next_result(tokens, index)?, Token::LeftCurly) {
                    return Err(String::from("expected '{'"));
                }

                // Loop statements
                loop {
                    match parse_statement(tokens, index, symbol_table, function_table, arr_table, label_table)? {
                        None => {
                            break;
                        }
                        Some(statements) => {
                            code += &statements;
                        }
                    }
                }

                if !matches!(next_result(tokens, index)?, Token::RightCurly) {
                    return Err(String::from("expected '}' after if"));
                }

                code += &format!("%jmp {}\n", endif);
                code += &format!("{}\n", elselabel);

                // Else part (optional)
                match peek(tokens, *index) {
                    None => {}

                    Some(token) => {
                        match token {
                            Token::Else => {
                                *index += 1;

                                if !matches!(next_result(tokens, index)?, Token::LeftCurly) {
                                    return Err(String::from("expected '{'"));
                                }

                                // looop statements
                                loop {
                                    match parse_statement(tokens, index, symbol_table, function_table, arr_table, label_table)? {
                                        None => {
                                            break;
                                        }
                                        Some(statements) => {
                                            code += &statements;
                                        }
                                    }
                                }

                                if !matches!(next_result(tokens, index)?, Token::RightCurly) {
                                    return Err(String::from("expected '}' after else"));
                                }
                            }

                            _ => {}
                        }
                    }
                }
                code += &format!("{}\n", endif);
                codenode = Some(code);
            }

            // Break statement
            Token::Break => {
                *index += 1;
                if let Some((_, end_label)) = label_table.back() {
                    let code = format!("%jmp {}\n", end_label);
                    codenode = Some(code);
                } else {
                    return Err(String::from("`break` statement not within loop"));
                }
                if !matches!(next_result(tokens, index)?, Token::Semicolon) {
                    return Err(String::from("expect ';' after 'break'"));
                }
            }

            // Continue statement
            Token::Continue => {
                *index += 1;
                if let Some((begin_label, _)) = label_table.back() {
                    let code = format!("%jmp {}\n", begin_label);
                    codenode = Some(code);
                } else {
                    return Err(String::from("`continue` statement not within loop"));
                }
                if !matches!(next_result(tokens, index)?, Token::Semicolon) {
                    return Err(String::from("expect ';' after 'continue'"));
                }
            } 

            _ => {
                return Err(String::from("invalid statement."));
            }

        }
        return Ok(codenode);
    }

    }
}

fn parse_expression(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<String>, function_table: &mut Vec<String>, arr_table: &mut Vec<String>) -> Result<Expression, String> {
    let mut e = parse_mul_expression(tokens, index, symbol_table, function_table, arr_table)?;
    loop {
        match peek_result(tokens, *index)? {
            Token::Plus => {
                *index += 1;
                let e2 = parse_mul_expression(tokens, index, symbol_table, function_table, arr_table)?;

                let temp = create_temp();
                let src1 = e.name;
                let src2 = e2.name;
                e.code += &format!("%int {temp}\n");
                e.code += &e2.code;
                e.code += &format!("%add {temp}, {src1}, {src2}\n");
                e.name = temp;
            }
            Token::Subtract => {
                *index += 1;
                let e2 = parse_mul_expression(tokens, index, symbol_table, function_table, arr_table)?;

                let temp = create_temp();
                let src1 = e.name;
                let src2 = e2.name;
                e.code += &format!("%int {temp}\n");
                e.code += &e2.code;
                e.code += &format!("%sub {temp}, {src1}, {src2}\n");
                e.name = temp;
            }
            _ => {
                break;
            }
            
        }
    }
    return Ok(e);
}

fn parse_mul_expression(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<String>, function_table: &mut Vec<String>, arr_table: &mut Vec<String>) -> Result<Expression, String> {
    let mut e = parse_term(tokens, index, symbol_table, function_table, arr_table)?;

    loop {
        match peek_result(tokens, *index)? {
            Token::Multiply => {
                *index += 1;
                let e2 = parse_term(tokens, index, symbol_table, function_table, arr_table)?;

                let temp = create_temp();
                let src1 = e.name;
                let src2 = e2.name;
                e.code += &format!("%int {temp}\n");
                e.code += &e2.code;
                e.code += &format!("%mult {temp}, {src1}, {src2}\n");
                e.name = temp;
            }
            Token::Modulus => {
                *index += 1;
                let e2 = parse_term(tokens, index, symbol_table, function_table, arr_table)?;

                let temp = create_temp();
                let src1 = e.name;
                let src2 = e2.name;
                e.code += &format!("%int {temp}\n");
                e.code += &e2.code;
                e.code += &format!("%mod {temp}, {src1}, {src2}\n");
                e.name = temp;
            }
            Token::Divide => {
                *index += 1;
                let e2 = parse_term(tokens, index, symbol_table, function_table, arr_table)?;

                let temp = create_temp();
                let src1 = e.name;
                let src2 = e2.name;
                e.code += &format!("%int {temp}\n");
                e.code += &e2.code;
                e.code += &format!("%div {temp}, {src1}, {src2}\n");
                e.name = temp;
            }
            _ => {
                break;
            }
        }
    }

    Ok(e)
}

// a term is either a Number, an Identifier, or an array element (arr[expression])
fn parse_term(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<String>, function_table: &mut Vec<String>, arr_table: &mut Vec<String>) -> Result<Expression, String> {
    let mut expr: Expression;
    match next_result(tokens, index)? {
        Token::Ident(ident) => {
            // Function call
            if matches!(peek_result(tokens, *index)?, Token::LeftParen) {
                *index += 1;

                if !find_symbol(function_table, ident){
                    return Err(format!("Function {ident} not initialised"));
                }

                // %call dest, func(a,b)
                let mut args_code = String::new();
                let mut args = Vec::new();

                // Parse all arguments
                loop {
                    if matches!(peek_result(tokens, *index)?, Token::RightParen) {
                        *index += 1; // Consume ')'
                        break;
                    }
                    let arg_expr = parse_expression(tokens, index, symbol_table, function_table, arr_table)?;
                    args_code += &arg_expr.code;
                    args.push(arg_expr.name);

                    if matches!(peek_result(tokens, *index)?, Token::Comma) {
                        *index += 1; // Consume ','
                    } else if matches!(peek_result(tokens, *index)?, Token::RightParen) {
                        *index += 1; // Consume ')'
                        break;
                    } else {
                        return Err(String::from("expected ',' or ')' in function call"));
                    }
                }

                let temp = create_temp();
                let mut call_code = format!("%int {}\n", temp);
                call_code += &format!("%call {}, {}(", temp, ident);
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        call_code += ", ";
                    }
                    call_code += arg;
                }
                call_code += &format!(")\n");

                expr = Expression {
                    code: args_code + &call_code,
                    name: temp,
                };
            } else if matches!(peek_result(tokens, *index)?, Token::LeftBracket) { // Array element
                *index += 1;
                if !find_symbol(&arr_table, ident){
                    return Err(format!("Array {ident} undeclared"));
                }
                let expr2 = parse_expression(tokens, index, symbol_table, function_table, arr_table)?;
                if !matches!(next_result(tokens, index)?, Token::RightBracket) {
                    return Err(String::from("expected ']'"));
                }
                let temp = create_temp();
                expr = Expression {
                    code : format!("%int {}\n", temp),
                    name : format!("{}", temp),
                };
                // %mov [array + i], src1
                expr.code += &format!("{}%mov {}, [{} + {}]\n", expr2.code, temp, ident, expr2.name);
            } else {
                if !find_symbol(&symbol_table, ident) {
                    return Err(format!("Error. undeclared variable {ident}"));
                }
                expr = Expression {
                    code : String::from(""),
                    name : ident.clone(),
                };
            }
            return Ok(expr);
        }
        Token::Num(num) => {
            expr = Expression {
                code : String::from(""),
                name : format!("{}", num),
            };
            return Ok(expr);
        }
        Token::LeftParen => {
            let expr = parse_expression(tokens, index, symbol_table, function_table, arr_table)?;
            if !matches!(next_result(tokens, index)?, Token::RightParen) {
                return Err(String::from("expected ')'"));
            }
            return Ok(expr);
        }
        _ => {
            return Err(String::from("invalid expression"));
        }

    }
}

// Array or int declaration
fn parse_declaration(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<String>, arr_table: &mut Vec<String>) -> Result<String, String> { 
    match next_result(tokens, index)? {
        Token::Int => {
            match next_result(tokens, index)? {
                Token::LeftBracket => {  // Array size is declared after 'int'
                    let size: i32;
                    match next_result(tokens, index)? {
                        Token::Num(num) => {
                            if *num <= 0 {
                                return Err(String::from("Arrays have to be at least one element long"));
                            }
                            size = *num;
                        },
                        _ => {
                            return Err(String::from("Error: expected Token::Num as length of array"));
                        }
                    }

                    if !matches!(next_result(tokens, index)?, Token::RightBracket) {
                        return Err(String::from("expect ']' closing array size declaration"));
                    }
                    // Expect identifier after array size
                    match next_result(tokens, index)? {
                        Token::Ident(ident) => {  // Variable name of the array
                            if find_symbol(&arr_table, ident) {
                                return Err(format!("Found a duplicate array variable {ident}"));
                            }
                            arr_table.push(ident.clone());
                            let statement = format!("%int[] {}, {}\n", ident, size);
                            Ok(statement)
                        },
                        _ => Err(String::from("expected identifier for array name")),
                    }
                },
                Token::Ident(ident) => {  // Normal int variable declaration
                    if find_symbol(&symbol_table, ident) {
                        return Err(format!("Found a duplicate variable {ident}"));
                    }
                    symbol_table.push(ident.clone());
                    let statement = format!("%int {}\n", ident);
                    Ok(statement)
                },
                _ => Err(String::from("expected '[' for array declaration or identifier for int variable")),
            }
        },
        _ => Err(String::from("invalid declaration, expected 'int' type")),
    }
}

// term [bool operator] term
fn parse_bool_operation(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<String>, function_table: &mut Vec<String>, arr_table: &mut Vec<String>) -> Result<Expression, String> {
    /*
        ** expr1 code
        ** expr2 code
        %int temp1
        %lt temp, expr1.name, expr2.code
    */
    
    let mut expr_ret = Expression {
        code : format!(""),
        name : format!(""),
    };
    let expr1 = parse_term(tokens, index, symbol_table, function_table, arr_table)?;
    expr_ret.code += &expr1.code;
    let b_operator = next_result(tokens, index)?;
    let expr2 = parse_term(tokens, index, symbol_table, function_table, arr_table)?;
    expr_ret.code += &expr2.code;
    let temp = create_temp();
    expr_ret.code += &format!("%int {temp}\n");
    match b_operator {
        Token::Equality => {
            expr_ret.code += &format!("%eq {temp}, {}, {}\n", expr1.name, expr2.name);
        },
        Token::NotEqual => {
            expr_ret.code += &format!("%neq {temp}, {}, {}\n", expr1.name, expr2.name);
        },      
        Token::Greater => {
            expr_ret.code += &format!("%gt {temp}, {}, {}\n", expr1.name, expr2.name);
        },
        Token::GreaterEqual => {
            expr_ret.code += &format!("%ge {temp}, {}, {}\n", expr1.name, expr2.name);
        },
        Token::Less => {
            expr_ret.code += &format!("%lt {temp}, {}, {}\n", expr1.name, expr2.name);
        },
        Token::LessEqual => {
            expr_ret.code += &format!("%le {temp}, {}, {}\n", expr1.name, expr2.name);
        },

        _ => { 
            return Err(String::from("expect boolean operator (i.e. \"<\" or \"==\")..."));
        }
    };

    expr_ret.name = temp;

    return Ok(expr_ret);
}

// parse programs with multiple functions
// loop over everything, outputting generated code.
fn parse_program(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
    let mut generated_code = String::from("");
    let mut function_table: Vec<String> = vec![];
    loop {
        match parse_function(tokens, index, &mut function_table)? {
            None => {
                break;
            }
            Some(func_code) => {
                generated_code += &func_code;
            }
        }
    }

    if !find_symbol(&function_table, &format!("main")){
        return Err(format!("Missing 'main' function"));
    }

    return Ok(generated_code);
}

fn find_symbol(symbol_table: &Vec<String>, symbol: &String) -> bool {
    for s in symbol_table{
        if s.eq(symbol){
            return true;
        }
    }
    return  false;
}

fn parse_function(tokens: &Vec<Token>, index: &mut usize, function_table: &mut Vec<String>) -> Result<Option<String>, String> {
    let mut symbol_table: Vec<String> = vec![];
    let mut arr_table: Vec<String> = vec![];
    let mut label_table: VecDeque<(String, String)> = VecDeque::new();
    
    match next(tokens, index) {
        None => {
            return Ok(None);
        }
        Some(token) => {
            if !matches !(token, Token::Func) {
                return Err(String::from("functions must begin with func"));
            }
        }
        // Not an Error & not last token
    }
    let func_ident = match next_result(tokens, index)? {
        Token::Ident(func_ident) => func_ident,
        _  => {return Err(String::from("functions must have a function identifier"));}
    };

    if find_symbol(&function_table, func_ident){
        return Err(format!("Error: Function {func_ident} already declared"));
    }
    function_table.push(func_ident.clone());

    if !matches !(next_result(tokens, index)?, Token::LeftParen) {
        return Err(String::from("expected '('"));
    }

    let mut code = format!("%func {} (", func_ident);
    let mut params: Vec<String> = vec![];

    // function parameters
    loop {
        match next_result(tokens, index)? {

            Token::RightParen => {
                break;
            }
            
            Token::Int => {
                match next_result(tokens, index)? {
                    Token::Ident(param) => {
                        if find_symbol(&symbol_table, param) {
                            return Err(format!("Found a duplicate variable {param}"));
                        }
                        symbol_table.push(param.clone());
                        code += &format!("%int {}", param);
                        params.push(param.clone());
                        match peek_result(tokens, *index)? { // lookahead
                            Token::Comma => {
                                code += &format!(", ");
                                *index += 1;
                            }
                            Token::RightParen => {}
                            _ => {
                                return Err(String::from("expected ',' or ')'"));
                            }
                        }
                    }
                    _ => {
                            return Err(String::from("expected ident function parameter"));
                    }
                }
            }

            _ => {
                return Err(String::from("expected 'int' keyword or ')' token"));
            }
       }
    }

    code += &format!(")\n");

    if !matches!(next_result(tokens, index)?, Token::LeftCurly) {
        return Err(String::from("expected '{' for function"));
    }

    // Loop statements
    loop {
        match parse_statement(tokens, index, &mut symbol_table, function_table, &mut arr_table, &mut label_table)? {
            None => {
                break;
            }
            Some(statement) => {
                code += &statement;
            }
        }
    }

    code += "%endfunc\n\n";

    if !matches!(next_result(tokens, index)?, Token::RightCurly) {
      return Err(String::from("expected '}' after function"));
    }

    return Ok(Some(code));
}




/*
----------------------------------------------------------------
TESTS
*/



  #[cfg(test)]
mod tests {
    use crate::Token;
    use crate::lex;

    #[test]
    fn lexer_test() {
        // test that lexer works on correct cases
        let toks = lex("1 + 2 + 3").unwrap();
        assert!(toks.len() == 5);
        assert!(matches!(toks[0], Token::Num(1)));
        assert!(matches!(toks[1], Token::Plus));
        assert!(matches!(toks[2], Token::Num(2)));
        assert!(matches!(toks[3], Token::Plus));
        assert!(matches!(toks[4], Token::Num(3)));

        let toks = lex("3 + 215 +-").unwrap();
        assert!(toks.len() == 5);
        assert!(matches!(toks[0], Token::Num(3)));
        assert!(matches!(toks[1], Token::Plus));
        assert!(matches!(toks[2], Token::Num(215)));
        assert!(matches!(toks[3], Token::Plus));
        assert!(matches!(toks[4], Token::Subtract));

        // test that the lexer catches invalid tokens
        assert!(matches!(lex("^^^"), Err(_)));
    }

}