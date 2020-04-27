# Oscar
[![Build Status](https://travis-ci.org/asimpson/oscar.svg?branch=master)](https://travis-ci.org/asimpson/oscar)

A CLI application to download Sesame Street videos from PBS. Ideally run in cron or another scheduler.

## Example
`@hourly oscar -so /mnt/nas/videos/`

## Options
`-d --dry-run`: run without actually downloading any files.

`-s --silent`: Do not log anything.

`-o --output`: Where to save movies, defaults to `/tmp/`.

`-S --show-slug`: What show to download. Get the slug by running `oscar list`.

## Subcommands
`list`: View available shows in "slug" format, e.g. `sesame-street`.

## Misc
Please note that each episode of Sesame Street at ~720p quality is `~500MB`. If you are on a metered or a slow connection this may be a problem.

Every episode that is downloaded gets logged by appending its ID to a text file `~/.oscar_history`. This file is checked every time `oscar` runs to validate if an episode is new or not.

Currently a new episode cycles in on Friday at 1am EST.
