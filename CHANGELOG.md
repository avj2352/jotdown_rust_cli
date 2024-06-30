# Jotdown

## version 1.2.0
- 06/29/2024
- Bump up version to `1.2.0`
- Highlight: Highlight topics with cyan color
- Feature: Show completed progress bar with percentage
- Remove `send_notification.rs`
- Release `1.2.0`

## version 1.1.0
- 05/17/2024
- Bump up version to `1.1.0`
- Bugfix: Empty string when tag annotations that are in the middle of the string
- Remove `ntfy` notification send when unit testing - `send_notifications.rs`
- Add bugfix unit test scripts
- Release `1.1.0`

## version 1.0.02
- 11/20/2023
- BAT scripts for windows build
- Add `~root~/release` folder to .gitignore list

## version 1.0.00
- 11/05/2023
- ✨ Official release of version 1.0.0 ✨
- Includes sort and notification features
- Includes all features of todo node js cli application

## version 0.5.01
- 10/28/2023
- 3 more features pending to make first release!
- Completed clear command `jd clear --done`
- Completed renumber command `jd renumber`
- POC: Send push notification to Mobile using `ntfy.sh` curl command!!
- Unit testing and refactoring

## version 0.5.00
- 10/15/2023
- Removed unused library crates - rustc-serializer and regex
- Brought down build size from 3.3 mb -> 795 kb ✨
- added config to cargo.toml to compress build size
- Completed move, add, remove features
- Fixed bug - check and cancel icons change icon
- unit testing
- updated README.md
- research on API calls to make - Microsoft, OneSignal

## version 0.1.03
- 10/07/2023
- Completed list subcommand
- fixed bug - parse json from file
- unit testing
- updated README.md


## version 0.1.02
- 10/02/2023
- Initialize repo
- Setup folder structure
- Install required dependencies
- Added Config file `config.rs` for DB file generation
