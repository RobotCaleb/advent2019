pub mod orbitmapper {
    use petgraph::{
        dot::{Config, Dot},
        graphmap::DiGraphMap,
        Direction,
    };

    #[derive(Debug)]
    pub struct OrbitMapper<'a> {
        orbit_graph: DiGraphMap<&'a str, i32>,
    }

    impl<'a> OrbitMapper<'a> {
        pub fn new() -> OrbitMapper<'a> {
            OrbitMapper {
                orbit_graph: DiGraphMap::<&str, i32>::new(),
            }
        }

        pub fn new_from_orbits(orbits: &'a Vec<&str>) -> OrbitMapper<'a> {
            let mut edges = vec![];
            for orbit in orbits {
                edges.push(OrbitMapper::parse_orbit_pair(orbit));
            }
            OrbitMapper {
                orbit_graph: DiGraphMap::<&str, i32>::from_edges(&edges),
            }
        }

        pub fn load_orbits(&mut self, orbits: &'a Vec<&str>) {
            let mut edges = vec![];
            for orbit in orbits {
                edges.push(OrbitMapper::parse_orbit_pair(orbit));
            }
            self.orbit_graph = DiGraphMap::<&str, i32>::from_edges(&edges);
        }

        pub fn parse_orbit_pair(pair: &str) -> (&str, &str) {
            let splits: Vec<&str> = pair.split(")").collect();
            (splits[0], splits[1])
        }

        pub fn get_dot(&self) -> String {
            format!(
                "{}",
                Dot::with_config(&self.orbit_graph, &[Config::EdgeNoLabel])
            )
        }

        pub fn count_orbits(&self) -> usize {
            let mut sum = 0;
            for neighbor in self
                .orbit_graph
                .neighbors_directed("COM", Direction::Outgoing)
            {
                sum += self.count_orbits_recursive(&neighbor, 1)
            }
            sum
        }

        fn count_orbits_recursive(&self, node: &'a str, depth: usize) -> usize {
            let mut sum = depth;
            for neighbor in self
                .orbit_graph
                .neighbors_directed(node, Direction::Outgoing)
            {
                sum += self.count_orbits_recursive(&neighbor, depth + 1)
            }
            sum
        }
    }
}
