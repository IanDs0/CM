use std::collections::HashMap;

fn moda(numbers: &[i32]) -> i32 {
    let mut frequency_map = HashMap::new();
    for number in numbers {
        *frequency_map.entry(number).or_insert(0) += 1;
    }
    let mut max_frequency = 0;
    let mut mode = 0;
    for (number, frequency) in frequency_map {
        if frequency > max_frequency {
            max_frequency = frequency;
            mode = *number;
        }
    }
    mode
}

fn main() {
    let numbers = [1, 2, 3, 4, 1, 1, 1, 4, 5, 5, 5];
    let numbers = moda(&numbers);
    println!("{}", numbers);
}