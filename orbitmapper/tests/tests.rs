extern crate orbitmapper;

#[cfg(test)]
mod tests {
    use orbitmapper::orbitmapper::OrbitMapper;
    #[test]
    fn test_pair_split() {
        assert_eq!(OrbitMapper::parse_orbit_pair("HBL)5W4"), ("HBL", "5W4", 1))
    }
    #[test]
    fn test_from_orbits() {
        let truth = "digraph {\n    0 [label=\"HBL\"]\n    1 [label=\"5W4\"]\n    2 [label=\"ABC\"]\n    3 [label=\"COM\"]\n    0 -> 1\n    1 -> 2\n    3 -> 0\n}\n";

        let input = vec!["HBL)5W4", "5W4)ABC", "COM)HBL"];
        let o = OrbitMapper::new_from_orbits(&input);
        o.count_orbits();
        assert_eq!(o.get_dot(), truth);
    }
    #[test]
    fn test_count_orbits() {
        let input = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ];
        let o = OrbitMapper::new_from_orbits(&input);
        assert_eq!(o.count_orbits(), 42);
    }
    #[test]
    fn test_shortest_path() {
        let input = vec!["A)B", "A)D", "B)C", "B)F", "C)E", "E)F", "D)E"];
        let mut o = OrbitMapper::new_from_orbits(&input);
        let mut path = o.get_shortest_path("A", "F");
        assert_eq!(path, Some((2, vec!["A", "B", "F"])));
        let map = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ];
        o.load_orbits(&map);
        path = o.get_shortest_path("YOU", "SAN");
        assert_eq!(path, Some((6, vec!["YOU", "K", "J", "E", "D", "I", "SAN"])));
    }
    #[test]
    fn test_orbit_transfer() {
        let input = vec!["A)B", "A)D", "B)C", "B)F", "C)E", "E)F", "D)E"];
        let mut o = OrbitMapper::new_from_orbits(&input);
        let mut path = o.get_orbit_transfer("A", "F");
        assert_eq!(path, Some((0, vec!["B"])));
        let map = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ];
        o.load_orbits(&map);
        path = o.get_orbit_transfer("YOU", "SAN");
        assert_eq!(path, Some((4, vec!["K", "J", "E", "D", "I"])));
    }
}
