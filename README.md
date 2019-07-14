portal
---

A fast directory jumper using short text queries. A rust rewrite of z (https://github.com/rupa/z) with supports ZSH, Bash, and Elvish.

*WARNING* Currently under heavy development and only works by importing an existing `.z` database. Most of below is the plan not the current state.

See [TODO](https://github.com/dmix/portal/blob/master/TODO.md) for latest progress

## Features

- Jump to a directory you most recently visited containing a keyword
- Ranks your history (ie, .zsh_history) of all the directories by # of visits and recency
- Alternatively list recent directories and select one manually
- Stored using [tantivy](https://github.com/tantivy-search/tantivy) allowing superfast full-text queries
- Manually tag directories

## Usage

### Supported shells

- [Elvish](https://github.com/elves/elvish)
- ZSH
- Bash (coming soon)
- Fish (coming soon)
- Others? (make an issue)

### Binary Installation

This is a work in progress. Homebrew, Archlinux, Debian, and Fedora packages coming soon.

Mac OS:

    cargo build
    cp target/debug/portal /usr/local/bin/portal

### Shell Integration

ZSH and Bash, add to `.bashrc` or `.zshrc`, you can use any shortcut name, I use `p`:

    function pp() {
        cd `portal $1`
    }

Elvish:

    fn p [dir]{
      cd (portal $dir)
    }

### Usage

Jump to a directory (keyword matches)

    p styles
    > cd ~/dev/_ruby/callpixels/vendor/assets/stylesheets

    p blog
    > cd ~/dev/_rust/blog

    p bin
    > cd /usr/local/bin

Tag a directory (tags tag predecdence over keyword matches)

    cd ~/long/directory/path/
    p tag work
    > Portal tagged directory with `work`: ~/long/directory/path/
    cd ~
    p work
    > cd ~/long/directory/path/

Manually list and select directory

   p list
   > .. lists directories, use j/k or arrow keys to navigate

## About

### Prior Art

Heavily inspired by `z` https://github.com/rupa/z and the rust rewrite of ls `exa` https://github.com/ogham/exa

### Why not use z?

I recently switch my terminal from ZSH to Elvish and since I used `z` daily I
noticed there was no simply binary version that could be used by any terminal.
So I wanted to build one that is decoupled from the shell but is easily
integrated with a few lines in a plugin form but also:

- Faster
- Full-text search and listing
- Support for tags

### Primary Author

Daniel McGrady
https://twitter.com/dmix

### License

MIT, contributions welcome!
