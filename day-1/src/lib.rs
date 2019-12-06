pub fn sum_fuel(mass: i32) -> i32 {
    let mut sum = 0i32;
    let mut fuel = get_fuel(mass);
    while fuel != 0 {
        let f = fuel;
        sum += fuel;
        fuel = get_fuel(f);
    }
    sum
}

fn get_fuel(mass: i32) -> i32 {
    let m1 = mass as f32 / 3.0;
    let m2 = m1.floor() as i32;
    if m2 - 2 > 0 {
        return m2 - 2;
    }
    0
}
