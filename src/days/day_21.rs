use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::Peekable;
use std::collections::HashMap;

use crate::days::day_21::Operation::*;
use crate::days::day_21::TreeNode::*;

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Clone, Copy)]
enum TreeNode {
    Node(NodeBody),
    Leaf(u32),
}

#[derive(Clone, Copy)]
enum NodeDescription {
    Node(Operation, u32, u32),
    Leaf(u32),
}

#[derive(Clone, Copy)]
struct NodeBody {
    operation: Operation,
    left_child: usize,
    right_child: usize,
    left_child_human: bool,
    right_child_human: bool,
}

impl NodeBody {
    fn new(
        operation: Operation, left_child: usize, 
        right_child: usize, left_child_human: bool,
        right_child_human: bool
    ) -> Self {
        Self {
            operation,
            left_child,
            right_child,
            left_child_human,
            right_child_human,
        }
    }
}


pub fn run(file_name: &str) -> std::io::Result<()> {
    let (root_index, human_index, tree) = build_tree(file_name)?;
    let p1_score = walk_tree(root_index, &tree);
    let p2_score = find_humm(root_index, human_index, &tree);

    println!("p1 score: {}", p1_score);
    println!("p2 score: {}", p2_score);

    Ok(())
}

fn walk_tree(index: usize, tree: &Vec<TreeNode>) -> u64 {
    match tree[index] {
        Leaf(val) => val.into(),
        Node(body) => {
            let left_val = walk_tree(body.left_child, tree);
            let right_val = walk_tree(body.right_child, tree);

            match body.operation {
                Add => left_val + right_val,
                Sub => left_val - right_val,
                Mul => left_val * right_val,
                Div => left_val / right_val
            }
        }
    }
}


fn find_humm(root_index: usize, human_index: usize, tree: &Vec<TreeNode>) -> u64 {
    match tree[root_index] {
        Leaf(_) => panic!(),
        Node(body) => 
            if body.left_child_human {
                let goal = walk_tree(body.right_child, tree);
                find_humm_walk(body.left_child, human_index, tree, goal)
            } else {
                let goal = walk_tree(body.left_child, tree);
                find_humm_walk(body.right_child, human_index, tree, goal)
            },
    }
}


fn find_humm_walk(index: usize, human_index: usize, tree: &Vec<TreeNode>, target_val: u64) -> u64 {
    println!("T: {}", target_val);
    match tree[index] {
        Leaf(_) => target_val,
        Node(body) => {
            let other_value = if body.left_child_human {
                walk_tree(body.right_child, tree)
            } else {
                walk_tree(body.left_child, tree)
            };

            match (body.operation, body.left_child_human) {
                (Add, true) => find_humm_walk(
                    body.left_child, human_index, tree, target_val - other_value
                ),
                (Add, false) => find_humm_walk(
                    body.right_child, human_index, tree, target_val - other_value
                ),
                (Sub, true) => find_humm_walk(
                    body.left_child, human_index, tree, target_val + other_value
                ),
                (Sub, false) => find_humm_walk(
                    body.right_child, human_index, tree, other_value - target_val
                ),
                (Mul, true) => find_humm_walk(
                    body.left_child, human_index, tree, target_val / other_value
                ),
                (Mul, false) => find_humm_walk(
                    body.right_child, human_index, tree, target_val / other_value
                ),
                (Div, true) => find_humm_walk(
                    body.left_child, human_index, tree, other_value * target_val
                ),
                (Div, false) => find_humm_walk(
                    body.right_child, human_index, tree, other_value / target_val
                )
            }
        }
    }
}



/* Parsing */
fn build_tree(
    file_name: &str
) -> std::io::Result<(usize, usize, Vec<TreeNode>)> {
    let file = fs::File::open(file_name)?;
    let reader = BufReader::new(file);
    
    let mut tree = Vec::new();
    let human_id = 136877;
    let mut human_index = 0;
    
    let mut unhandeld_descriptions: Vec<(Operation, u32, u32, u32)> = Vec::new();
    let mut waiting_for: HashMap<u32, usize> = HashMap::new(); // Node id -> description in vec 
    let mut seen_nodes: HashMap<u32, usize> = HashMap::new(); // node id -> node index

    for line in reader.lines() {
        let (node_id, node_description) = parse_node_description(&line?).unwrap();

        // println!("{}: ", node_id);

        // Check if can add to tree 
        let node_index = match node_description {
            NodeDescription::Leaf(val) => {
                tree.push(Leaf(val));
                // println!("{}", val);
                tree.len() - 1
            },
            NodeDescription::Node(opp, left_id, right_id) => {
                // println!("{} _ {}", left_id, right_id);

                if !seen_nodes.contains_key(&left_id) || 
                   !seen_nodes.contains_key(&right_id) {
                    let unhandeld_index = unhandeld_descriptions.len();

                    unhandeld_descriptions.push((
                        opp, left_id, right_id, node_id
                    ));

                    if waiting_for.contains_key(&left_id) || 
                       waiting_for.contains_key(&right_id) {
                        panic!();
                    }

                    if seen_nodes.contains_key(&left_id) {
                        waiting_for.insert(right_id, unhandeld_index);
                    } else if seen_nodes.contains_key(&right_id) {
                        waiting_for.insert(left_id, unhandeld_index);
                    } else {
                        waiting_for.insert(left_id, unhandeld_index);
                        waiting_for.insert(right_id, unhandeld_index);
                    }
                    continue;
                }

                let left_index = *seen_nodes.get(&left_id).unwrap();
                let right_index = *seen_nodes.get(&right_id).unwrap();

                let left_human = is_humm_or_child_is_humm(
                        &tree[left_index], left_index, human_index
                    );
                
                let right_human = is_humm_or_child_is_humm(
                        &tree[right_index], right_index, human_index
                    );

                tree.push(Node(NodeBody::new(
                    opp, left_index, right_index, left_human, right_human
                )));
                tree.len() - 1
            }
        };

        if node_id == human_id {
            human_index = node_index;
        }

        seen_nodes.insert(node_id, node_index);

        // Added to tree check if used for any other nodes 
        let mut node_id = node_id;

        while waiting_for.contains_key(&node_id) {
            let unhandeld_index = *waiting_for.get(&node_id).unwrap();
            waiting_for.remove(&node_id);

            let (
                unhandeld_opp, unhandeld_left, 
                unhandeld_right, unhandeld_id
            ) = unhandeld_descriptions[unhandeld_index];

            // print!(" - {}", unhandeld_id);

            if !seen_nodes.contains_key(&unhandeld_left) || 
                !seen_nodes.contains_key(&unhandeld_right) {
                    // println!(" ! {} || {}", unhandeld_left, unhandeld_right);
                break;
            }

            // println!(" . {} || {}", unhandeld_left, unhandeld_right);

            let left_index = *seen_nodes.get(&unhandeld_left).unwrap();
            let right_index = *seen_nodes.get(&unhandeld_right).unwrap();

            let left_human = is_humm_or_child_is_humm(
                &tree[left_index], left_index, human_index
            );
        
            let right_human = is_humm_or_child_is_humm(
                &tree[right_index], right_index, human_index
            );
            
            tree.push(Node(NodeBody::new(
                unhandeld_opp,
                left_index,
                right_index,
                left_human,
                right_human
            )));

            node_id = unhandeld_id;
            seen_nodes.insert(node_id, tree.len() - 1);
        }
    }

    if waiting_for.len() > 0 {
        println!("{}", waiting_for.len());
        for (id, _) in waiting_for.into_iter() {
            println!(" - {}", id);
        }
        panic!();
    }

    let root_index = *seen_nodes.get(&308639).unwrap();

    Ok((root_index, human_index, tree))
}


fn is_humm_or_child_is_humm(
    node: &TreeNode, node_index: usize, human_index: usize
) -> bool {
    if node_index == human_index {
        return true;
    }

    match node {
        Leaf(_) => false,
        Node(body) => body.left_child_human || body.right_child_human
    }
}


fn parse_node_description(
    line: &str
) -> Option<(u32, NodeDescription)> {
    let mut chars = line.chars().into_iter().peekable();
    
    let node_id = parse_monkey_id(&mut chars)?;
    inplace_skip(&mut chars, 2)?;

    let body = match parse_num(&mut chars) {
        Some(val) => NodeDescription::Leaf(val),
        None => {
            let left_child_id = parse_monkey_id(&mut chars)?;
            inplace_skip(&mut chars, 1)?;
            let operation = parse_operation(&mut chars)?;
            inplace_skip(&mut chars, 1)?;
            let right_child_id = parse_monkey_id(&mut chars)?;

            NodeDescription::Node(operation, left_child_id, right_child_id)
        }
    };

    Some((node_id, body))
}


fn parse_operation<I>(
    chars: &mut Peekable<I>
) -> Option<Operation> 
where I: Iterator<Item = char> {
    let output = match chars.peek() {
        Some('+') => Some(Add),
        Some('-') => Some(Sub),
        Some('*') => Some(Mul),
        Some('/') => Some(Div),
        _ => None
    };

    if !output.is_none() {
        chars.next();
    }

    output
}


fn parse_monkey_id<I>(
    chars: &mut Peekable<I>
) -> Option<u32> 
where I: Iterator<Item = char> {
    let mut output = None;

    loop {
        match chars.peek() {
            ch @ Some('a'..='z') => {
                output = output.or(Some(0)).map(|v| v * 26 + (
                    *ch.unwrap() as u32 - 'a' as u32
                ));
                chars.next();
            },
            _ => break,
        }
    }

    output
}


fn inplace_skip<I>(
    chars: &mut Peekable<I>, n: u32
) -> Option<()> 
where I: Iterator<Item = char> {
    for _ in 0..n {
        match chars.next() {
            Some(_) => (),
            None => return None,
        }
    }
    Some(())
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