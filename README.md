# Oscar
[![Build Status](https://travis-ci.org/asimpson/oscar.svg?branch=master)](https://travis-ci.org/asimpson/oscar)

A CLI application to download full TV episodes from PBS. Ideally run in `cron` or another scheduler.

## Example
`@hourly oscar -so /mnt/nas/videos/`

## Installation

### Pre-built binaries
Pre-built binaries are available via [Github releases](https://github.com/asimpson/oscar/releases) (which are built by [TravisCI](https://travis-ci.org/asimpson/oscar)).

### Cargo
`cargo install oscar`

### From source
Requires Rust and Cargo to be installed on the host system.

To build: `cargo build --release`

## Subcommands
`list`: View available shows in "slug" format, e.g. `sesame-street`.

FYI `oscar` defaults to downloading Sesame Street episodes.

## Options
`-d --dry-run`: run without actually downloading any files.

`-s --silent`: Do not log anything.

`-o --output`: Where to save episodes, defaults to `/tmp/`.

`-S --show-slug`: What show to download. Get the slug by running `oscar list`.

## Misc
Please note that each episode at ~720p quality is around `500MB`. If you are on a metered or a slow connection this may be a problem.

Every episode that is downloaded gets logged by appending its ID to a text file `~/.oscar_history`. This file is checked every time `oscar` runs to validate if an episode is new or not.

Currently new episodes cycle in on Friday at 1am EST.
