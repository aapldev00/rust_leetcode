fn is_palindrome(x: i32) -> bool {
    let x_str = x.to_string();
    let v: Vec<char> = x_str.chars().collect();
    for i in 0..x_str.len() {
        if v[i] != v[x_str.len() - i - 1] {
            return false;
        }
    }
    true
}

// Implementación mediocre.