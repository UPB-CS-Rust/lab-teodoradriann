fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let mut max: i32 = -1;
    let mut min: i32 = 999;

    for i in 0..input.len() {
        if input[i] < min {
            min = input[i];
        }
        if input[i] > max {
            max = input[i];
        }
    }

    println!("{} is largest and {} is smallest", max, min);
}
