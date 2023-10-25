# About
Shell History Cleaner is a simple program that cleans the bash/shell history for you.

Shell history is the commands that you see when pressing `Up` or `Ctrl R` (to search) in the terminal.


# Use
Install Rust/cargo. After that:
```sh
cargo install --force shell-history-cleaner

shell-history-cleaner --dedup "$HISTFILE"  # remove duplicates
shell-history-cleaner --dedup --remove 'youtube-dl .*' "$HISTFILE"  # also remove video downloads
```


# Help
```
USAGE:
    shell-history-cleaner [OPTIONS] <TARGET_FILE>

ARGS:
    <TARGET_FILE>
            Target file to clean. You can use "$HISTFILE" to clean up the shell history.

OPTIONS:
    -d, --dedup
            De-duplicate lines to only keep one last occurrence of each dup. In contrast to bash
            built-in deduplication, this also works if the duplicates are sparse and do not
            immediately follow each other.

    -r, --remove <REMOVE>
            Lines to remove. For example, 'yt-dlp.*' will remove lines starting with 'yt-dlp'.
            Can be specified multiple times.
            
            The patterns are regular expressions, assuming the whole line is matched, as defined
            here: https://docs.rs/regex/latest/regex/#syntax
            
            Another real-life example:
            '(ps aux.*|git checkout .*|git branch .*| .*|yt-dlp .*|chmod .*|echo .*|man .*|rg .*)'

    -h, --help
            Print help information
```


# Useful references
Make your bash history unlimited: [https://superuser.com/a/664061/162466](https://superuser.com/a/664061/162466)


# Implementation details
This program loads the file contents in memory. Don't use this script if your history file is 1Gb (hope it's not though:)


# License
GPLv3 or, at your option, any later version of the license.  
Main GitLab repo: [https://gitlab.com/vn971/shell-history-cleaner](https://gitlab.com/vn971/shell-history-cleaner)  
Github mirror: [https://github.com/vn971/shell-history-cleaner](https://github.com/vn971/shell-history-cleaner)
