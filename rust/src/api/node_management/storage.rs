use std::sync::Mutex;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

lazy_static! {
    pub static ref NODE_LIST: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

/// Save the current list of nodes to a file.
pub fn save_nodes_to_file(filename: &str) -> io::Result<()> {
    let nodes = NODE_LIST.lock().unwrap();
    let mut file = File::create(filename)?;
    for node in nodes.iter() {
        writeln!(file, "{}", node)?;
    }
    Ok(())
}

/// Load nodes from a file into the network.
pub fn load_nodes_from_file(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut nodes = NODE_LIST.lock().unwrap();
    for line in reader.lines() {
        if let Ok(address) = line {
            if !nodes.contains(&address) {
                nodes.push(address);
            }
        }
    }
    Ok(())
}