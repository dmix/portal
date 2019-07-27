🖲️ portal
---

A fast directory jumper using short text queries. A rust rewrite of z (https://github.com/rupa/z) with supports ZSH, Bash, and Elvish.

**WARNING** Currently under heavy development. WHAT WORKS: Basic tracking on every `cd` and jumping to the directory is working, with basic ZSH and Elvish plugins (see below). WHAT'S NEXT: More work needs to be done for tagging directories, searching directory history with fzf, Bash support, and more. See [TODO](https://github.com/dmix/portal/blob/master/TODO.md) for latest progress

## Features
--- 

- Jump to a directory you most recently visited containing a keyword
- Ranks your history (ie, .zsh_history) of all the directories by # of visits and recency
- Stored using [tantivy](https://github.com/tantivy-search/tantivy) allowing superfast full-text queries
- List recent directories or search entire history, with fzf integration to filter/select one manually (TODO)
- Manually tag directories (TODO)

### Supported shells

- [Elvish](https://github.com/elves/elvish)
- ZSH
- Bash (coming soon)
- Fish (coming soon)
- Others? (make an issue)

### Binary Installation

This is a work in progress. Homebrew, Archlinux, Debian, and Fedora packages coming soon.

Mac OS:

    git clone https://github.com/dmix/portal/
    cd portal
    make install
    
This will install `portal` to your /usr/local/bin

### Shell Integration

--- 

#### ZSH

**ZSH Install**

Using [zplug](https://github.com/zplug/zplug):

    zplug "dmix/portal", use: "portal.pluginz.zsh"

Using [antigen](https://github.com/zsh-users/antigen)

    antigen bundle zsh-users/zsh-syntax-highlighting

Using [zgen](https://github.com/tarjoilija/zgen)

    zgen load dmix/portal

Manually

Download `plugins/portal.plugin.zsh` and add to `.zshrc`

    source portal.plugin.zsh

**ZSH Usage**

Normally I use `p` as the command but that is used by zsh for `print` so the default is z:

    z <directory name>

Which is a shortcut for:

    portal jump <directory name>

#### Elvish:

**Elvish Install**

Install using [epm](https://elv.sh/ref/epm.html)

    $epm:install github.com/dmix/portal

Add to your ~/.elvish/rc.elv

    use epm
    use "github.com/dmix/portal/plugins/portal.plugin"

**Elvish Usage**

    p <directory name>
    
Which is a shortcut for:

    portal jump <directory name>

Bash:

Plugin coming soon. The ZSH plugin could easily be ported.

### Usage
--- 

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
--- 

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
