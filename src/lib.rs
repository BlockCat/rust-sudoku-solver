#![feature(vec_remove_item)]
extern crate z3;

pub mod z3s;

#[derive(Debug, Default, Clone)]
pub struct CellCoord { x: u8, y: u8 }
impl CellCoord {
    pub fn new(x: u8, y: u8) -> CellCoord {
        CellCoord { x: x, y: y }
    }
}

#[derive(Debug, Clone)]
pub struct Graph {    
    pub cells: [[Cell; 9]; 9]
}

#[derive(Debug, Default, Clone)]
pub struct Cell {
    value: Option<u8>,
    neighbours: Vec<CellCoord>,
}

impl Graph {
    pub fn new() -> Graph {        
        let mut graph = Graph {
            cells: Default::default(),
        };
        
        for i in 0..9 {
            for j in 0..9 {                                
                let mut cell = &mut graph.get_cell_mutable(&CellCoord::new(i , j));
                cell.neighbours = cell.calculate_neighbours(&CellCoord::new(i, j));
            }
        }
        graph      
    }

    pub fn is_filled(&mut self) -> bool {
         for y in 0..9 {            
            for x in 0..9 {
                if let None = self.get_cell(&CellCoord::new(x, y)).get_value() {
                    return false;
                }
            }
         }
         true
    } 

    pub fn get_cell_mutable(&mut self, coords: &CellCoord) -> &mut Cell {
        &mut self.cells[coords.x as usize][coords.y as usize]
    }

    pub fn get_cell(&self, coords: &CellCoord) -> &Cell {
        &self.cells[coords.x as usize][coords.y as usize]
    }

    pub fn pretty_print(&self) {
        for y in 0..9 {
            print!("||");
            for x in 0..9 {
                let c = match self.get_cell(&CellCoord::new(x, y)).get_value() {
                    None => " ".to_string(),
                    Some(v) => v.to_string(),
                };
                print!("{} ", c);
                
            }            
            println!("||");
        }
        println!("");
    } 
}

impl Cell {
    fn calculate_neighbours(&self, position: &CellCoord) -> Vec<CellCoord> {
        let (i, j) = (position.x, position.y);
        let mut neighbours = Vec::with_capacity(20);

        let global_x = i / 3;
        let global_y = j / 3;
        
        //add local cube minus lines to vector
        for x in (global_x * 3)..(global_x * 3 + 3)  {
            for y in (global_y * 3)..(global_y * 3 + 3)  {
                if i != x || j != y {
                    neighbours.push(CellCoord{ x: x as _, y: y as _});
                }
            }   
        }

        // Add horizontal line to vector
        for x in 0..9 {
            if x != i {
                neighbours.push(CellCoord{ x: x as _, y: j as _});                
            }
        }
            // Add vertical line to vector
        for y in 0..9 {
            if y != j {
                neighbours.push(CellCoord{ x: i as _, y: y as _});                
            }
        }

        neighbours
    }



    pub fn get_possible_values(&self, graph: &Graph) -> Option<Vec<u8>> {
        if let None = self.get_value() {
            let neighbours = &self.neighbours;
            let mut values = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9];

            for n in neighbours {
                let c = graph.get_cell(n);
                if let Some(v) = c.get_value() {                    
                    values.remove_item(&v);
                }                
            }
            Some(values)
        } else {
            None
        }
    }

    pub fn get_value(&self) -> Option<u8> {
        self.value
    }

    pub fn set_value(&mut self, value: u8) {
        if value > 0 && value <= 9 {
            self.value = Some(value);
        } else {
            panic!("Cell received incorrect value: {}", value);
        }
    }
}