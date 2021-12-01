use crate::get_input;

pub fn run() {
    // let input = get_input("1", "1"); // a: 7 b: 5
    let input = get_input("1", "");
    let input = input.map(|s| s.parse::<u16>().expect("parse error")).collect::<Vec<_>>();

    let increasing_count = input.windows(2)
        .fold(0u16, |acc, windows| {
            if windows[0] < windows[1] { acc + 1 } else { acc }
        });

    println!("A: {}", increasing_count);

    let increasing_sum_count = input.windows(4)
        .fold(0u16, |acc, windows| {
            let sum1 = windows[0] + windows[1] + windows[2];
            let sum2 = windows[1] + windows[2] + windows[3];
            if sum1 < sum2 { acc + 1 } else { acc }
        });

    println!("B: {}", increasing_sum_count);
}
