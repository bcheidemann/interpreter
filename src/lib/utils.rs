pub fn is_digit(char: char) -> bool {
    char >= '0' && char <= '9'
}

pub fn is_alpha(char: char) -> bool {
    (char >= 'a' && char <= 'z') || (char >= 'A' && char <= 'Z') || char == '_'
}

pub fn is_alpha_numeric(char: char) -> bool {
    is_alpha(char) || is_digit(char)
}
