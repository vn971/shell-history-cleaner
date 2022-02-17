# About
Shell History Cleaner is a program that cleans up the shell history for you.

Shell history is the commands that you see when pressing `Up` or `Ctrl R` (to search) in the terminal.

# Use
```sh
cargo install --force shell-history-cleaner
shell-history-cleaner --dedup 1  # de-duplicates your shell history
```
For more examples and options, see below

# Example
Suppose you have a huge HISTFILE, such as:
```sh
...
git status
git checkout -b branchname
...
git status
yt-dlp 'https://youtube.com/video....'
yt-dlp 'https://youtube.com/video2....'
yt-dlp 'https://youtube.com/video3....'
...
git status
```

You want to clean it, to make navigation with `Ctrl R` easier:

* Remove all `yt-dlp` commands (the youtube video downloader)
* Keep a maximum of 1000 last `git checkout` commands
* De-duplicate entries to only keep the one last occurrence of each dup

To do that, run:
```sh
shell_history_cleaner --dedup=1 --ignore='yt-dlp .*' --limit=1000:'curl .*'
```

# Manual
```txt
TODO
```

# Useful references
* Make your bash history unlimited: https://superuser.com/a/664061/162466

# Implementation details
This program loads the whole contents of the file in memory. Don't use this script if your history file is 1Gb (hope not though :shrug:)

# License
GPLv3 or, at your option, any later version of the license.
