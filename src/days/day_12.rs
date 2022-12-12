use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::VecDeque;

static START_VAL: u32 = 26;
static END_VAL: u32 = 27;

type NodeIndex = usize;
type EdgeIndex = usize;

struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

struct NodeData {
    first_outgoing_edge: Option<EdgeIndex>,
}

struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

struct Successors<'graph> {
    graph: &'graph Graph,
    currend_edge_index: Option<EdgeIndex>,
}


pub fn run(file_name: &str) -> std::io::Result<()> {
    let (graph, start_index, end_index, width, height, matrix) = parse_graph(file_name)?;

    let mut distances = Vec::new();

    distances.push(find_shortest_path(&graph, start_index, end_index, width, height, false).unwrap());

    for (i, v) in matrix.iter().enumerate() {
        if *v != 0 {
            continue;
        }

        if let Some(distance) = find_shortest_path(&graph, i, end_index, width, height, false) {
            distances.push(distance);
        } else {
            println!("no soln");
        }
    }

    println!("min: {}", distances.iter().min().unwrap());

    Ok(())
}

fn find_shortest_path(
    graph: &Graph, start_index: NodeIndex, end_index: NodeIndex, 
    width: u32, height: u32, debug: bool
) -> Option<u32> {
    let mut distances = vec![u32::MAX; graph.nodes.len()];
    let mut previous = vec![None as Option<NodeIndex>; graph.nodes.len()];
    let mut visited = vec![false; graph.nodes.len()];

    distances[start_index] = 0;

    // Dikstras search 
    println!("start: {}", start_index);
    while let Some(node_index) = min_node(&distances, &visited) {
        visited[node_index] = true;

        if node_index == end_index || distances[node_index] == u32::MAX{
            break;
        }

        let pos = index_to_pos(node_index, width);

        if debug {
            println!("min_node: ({},{}), {}", pos.0, pos.1, distances[node_index]);
        }

        for neighbor_index in graph.successors(node_index) {
            if visited[neighbor_index] {
                continue;
            }

            let distance = distances[node_index] + 1;

            if distance < distances[neighbor_index] {
                // New shortest path to neightbor 
                distances[neighbor_index] = distance;
                previous[neighbor_index] = Some(node_index);
            }
        }
    }

    if !visited[end_index] {
        return None;
    }

    println!("Distance: {}", distances[end_index]);

    if !debug {
        return Some(distances[end_index]);
    }

    let mut path = VecDeque::new();
    let mut prev = end_index;

    path.push_front(prev);

    while let Some(next) = previous[prev] {
        path.push_front(next);
        prev = next;
    }

    println!("Path:");
    for row in 0..height {
        for column in 0..width {
            let index = get_index(width, row, column);
            let in_path = path.contains(&index);

            if !in_path {
                print!(".");
                continue;
            } else if index == end_index {
                print!("E");
                continue;
            }

            let mut index_in_path = 0;

            for (i, v) in path.iter().enumerate() {
                if *v == index {
                    index_in_path = i;
                    break;
                }
            }

            let (next_row, next_column) = index_to_pos(*path.get(index_in_path + 1).unwrap(), width);
            let row_diff: i32 = next_row as i32 - row as i32;
            let col_diff: i32 = next_column as i32 - column as i32;
             
            match (row_diff, col_diff) {
                (0, 1) => print!(">"),
                (0, -1) => print!("<"),
                (-1, 0) => print!("^"),
                (1, 0) => print!("v"),
                _ => {
                    println!("\n wtf: {}, {}", row_diff, col_diff);
                    panic!();
                }
            }
        }
        println!();
    } 

    Some(distances[end_index])
}


fn min_node(distances: &Vec<u32>, visited: &Vec<bool>) -> Option<NodeIndex> {
    let mut min_val = None as Option<u32>;
    let mut min_node = None as Option<NodeIndex>;

    for (i, distance) in distances.iter().enumerate() {
        if visited[i] {
            continue;
        }

        if let Some(v) = min_val {
            if v < *distance {
                continue;
            }
        }
        
        min_val = Some(*distance);
        min_node = Some(i);
    }

    min_node
}

fn index_to_pos(index: NodeIndex, width: u32) -> (u32, u32) {
    ((index as u32 / width), (index as u32 % width))
}



/* Graph Funtionality */
impl Graph {
    fn add_node(&mut self) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData {
            first_outgoing_edge: None,
        });
        index
    }

    fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target: target,
            next_outgoing_edge: node_data.first_outgoing_edge,
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors { 
            graph: self,
            currend_edge_index: first_outgoing_edge,
        }
    }
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.currend_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.currend_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}

/* Parsing */
fn parse_graph(file_name: &str) -> std::io::Result<(Graph, NodeIndex, NodeIndex, u32, u32, Vec<u32>)> {
    let (matrix, width, height) = parse_as_matrix(file_name)?;

    let mut graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
    };

    let mut start_index = 0;
    let mut end_index = 0;

    // Create all nodes 
    for row in 0..height {
        for column in 0..width {
            let ni = graph.add_node();
            let mi = get_index(width, row, column);

            if ni != mi {
                println!("hmmmm {} {}", mi, ni);
            }

            
            match matrix[mi] {
                26 => start_index = mi,
                27 => end_index = mi,
                _ => (),
            };
        }
    }    
    
    println!("Graph Form:");
    // Create all edges
    for row in 0..height {
        for column in 0..width {
            add_edges(
                &mut graph, &matrix, row, 
                column, width, height
            );
        }
    } 

    Ok((graph, start_index, end_index, width, height, matrix))
}

fn add_edges(
        graph: &mut Graph, matrix: &Vec<u32>, 
        row: u32, column: u32, width: u32, height: u32
) {
    let offsets: Vec<(i32, i32)> = Vec::from([(0, 1), (0, -1), (1, 0), (-1, 0)]);
    let node_index = get_index(width, row, column);
    let node_val = matrix[node_index];
    let node_ch = char::from_u32(node_val + 'a' as u32).unwrap();

    print!("  n[{}]({},{}) ->", node_ch, row, column);
    
    for offset in offsets {
        let edge_row = offset.0 + row as i32;
        let edge_col = offset.1 + column as i32;
        let edge_ch = char::from_u32(node_val + 'a' as u32).unwrap();

        if !is_valid_index(width, height, edge_row, edge_col) {
            continue; // Out of bounds 
        }

        let edge_index = get_index(
            width, edge_row.try_into().unwrap(), edge_col.try_into().unwrap()
        );
        let edge_val = match matrix[edge_index] {
            27 => 25,
            26 => 0,
            v @ _ => v
        };

        if edge_val > (node_val + 1) {
            continue; // Edge position is too high up
        }

        print!(" e[{}]({},{}),", edge_ch, edge_row, edge_col);

        graph.add_edge(node_index, edge_index);
    }

    println!();
}


fn parse_as_matrix(file_name: &str) -> std::io::Result<(Vec<u32>, u32, u32)> {
    let file = fs::File::open(file_name)
        .expect("File not found");
    let reader = BufReader::new(file);

    let mut matrix = Vec::new();
    let mut width = 0;
    let mut height = 0; 

    for line in reader.lines() {
        for ch in line?.chars() {
            let val = match ch {
                'S' => START_VAL,
                'E' => END_VAL,
                _ => (ch as u32) - ('a' as u32),
            };

            matrix.push(val);
        }

        if width == 0 {
            width = matrix.len() as u32;
        }

        height += 1;
    }


    println!("Matrix Form: ");
    for row in 0..height {
        print!("  ");
        for column in 0..width {
            let out = match matrix[get_index(width, row, column)] {
                26 => 'S',
                27 => 'E',
                v @ _ => char::from_u32('a' as u32 + v).unwrap(), 
            };

            print!("{}", out);
        }
        println!();
    }


    Ok((matrix, width, height))
}


fn get_index(width: u32, row: u32, column: u32) -> usize {
    return ((width * row) + column).try_into().unwrap();
}

fn is_valid_index(width: u32, height: u32, row: i32, column: i32) -> bool {
    row >= 0 && column >= 0 && row < height.try_into().unwrap() && column < width.try_into().unwrap()
}