#![feature(test)]

use itertools::Itertools;

use search_engine::{SearchEngine, SearchProblem, SearchStrategy};

pub struct JugsProblem {
    pub j1_max: u32,
    pub j2_max: u32,
    pub target: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JugsNode {
    pub j1: u32,
    pub j2: u32,
}

impl SearchProblem<JugsNode> for JugsProblem {
    fn expand(&self, node: &JugsNode) -> Vec<JugsNode> {
        let mut successors = vec![];

        // Empty j1
        if node.j1 > 0 {
            successors.push(JugsNode { j1: 0, j2: node.j2 });
        }

        // Empty j2
        if node.j2 > 0 {
            successors.push(JugsNode { j1: node.j1, j2: 0 });
        }

        // Fill j1
        if node.j1 < self.j1_max {
            successors.push(JugsNode {
                j1: self.j1_max,
                j2: node.j2,
            });
        }

        // Fill j2
        if node.j2 < self.j2_max {
            successors.push(JugsNode {
                j1: node.j1,
                j2: self.j2_max,
            });
        }

        // Pour j1 into j2
        if node.j1 > 0 && node.j2 < self.j2_max {
            // More water than space in j2
            if node.j1 > self.j2_max - node.j2 {
                successors.push(JugsNode {
                    j1: node.j1 - (self.j2_max - node.j2),
                    j2: self.j2_max,
                });
            } else {
                successors.push(JugsNode {
                    j1: 0,
                    j2: node.j1 + node.j2,
                });
            }
        }

        // Pour j2 into j1
        if node.j2 > 0 && node.j1 < self.j1_max {
            if node.j2 > self.j1_max - node.j1 {
                successors.push(JugsNode {
                    j1: self.j1_max,
                    j2: node.j2 - (self.j1_max - node.j1),
                });
            } else {
                successors.push(JugsNode {
                    j1: node.j2 + node.j1,
                    j2: 0,
                });
            }
        }

        successors
    }

    fn goal_state(&self, node: &JugsNode) -> bool {
        node.j1 == self.target || node.j2 == self.target
    }
}

fn main() {
    let start_node = JugsNode { j1: 0, j2: 0 };

    let jugs_problem = JugsProblem {
        j1_max: 1234,
        j2_max: 12345,
        target: 543,
    };

    let search_engine = SearchEngine {
        start_node,
        search_problem: jugs_problem,
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
        let jugs_problem = JugsProblem {
            j1_max: 1234,
            j2_max: 12345,
            target: 543,
        };

        let search_engine = SearchEngine {
            start_node: JugsNode { j1: 0, j2: 0 },
            search_problem: jugs_problem,
            search_strategy: SearchStrategy::BFS,
        };

        b.iter(|| search_engine.run_search());
    }
}
