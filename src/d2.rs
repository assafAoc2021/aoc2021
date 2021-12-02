use crate::get_input;

pub fn run() {
    type Distance = u32;

    let input = get_input("2", "");
    // let input = get_input("2", "1");  // A: 150 B: 900
    let input = input.collect::<Vec<_>>();

    let mut current_distance = 0;
    let mut a_current_depth_b_current_aim = 0;

    let mut b_current_depth = 0;

    for line in input.iter() {
        let mut splitted = line.split(" ");
        let direction = splitted.next().expect("empty line");
        let amount = splitted.next().expect("no distance").parse::<Distance>().expect("can't parse distance");

        match direction {
            "forward" => {
                current_distance += amount;
                b_current_depth += (a_current_depth_b_current_aim * amount);
            }
            "down" => a_current_depth_b_current_aim += amount,
            "up" => a_current_depth_b_current_aim -= amount,
            _ => panic!("unrecognized movement type: {}", direction),
        };
    }

    println!("A: {}", current_distance * a_current_depth_b_current_aim);
    println!("B: {}", current_distance * b_current_depth);
}