# safe-rm (Safe Rm)

[![safe-rm](https://img.shields.io/crates/v/safe-rm.svg)](https://crates.io/crates/safe-rm)
[![travis-ci](https://travis-ci.com/alongwy/safe-rm.svg?branch=master)](https://travis-ci.com/alongwy/safe-rm)

`safe-rm` is a command-line deletion tool focused on safety, ergonomics, and performance.  It favors a simple interface, and does /not/ implement the xdg-trash spec or attempt to achieve the same goals.

Deleted files get sent to the graveyard (`/tmp/graveyard-$USER` by default, see [notes](https://github.com/alongwy/safe-rm#-notes) on changing this) under their absolute path, giving you a chance to recover them.  No data is overwritten.  If files that share the same path are deleted, they will be renamed as numbered backups.

`safe-rm` is made for lazy people.  If any part of the interface could be more intuitive, please open an issue or pull request.

## ⚰ Installation
Or get a binary [release](https://github.com/alongwy/safe-rm/releases) (Linux x86_64, ARMv7 and macOS), untar it, and move it somewhere on your $PATH:
```bash
$ tar xvzf safe-rm-*.tar.gz
$ mv safe-rm /usr/local/bin
```

or build it:
```bash
$ cargo install safe-rm
```

## ⚰ Usage
```
USAGE:
    safe-rm [FLAGS] [OPTIONS] [TARGET]...

FLAGS:
    -d, --decompose    Permanently deletes (unlink) the entire graveyard
    -h, --help         Prints help information
    -i, --inspect      Prints some info about TARGET before prompting for action
    -s, --seance       Prints files that were sent under the current directory
    -V, --version      Prints version information

OPTIONS:
        --graveyard <graveyard>    Directory where deleted files go to rest
    -u, --unbury <target>       Undo the last removal by the current user, or specify some file(s) in the graveyard.  Combine with -s to restore everything printed by -s.

ARGS:
    <TARGET>...    File or directory to remove
```
Basic usage -- easier than rm
```bash
$ safe-rm dir1/ file1
```
Undo the last deletion
```bash
$ safe-rm -u
# Returned /tmp/graveyard-jack/home/jack/file1 to /home/jack/file1
```

Print some info (size and first few lines in a file, total size and first few files in a directory) about the target and then prompt for deletion
```bash
$ safe-rm -i file1
dir1: file, 1337 bytes including:
> Position: Shooting Guard and Small Forward ▪ Shoots: Right
> 6-6, 185lb (198cm, 83kg)
Send file1 to the graveyard? (y/n) y
```
Print files that were deleted from under the current directory
```bash
$ safe-rm -s
/tmp/graveyard-jack/home/jack/file1
/tmp/graveyard-jack/home/jack/dir1
```
Name conflicts are resolved
```bash
$ touch file1
$ safe-rm file1
$ safe-rm -s
/tmp/graveyard-jack/home/jack/dir1
/tmp/graveyard-jack/home/jack/file1
/tmp/graveyard-jack/home/jack/file1~1
```
-u also takes the path of a file in the graveyard
```bash
$ safe-rm -u /tmp/graveyard-jack/home/jack/file1
Returned /tmp/graveyard-jack/home/jack/file1 to /home/jack/file1
```
Combine -u and -s to restore everything printed by -s
```bash
$ safe-rm -su
Returned /tmp/graveyard-jack/home/jack/dir1 to /home/jack/dir1
Returned /tmp/graveyard-jack/home/jack/file1~1 to /home/jack/file1~1
```
## ⚰ Notes
- You probably shouldn't alias `rm` to `safe-rm`.  Unlearning muscle memory is hard, but it's harder to ensure that every `rm` you make (as different users, from different machines and application environments) is the aliased one.
- If you have `$XDG_DATA_HOME` environment variable set, `safe-rm` will use `$XDG_DATA_HOME/graveyard` instead of the `/tmp/graveyard-$USER`.
- If you want to put the graveyard somewhere else (like `~/.local/share/Trash`), you have two options, in order of precedence:
  1. Alias `safe-rm` to `safe-rm --graveyard ~/.local/share/Trash`
  2. Set the environment variable `$GRAVEYARD` to `~/.local/share/Trash`.
  This can be a good idea because if the graveyard is mounted on an in-memory filesystem (as /tmp is in Arch Linux), deleting large files can quickly fill up your RAM.  It's also much slower to move files across filesystems, although the delay should be minimal with an SSD.
- In general, a deletion followed by a `--unbury` should be idempotent.
- The deletion log is kept in `.record`, found in the top level of the graveyard.

## ⚰ Todo List

- [ ] Add auto suggestions generate
- [ ] Add protect files config

## ⚰ Credits
Special thanks to:
+ [nivekuil](https://github.com/nivekuil/rip) for his source code.