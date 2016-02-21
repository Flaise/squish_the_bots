// A simple example bot that goes straight for enemy bots.
// The only defensive behavior it has is to avoid throwing itself off a ledge.

use std::collections::HashMap;
use std::fmt;
use std::io::{self, Read, Write};
use std::mem::transmute;
use std::net::*;
use std::thread;
use self::Direction::*;


#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn all() -> Vec<Direction> {
        vec![North, East, South, West]
    }
    
    fn to_delta(self) -> (i8, i8) {
        match self {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),
        }
    }
    
    fn serialize(self) -> u8 {
        match self {
            North => 0,
            East => 1,
            South => 2,
            West => 3,
        }   
    }
}

#[derive(Copy, Clone)]
enum Action {
    LookAt(i8, i8),
    Move(Direction),
    Drill(Direction),
}
impl Action {
    fn serialize(self) -> Vec<u8> {
        match self {
            Action::LookAt(dx, dy) => unsafe {vec![1, transmute(dx), transmute(dy)]},
            Action::Move(direction) => vec![2, direction.serialize()],
            Action::Drill(direction) => vec![3, direction.serialize()],
        }
    }
}
impl fmt::Display for Action {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Action::LookAt(dx, dy) => write!(formatter, "Looking at [{}, {}].", dx, dy),
            Action::Move(direction) => write!(formatter, "Moving {:?}.", direction),
            Action::Drill(direction) => write!(formatter, "Drilling {:?}.", direction),
        }
    }
}


#[derive(Debug)]
enum Thing {
    Abyss,
    Bot,
    Block,
    Floor,
}
fn code_to_thing(code: u8) -> Option<Thing> {
    match code {
        0 => Some(Thing::Floor),
        1 => Some(Thing::Bot),
        2 => Some(Thing::Block),
        3 => Some(Thing::Abyss),
        _ => None,
    }
}

#[derive(Copy, Clone)]
enum Sighting {
    Abyss,
    Floor {
        last_spotted: u32,
    },
    Block {
        last_spotted: u32,
        heavy: bool,
    },
    Bot {
        last_spotted: u32,
    },
}
impl Sighting {
    fn new(thing: Thing, now: u32) -> Sighting {
        match thing {
            Thing::Abyss => Sighting::Abyss,
            Thing::Bot => Sighting::Bot {
                last_spotted: now,
            },
            Thing::Floor => Sighting::Floor {
                last_spotted: now,
            },
            Thing::Block => Sighting::Block {
                last_spotted: now,
                heavy: false,
            },
        }
    }
    
    fn heavy(now: u32) -> Sighting {
        Sighting::Block {
            last_spotted: now,
            heavy: true,
        }
    }
}


struct Model {
    here: (i32, i32),
    sightings: HashMap<(i32, i32), Sighting>,
    last_action: Option<Action>,
    time: u32,
}
impl Model {
    fn new() -> Model {
        let mut result = Model {
            here: (0, 0),
            sightings: HashMap::new(),
            last_action: None,
            time: 0,
        };
        result.see_here();
        result
    }
    
    fn see_here(&mut self) {
        self.see(0, 0, Thing::Floor);
    }
    
    fn tick(&mut self) {
        self.time += 1;
        self.see_here();
    }
    
    fn tick_count(&mut self, ticks: u32) {
        self.time += ticks;
        self.see_here();
    }
    
    fn direction_to_absolute(&self, direction: Direction) -> (i32, i32) {
        let (dx, dy) = direction.to_delta();
        self.relative_to_absolute(dx, dy)
    }
    
    fn relative_to_absolute(&self, dx: i8, dy: i8) -> (i32, i32) {
        let (x, y) = self.here;
        (x + dx as i32, y + dy as i32)
    }
    
    fn see(&mut self, dx: i8, dy: i8, thing: Thing) {
        let there = self.relative_to_absolute(dx, dy);
        self.sightings.insert(there, Sighting::new(thing, self.time));
    }
    
    fn at_relative(&self, dx: i8, dy: i8) -> Option<Sighting> {
        let there = self.relative_to_absolute(dx, dy);
        self.sightings.get(&there).map(Clone::clone)
    }
    
    fn any_unknown_relative(&self) -> (i8, i8) {
        let mut search_radius = 1;
        
        loop {
            for y in -search_radius..search_radius + 1 {
                for x in -search_radius..search_radius + 1 {
                    if x == 0 && y == 0 {
                        continue;
                    }
                    match self.at_relative(x, y) {
                        None => return (x, y),
                        Some(Sighting::Abyss) => continue,
                        
                        Some(Sighting::Floor { last_spotted }) |
                        Some(Sighting::Block { last_spotted, .. }) |
                        Some(Sighting::Bot { last_spotted }) => {
                            if last_spotted + 20 < self.time {
                                return (x, y)
                            }
                        }
                    }
                }
            }
            search_radius += 1;
        }
    }
    
    fn look_at_any_unknown(&self) -> Action {
        let (dx, dy) = self.any_unknown_relative();
        Action::LookAt(dx, dy)
    }
    
    fn search(&self) -> Action {
        let mut options = self.wander_actions();
        let scan = self.look_at_any_unknown();
        options.push(scan);
        options.push(scan);
        options.push(scan);
        
        let random = self.random_seed();
        
        options[random as usize % options.len()]
    }
    
    fn random_seed(&self) -> u32 {
        for sighting in self.sightings.values() {
            match *sighting {
                Sighting::Abyss => continue,
                Sighting::Floor { last_spotted, .. } => return self.time - last_spotted,
                Sighting::Bot { last_spotted, .. } => return self.time - last_spotted,
                Sighting::Block { last_spotted, .. } => return self.time - last_spotted,
            }
        }
        self.time
    }
    
    fn wander_actions(&self) -> Vec<Action> {
        let mut result = vec![];
        
        for direction in Direction::all() {
            let (dx, dy) = direction.to_delta();
            match self.at_relative(dx, dy) {
                None => result.push(Action::LookAt(dx, dy)),
                Some(Sighting::Abyss) => continue,
                Some(Sighting::Floor {..}) => result.push(Action::Move(direction)),
                Some(Sighting::Bot {..}) => return vec![Action::Drill(direction)],
                Some(Sighting::Block { heavy: true, .. }) => result.push(Action::Drill(direction)),
                Some(Sighting::Block { heavy: false, .. }) => result.push(Action::Move(direction)),
            }
        }
        
        result
    }
    
    fn success(&mut self) {
        match self.last_action {
            Some(Action::Move(direction)) => self.moved(direction, 3),
            Some(Action::Drill(direction)) => self.moved(direction, 5),
            _ => panic!(),
        }
    }
    
    fn moved(&mut self, direction: Direction, ticks: u32) {
        let (dx, dy) = direction.to_delta();
        self.here.0 += dx as i32;
        self.here.1 += dy as i32;
        self.tick_count(ticks);
    }
    
    fn too_heavy(&mut self) {
        match self.last_action {
            Some(Action::Move(direction)) => {
                let there = self.direction_to_absolute(direction);
                self.sightings.insert(there, Sighting::heavy(self.time));
            },
            _ => panic!(),
        }
    }
}


pub fn start<A: ToSocketAddrs+Send+'static>(address: A, name: String, logs: bool)
        -> io::Result<thread::JoinHandle<()>> {
    let mut model = Model::new();
    
    let builder = thread::Builder::new().name(name.clone());
    
    let log = move|message: &str| {
        if logs {
            println!("{}: {}", name, message);
        }
    };
    
    builder.spawn(move|| {
        let mut stream = TcpStream::connect(address).unwrap();
        
        log("Connected.");
        
        loop {
            let mut buf = [0; 1];
            if stream.read(&mut buf).unwrap() != 1 {
                panic!();
            }
            match buf[0] {
                1 => {
                    log(&*format!("It's my turn. I am at [{}, {}].", model.here.0, model.here.1));
                    
                    let action = model.search();
                    log(&*format!("{}", action));
                    stream.write_all(&action.serialize()).unwrap();
                    model.last_action = Some(action);
                    model.tick();
                }
                3 => {
                    log("Successful.");
                    model.success();
                }
                4 => {
                    log("It was too heavy to push.");
                    model.too_heavy();
                }
                5 => {
                    log("A new round is starting.");
                    
                    model = Model::new();
                }
                6 => {
                    if stream.read(&mut buf).unwrap() != 1 {
                        panic!();
                    }
                    
                    match model.last_action {
                        Some(Action::LookAt(dx, dy)) => {
                            let thing = code_to_thing(buf[0]);
                            if thing.is_none() {
                                log(&*format!("I don't know what that is. ({})", buf[0]));
                                panic!();
                            }
                            let thing = thing.unwrap();
                            
                            log(&*format!("I see a(n) {:?}.", thing));
                            
                            model.see(dx, dy, thing);
                        }
                        _ => {
                            log("I didn't look at anything.");
                            panic!();
                        }
                    }
                    
                }
                code @ _ => {
                    log(&*format!("I don't know what the server meant by that. ({})", code));
                    panic!();
                }
            }
        }
    })
}
