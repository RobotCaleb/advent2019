extern crate fuel;
extern crate intcode;

struct Ship {
    modules_masses: Vec<i32>,
    gravity_assist_program: Vec<i32>,
    intcode_computer: intcode::IntCode,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            modules_masses: vec![
                140403, 114038, 56226, 132100, 62000, 111395, 91372, 126850, 145044, 79273, 91088,
                84429, 58971, 107626, 149678, 85268, 105251, 115850, 115947, 74982, 75008, 97761,
                121022, 148319, 125244, 138640, 86968, 144443, 137218, 139558, 128776, 53593,
                133805, 64245, 113120, 63085, 59209, 51671, 63956, 139163, 119501, 77432, 51040,
                137313, 58973, 64708, 76505, 108041, 101124, 133219, 95907, 57933, 117791, 76209,
                102960, 90848, 141969, 91297, 146254, 84585, 103447, 83172, 76648, 111340, 118543,
                52957, 86004, 131965, 90898, 90909, 52217, 144674, 97058, 72387, 57962, 147792,
                114025, 100193, 77582, 146708, 54283, 143979, 99582, 149890, 73229, 56045, 63240,
                124091, 103324, 125187, 74027, 120344, 105333, 100939, 131454, 109570, 149398,
                140535, 57379, 138385,
            ],
            gravity_assist_program: vec![
                1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 19, 5, 23, 2, 23,
                6, 27, 1, 27, 5, 31, 2, 6, 31, 35, 1, 5, 35, 39, 2, 39, 9, 43, 1, 43, 5, 47, 1, 10,
                47, 51, 1, 51, 6, 55, 1, 55, 10, 59, 1, 59, 6, 63, 2, 13, 63, 67, 1, 9, 67, 71, 2,
                6, 71, 75, 1, 5, 75, 79, 1, 9, 79, 83, 2, 6, 83, 87, 1, 5, 87, 91, 2, 6, 91, 95, 2,
                95, 9, 99, 1, 99, 6, 103, 1, 103, 13, 107, 2, 13, 107, 111, 2, 111, 10, 115, 1,
                115, 6, 119, 1, 6, 119, 123, 2, 6, 123, 127, 1, 127, 5, 131, 2, 131, 6, 135, 1,
                135, 2, 139, 1, 139, 9, 0, 99, 2, 14, 0, 0,
            ],
            intcode_computer: intcode::IntCode::new(),
        }
    }
}

fn main() {
    let mut ship = Ship::new();

    let mut fuel = 0i32;
    for mass in ship.modules_masses {
        fuel += fuel::sum_fuel(mass);
    }
    println!("Fuel required: {}", fuel);

    ship.intcode_computer.load(&ship.gravity_assist_program);
    ship.intcode_computer.state[1] = 12;
    ship.intcode_computer.state[2] = 2;
    ship.intcode_computer.run();
    println!("Gravity state: {}", ship.intcode_computer.state[0]);

    for n in 0..100 {
        for v in 0..100 {
            ship.intcode_computer.load(&ship.gravity_assist_program);
            ship.intcode_computer.state[1] = n;
            ship.intcode_computer.state[2] = v;
            ship.intcode_computer.run();
            if ship.intcode_computer.state[0] == 19690720 {
                println!("Found noun: {} and verb: {}", n, v);
                println!("100 * noun + verb = {}", 100 * n + v);
                break;
            }
        }
    }
}
