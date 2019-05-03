extern crate sudokusolver;

use sudokusolver::Graph;
use sudokusolver::CellCoord;
use std::collections::VecDeque;
use std::io::stdin;
fn main() {

    let graph = read_test();
    graph.pretty_print();

    match solve_z3(graph) {
        Some(solution) => solution.pretty_print(),
        None => println!("No solution found")
    }
}

fn solve_z3(graph: Graph) -> Option<Graph> {
    sudokusolver::z3s::start(graph)
}

fn solve_tree_search(graph: Graph) -> Option<Graph> {
    algorithm(&graph)
}

fn read_test() -> Graph {
    let lines = include_str!("../test.txt").lines().map(|x| String::from(x)).collect();
    parse_graph_strings(lines)
}
fn read_console() -> Graph {
    println!("Please fill in 9 lines of 9 characters each");
    println!("Or fill in 0 if empty");
    
    let mut input = vec![];
    for ii in 0..9 {

        if ii % 3 == 0 {
            println!("_________");
        }  
        
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Dit not enter a correct string");
        input.push(s);      

    }

    parse_graph_strings(input)
}

fn parse_graph_strings(lines: Vec<String>) -> Graph {
    let mut graph = Graph::new();

    if lines.len() != 9 { panic!("Invalid amount of lines") }
    for y in 0..9 {
        let line = &lines[y];
        for x in 0..9 {
            let number = line[x..x+1].parse::<u8>().unwrap();
            if number > 0 {
                graph.get_cell_mutable(&CellCoord::new(x as _, y as _)).set_value(number);
            }
        }
    }

    graph
}

// We are going to solve the sudoku
// using graph colouring algorithm because that is basically what sudokus are.
// the basic procedure is:
// -1. Find node i with least possible values that has no value assigned.
// -2. Branch on possible values assigned
// -3. Check if there are neighbours
// -4. Search neighbour of i with fewest possible values and no assignment
//      a. If all neighbours have a vallue assigned. Find some other node and continue to 1

fn algorithm(gg: &Graph) -> Option<Graph> {

    let mut stack: VecDeque<(String, Graph)> = VecDeque::new();
    stack.push_front(("root".to_string(), gg.clone()));

    while !stack.is_empty() {
        if let Some((_tag, mut graph)) = stack.pop_front() {            
            
            // For every cell that has only one value possible, fill it            
            let mut changed = true;
            while changed {
                changed = false;
                for i in 0..9 {
                    for j in 0..9 {
                        let coords = CellCoord::new(i, j);
                        if let Some(values) = graph.get_cell(&coords).get_possible_values(&graph) {
                            if values.len() == 1 {
                                graph.get_cell_mutable(&coords).set_value(values[0]);                        
                                changed = true;
                            }
                        }
                    }
                }
            }

            
            if graph.is_filled() {
                return Some(graph);
            }

            let mut smallest = (10, None);
            for i in 0..9 {
                for j in 0..9 {         
                    let cell = &graph.get_cell(&CellCoord::new(i, j));
                    if let Some(values) = cell.get_possible_values(&graph) {
                        if values.len() <= smallest.0 && values.len() >= 1 {
                            smallest = (values.len(), Some((i, j)));                    
                        }
                    }
                }
            }

            if let Some((x, y)) = smallest.1 {
                let coords = CellCoord::new(x, y);
                if let Some(values) = graph.get_cell(&coords).get_possible_values(&graph){
                    for c in values {
                        let mut cl = graph.clone();
                        cl.get_cell_mutable(&coords).set_value(c);
                        stack.push_front((format!("({}, {}) -> {}", x, y, c), cl));
                    }            
                }
            }
        }
    }
    None
}