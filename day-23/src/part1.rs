use crate::custom_error::AocError;
use itertools::Itertools;
use petgraph::Graph;
use std::collections::{HashMap, HashSet};
use std::io::Error;

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

    let cliques = graph
        .node_indices()
        .into_iter()
        .flat_map(|n1| {
            graph.node_indices().into_iter().flat_map({
                let graph = graph.clone();
                move |n2| {
                    graph.node_indices().into_iter().filter_map({
                        let graph = graph.clone();
                        move |n3| {
                            if n1.index() < n2.index()
                                && n2.index() < n3.index()
                                && graph.contains_edge(n1, n2)
                                && graph.contains_edge(n2, n3)
                                && graph.contains_edge(n3, n1)
                            {
                                Some((n1, n2, n3))
                            } else {
                                None
                            }
                        }
                    })
                }
            })
        })
        .filter(|(n1, n2, n3)| {
            graph.node_weight(*n1).unwrap().starts_with("t")
                || graph.node_weight(*n2).unwrap().starts_with("t")
                || graph.node_weight(*n3).unwrap().starts_with("t")
        })
        .collect_vec();

    Ok(cliques.len().to_string())
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
        assert_eq!("7", process(input)?);
        Ok(())
    }
}
