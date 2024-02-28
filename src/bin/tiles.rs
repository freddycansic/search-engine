#![feature(test)]

use itertools::Itertools;

use search_engine::{SearchEngine, SearchProblem, SearchStrategy};

pub struct TilesProblem {}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TilesNode {
    board: Vec<Vec<Option<u32>>>,
    empty_square: Point,
}

impl SearchProblem<TilesNode> for TilesProblem {
    fn expand(&self, node: &TilesNode) -> Vec<TilesNode> {
        let successors = vec![];

        if node.empty_square.x > 0 {
            let mut left_shift = node.clone();
            left_shift.board[node.empty_square.x][node.empty_square.y] =
                node.board[node.empty_square.x - 1][node.empty_square.y];

            // 1 2 3
            // 4   5
            // 6 7 8

            // 1 2 3
            //   4 5
            // 6 7 8

            left_shift.board[node.empty_square.x][node.empty_square.y] = None;
            left_shift.empty_square.x -= 1;
        }

        successors
    }

    fn goal_state(&self, node: &TilesNode) -> bool {
        false
    }
}

fn main() {
    let start_node = TilesNode {};

    let tiles_problem = TilesProblem {};

    let search_engine = SearchEngine {
        start_node,
        search_problem: tiles_problem,
        search_strategy: SearchStrategy::BFS,
    };

    let solution_path = search_engine.run_search();

    match solution_path {
        Some(path) => {
            println!("{:?}", path.iter().format("\n"));
            println!("Found a solution in {} steps", path.len());
        }
        None => println!("Could not find a solution."),
    };
}

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;

    #[bench]
    fn bench_run_search(b: &mut test::Bencher) {
        let start_node = TilesNode {};

        let tiles_problem = TilesProblem {};

        let search_engine = SearchEngine {
            start_node,
            search_problem: tiles_problem,
            search_strategy: SearchStrategy::BFS,
        };

        b.iter(|| search_engine.run_search());
    }
}
