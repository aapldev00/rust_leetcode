fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() { return String::new(); }

    
    let mut prefix = strs[0].clone();

    for s in strs.iter().skip(1) {
        let mut nuevo_prefijo = String::new();
        
        
        for (c1, c2) in prefix.chars().zip(s.chars()) {
            if c1 == c2 {
                nuevo_prefijo.push(c1);
            } else {
                break; 
            }
        }
        
        prefix = nuevo_prefijo;
        if prefix.is_empty() { break; } 
    }

    prefix
}

// Buena implementación.