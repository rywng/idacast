# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.6](https://github.com/rywng/idacast/compare/v0.1.5...v0.1.6) - 2025-11-21

### Fixed

- *(ui)* Fix the formatting of time remaining.

### Other

- Merge remote-tracking branch 'refs/remotes/origin/master'
- *(README)* Use pure image instead of table
- *(README)* Try to fix the image formatting

## [0.1.5](https://github.com/rywng/idacast/compare/v0.1.4...v0.1.5) - 2025-11-18

### Added

- Add a reminder that fest page is not yet available
- Fine-tune the position of description text
- *(app)* Now able to view challenge events with better time display
- *(app)* Support loading the schedules to the software memory
- *(app)* Add ability to deserialize league match schedule

### Fixed

- use id in favor of leagueMatchEventId

### Other

- remove unused imports
- format code
- Upload screenshots to README
- Add screenshots of the cli
- *(app)* Fix formatting
- *(ui)* change function names to make it clear

## [0.1.4](https://github.com/rywng/idacast/compare/v0.1.3...v0.1.4) - 2025-11-18

### Added

- *(ui)* Implement scrolling for work
- *(ui)* Better time display for schedules

## [0.1.3](https://github.com/rywng/idacast/compare/v0.1.2...v0.1.3) - 2025-11-11

### Fixed

- Cached status not showing up correctly

### Other

- update deps
- add samples for schedules
- fix doc link

## [0.1.2](https://github.com/rywng/idacast/compare/v0.1.1...v0.1.2) - 2025-07-28

### Added

- *(ui)* Show either the schedule is cached or not
- *(ui)* Display work data
- *(data)* Allow the program to represent Salmon run data
- *(deserialize)* Add ability to decode regular coop schedules
- *(ui)* WIP add support for working UI
- *(ui)* Add tabs to support rendering other information
- *(data)* Remove un-needed error types
- *(caching)* Improve caching logic

### Other

- *(app)* optimize grammar for some functions
- *(app)* Clean up the app code a little bit, prevent from panic
- *(data)* Use trait implementations to organize schedules
- *(README)* Update readme to reflect state of software
- *(data)* Simplify structs into one
- *(app)* separate different data attached to different UIs
- change visibility of cache storage name
- Move ui module to app, and restrict visibility
- Clean up code
- *(ui)* Use horizontal layout for rendering
- *(render)* Use block to better align the text
- Add feature desc
- *(cache)* one less string conversion
- Update readme to reflect the current state of software
- remove irrelavant comment
- Format code

## [0.1.1](https://github.com/rywng/idacast/compare/v0.1.0...v0.1.1) - 2025-07-25

### Added

- *(cache)* Allow user to clear cache
- *(cache)* Use set cache name
- *(data)* Cache network requests for auto-updates

### Other

- Format code
- Update README
- more elegant handling of update interval
