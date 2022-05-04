pub fn aan(s: &str) -> String {
    match s.chars().next() {
        Some(c) => match c {
            'a' | 'e' | 'i' | 'o' | 'u' => format!("an {}", s).to_string(),
            _ => format!("a {}", s).to_string(),
        },
        None => "".to_string(),
    }
}