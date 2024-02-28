use std::collections::VecDeque;

use itertools::Itertools;
use petgraph::graph::{DiGraph, NodeIndex};

pub trait SearchProblem<SearchNode: Clone + PartialEq> {
    fn expand(&self, node: &SearchNode) -> Vec<SearchNode>;
    fn goal_state(&self, node: &SearchNode) -> bool;
}

pub enum SearchStrategy {
    Random,
    BFS,
    DFS,
}

pub struct SearchEngine<SearchNode, Problem>
where
    SearchNode: Clone + PartialEq + std::fmt::Debug,
    Problem: SearchProblem<SearchNode>,
{
    pub start_node: SearchNode,
    pub search_problem: Problem,
    pub search_strategy: SearchStrategy,
}

impl<SearchNode, Problem> SearchEngine<SearchNode, Problem>
where
    SearchNode: Clone + PartialEq + std::fmt::Debug,
    Problem: SearchProblem<SearchNode>,
{
    pub fn run_search(&self) -> Option<Vec<SearchNode>> {
        let mut open = VecDeque::new();
        let mut closed = vec![];
        let mut search_tree = DiGraph::<SearchNode, ()>::new();

        let start_node_index = search_tree.add_node(self.start_node.clone());
        open.push_back((self.start_node.clone(), start_node_index));

        while !open.is_empty() {
            let (current_node, current_node_index) = self.search(&mut open);

            closed.push(current_node.clone());

            if self.search_problem.goal_state(&current_node) {
                return Some(self.backtrack(current_node_index, search_tree));
            }

            let new_successors = self.find_new_successors(&current_node, &open, &closed);

            for new_successor in new_successors {
                let successor_index = search_tree.add_node(new_successor.clone());
                search_tree.add_edge(current_node_index, successor_index, ());

                open.push_back((new_successor.clone(), successor_index));
            }
        }

        None
    }

    fn search(&self, open: &mut VecDeque<(SearchNode, NodeIndex)>) -> (SearchNode, NodeIndex) {
        match self.search_strategy {
            SearchStrategy::DFS => open.pop_back().unwrap(),
            SearchStrategy::BFS => open.pop_front().unwrap(),
            SearchStrategy::Random => open.remove(fastrand::usize(..open.len())).unwrap(),
        }
    }

    fn find_new_successors(
        &self,
        current_node: &SearchNode,
        open: &VecDeque<(SearchNode, NodeIndex)>,
        closed: &[SearchNode],
    ) -> Vec<SearchNode> {
        self.search_problem
            .expand(&current_node)
            .into_iter()
            .filter(|successor| {
                !closed.contains(successor)
                    && open.iter().find(|(node, _)| successor == node).is_none()
            })
            .collect_vec()
    }

    fn backtrack(
        &self,
        solution_node_index: NodeIndex,
        search_tree: DiGraph<SearchNode, ()>,
    ) -> Vec<SearchNode> {
        let mut solution_path = vec![];
        let mut current_node_index = solution_node_index;

        while search_tree[current_node_index] != self.start_node {
            solution_path.push(search_tree[current_node_index].clone());

            current_node_index = search_tree
                .neighbors_directed(current_node_index, petgraph::Direction::Incoming)
                .into_iter()
                .next()
                .unwrap();
        }

        solution_path.into_iter().rev().collect_vec()
    }
}
