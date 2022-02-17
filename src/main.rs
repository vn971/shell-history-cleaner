#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::time::SystemTime;

const DEFAULT_REGEXP: &str = "^(dict.*|sdcv .*| .*|git checkout .*|git branch .*|ps aux.*|youtube-dl .*|yt-dlp .*|chmod.*|echo.*|man .*)$";
const DEFAULT_MAXIMUM_OCCURRENCES: u32 = 3;

fn print_usage() {
    eprintln!("This program mutates a file to remove duplicate lines. Example usage:");
    eprintln!("    $0 \"$HISTFILE\"");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let target_file_name = match args.as_slice() {
        [_, t] => t.to_string(),
        _ => {
            eprintln!("ERROR: expected exactly one command-line argument.");
            print_usage();
            std::process::exit(1)
        }
    };
    let target_file = Path::new(&target_file_name);
    let ignore_regex = std::env::var("ignore_regexp")
        .ok()
        .unwrap_or_else(|| DEFAULT_REGEXP.to_string());
    let ignore_regex: Regex = Regex::new(&ignore_regex).unwrap();

    let maximum_occurrences = std::env::var("maximum_occurrences")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or_else(|| DEFAULT_MAXIMUM_OCCURRENCES);

    let file_as_string = std::fs::read_to_string(&target_file)
        .unwrap_or_else(|err| panic!("Failed to read file, {}", err));

    let mut map: HashMap<String, u32> = HashMap::new();
    let mut output: Vec<String> = Vec::new();
    let mut lines_ignored: u32 = 0;

    for line in file_as_string.lines().rev() {
        let (line_copy, value) = if let Some(old_key_value) = map.remove_entry(line) {
            old_key_value
        } else {
            (line.to_string(), 0)
        };
        if value < maximum_occurrences && !ignore_regex.is_match(line) {
            output.push(line.to_string());
        } else {
            lines_ignored += 1;
        }
        map.insert(line_copy, value + 1);
    }

    // We reverse the file contents twice to keep the _newest_ entries
    // when de-duplicating, not the oldest ones.
    output.reverse();

    // create the new output file. This is not an atomic operation in the filesystem.
    let output_file = format!("{}.swap", target_file_name);
    std::fs::write(&output_file, output.join("\n"))
        .unwrap_or_else(|err| panic!("Error writing output, {}", err));

    // move the old file to a backup name
    let unix_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|err| panic!("SystemTime before EPOCH, {}", err));
    let backup_file = format!("{}-{}.bak", target_file_name, unix_time.as_secs());
    std::fs::rename(&target_file, &backup_file)
        .unwrap_or_else(|err| panic!("Error backing up {}, {}", target_file_name, err));

    // move the new output file to the target destination
    std::fs::rename(&output_file, &target_file).unwrap_or_else(|err| {
        panic!(
            "Error moving (renaming) {} to {}, {}",
            backup_file, target_file_name, err
        )
    });
    eprintln!("Success! Removed {} lines and kept {}", lines_ignored, output.len());
    eprintln!("Original file backed up to {}", backup_file);
}
