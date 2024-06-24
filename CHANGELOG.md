# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- Updated dependencies
- Updated documentation on filesystem support and inter-filesystem transfers

## [1.2.0] - 2023-01-15
### Added
- copy parameter

### Fixed
- dry run doesn't delete anymore

## [1.1.0] - 2022-06-03
### Added
- Bookmark support
- Optimized tag writeback when removing non existent tags

## [1.0.0] - 2022-05-16
### Added
- Completely changed parameter interface which enables multiple actions in one run
- Faster search with precompilation and short-circuit evaluation
- Allows renaming of tags with regex support
- Can show all used tags in a set of files
- Added dry_run option

## [0.1.0] - 2022-05-10
### Added
- Initial functionality
