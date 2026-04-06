fn is_valid(s: String) -> bool {
    let mut pila = Vec::new();
    
    for c in s.chars() {
        match c {
            
            '(' | '[' | '{' => pila.push(c),
            
            
            ')' => if pila.pop() != Some('(') { return false; },
            ']' => if pila.pop() != Some('[') { return false; },
            '}' => if pila.pop() != Some('{') { return false; },
            
            
            _ => (),
        }
    }
    
    
    pila.is_empty()
}