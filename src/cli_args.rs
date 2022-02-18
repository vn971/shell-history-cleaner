use clap::AppSettings;
use clap::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(
    rename_all = "kebab-case",
    setting = AppSettings::DeriveDisplayOrder,
    next_line_help = true,
)]
pub struct CliArgs {
    #[structopt(
        short = 'd',
        long = "dedup",
        help = "De-duplicate lines to only keep one last occurrence of each dup. \
            In contrast to bash built-in deduplication, this also works if the duplicates \
            are sparse and do not immediately follow each other."
    )]
    pub dedup: bool,

    #[structopt(
        short = 'r',
        long = "remove",
        help = "Lines to remove. For example, 'yt-dlp.*' will remove lines starting with 'yt-dlp'.{n}\
            Can be specified multiple times.{n}{n}\
            The patterns are regular expressions, assuming the whole line is matched, \
            as defined here: https://docs.rs/regex/latest/regex/#syntax {n}{n}\
            Another real-life example: {n}\
            '(ps aux.*|git checkout .*|git branch .*| .*|yt-dlp .*|chmod .*|echo .*|man .*)'",
        multiple_occurrences = true
    )]
    pub remove: Vec<String>,

    //     #[structopt(
    //         long = "limit",
    //         help = "Line patterns to limit. For example, '1234:git checkout.*' will limit the preserved 'git checkout' commands to the last 1234 occurrences.
    // NOT SUPPORTED YET",
    //         multiple_occurrences = true
    //     )]
    //     pub limit: Vec<String>,

    // Positional arguments:
    #[structopt(
        help = "Target file to clean. You can use \"$HISTFILE\" to clean up the shell history.",
        required = true
    )]
    pub target_file: PathBuf,
}
