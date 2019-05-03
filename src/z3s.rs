use crate::*;
use z3::*;

pub fn start(mut graph: Graph) -> Graph {
    let config = Config::new();
    let context = Context::new(&config);
    
    println!("Start solver");
    let solver = Solver::new(&context);
    
    println!("Start cells");
    let named: Vec<Vec<Ast>> = graph.cells.iter().enumerate()
        .map(|(y, row)| {
            row.iter().enumerate().map(|(x, cell)| {
                if let Some(v) = cell.value {
                    context.from_i64(v as i64)
                } else {
                    context.named_int_const(&format!("{},{}", x, y))
                }
            }).collect::<Vec<_>>()
        }).collect();

    let one = context.from_i64(1); 

    for i in 9..=10 {
        solver.check();
        let nine = context.from_i64(i);    
        
        for (x, row) in graph.cells.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if cell.value.is_none() {
                    solver.assert(&named[x][y].ge(&one));
                    solver.assert(&named[x][y].le(&nine));
                    
                    for neighbour in cell.neighbours.iter().map(|CellCoord { x, y }| &named[*x as usize][*y as usize]) {
                        solver.assert(&named[x][y]._eq(neighbour).not());
                    }
                }
            }
        }
        if solver.check() {
            println!("I think it worked?");
            let model = solver.get_model();
            for (x, row) in graph.cells.iter_mut().enumerate() {
                for (y, cell) in row.iter_mut().enumerate() {
                    if cell.value.is_none() {                    
                        cell.value = Some(dbg!(model.eval(&named[x][y]).unwrap().as_i64().unwrap()) as u8);
                    }
                }
            }
        } else {
            println!("Didn't work?");            
        }
    }

    graph
}