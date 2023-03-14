pub fn is_operator(char: char) -> bool {
    match char {
        '+' | '-' => true,
        _ => false
    }
}

pub fn is_parens(char: char) -> bool {
    match char {
        '(' | ')' => true,
        _ => false
    }
}


pub fn is_whitespace(char: char) -> bool {
    match char {
        ' ' => true,
        _ => false
    }
}

pub fn is_doublequote(char: char) -> bool {
    match char {
        '"' => true,
        _ => false
    }
}
