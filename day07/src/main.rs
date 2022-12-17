use std::{
    error::Error,
    fmt::Display,
    ops::{Index, IndexMut},
};

use util::input_lines;

#[derive(Debug)]
struct DirEntryData {
    parent: Option<EntryArenaId>,
    name: String,
    entries: Vec<EntryArenaId>,
}
impl DirEntryData {
    fn find(&self, entries: &EntryArena, name: &str) -> Option<EntryArenaId> {
        self.entries
            .iter()
            .find(|&&e| match &entries[e] {
                Entry::Dir(data) => data.name == name,
                Entry::File(data) => data.name == name,
            })
            .copied()
    }
}

#[derive(Debug)]
struct FileEntryData {
    parent: EntryArenaId,
    name: String,
    size: u64,
}

#[derive(Debug)]
enum Entry {
    Dir(DirEntryData),
    File(FileEntryData),
}

impl Entry {
    fn parent(&self) -> Option<EntryArenaId> {
        match self {
            Entry::Dir(data) => data.parent,
            Entry::File(data) => Some(data.parent),
        }
    }
    fn size(&self, entries: &EntryArena) -> u64 {
        match self {
            Entry::File(data) => data.size,
            Entry::Dir(data) => data
                .entries
                .iter()
                .map(|&entry_id| entries[entry_id].size(entries))
                .sum(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct EntryArenaId(usize);

#[derive(Debug)]
struct EntryArena(Vec<Entry>);

impl EntryArena {
    fn new() -> EntryArena {
        EntryArena(vec![])
    }

    fn next_id(&self) -> usize {
        self.0.len()
    }

    fn new_dir_entry(&mut self, data: DirEntryData) -> EntryArenaId {
        let id = self.next_id();
        self.0.push(Entry::Dir(data));
        EntryArenaId(id)
    }

    fn new_file_entry(&mut self, data: FileEntryData) -> EntryArenaId {
        let id = self.next_id();
        self.0.push(Entry::File(data));
        EntryArenaId(id)
    }
}

impl Index<EntryArenaId> for EntryArena {
    type Output = Entry;

    fn index(&self, index: EntryArenaId) -> &Self::Output {
        &self.0[index.0]
    }
}

impl IndexMut<EntryArenaId> for EntryArena {
    fn index_mut(&mut self, index: EntryArenaId) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Entry::Dir(data) => format!("dir[{}]", &data.name),
            Entry::File(data) => format!("file[{}]", &data.name),
        };
        write!(f, "{}", name)
    }
}

const INPUT_FILE: &str = "input.txt";
fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = input_lines(INPUT_FILE);
    let mut entries = EntryArena::new();

    let root_dir = entries.new_dir_entry(DirEntryData {
        parent: None,
        name: "/".to_string(),
        entries: vec![],
    });

    let mut current_dir = root_dir;

    while let Some(Ok(line)) = lines.next() {
        let line = line.split_whitespace().collect::<Vec<_>>();

        if let "$" = line[0] {
            match &line[1..] {
                ["cd", dir] => {
                    println!("chdir to {dir:?}");
                    match *dir {
                        "/" => current_dir = root_dir,
                        ".." => current_dir = entries[current_dir].parent().unwrap(),
                        _ => {
                            let Entry::Dir(dir_data) = &entries[current_dir] else {
                                Err(format!("cannot cd into file '{dir}'"))?
                            };
                            current_dir = dir_data.find(&entries, dir).ok_or_else(|| {
                                format!("dir '{}' not found in {}", dir, entries[current_dir])
                            })?;
                        }
                    }
                }
                ["ls"] => println!("list dir {current_dir:?}"),
                _ => panic!("unknown command received: {line:?}"),
            }
        } else {
            let entry_id = parse_ls_output(&mut entries, current_dir, &line[..]);
            let entry = &entries[entry_id];
            println!("{entry:?}");
            if let Entry::Dir(data) = &mut entries[current_dir] {
                data.entries.push(entry_id);
            }
        }
    }

    let total_disk_space = 70000000;
    let needed_space = 30000000;
    let root_dir_size = entries[root_dir].size(&entries);
    let free_space = total_disk_space - root_dir_size;
    dbg!(free_space);

    let mut dirs: Vec<_> = entries
        .0
        .iter()
        .filter_map(|e| match e {
            Entry::Dir(data) => {
                let size = e.size(&entries);
                Some((data.name.clone(), size))
            }
            _ => None,
        })
        .collect();
    dirs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    dbg!(&dirs);

    for (dir_name, dir_size) in dirs {
        if (free_space + dir_size) >= needed_space {
            println!("{dir_name}, {dir_size}");
            break;
        }
    }

    Ok(())
}

fn parse_ls_output(
    entry_arena: &mut EntryArena,
    current_dir: EntryArenaId,
    output: &[&str],
) -> EntryArenaId {
    // println!("output | {output:?}");
    match output {
        ["dir", name] => entry_arena.new_dir_entry(DirEntryData {
            parent: Some(current_dir),
            name: name.to_string(),
            entries: vec![],
        }),
        [size, name] => entry_arena.new_file_entry(FileEntryData {
            parent: current_dir,
            name: name.to_string(),
            size: size.parse().unwrap(),
        }),
        _ => panic!("unexpected output"),
    }
}
