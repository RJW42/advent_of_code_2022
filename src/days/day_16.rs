use std::fmt;
use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::Peekable;

use std::collections::HashMap;
use bit_vec::BitVec;

type NodeIndex = usize;
type EdgeIndex = usize;

struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
    id_node_map: HashMap<u32, NodeIndex>,
    shortest_distance_map: HashMap<(NodeIndex, NodeIndex), u32>
}

struct NodeData {
    id: u32,
    flow_rate: u32,
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
    let graph = build_graph(file_name)?;

    part_one(&graph);
    part_two(&graph);
    
    Ok(())
}


fn part_one(graph: &Graph) {
    let mut open_valves = BitVec::from_elem(graph.nodes.len(), false);
    let mut score_map = HashMap::new();

    let score = dfs(
        graph, *graph.id_node_map.get(&0).unwrap(), 0, 30, &mut open_valves, &mut score_map
    );

    println!("{}", score);
}

fn part_two(graph: &Graph) {
    let mut open_valves = BitVec::from_elem(graph.nodes.len(), false);
    let mut score_map = HashMap::new();

    dfs(
        graph, *graph.id_node_map.get(&0).unwrap(), 
        0, 26, &mut open_valves, &mut score_map
    );

    let mut max_score = 0;

    for (set1, score1) in &score_map {
        for (set2, score2) in &score_map {
            let score = score1 + score2;

            if score > max_score && no_overlap(set1, set2) {
                max_score = score;
            }
        }
    }

    println!("{}", max_score);
}

fn no_overlap(b1: &BitVec, b2: &BitVec) -> bool {
    let mut b1_c = b1.clone();
    b1_c.and(b2);
    b1_c.none()
}

fn dfs(
        graph: &Graph, current_node: NodeIndex, current_score: u32, 
        time_remaning: u32, open_valves: &mut BitVec, score_map: &mut HashMap<BitVec, u32>
) -> u32 {
    let max_child = graph.successors(current_node).into_iter().filter_map(|next_node| {
        if open_valves[next_node] || time_remaning <= graph.get_distance(current_node, next_node).unwrap() {
            if let Some(score) = score_map.get(open_valves) {
                if *score < current_score {
                    score_map.insert(open_valves.clone(), current_score);
                }
            } else {
                score_map.insert(open_valves.clone(), current_score);
            }
            return None;
        }

        open_valves.set(next_node, true);
        
        let time_remaning = (time_remaning - graph.get_distance(current_node, next_node).unwrap()) - 1;
        let aditional_score = time_remaning * graph.nodes[next_node].flow_rate;
        let score = dfs(
            graph, next_node, current_score + aditional_score, 
            time_remaning, open_valves, score_map
        );

        open_valves.set(next_node, false);

        Some(score)
    }).max();

    if let Some(score) = max_child {
        if score > current_score {
            return score;
        }
    } 

    current_score
}


/* Parsing Code */
fn build_graph(file_name: &str) -> std::io::Result<Graph> {
    let mut graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
        id_node_map: HashMap::new(),
        shortest_distance_map: HashMap::new(),
    };


    // Add all edges and nodes 
    for (source_node, target_id) in init_all_nodes(&mut graph, file_name)?.iter() {
        let target_node = *graph.id_node_map.get(&target_id).unwrap();

        graph.add_edge(
            *source_node, target_node
        );

        graph.shortest_distance_map.insert(
            (*source_node, target_node), 1
        );
    }

    // Compute the shortest distance between all nodes
    for node in 0..graph.nodes.len() {
        graph.shortest_distance_map.insert(
            (node, node), 0
        );
    }

    for node_k in 0..graph.nodes.len() {
        for node_i in 0..graph.nodes.len() {
            for node_j in 0..graph.nodes.len() {
                let dist_i_j = graph.get_distance(node_i, node_j);
                let dist_i_k = graph.get_distance(node_i, node_k);
                let dist_k_j = graph.get_distance(node_k, node_j);

                match (dist_i_j, dist_i_k, dist_k_j) {
                    (None, Some(v1), Some(v2)) => {
                        graph.shortest_distance_map.insert((node_i, node_j), v1 + v2);
                    },
                    (Some(v0), Some(v1), Some(v2)) if v0 > v1 + v2 => {
                        graph.shortest_distance_map.insert((node_i, node_j), v1 + v2);
                    },
                    _ => (),
                };
            } 
        }
    }

    let mut output_graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
        id_node_map: HashMap::new(),
        shortest_distance_map: HashMap::new(),
    };

    let mut old_to_new_map = HashMap::new();

    for (old_node_i, old_node) in graph.nodes.iter().enumerate() {
        if old_node.flow_rate == 0 && old_node.id != 0 {
            continue;
        }

        let new_i = output_graph.add_node(old_node.id, old_node.flow_rate);
        old_to_new_map.insert(old_node_i, new_i);
    }

    for old_node_i in 0..graph.nodes.len() {
        if !old_to_new_map.contains_key(&old_node_i) {
            continue;
        }

        for old_target_i in 0..graph.nodes.len() {
            if !old_to_new_map.contains_key(&old_target_i) || old_node_i == old_target_i ||   
                graph.nodes[old_target_i].flow_rate == 0 {
                continue;
            }

            let new_source = *old_to_new_map.get(&old_node_i).unwrap();
            let new_target = *old_to_new_map.get(&old_target_i).unwrap(); 

            output_graph.add_edge(new_source, new_target);
            output_graph.shortest_distance_map.insert(
                (new_source, new_target), graph.get_distance(
                    old_node_i, old_target_i, 
                ).unwrap()
            );
        }
    }


    output_graph.print();

    Ok(output_graph)
}


fn init_all_nodes(graph: &mut Graph, file_name: &str) -> std::io::Result<Vec<(NodeIndex, u32)>> {
    let file = fs::File::open(file_name)
        .expect("Failed to open");
    let reader = BufReader::new(file);
    let mut edges_to_add = Vec::new();

    // Init all nodes 
    for line in reader.lines() {
        let (id, rate, connections) = parse_valve_definition(&line?).unwrap();
        let index = graph.add_node(id, rate);

        for connection in connections {
            edges_to_add.push(
                (index, connection)
            );
        }
    }

    Ok(edges_to_add)
}



impl Graph {
    fn add_node(&mut self, id: u32, flow_rate: u32) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData {
            id: id,
            flow_rate: flow_rate,
            first_outgoing_edge: None,
        });
        self.id_node_map.insert(id, index);
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

    fn get_distance(&self, source: NodeIndex, target: NodeIndex) -> Option<u32> {
        self.shortest_distance_map.get(&(source, target)).map(|x| *x)
    }

    fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors { 
            graph: self,
            currend_edge_index: first_outgoing_edge,
        }
    }

    fn print(&self) {
        println!("graph: ");
        for (node_i, node) in self.nodes.iter().enumerate() {
            println!("  {} fr {}: ", node, node.flow_rate);
            for (next_i, next_node) in self.nodes.iter().enumerate() {
                if let Some(dist) = self.get_distance(node_i, next_i) {
                    println!("    {} -> {} fr {}", dist, next_node, next_node.flow_rate);
                }
            }
            println!();
        }
    }
}


impl fmt::Display for NodeData {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}{}", 
            char::from_u32(self.id / 26 + 'A' as u32).unwrap(),
            char::from_u32(self.id % 26 + 'A' as u32).unwrap()
        )
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

fn parse_valve_definition(line: &str) -> Option<(u32, u32, Vec<u32>)> {
    let chars_vec = line.chars().collect::<Vec<char>>();
    let mut chars = chars_vec.iter().skip(6).peekable();

    let id = parse_valve_id(&mut chars)?;
    advance_to_first_char_after_equals(&mut chars)?;
    let rate = parse_num(&mut chars)?;

    let mut chars = chars.skip(24).peekable();

    if let Some(' ') = chars.peek() {
        chars.next();
    }

    let connections = parse_valves_list(&mut chars)?;

    Some((
        id, rate, connections
    ))
}


fn parse_valves_list<'a, I>(chars: &mut Peekable<I>) -> Option<Vec<u32>> 
where I: Iterator<Item = &'a char> {
    let mut output = Vec::new();

    loop { match chars.peek() {
        Some(' ') => { chars.next(); },
        Some(',') => { chars.next(); },
        None => break,
        _ => {
            if let Some(id) = parse_valve_id(chars) {
                output.push(id);
            } else {
                break;
            }
        }
    }}

    Some(output)
}


fn parse_valve_id<'a, I>(chars: &mut Peekable<I>) -> Option<u32> 
where I: Iterator<Item = &'a char> {
    let mut output = None;
    let mut output_val = 0;

    loop { match chars.peek() {
        ch @ Some('A'..='Z') => {
            output_val = output_val * 26 + (
                **ch.unwrap() as u32 - 'A' as u32
            );
            output = Some(output_val);
            chars.next();
        },
        _ => break,
    }; }

    output
}


fn advance_to_first_char_after_equals<'a, I>(chars: &mut Peekable<I>) -> Option<()>
where I: Iterator<Item = &'a char> {
    while let Some(ch) = chars.next() {
        if *ch == '=' {
            return Some(());
        }
    }
    return None;
}


fn parse_num<'a, I>(chars: &mut Peekable<I>) -> Option<u32> 
where I: Iterator<Item = &'a char> {
    let mut output = None;
    let mut output_val = 0;

    loop {
        match chars.peek() {
            ch @ Some('0'..='9') => {
                output_val = output_val * 10 + (
                    **ch.unwrap() as u32 - '0' as u32
                );
                output = Some(output_val);
                chars.next();
            },
            _ => break,
        };
    }

    output
}