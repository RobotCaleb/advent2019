extern crate vector;

pub mod fuel_calculator {
    pub fn sum_fuel(mass: f32) -> f32 {
        let mut sum = 0.0;
        let mut fuel = get_fuel(mass);
        while fuel != 0.0 {
            let f = fuel;
            sum += fuel;
            fuel = get_fuel(f);
        }
        sum
    }

    fn get_fuel(mass: f32) -> f32 {
        let m1 = mass / 3.0;
        let m2 = m1.floor();
        if m2 - 2.0 > 0.0 {
            return m2 - 2.0;
        }
        0.0
    }
}

pub mod fuel_manager {
    #[derive(Debug)]
    pub struct FuelManager {
        pub lines: Vec<FuelLine>,
    }

    impl FuelManager {
        pub fn new() -> FuelManager {
            FuelManager { lines: vec![] }
        }

        fn add_line(&mut self, line: FuelLine) {
            self.lines.push(line);
        }

        pub fn add_strip(&mut self, line: &str) {
            let split = line.split(",");
            let mut fl = FuelLine::new();
            for s in split {
                fl.add_section(s);
            }
            self.add_line(fl);
        }

        pub fn get_intersection(&self, first: usize, second: usize) -> Option<vector::Vector2> {
            let fl1 = &self.lines[first];
            let fl2 = &self.lines[second];
            let mut closest = vector::Vector2::new(std::f32::MAX, std::f32::MAX);
            for i1 in 0..fl1.segments.len() {
                let s1 = &fl1.segments[i1];
                for i2 in 0..fl2.segments.len() {
                    let s2 = &fl2.segments[i2];
                    let int = s1.get_intersection(&s2);
                    match int {
                        Some(v) => {
                            if v == vector::Vector2::new(0.0, 0.0) && i1 == 0 && i2 == 0 {
                                continue;
                            }
                            let dist = vector::Vector2::new(0.0, 0.0).dist_man(&v);
                            let dist2 = vector::Vector2::new(0.0, 0.0).dist_man(&closest);
                            if dist < dist2 {
                                closest = v;
                            }
                        }
                        None => continue,
                    }
                }
            }

            if closest == vector::Vector2::new(std::f32::MAX, std::f32::MAX) {
                return None;
            }
            Some(closest)
        }
    }

    #[derive(Debug)]
    pub struct FuelLine {
        cur: vector::Vector2,
        pub segments: Vec<FuelSegment>,
    }

    impl FuelLine {
        pub fn new() -> FuelLine {
            FuelLine {
                cur: vector::Vector2::new(0.0, 0.0),
                segments: vec![],
            }
        }

        fn add_segment(&mut self, line: FuelSegment) {
            self.segments.push(line);
        }

        pub fn add_section(&mut self, dir: &str) {
            let dir_new = dir.split_at(1);
            let (d, f) = dir_new;
            let far = f.parse::<f32>().unwrap();
            let end = match d.to_uppercase().as_ref() {
                "R" => vector::Vector2::new(self.cur.x + far, self.cur.y + 0.0),
                "D" => vector::Vector2::new(self.cur.x + 0.0, self.cur.y - far),
                "L" => vector::Vector2::new(self.cur.x - far, self.cur.y + 0.0),
                "U" => vector::Vector2::new(self.cur.x + 0.0, self.cur.y + far),
                _ => {
                    println!("Bad input to section add: {}", dir);
                    std::process::exit(0)
                }
            };
            self.add_segment(FuelSegment::new(self.cur, end));
            self.cur = end;
        }
    }

    #[derive(PartialEq, Debug)]
    pub struct FuelSegment {
        pub start: vector::Vector2,
        pub end: vector::Vector2,
    }

    impl FuelSegment {
        pub fn new(start: vector::Vector2, end: vector::Vector2) -> FuelSegment {
            FuelSegment {
                start: start,
                end: end,
            }
        }

        pub fn get_intersection(&self, other: &FuelSegment) -> Option<vector::Vector2> {
            let v1 = vector::Vector2::new(self.end.x - self.start.x, self.end.y - self.start.y);
            let v2 = vector::Vector2::new(other.end.x - other.start.x, other.end.y - other.start.y);

            let s = (-v1.y * (self.start.x - other.start.x)
                + v1.x * (self.start.y - other.start.y))
                / (-v2.x * v1.y + v1.x * v2.y);
            let t = (v2.x * (self.start.y - other.start.y) - v2.y * (self.start.x - other.start.x))
                / (-v2.x * v1.y + v1.x * v2.y);

            if s >= 0.0 && s <= 1f32 && t >= 0.0 && t <= 1f32 {
                // Collision detected
                return Some(vector::Vector2::new(
                    self.start.x + (t * v1.x),
                    self.start.y + (t * v1.y),
                ));
            }

            // No collision
            return None;
        }
    }
}
