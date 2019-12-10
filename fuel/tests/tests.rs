extern crate fuel;

#[cfg(test)]
mod tests {
    #[test]
    fn fuel_12() {
        assert_eq!(fuel::fuel_calculator::sum_fuel(12.0), 2.0);
    }
    #[test]
    fn fuel_14() {
        assert_eq!(fuel::fuel_calculator::sum_fuel(14.0), 2.0);
    }
    #[test]
    fn fuel_1969() {
        assert_eq!(fuel::fuel_calculator::sum_fuel(1969.0), 966.0);
    }
    #[test]
    fn fuel_100756() {
        assert_eq!(fuel::fuel_calculator::sum_fuel(100756.0), 50346.0);
    }
    #[test]
    fn line_1() {
        let mut fl = fuel::fuel_manager::FuelLine::new();
        fl.add_section("R45");
        assert_eq!(fl.segments.len(), 1);
        assert_eq!(fl.segments[0].start, vector::Vector2::zero());
        assert_eq!(fl.segments[0].end, vector::Vector2::new(45.0, 0.0));
        fl.add_section("u45");
        assert_eq!(fl.segments.len(), 2);
        assert_eq!(fl.segments[0].start, vector::Vector2::zero());
        assert_eq!(fl.segments[0].end, vector::Vector2::new(45.0, 0.0));
        assert_eq!(fl.segments[1].start, vector::Vector2::new(45.0, 0.0));
        assert_eq!(fl.segments[1].end, vector::Vector2::new(45.0, 45.0));
    }
    #[test]
    fn line_2() {
        let mut fm = fuel::fuel_manager::FuelManager::new();
        fm.add_strip("R45,U45");
        assert_eq!(fm.lines[0].segments.len(), 2);
        assert_eq!(fm.lines[0].segments[0].start, vector::Vector2::zero());
        assert_eq!(fm.lines[0].segments[0].end, vector::Vector2::new(45.0, 0.0));
        assert_eq!(
            fm.lines[0].segments[1].start,
            vector::Vector2::new(45.0, 0.0)
        );
        assert_eq!(
            fm.lines[0].segments[1].end,
            vector::Vector2::new(45.0, 45.0)
        );
    }
    #[test]
    fn day_3_1() {
        let mut fm = fuel::fuel_manager::FuelManager::new();
        fm.add_strip("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        fm.add_strip("U62,R66,U55,R34,D71,R55,D58,R83");
        let hits = fm.get_all_intersections(0, 1).unwrap();
        let hit = fm.get_closest_intersection(&hits);
        assert_eq!(vector::Vector2::zero().dist_man(&hit.unwrap()), 159.0);
    }
    #[test]
    fn day_3_2() {
        let mut fm = fuel::fuel_manager::FuelManager::new();
        fm.add_strip("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        fm.add_strip("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let hits = fm.get_all_intersections(0, 1).unwrap();
        let hit = fm.get_closest_intersection(&hits);
        assert_eq!(vector::Vector2::zero().dist_man(&hit.unwrap()), 135.0);
    }
    #[test]
    fn line_3() {
        let mut fm = fuel::fuel_manager::FuelManager::new();
        fm.add_strip("R1,U1,L1");
        fm.add_strip("U2");
        println!("{:#?}", fm);
        let hit = fm.get_all_intersections(0, 1).unwrap();
        println!("{:#?}", hit);
        let fewest = fm.get_dist_to_shortest_walk(&hit, 0, 1);
        assert_eq!(fewest, 4.0);
    }
    #[test]
    fn day_3_3() {
        let mut fm = fuel::fuel_manager::FuelManager::new();
        fm.add_strip("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        fm.add_strip("U62,R66,U55,R34,D71,R55,D58,R83");
        let hit = fm.get_all_intersections(0, 1).unwrap();
        let fewest = fm.get_dist_to_shortest_walk(&hit, 0, 1);
        assert_eq!(fewest, 610.0);
    }
    #[test]
    fn day_3_4() {
        let mut fm = fuel::fuel_manager::FuelManager::new();
        fm.add_strip("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        fm.add_strip("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let hit = fm.get_all_intersections(0, 1).unwrap();
        let fewest = fm.get_dist_to_shortest_walk(&hit, 0, 1);
        assert_eq!(fewest, 410.0);
    }
}
