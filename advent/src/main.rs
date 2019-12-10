extern crate fuel;
extern crate intcode;
extern crate password;
extern crate vector;

use fuel::fuel_manager;
use password::password::brute_1;
use password::password::brute_2;

struct Ship {
    modules_masses: Vec<f32>,
    gravity_assist_program: Vec<i32>,
    intcode_computer: intcode::IntCode,
    fuel_manager: fuel_manager::FuelManager,
    password_range: (usize, usize),
}

impl Ship {
    fn new() -> Ship {
        Ship {
            modules_masses: vec![
                140403.0, 114038.0, 56226.0, 132100.0, 62000.0, 111395.0, 91372.0, 126850.0,
                145044.0, 79273.0, 91088.0, 84429.0, 58971.0, 107626.0, 149678.0, 85268.0,
                105251.0, 115850.0, 115947.0, 74982.0, 75008.0, 97761.0, 121022.0, 148319.0,
                125244.0, 138640.0, 86968.0, 144443.0, 137218.0, 139558.0, 128776.0, 53593.0,
                133805.0, 64245.0, 113120.0, 63085.0, 59209.0, 51671.0, 63956.0, 139163.0,
                119501.0, 77432.0, 51040.0, 137313.0, 58973.0, 64708.0, 76505.0, 108041.0,
                101124.0, 133219.0, 95907.0, 57933.0, 117791.0, 76209.0, 102960.0, 90848.0,
                141969.0, 91297.0, 146254.0, 84585.0, 103447.0, 83172.0, 76648.0, 111340.0,
                118543.0, 52957.0, 86004.0, 131965.0, 90898.0, 90909.0, 52217.0, 144674.0, 97058.0,
                72387.0, 57962.0, 147792.0, 114025.0, 100193.0, 77582.0, 146708.0, 54283.0,
                143979.0, 99582.0, 149890.0, 73229.0, 56045.0, 63240.0, 124091.0, 103324.0,
                125187.0, 74027.0, 120344.0, 105333.0, 100939.0, 131454.0, 109570.0, 149398.0,
                140535.0, 57379.0, 138385.0,
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
            fuel_manager: fuel_manager::FuelManager::new(),
            password_range: (197487, 673251),
        }
    }
}

fn main() {
    let mut ship = Ship::new();

    // day 1
    let mut fuel = 0.0;
    for mass in ship.modules_masses {
        fuel += fuel::fuel_calculator::sum_fuel(mass);
    }
    println!("Fuel required: {}", fuel);

    // day 2
    ship.intcode_computer.load(&ship.gravity_assist_program);
    ship.intcode_computer.state[1] = 12;
    ship.intcode_computer.state[2] = 2;
    ship.intcode_computer.run();
    println!("Gravity state: {}", ship.intcode_computer.state[0]);

    // day 2
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

    // day 3
    ship.fuel_manager.add_strip("R994,D213,L483,U102,L292,U153,L123,D568,R54,U482,L971,D679,L17,U492,R448,U75,R472,D209,R872,D271,L271,U175,L373,D968,R867,U191,R746,U717,L918,D171,L205,D399,R889,D626,R683,D853,R571,U42,L406,D390,R716,U364,L596,U226,L34,D805,R474,D304,R54,D854,L140,U923,L800,D833,L127,D576,R779,U433,L270,U602,L266,D874,R275,D146,L469,U623,R932,D564,R683,D769,L824,U818,L743,U846,R607,D860,R989,U927,R467,D465,R915,D531,L206,U366,R476,U516,R688,D144,L3,D612,L50,D519,R765,D684,L100,U233,R898,U957,R335,D614,R672,U428,R104,U294,L817,D83,R165,U328,L348,D231,R192,D982,R930,D949,R849,D144,R181,D691,R953,D796,L262,D883,L141,U264,L876,U445,R705,U802,L418,U177,R695,U138,L486,D598,R473,D704,R322,U818,R740,U218,R718,D942,R621,D277,L277,U338,L134,U586,L32,U89,L117,U296,R954,D775,L744,D655,R405,U67,R586,U37,L976,D486,L431,U671,R871,D865,R671,D434,L754,D933,L353,D430,L494,D729,R88,D438,R301,D111,L70,U561,L446,D973,R646,D419,R499,D496,L933,D463,L624,U999,L899,U954,R971,U533,R110,U483,R100,D452,R143,D73,L144,D220,L621,U62,R695,U270,L655,U404,L207,U946,L989,U767,L953,U271,L206,U315,L675,D579,L910,D413,R283,D118,L405,U724,L144,U969,R256,D940,L592,D714,L898,D937,R3,D175,L578,D981,R110,D217,L60,D57,R559,D913,R251,D883,L431,U450,L755,D126,L236,U532,R62,U33,L499,D29,R304,U25,R923,U732,L930,U812,R923,D12,L991,U223,R58,D638,R166,D980,R999,D825,L707,D956,L107,U676,R263,U949,L924,D928,L747,U641,L179,D398,R714,U715,R525,D515,R887,D21,R100,D424,L265,U704,R119,U77,L619,D308,R857,U272,L571,U650,L793");
    ship.fuel_manager.add_strip("L1010,D906,R561,D862,R541,D243,L958,D538,L930,U270,R282,D56,R692,U625,R962,U95,L834,D477,R375,D859,L158,D664,L874,U817,R218,U428,L194,U713,R1,D291,R608,D79,L582,U570,R952,U217,R441,U43,L966,U40,R103,U419,L387,U198,R306,U239,R714,U193,L111,D98,L308,U856,L702,D596,R903,D607,R86,D967,L191,D676,R606,D638,R512,D373,L623,U596,L151,U394,L241,D429,L324,D710,L663,U821,L351,D538,L917,U449,R120,D746,R833,U812,L691,D751,L631,U612,L107,D276,L997,D764,R644,U823,L574,U276,L48,U968,L97,D118,L976,U511,L976,U862,L232,D425,R586,U665,R893,D744,R317,D152,L406,U997,R377,D552,R226,D960,L232,D834,L911,U548,L127,U845,L687,U835,R18,D524,R226,D896,R89,U497,L748,U105,R174,U17,R212,U347,L942,U633,R799,U907,L971,D490,R690,D399,L725,U807,R244,U558,L754,D490,L726,D126,L639,D548,L18,D173,L188,D33,L707,D980,L728,U663,R593,D172,R314,U873,R409,U476,R312,U970,R28,U514,L378,D681,R611,U733,R135,U682,L736,D253,L432,D736,L777,U447,L174,D664,L652,D984,L714,D990,R608,D383,R51,U497,L343,D569,R515,D144,L949,D827,R112,U595,L109,D215,L401,U635,L953,U20,R780,U324,R955,D346,R762,D693,R56,U341,R481,U70,R385,U330,R278,D923,L835,D733,R235,U628,R505,U372,R469,D659,R618,U899,L21,U698,R688,U409,R775,D405,R846,D783,R675,U261,R721,D637,R957,D355,L50,U759,R769,D612,R538,U923,L780,U786,R104,D66,L67,U175,L820,D723,R124,U937,R923,D130,R758,U678,R215,U671,R366,U163,L783,U790,L832,D731,L736,D879,R508,U433,R705,U939,L969,U920,R683,D188,L349,U812,L36,D333,L88,U356,L140,D735,L217,D365,R23,D88,L20,D854,L437,U153,L307");
    match ship.fuel_manager.get_all_intersections(0, 1) {
        Some(ints) => {
            let closest = ship.fuel_manager.get_closest_intersection(&ints).unwrap();
            println!(
                "Closet fuel line intersection is at {:#?} at a distance of {}",
                closest,
                vector::Vector2::zero().dist_man(&closest)
            );
            let fewest = ship.fuel_manager.get_dist_to_shortest_walk(&ints, 0, 1);
            println!("Fewest steps: {}", fewest);
        }
        None => println!("No intersections found: {}:{}", file!(), line!()),
    }

    // day 4
    let start = ship.password_range.0;
    let end = ship.password_range.1;
    println!(
        "Passwords in range first part {}-{}: {}",
        start,
        end,
        brute_1(start, end)
    );
    println!(
        "Passwords in range second part {}-{}: {}",
        start,
        end,
        brute_2(start, end)
    );
}
