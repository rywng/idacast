# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
