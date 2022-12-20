use std::fs;
use std::fmt;
use std::cmp;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::Peekable;
use std::collections::HashMap;

use crate::days::day_19::ResourceType::*;

struct Factory {
    resources: Resource,
    blueprints: Vec<RobotBlueprint>,
    robots: HashMap<ResourceType, u16>,
    ore_upper_limit: u16,
    clay_upper_limit: u16,
    obsidian_upper_limit: u16,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Resource {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
}

#[derive(Clone, Copy)]
struct RobotBlueprint {
    produces: ResourceType, 
    cost: Resource,
}


pub fn run(file_name: &str) -> std::io::Result<()> {
    let factories = parse_factories(file_name)?;

    part_one(&factories);
    
    Ok(())
}

fn part_one(factories: &Vec<Factory>) {
    let mut score = 0;

    for (i, factory) in factories.into_iter().enumerate() {
        let mut current_best = 0;
        let factory_best = simulate_factory(
            &mut Factory::from(&factory), 32, &mut current_best, false
        );
        factory.print_small();
        println!("Factory Result: {}", factory_best);
        println!();

        score += (i as u16 + 1) * factory_best;
    }

    println!("Score: {}", score);
}

fn simulate_factory(factory: &mut Factory, minutes_left: u16, current_best: &mut u16, debug: bool) -> u16 {
    if minutes_left == 0 {
        return factory.resources.geode;
    }

    let max_geodes = max_possible(
        minutes_left, factory.resources.geode
    );

    if max_geodes < *current_best {
        return *current_best;
    }

    if debug {
        println!("M: {}", minutes_left);
        factory.print_small();
    }

    let mut best_result = factory.resources.geode + (
        factory.robots.get(&Geode).unwrap() * minutes_left
    );

    for blueprint_i in 0..factory.blueprints.len() {
        let blueprint = factory.blueprints[blueprint_i];
        let time = factory.time_to_build(&blueprint);

        if time.is_none() || time.unwrap() >= minutes_left {
            continue;
        }

        if !factory.should_build_robot(&blueprint) {
            continue;
        }
        
        let time = time.unwrap();

        let added_resource = factory.collect_resources(time + 1);
        let old_count = factory.build_robot(&blueprint);

        let result = simulate_factory(
            factory, minutes_left - time - 1, current_best, debug
        );

        best_result = cmp::max(best_result, result);

        if best_result > *current_best {
            *current_best = best_result;
        }

        factory.resources.add(&blueprint.cost);
        factory.resources.sub(&added_resource);
        factory.robots.insert(blueprint.produces, old_count);
    }

    best_result
}

fn max_possible(minutes_left: u16, number_of_geode_bots: u16) -> u16 {
    (minutes_left * number_of_geode_bots) + (((minutes_left + 1) * minutes_left ) / 2)
}



/* Util */
impl Factory {
    fn new(blueprints: Vec<RobotBlueprint>) -> Factory {
        let mut robots = HashMap::new();
        robots.insert(Ore, 1);
        robots.insert(Clay, 0);
        robots.insert(Obsidian, 0);
        robots.insert(Geode, 0);

        let mut ore_upper_limit = 0;
        let mut clay_upper_limit = 0;
        let mut obsidian_upper_limit = 0;

        for blueprint in &blueprints {
            ore_upper_limit += blueprint.cost.ore;
            clay_upper_limit += blueprint.cost.clay;
            obsidian_upper_limit += blueprint.cost.obsidian;
        }

        Factory {
            resources: Resource::new(0, 0, 0, 0),
            blueprints: blueprints,
            robots: robots,
            ore_upper_limit: ore_upper_limit,
            clay_upper_limit: clay_upper_limit,
            obsidian_upper_limit: obsidian_upper_limit,
        }
    }

    fn from(other: &Factory) -> Factory {
        let mut blueprints = Vec::new();
        for blueprint in &other.blueprints {
            blueprints.push(*blueprint);
        }   
        let mut robots = HashMap::new();
        for (rbt, amt) in &other.robots {
            robots.insert(*rbt, *amt);
        }
        Factory {
            resources: other.resources,
            blueprints: blueprints,
            robots: robots,
            ore_upper_limit: other.ore_upper_limit,
            clay_upper_limit: other.clay_upper_limit,
            obsidian_upper_limit: other.obsidian_upper_limit,
        }
    }

    fn print_small(&self) {
        println!("Factory: ");
        println!("  - Resources: {}", self.resources);
        print!("  - Robots: ");
        for (robot_type, amount) in &self.robots {
            print!("{} x {}, ", robot_type, amount);
        }
        println!();
    }


    fn collect_resources(&mut self, amount: u16) -> Resource {
        let amount_to_add = Resource::new(
            *self.robots.get(&Ore).unwrap() * amount,
            *self.robots.get(&Clay).unwrap() * amount,
            *self.robots.get(&Obsidian).unwrap() * amount,
            *self.robots.get(&Geode).unwrap() * amount
        );

        self.resources.add(&amount_to_add);

        amount_to_add
    }

    fn build_robot(&mut self, robot: &RobotBlueprint) -> u16 {
        let old_count = *self.robots.get(&robot.produces).unwrap();
        self.robots.insert(robot.produces, old_count + 1);
        self.resources.sub(&robot.cost);
        old_count
    }

    fn time_to_build(&self, robot: &RobotBlueprint) -> Option<u16> {
        let cost = &robot.cost;

        Some(cmp::max(
            self.time_to_resource(self.resources.ore, cost.ore, &Ore)?,
            cmp::max(
                self.time_to_resource(self.resources.clay, cost.clay, &Clay)?,
                self.time_to_resource(self.resources.obsidian, cost.obsidian, &Obsidian)?
            )
        ))
    }

    fn should_build_robot(&self, robot: &RobotBlueprint) -> bool {
        let amount = *self.robots.get(&robot.produces).unwrap();

        match robot.produces {
            Ore => amount < self.ore_upper_limit,
            Clay => amount < self.clay_upper_limit,
            Obsidian => amount < self.obsidian_upper_limit,
            Geode => true,

        }
    }


    fn time_to_resource(&self, current_resource: u16, min_resource: u16, resource_type: &ResourceType) -> Option<u16> {
        if min_resource == 0 || current_resource >= min_resource {
            return Some(0);
        }

        let number_of_robots = *self.robots.get(resource_type)?;
        let min_resource = min_resource - current_resource;

        if number_of_robots == 0 {
            return None;
        }

        Some(
            min_resource / number_of_robots + (
                if min_resource % number_of_robots == 0 { 0 } else { 1 }
            )
        )
    }
}

impl Resource {
    fn new(ore: u16,clay: u16,obsidian: u16, geode: u16) -> Resource {
        Resource {
            ore: ore,
            clay: clay,
            obsidian: obsidian,
            geode: geode,
        }
    }

    fn add(&mut self, other: &Resource) {
        self.ore += other.ore;
        self.clay += other.clay;
        self.obsidian += other.obsidian;
        self.geode += other.geode;
    }

    fn sub(&mut self, other: &Resource) {
        self.ore -= other.ore;
        self.clay -= other.clay;
        self.obsidian -= other.obsidian;
        self.geode -= other.geode;
    }
}

impl RobotBlueprint {
    fn new(produces: ResourceType, cost: Resource) -> RobotBlueprint {
        RobotBlueprint {
            produces: produces,
            cost: cost,
        }
    }
}


impl fmt::Display for ResourceType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", 
            match self {
                Ore => "Ore",
                Clay => "Clay",
                Obsidian => "Obsidian",
                Geode => "Geode",
            }
        )
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "(Or: {}, Cl: {}, Ob: {}, Ge: {})", 
            self.ore, self.clay, self.obsidian, self.geode
        )
    }
}


impl fmt::Display for RobotBlueprint {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Cost: {} -> Output: {}", 
            self.cost, self.produces
        )
    }
}



/* Parsing */
fn parse_factories(file_name: &str) -> std::io::Result<Vec<Factory>> {
    let file = fs::File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut factories = Vec::new();

    for line in reader.lines() {
        let blueprints = parse_robot_blueprints(&line?).unwrap();

        factories.push(
            Factory::new(blueprints)
        );
    }

    Ok(factories)
}


fn parse_robot_blueprints(line: &str) -> Option<Vec<RobotBlueprint>> {
    let mut chars = line.chars().into_iter().peekable();

    advance_to_first_char_after(&mut chars, ':')?;
    inplace_skip(&mut chars, 22)?;
    let ore_robot_ore_cost = parse_num(&mut chars)?;
    inplace_skip(&mut chars, 28)?;
    let clay_robot_ore_cost = parse_num(&mut chars)?;
    inplace_skip(&mut chars, 32)?;
    let obsidian_robot_ore_cost = parse_num(&mut chars)?;
    inplace_skip(&mut chars, 9)?;
    let obsidian_robot_clay_cost = parse_num(&mut chars)?;
    inplace_skip(&mut chars, 30)?;
    let geode_robot_ore_cost = parse_num(&mut chars)?;
    inplace_skip(&mut chars, 9)?;
    let geode_robot_obsidian_cost = parse_num(&mut chars)?;

    Some(vec![
        RobotBlueprint::new(Geode, Resource::new(
            geode_robot_ore_cost, 0, geode_robot_obsidian_cost, 0
        )),
        RobotBlueprint::new(Obsidian, Resource::new(
            obsidian_robot_ore_cost, obsidian_robot_clay_cost, 0, 0 
        )),
        RobotBlueprint::new(Clay, Resource::new(
            clay_robot_ore_cost, 0, 0, 0
        )),
        RobotBlueprint::new(Ore, Resource::new(
            ore_robot_ore_cost, 0, 0, 0
        ))
    ])
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


fn advance_to_first_char_after<I>(
    chars: &mut Peekable<I>, target: char
) -> Option<()> 
where I: Iterator<Item = char> {
    loop {
        match chars.peek() {
            Some(ch) if *ch == target => {
                chars.next();
                break;
            },
            Some(_) => { chars.next(); },
            None => return None,
        }
    }

    Some(())
}


fn parse_num<I>(chars: &mut Peekable<I>) -> Option<u16> 
where I: Iterator<Item = char> {
    let mut output = None;
    let mut output_val = 0;

    loop {
        match chars.peek() {
            ch @ Some('0'..='9') => {
                output_val = output_val * 10 + (
                    *ch.unwrap() as u16 - '0' as u16
                );
                output = Some(output_val);
                chars.next();
            },
            _ => break,
        };
    }

    output
}

