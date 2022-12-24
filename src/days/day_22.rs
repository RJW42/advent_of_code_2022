use std::fs;
use std::fmt;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::Peekable;
use std::collections::HashMap;

use crate::days::day_22::Direction::*;
use crate::days::day_22::Heading::*;

type NodeIndex = usize;
type FaceIndex = usize;

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
    Forward(u32)
}

#[derive(Copy, Clone)]
enum Heading {
    North,
    East,
    South,
    West
}

struct Graph {
    width: u32,
    height: u32,
    nodes: Vec<NodeData>,
    faces: Vec<FaceData>,
    position_node_map: HashMap<(u32, u32), NodeIndex>,
    position_face_map: HashMap<(u32, u32), FaceIndex>,
    top_left_x: u32,
    top_left_y: u32,
    face_width: u32,
}

struct NodeData {
    face: FaceIndex,
    north_index: (NodeIndex, Heading),
    east_index: (NodeIndex, Heading),
    west_index: (NodeIndex, Heading),
    south_index: (NodeIndex, Heading),
    is_wall: bool,
    x: u32,
    y: u32,
}

struct FaceData {
    top_left_x: u32,
    top_left_y: u32,
    north_index: Option<FaceIndex>,
    east_index: Option<FaceIndex>,
    west_index: Option<FaceIndex>,
    south_index: Option<FaceIndex>,
}


pub fn run(file_name: &str) -> std::io::Result<()> {
    let (graph, directions) = parse_graph_and_path(file_name, true)?;
    let mut path = Vec::new();

    graph.print();
    graph.print_faces();

    println!("{}", graph.face_width);

    for (i, f) in graph.faces.iter().enumerate() {
        print!("{} => ", i);
        if let Some(f) = f.north_index {
            print!("N:{}, ", f);
        }
        if let Some(f) = f.east_index {
            print!("E:{}, ", f);
        }
        if let Some(f) = f.south_index {
            print!("S:{}, ", f);
        }
        if let Some(f) = f.west_index {
            print!("W:{}, ", f);
        }
        println!();
    }

    path.push((
        *graph.position_node_map.get(
            &(graph.top_left_x, graph.top_left_y)
        ).unwrap(), East
    ));

    let score = walk_graph(
        graph.top_left_x, graph.top_left_y, 
        &graph, &directions, 0, &mut path, East
    );

    graph.print_with_path(&path);

    println!("output: {}", score);

    Ok(())
}

fn walk_graph(
    x: u32, y: u32, graph: &Graph, directions: &Vec<Direction>, 
    index: usize, path: &mut Vec<(NodeIndex, Heading)>, current_heading: Heading
) -> u32 {
    if index == directions.len() {
        let partial_ouput = 1000 * (y + 1) + 4 * (x + 1);

        return partial_ouput + match current_heading {
            East => 0,
            South => 1,
            West => 2,
            North => 3,
        };
    }

    let (new_x, new_y, new_heading) = match directions[index] {
        heading_change @ (Left | Right) => {
            let new_heading = get_new_heading(
                heading_change, current_heading
            );

            let path_index = path.len() - 1;
            let (node_index, _) = path[path_index];
            path[path_index] = (node_index, new_heading);

            (x, y, new_heading)
        },
        Forward(distance) => {
            let (mut final_x, mut final_y) = (x, y);
            let mut current_heading = current_heading;
            let mut current_node = *graph.position_node_map.get(
                &(x, y)
            ).unwrap();

            for _ in 0..distance {
                let (next_node, edge_heading) = graph.nodes[current_node].get_edge(
                    current_heading
                );

                if graph.nodes[next_node].is_wall {
                    break;
                }

                current_node = next_node;
                current_heading = edge_heading;
                
                final_x = graph.nodes[current_node].x;
                final_y = graph.nodes[current_node].y;

                path.push((
                    current_node, current_heading
                ));
            }

            (final_x, final_y, current_heading)
        },
    };

    walk_graph(
        new_x, new_y, graph, directions, 
        index + 1, path, new_heading
    )
}


fn get_new_heading(
    direction: Direction, current_heading: Heading
) -> Heading {
    match (direction, current_heading) {
        (Left, North) => West,
        (Left, East) => North,
        (Left, South) => East,
        (Left, West) => South,
        (Right, North) => East,
        (Right, East) => South,
        (Right, South) => West,
        (Right, West) => North,
        (_, heading) => heading
    }
}


fn get_oppside(heading: Heading) -> Heading {
    match heading {
        North => South,
        South => North,
        East => West,
        West => East
    }
}




/* Core Graph Code */
impl Graph {
    fn add_node(&mut self, x: u32, y: u32, is_wall: bool) -> NodeIndex {
        let index = self.nodes.len();

        if self.top_left_x == 0 {
            self.top_left_x = x;
        }

        if self.face_width == 0 && x < self.top_left_x {
            self.face_width = y;
        }

        self.nodes.push(NodeData {
            face: 0,
            north_index: (0, North),
            east_index: (0, East),
            west_index: (0, West),
            south_index: (0, South),
            is_wall: is_wall,
            x: x,
            y: y,
        });
        self.position_node_map.insert((x, y), index);
        index
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!(
                    "{}", match self.position_node_map.get(&(x, y)) {
                        None => ' ',
                        Some(index) => {
                            if self.nodes[*index].is_wall {
                                '#'
                            } else {
                                '.'
                            }
                        }
                    }

                );
            }
            println!();
        }
    }


    fn print_with_path(&self, path: &Vec<(NodeIndex, Heading)>) {
        let mut index_to_heading = HashMap::new();

        for (index, heading) in path {
            index_to_heading.insert(index, heading);
        }

        for y in 0..self.height {
            for x in 0..self.width {
                print!(
                    "{}", match self.position_node_map.get(&(x, y)) {
                        None => ' ',
                        Some(index) => {
                            if self.nodes[*index].is_wall {
                                '#'
                            } else {
                                match index_to_heading.get(index) {
                                    None => '.',
                                    Some(North) => '^',
                                    Some(South) => 'v',
                                    Some(East) => '>',
                                    Some(West) => '<',
                                }
                            }
                        }
                    }

                );
            }
            println!();
        }
    }

    fn print_faces(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!(
                    "{}", match self.position_face_map.get(&(x, y)) {
                        None => ' ',
                        Some(index) => char::from_u32(*index as u32 + '0' as u32).unwrap()
                    }

                );
            }
            println!();
        }
    }

    fn will_leave_face(&self, x: u32, y: u32, heading: Heading) -> bool {
        let face_index = *self.position_face_map.get(&(x, y)).unwrap();
        let face = &self.faces[face_index];

        match heading {
            North =>  y == face.top_left_y,
            East => x == face.top_left_x + self.face_width - 1,
            South => y == face.top_left_y + self.face_width - 1,
            West => x == face.top_left_x,
        }
    }

    fn get_outgoing_position_on_face_edge(&self, x: u32, y: u32, heading: Heading, face: FaceIndex) -> u32 {
        let face = &self.faces[face];

        match heading {
            North => x - face.top_left_x,
            South => x - face.top_left_x,
            East => y - face.top_left_y,
            West => y - face.top_left_y
        }
    }

    fn get_incoming_position_on_face_edge(&self, position: u32, heading: Heading, face: FaceIndex) -> (u32, u32) {
        let face = &self.faces[face];

        match heading {
            North => (position + face.top_left_x, face.top_left_y),
            South => (position + face.top_left_x, face.top_left_y + self.face_width - 1),
            East => (face.top_left_x + self.face_width - 1, position + face.top_left_y),
            West => (face.top_left_x, position + face.top_left_y),
        }
    }
}

impl NodeData {
    fn get_edge(&self, heading: Heading) -> (NodeIndex, Heading) {
        match heading {
            North => self.north_index,
            East => self.east_index,
            South => self.south_index,
            West => self.west_index,
        }
    }
}

impl FaceData {
    fn get_edge(&self, heading: Heading) -> Option<FaceIndex> {
        match heading {
            North => self.north_index,
            East => self.east_index,
            South => self.south_index,
            West => self.west_index,
        }
    }

    fn set_edge(&mut self, heading: Heading, face: FaceIndex) {
        match heading {
            North => self.north_index = Some(face),
            East => self.east_index = Some(face),
            South => self.south_index = Some(face),
            West => self.west_index = Some(face),
        };
    }

    fn get_connection_heading(&self, face: FaceIndex) -> Option<Heading> {
        if self.north_index == Some(face) {
            Some(North)
        } else if self.east_index == Some(face) {
            Some(East)
        } else if self.south_index == Some(face) {
            Some(South)
        } else if self.west_index == Some(face) {
            Some(West)
        } else {
            None
        }
    }
}


impl fmt::Display for NodeData {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", 
            match self.is_wall {
                true => '#',
                false => '.'
            }
        )
    }
}


impl fmt::Display for Heading {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", 
            match self {
                North => 'N',
                East => 'E',
                South => 'S',
                West =>  'W',
            }
        )
    }
}


/* Parsing */
fn parse_graph_and_path(file_name: &str, p2: bool) -> std::io::Result<(Graph, Vec<Direction>)> {
    let file = fs::File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let graph = parse_graph(&mut lines, p2)?;
    let directions = parse_directions(&lines.next().unwrap()?);
    
    Ok((graph, directions))
}


fn parse_directions(line: &str) -> Vec<Direction> {
    let mut directions = Vec::new();
    let mut chars = line.chars().into_iter().peekable();

    loop {
        match chars.peek() {
            Some('L') => {
                chars.next();
                directions.push(Left);
            },
            Some('R') => {
                chars.next();
                directions.push(Right);
            }
            Some(_) => {
                directions.push(Forward(
                    parse_num(&mut chars).unwrap()
                ));
            },
            _ => { break; }
        };
    }

    directions
}


fn parse_graph<I>(
    lines: &mut std::io::Lines<I>, p2: bool
) -> std::io::Result<Graph>
where I: std::io::BufRead {
    let mut graph = Graph {
        width: 0,
        height: 0,
        nodes: Vec::new(),
        faces: Vec::new(),
        position_node_map: HashMap::new(),
        position_face_map: HashMap::new(),
        top_left_x: 0,
        top_left_y: 0,
        face_width: 0,
    };

    // Add Nodes 
    for (y_pos, l) in lines.enumerate() {
        let line = l?;

        if line.len() == 0 {
            break;
        }   

        parse_graph_line(y_pos as u32, &line, &mut graph);
    } 

    add_graph_faces(&mut graph);
    add_grade_edges(&mut graph, p2);

    Ok(graph)
}

fn add_graph_faces(
    graph: &mut Graph
) {
    graph.face_width = gcd(graph.width, graph.height);

    add_face(graph.top_left_x, graph.top_left_y, graph);

    let number_of_faces = graph.faces.len();

    for f in 0..number_of_faces {
        fold_l_faces(f, graph);
    }
    for f in 0..number_of_faces {
        fold_l_faces(f, graph);
    }
    //fold_l_faces(1, graph);
    // fold_l_faces(2, graph);

}

fn add_face(x: u32, y: u32, graph: &mut Graph) -> Option<FaceIndex> {
    if graph.position_face_map.contains_key(&(x, y)) {
        return graph.position_face_map.get(&(x, y)).map(|f| *f);
    } 
    
    if !graph.position_node_map.contains_key(&(x, y)) {
        return None;
    }

    let face_index = graph.faces.len();

    graph.faces.push(FaceData {
        top_left_x: x,
        top_left_y: y,
        north_index: None,
        east_index: None,
        west_index: None,
        south_index: None,
    });

    for face_x in x..(x + graph.face_width) {
        for face_y in y..(y + graph.face_width) {
            graph.position_face_map.insert((face_x, face_y), face_index);
            graph.nodes[
                *graph.position_node_map.get(&(face_x, face_y)).unwrap()
            ].face = face_index;
        }
    }

    graph.faces[face_index].east_index = add_face(x + graph.face_width, y, graph);
    graph.faces[face_index].south_index = add_face(x, y + graph.face_width, graph);

    if x > 0 {
        graph.faces[face_index].west_index = add_face(x - graph.face_width, y, graph);
    }

    if y > 0 {
        graph.faces[face_index].north_index = add_face(x, y - graph.face_width, graph);
    }


    Some(face_index)
}


fn fold_l_faces(face: FaceIndex, graph: &mut Graph) {
    static HEADINGS: [Heading; 4] = [North, East, South, West];

    for heading in HEADINGS.iter() {
        let face_to_f1_h = *heading;
        let face_to_f2_h = get_new_heading(Left, *heading);

        let face_1 = graph.faces[face].get_edge(face_to_f1_h);
        let face_2 = graph.faces[face].get_edge(face_to_f2_h);

        if face_1.is_none() || face_2.is_none() {
            continue;
        }

        let face_1 = face_1.unwrap();
        let face_2 = face_2.unwrap();

        let f1_to_face_h = graph.faces[face_1].get_connection_heading(face).unwrap();
        let f2_to_face_h = graph.faces[face_2].get_connection_heading(face).unwrap();

        let f1_to_f2_h = get_new_heading(Right, f1_to_face_h);
        let f2_to_f1_h = get_new_heading(Left, f2_to_face_h);

        if graph.faces[face_1].get_edge(f1_to_f2_h).is_some() ||
           graph.faces[face_2].get_edge(f2_to_f1_h).is_some() {
            continue;
        }

        graph.faces[face_2].set_edge(f2_to_f1_h, face_1);
        graph.faces[face_1].set_edge(f1_to_f2_h, face_2);
    }
}


fn add_grade_edges(
    graph: &mut Graph, p2: bool
) {
    for y in 0..graph.height {
        for x in 0..graph.width {
            if !graph.position_node_map.contains_key(&(x, y)) {
                continue;
            }

            let upped_edge = get_edge_index(x, y, 0, -1, &graph, p2);
            let lower_edge = get_edge_index(x, y, 0, 1, &graph, p2);
            let left_edge = get_edge_index(x, y, -1, 0, &graph, p2);
            let right_edge = get_edge_index(x, y, 1, 0, &graph, p2);

            let index = *graph.position_node_map.get(&(x, y)).unwrap();

            graph.nodes[index].north_index = upped_edge;
            graph.nodes[index].east_index = right_edge;
            graph.nodes[index].south_index = lower_edge;
            graph.nodes[index].west_index = left_edge;
        }
    }
}


fn get_edge_index(
    x: u32, y: u32, x_offset: i32, y_offset: i32, graph: &Graph, p2: bool
) -> (NodeIndex, Heading) {
    if p2 {
        get_edge_index_p2(x, y, x_offset, y_offset, graph)
    } else {
        get_edge_index_p1(x, y, x_offset, y_offset, graph)
    }
}


fn get_edge_index_p1(
    x: u32, y: u32, x_offset: i32, y_offset: i32, graph: &Graph
) -> (NodeIndex, Heading) {
    let mut x = x as i32;
    let mut y = y as i32;

    let heading = match (x_offset, y_offset) {
        (1, 0) => East,
        (-1, 0) => West,
        (0, 1) => South,
        (0, -1) => North,
        _ => panic!(),
    };

    loop {
        x += x_offset;
        y += y_offset;

        if x >= graph.width as i32 {
            x = 0;        
        } else if x < 0 {
            x = graph.width as i32;
        }
        if y >= graph.height as i32 {
            y = 0;
        } else if y < 0 {
            y = graph.height as i32;
        }
        
        if graph.position_node_map.contains_key(&(x as u32, y as u32)) {
            break;
        }
    }

    (*graph.position_node_map.get(&(x as u32, y as u32)).unwrap(), heading)
}

fn get_edge_index_p2(
    x: u32, y: u32, x_offset: i32, y_offset: i32, graph: &Graph
) -> (NodeIndex, Heading) {
    let heading = match (x_offset, y_offset) {
        (1, 0) => East,
        (-1, 0) => West,
        (0, 1) => South,
        (0, -1) => North,
        _ => panic!(),
    };

    if !graph.will_leave_face(x, y, heading) {
        let x = x as i32 + x_offset;
        let y = y as i32 + y_offset;

        return (*graph.position_node_map.get(&(x as u32, y as u32)).unwrap(), heading);
    }

    let current_face = *graph.position_face_map.get(&(x, y)).unwrap();
    let next_face = graph.faces[current_face].get_edge(heading).unwrap();
    let connecting_heading = graph.faces[next_face].get_connection_heading(current_face).unwrap();
    
    let connecting_position = graph.get_outgoing_position_on_face_edge(
        x, y, heading, current_face
    );

    let connecting_position = get_face_change_offset(
        heading, connecting_heading, graph, connecting_position
    );

    let incoming_position = graph.get_incoming_position_on_face_edge(
        connecting_position, connecting_heading, next_face
    );

    (*graph.position_node_map.get(&incoming_position).unwrap(), get_oppside(connecting_heading))
}


fn get_face_change_offset(outgroing_heading: Heading, incoming_heading: Heading, graph: &Graph, pos: u32) -> u32 {
    match (outgroing_heading, incoming_heading) {
        (North, South) | (South, North) | 
        (East, West) | (West, East) |
        (North, West) | (West, North) |
        (South, East) | (East, South) => pos,
        (North, East) | (East, North) | 
        (South, South) | (North, North) |
        (East, East) | (West, West)  => (graph.face_width - 1) - pos,
        (South, West) | (West, South) => (graph.face_width - 1) - pos,
    }
}


fn parse_graph_line(
    y: u32, line: &str, graph: &mut Graph
) {
    for (x, ch) in line.chars().enumerate() {
        let is_wall = match ch {
            '.' => false,
            '#' => true,
            _ => { continue; },
        };

        let x = x as u32;

        if x >= graph.width {
            graph.width = x + 1;
        }
        if y >= graph.height {
            graph.height = y + 1;
        }

        graph.add_node(
            x, y, is_wall
        );
    }
}


fn parse_num<I>(
    chars: &mut Peekable<I>
) -> Option<u32> 
where I: Iterator<Item = char> {
    let mut output = None;
    let mut output_val = 0;

    loop {
        match chars.peek() {
            ch @ Some('0'..='9') => {
                output_val = output_val * 10 + (
                    *ch.unwrap() as u32 - '0' as u32
                );
                output = Some(output_val);
                chars.next();
            },
            _ => break,
        };
    }

    output
}


fn gcd(first: u32, second: u32) -> u32 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}