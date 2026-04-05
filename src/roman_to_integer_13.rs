fn roman_to_int(s: String) -> i32 {
    let mut numbers: Vec<i32> = vec![0; s.len()];
    for (i, c) in s.chars().enumerate() {
        numbers[i] = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
    }
    let mut total = 0;
    for i in 0..numbers.len() {
        if i + 1 < numbers.len() && numbers[i] < numbers[i + 1] {
            total -= numbers[i];
        } else {
            total += numbers[i];
        }
    }
    total
}

// Buena implementación.