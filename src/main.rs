#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

mod cli_args;

use crate::cli_args::CliArgs;
use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

fn main() {
    let cli_args: CliArgs = Parser::parse();
    let target_file: PathBuf = cli_args.target_file.clone();
    let target_file_name = target_file.to_str().unwrap_or_else(|| {
        panic!(
            "Error converting target file path to UTF-8: {:?}",
            target_file
        )
    });

    let mut remove_regexes = Vec::new();
    for remove_string in &cli_args.remove {
        match Regex::new(&format!("^{}$", remove_string)) {
            Err(err) => panic!(
                "Failed to interpret --remove pattern {} as a regular expression, {}",
                remove_string, err
            ),
            Ok(re) => remove_regexes.push(re),
        }
    }

    let file_as_string = std::fs::read_to_string(&target_file)
        .unwrap_or_else(|err| panic!("Failed to read target file {}, {}", target_file_name, err));

    let mut map: HashMap<String, u32> = HashMap::new();
    let mut output: Vec<String> = Vec::new();
    let mut lines_removed: u32 = 0;
    let mut lines_dedupplicated: u32 = 0;

    // We iterate the lines of the file in reverse order to keep the _newest_ entries
    // when de-duplicating, not the oldest ones.
    // See also below.
    for line in file_as_string.lines().rev() {
        let (line_copy, value) = if let Some(old_key_value) = map.remove_entry(line) {
            old_key_value
        } else {
            (line.to_string(), 0)
        };
        if cli_args.dedup && value >= 1 {
            lines_dedupplicated += 1;
        } else if remove_regexes.iter().any(|r| r.is_match(line)) {
            lines_removed += 1;
        } else {
            output.push(line.to_string());
        }
        map.insert(line_copy, value + 1);
    }

    // Reverse the order of the lines back to the original (see also above).
    output.reverse();

    // create the new output file. This is not an atomic operation in the filesystem.
    let output_file = format!("{}.swap", target_file_name);
    std::fs::write(&output_file, output.join("\n"))
        .unwrap_or_else(|err| panic!("Error writing output, {}", err));
    std::fs::write(&output_file, "\n")
        .unwrap_or_else(|err| panic!("Error writing the final newline, {}", err));

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
    eprintln!(
        "Success! Deduplicated {} lines, removed {} and kept {}",
        lines_dedupplicated,
        lines_removed,
        output.len()
    );
    eprintln!("Original file backed up to {}", backup_file);
}
