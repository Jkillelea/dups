extern crate sha256;

use std::io::{self, prelude::*, BufReader};
use std::fs;
use std::path;
use std::env;
use std::collections::HashMap;
use std::path::PathBuf;

/// Compute the Sha256 for every file in a directory. Returns a map from Sha256 digest to
/// a list of paths
fn scan_dir_recursive(path: &path::Path) -> io::Result<HashMap<String, Vec<PathBuf>>> {
    let mut map: HashMap<String, Vec<PathBuf>> = HashMap::new();

    if let Ok(mut readdir) = fs::read_dir(path) {
        while let Some(Ok(entry)) = readdir.next() {

            if let Ok(meta) = entry.metadata() {

                if meta.is_file() {
                    // Compute sha256 of file
                    let mut f = BufReader::new(fs::File::open(entry.path())?);
                    let mut buffer = Vec::with_capacity(meta.len() as usize);
                    f.read_to_end(&mut buffer)?;

                    let sha = sha256::digest(buffer.as_slice());

                    // add to map
                    if let Some(list) = map.get_mut(&sha) {
                        list.push(entry.path());
                    } else {
                        map.insert(sha, vec![entry.path()]);
                    }
                } else if meta.is_dir() {
                    // recurse
                    map.extend(scan_dir_recursive(&entry.path())?);
                } else if meta.is_symlink() {
                    // nothing
                } else {
                    eprintln!("Unknown file type {:?}!", meta.file_type());
                }
            }
        }
    }

    Ok(map)
}

fn main() -> io::Result<()> {
    for arg in env::args().skip(1) {
        scan_dir_recursive(path::Path::new(&arg))?
            .iter()
            .filter(|(_, v)| v.len() > 1)
            .for_each(|(_, v)| {
                println!("{}:", v.len());
                for ele in v.iter() {
                    println!("\t{}", ele.to_str().unwrap_or(""));
                }
            });
    }

    Ok(())
}
