use crate::custom_error::AocError;
use glam::IVec2;
use petgraph::algo::kosaraju_scc;
use petgraph::Graph;
use std::collections::HashMap;

const ABOVE: IVec2 = IVec2::new(0, -1);
const BELOW: IVec2 = IVec2::new(0, 1);
const LEFT: IVec2 = IVec2::new(-1, 0);
const RIGHT: IVec2 = IVec2::new(1, 0);

const SURROUNDINGS: [IVec2; 4] = [ABOVE, BELOW, LEFT, RIGHT];

fn parse(input: &str) -> Graph<(), ()> {
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
        let nodeindex = graph.add_node(());
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

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let graph = parse(_input);

    let price = kosaraju_scc(&graph)
        .iter()
        .map(|nodes| {
            let area = nodes.len() as u64;
            let perimeter = nodes
                .iter()
                .map(|node| 4 - graph.neighbors(*node).count() as u64)
                .sum::<u64>();

            area * perimeter
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
        assert_eq!("140", process(input)?);
        Ok(())
    }

    #[test]
    fn test_second() -> miette::Result<()> {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!("772", process(input)?);
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
        assert_eq!("1930", process(input)?);
        Ok(())
    }
}
