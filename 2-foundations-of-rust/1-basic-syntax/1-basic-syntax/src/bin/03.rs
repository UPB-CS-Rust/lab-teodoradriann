fn main() {
    let input: [i32; 8] = [23, 82, 16, 45, 21, 94, 12, 34];

    let mut _max: i32;

    // for i in 0..input.len() {
    //     if input[i] < min {
    //         min = input[i];
    //     }
    //     if input[i] > max {
    //         max = input[i];
    //     }
    // }

    let option_max: Option<&i32> = input.iter().max();

    let max: &i32 = match option_max {
        Some(x) => x, 
        None => &0,
    };
    println!("{:?} is largest and is smallest", max);
}
