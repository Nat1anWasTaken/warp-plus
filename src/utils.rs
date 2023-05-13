use rand::Rng;

pub fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut result = String::with_capacity(length);
    let choices = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    for _ in 0..length {
        let choice: u8 = rng.gen_range(0..choices.len() as u8);
        let chosen_char = choices.chars().nth(choice as usize).unwrap();
        result.push(chosen_char);
    }
    result
}

pub fn generate_random_digits(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut result = String::with_capacity(length);
    let choices = "0123456789";

    for _ in 0..length {
        let choice: u8 = rng.gen_range(0..choices.len() as u8);
        let chosen_char = choices.chars().nth(choice as usize).unwrap();
        result.push(chosen_char);
    }
    result
}