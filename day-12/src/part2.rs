use crate::custom_error::AocError;
use glam::IVec2;
use itertools::Itertools;
use petgraph::algo::kosaraju_scc;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::collections::HashMap;

const ABOVE: IVec2 = IVec2::new(0, -1);
const BELOW: IVec2 = IVec2::new(0, 1);
const LEFT: IVec2 = IVec2::new(-1, 0);
const RIGHT: IVec2 = IVec2::new(1, 0);

const SURROUNDINGS: [IVec2; 4] = [ABOVE, RIGHT, BELOW, LEFT];

fn parse(input: &str) -> Graph<IVec2, ()> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (IVec2::new(x as i32, y as i32), c))
        })
        .collect::<HashMap<_, _>>();

    let mut graph = Graph::new();
    let mut index_mapping = HashMap::new();

    for (coordinates, _) in &map {
        let nodeindex = graph.add_node(coordinates.clone());
        index_mapping.insert(coordinates, nodeindex);
    }

    for (coordinates, c) in &map {
        SURROUNDINGS
            .iter()
            .map(|&direction| coordinates + direction)
            .filter(|neighbor| map.get(neighbor) == Some(c))
            .for_each(|neighbor| {
                match (index_mapping.get(coordinates), index_mapping.get(&neighbor)) {
                    (Some(index), Some(neighbor)) => {
                        graph.add_edge(index.clone(), neighbor.clone(), ());
                    }
                    _ => {}
                }
            })
    }
    graph
}

fn count_corners(graph: &Graph<IVec2, ()>, node: &NodeIndex, node_group: &Vec<NodeIndex>) -> u64 {
    SURROUNDINGS
        .iter()
        .circular_tuple_windows()
        .take(4)
        .map(|(d1, d2)| {
            let node_coord = graph.node_weight(*node).unwrap();
            let node_group_coordinates = node_group
                .iter()
                .filter_map(|node| graph.node_weight(*node).cloned())
                .collect_vec();

            let outer_corner = !node_group_coordinates.contains(&(*node_coord + *d1))
                && !node_group_coordinates.contains(&(*node_coord + *d2));

            let inner_corner = node_group_coordinates.contains(&(*node_coord + *d1))
                && node_group_coordinates.contains(&(*node_coord + *d2))
                && !node_group_coordinates.contains(&(*node_coord + *d1 + *d2));

            outer_corner as u64 + inner_corner as u64
        })
        .sum()
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let graph = parse(_input);

    let price = kosaraju_scc(&graph)
        .iter()
        .map(|nodes| {
            let area = nodes.len() as u64;
            let walls = nodes
                .iter()
                .map(|node| count_corners(&graph, &node, &nodes))
                .sum::<u64>();
            area * walls
        })
        .sum::<u64>();

    Ok(price.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> miette::Result<()> {
        let input = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!("80", process(input)?);
        Ok(())
    }

    #[test]
    fn test_second() -> miette::Result<()> {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!("236", process(input)?);
        Ok(())
    }

    #[test]
    fn test_third() -> miette::Result<()> {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!("368", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1206", process(input)?);
        Ok(())
    }
}
