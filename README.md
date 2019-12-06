# Oscar
A CLI application to download Sesame Street videos from PBS. Ideally run in cron or another scheduler.

## Example
`@hourly oscar -so /mnt/nas/videos/`

## Options
`-d --dry-run`: run without actually downloading any files.

`-s --silent`: Do not log anything.

`-o --output`: Where to save movies, defaults to `/tmp/`.

## Misc
Please note that each episode of Sesame Street at 720p quality is `~500MB`. If you are on a metered or a slow connection this may be a problem.

Every episode that is downloaded gets logged by appending its ID to a text file `~/.oscar_history`. This file is checked every time `oscar` runs to validate if an episode is new or not.

Currently a new episode cycles in on Friday at 1am EST.
