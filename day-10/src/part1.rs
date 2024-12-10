use std::collections::HashMap;
use glam::IVec2;
use itertools::Itertools;
use petgraph::algo::has_path_connecting;
use crate::custom_error::AocError;
use petgraph::Directed;
use petgraph::graph::NodeIndex;

const ABOVE: IVec2 = IVec2::new(0, -1);
const BELOW: IVec2 = IVec2::new(0, 1);
const LEFT: IVec2 = IVec2::new(-1, 0);
const RIGHT: IVec2 = IVec2::new(1, 0);

const SURROUNDINGS: [IVec2; 4] = [ABOVE, BELOW, LEFT, RIGHT];


fn parse(input: &str) -> petgraph::Graph<u8, (), Directed> {
    let map = input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, c)| {
            (IVec2::new(x as i32, y as i32), c.to_digit(10).unwrap() as u8)
        })
    }).collect::<HashMap<_, _>>();

    let mut graph = petgraph::Graph::<u8, (), Directed>::new();
    let mut index_mapping = HashMap::new();

    for (coordinates, height) in &map {
        let nodeindex = graph.add_node(*height);
        index_mapping.insert(coordinates, nodeindex);
    }

    for (coordinates, height) in &map {
        SURROUNDINGS
            .iter()
            .map( |&direction| coordinates + direction )
            .filter(|neighbor| map.get(neighbor) == Some(&(height + 1)))
            .for_each(|neighbor| {
                match (index_mapping.get(coordinates), index_mapping.get(&neighbor)) {
                    (Some(index), Some(neighbor)) => {
                        graph.add_edge(index.clone(), neighbor.clone(), ());
                    },
                    _ => {},
                }
            })
    }
    graph
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let graph = parse(_input);

    let trail_heads = graph
        .raw_nodes()
        .iter()
        .enumerate()
        .filter_map(|(index, node)| {
            match node.weight {
                0 => Some(index),
                _ => None,
            }
        }).collect::<Vec<_>>();

    let targets = graph
        .raw_nodes()
        .iter()
        .enumerate()
        .filter_map(|(index, node)| {
            match node.weight {
                9 => Some(index),
                _ => None,
            }
        }).collect::<Vec<_>>();

    let result = trail_heads.iter()
        .cartesian_product(targets.iter())
        .filter(|(&trail_head, &target)| {
            has_path_connecting(&graph, NodeIndex::new(trail_head), NodeIndex::new(target), None)
        })
        .count();

    Ok(result.to_string())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("36", process(input)?);
        Ok(())
    }
}
