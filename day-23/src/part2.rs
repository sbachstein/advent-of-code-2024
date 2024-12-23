use crate::custom_error::AocError;
use itertools::Itertools;
use petgraph::data::DataMap;
use petgraph::graph::NodeIndex;
use petgraph::{Graph, Undirected};
use std::collections::{HashMap, HashSet};
use std::io::Error;

fn augment_cliques(
    cliques: &Vec<Vec<NodeIndex>>,
    graph: &Graph<&str, (), Undirected>,
) -> Vec<Vec<NodeIndex>> {
    let mut augmented_cliques = Vec::new();

    for clique in cliques {
        for node in graph.node_indices() {
            if clique.iter().all(|n| graph.contains_edge(node, *n)) {
                let mut augmented_clique = clique.clone();
                augmented_clique.push(node);
                augmented_clique.sort();
                augmented_cliques.push(augmented_clique);
            }
        }
    }

    augmented_cliques.into_iter().unique().collect_vec()
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut computers = HashSet::new();

    let connections = _input
        .lines()
        .map(|line| {
            let mut connection = line.split("-");
            let computer1 = connection.next().ok_or(Error::other("Invalid input"))?;
            let computer2 = connection.next().ok_or(Error::other("Invalid input"))?;

            computers.insert(computer1);
            computers.insert(computer2);

            Ok((computer1, computer2))
        })
        .collect::<Result<Vec<_>, AocError>>()?;

    let mut graph = Graph::new_undirected();

    let index_mapping = computers
        .iter()
        .map(|&computer| (computer, graph.add_node(computer)))
        .collect::<HashMap<_, _>>();

    connections.iter().for_each(|(computer1, computer2)| {
        graph.add_edge(index_mapping[*computer1], index_mapping[*computer2], ());
    });

    let mut cliques: Vec<Vec<_>> = graph.node_indices().map(|node| vec![node]).collect();
    let mut maximum_clique_size = 1usize;

    loop {
        let augmented_cliques = augment_cliques(&cliques, &graph);

        if let Some(new_maximum_clique_size) =
            augmented_cliques.iter().map(|clique| clique.len()).max()
        {
            if new_maximum_clique_size <= maximum_clique_size {
                break;
            } else {
                cliques = augmented_cliques
                    .into_iter()
                    .filter(|clique| clique.len() == new_maximum_clique_size)
                    .collect();
                maximum_clique_size = new_maximum_clique_size;
            }
        } else {
            break;
        }
    }

    let result = cliques
        .get(0)
        .ok_or(Error::other("No computers in the data"))?
        .iter()
        .map(|n| graph.node_weight(*n).unwrap().to_string())
        .sorted()
        .join(",");

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        assert_eq!("co,de,ka,ta", process(input)?);
        Ok(())
    }
}
