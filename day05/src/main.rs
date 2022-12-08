use std::{
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
    io::{self, Read},
};

use util::input_lines;

const INPUT_FILE: &str = "input.txt";

type Stack = VecDeque<Crate>;

#[derive(Debug)]
struct Crate {
    name: char,
}

#[derive(Debug)]
struct MoveCommand {
    count: usize,
    from: usize,
    to: usize,
}
impl MoveCommand {
    fn new(count: &str, from: &str, to: &str) -> MoveCommand {
        MoveCommand {
            count: count.parse().unwrap(),
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
        }
    }
}

fn main() -> io::Result<()> {
    let mut stacks: BTreeMap<usize, RefCell<Stack>> = BTreeMap::new();
    // let mut stacks: VecDeque<RefCell<Stack>> = VecDeque::new();

    let mut commands = Vec::new();

    for line in input_lines(INPUT_FILE) {
        let line = line?;
        if line.starts_with(' ') || line.starts_with('[') {
            let ret = parse_box_line(&line);
            for (i, mycrate) in ret.into_iter().enumerate() {
                if let Some(mycrate) = mycrate {
                    let stack_number = i + 1;
                    let e = stacks.entry(stack_number).or_default();
                    e.get_mut().push_front(mycrate);
                }
            }
        } else {
            if let Some(command) = parse_command_line(&line) {
                commands.push(command);
            }
        }
    }
    print_stacks(&stacks);

    println!("{:?}", commands);

    let mut crane_mover_9001 = Vec::new();

    for command in commands {
        {
            let mut from_stack = stacks.get(&command.from).unwrap().borrow_mut();
            let mut to_stack = stacks.get(&command.to).unwrap().borrow_mut();

            for _ in 0..command.count {
                crane_mover_9001.push(from_stack.pop_back().unwrap());
            }
            for _ in 0..crane_mover_9001.len() {
                to_stack.push_back(crane_mover_9001.pop().unwrap())
            }
        }
        print_stacks(&stacks);
    }

    println!("{:?}", stacks);

    for (_, top) in stacks {
        print!("{}", top.borrow().back().unwrap().name);
    }
    println!();
    Ok(())
}

fn parse_box_line(line: &str) -> Vec<Option<Crate>> {
    // let mut crate_map = HashMap::new();
    let mut reader = line.as_bytes();
    let mut buf = [0; 4];
    let mut stack_count = 0;
    let mut crates = Vec::new();

    while reader.read(&mut buf).unwrap() != 0 {
        let buf = buf.map(|n| n as char);
        match buf {
            [' ', ' ', ' ', ' '] => crates.push(None),
            ['[', name, ']', ' '] => crates.push(Some(Crate { name })),
            [' ', number, ' ', ' '] => {
                let number = number.to_digit(10).unwrap();
                if number > stack_count {
                    stack_count = number;
                }
            }
            _ => (),
        };
        // print!("{:?}", buf);
    }
    // println!("{:?}", &crates);
    crates
}

fn parse_command_line(line: &str) -> Option<MoveCommand> {
    let line = line.split(' ').collect::<Vec<_>>();
    match line[..] {
        ["move", count, "from", from, "to", to] => Some(MoveCommand::new(count, from, to)),
        _ => None,
    }
    // println!("{:?}", line);
}

fn print_stacks(stacks: &BTreeMap<usize, RefCell<Stack>>) {
    println!();
    for (key, value) in stacks {
        print!("[{key}]: ");
        for mycrate in value.borrow().iter() {
            print!("[{}]", mycrate.name);
        }
        println!();
    }
    println!();
}
