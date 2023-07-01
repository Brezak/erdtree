# Change Log
All notable changes to this project will be documented in this file.
 
This project adheres to [Semantic Versioning](http://semver.org/).

## [3.1.0] - 2023-07-01

### [What's new](https://github.com/solidiquis/erdtree/pull/202)
- [Multiple configs with .erdtree.toml](https://github.com/solidiquis/erdtree/pull/201)
- [Increased performance with --suppress-size](https://github.com/solidiquis/erdtree/pull/203)
- [-x, --one-file-system](https://github.com/solidiquis/erdtree/pull/204)
- [Support for NO_COLOR](https://github.com/solidiquis/erdtree/pull/205)
- [Inverted flat layout](https://github.com/solidiquis/erdtree/pull/206)
- [Aliases: atime, ctime, and mtime](https://github.com/solidiquis/erdtree/pull/207)
- [ctrlc handler to restore cursor if progress indicator is enabled](https://github.com/solidiquis/erdtree/pull/210)


### [.erdtree.toml](https://github.com/solidiquis/erdtree/pull/201)

First and foremost:

**`.erdtreerc` is planned for deprecation by v3.3 so please migrate to `.erdtree.toml` by then.**

There is now support for multiple configs:

```
-c, --config <CONFIG>  Use configuration of named table rather than the top-level table in .erdtree.toml
```

So given the following example `.erdtree.toml`:

```toml
icons = true
human = true

# Compute file sizes like `du`
[du]
disk_usage = "block"
icons = true
layout = "flat"
no-ignore = true
no-git = true
hidden = true
level = 1

# Do as `ls -l`
[ls]
icons = true
human = true
level = 1
suppress-size = true
long = true
no-ignore = true
hidden = true

# How many lines of Rust are in this code base?
[rs]
disk-usage = "line"
level = 1
pattern = "\\.rs$"
```

```
$ erd

# equivalant to 

$ erd --human --icons
```

...

```
$ erd -c ls

# equivalent to

$ erd --icons --human --level 1 --suppress-size --long --no-ignore --hidden
```

etc.

For further information on how to get started with the new `.erdtree.toml` please refer to the [README.md](README.md#toml-file).

**NOTE**:
- Multiple configs are only supported via `.erdtree.toml`.
- If you have both a `.erdtree.toml` and `.erdtreerc` then the latter will take precedent. Attempts to use `--config` in said case will result in an error and a warning asking to migrate from `.erdtreerc` to `.erdtree.toml`.
- Again: **`.erdtreerc` will be deprecated in favor of `.erdtree.toml` by v3.3**


## [3.0.2] - 2023-06-12
- Added support for `.erdtree.toml`

### Bug fixes
- [Fixed segmentation fault that occurs with `--long` when file has an invalid user or group](https://github.com/solidiquis/erdtree/pull/193)
- [Updated deprecated Nerd symbols](https://github.com/solidiquis/erdtree/pull/192)
- [Change CJK and emoji block icons](https://github.com/solidiquis/erdtree/pull/194)

**Full Changelog**: https://github.com/solidiquis/erdtree/compare/v3.0.1...v3.0.2

## [3.0.1] - 2023-05-28

### Bug fixes
- Fixed issue where empty directories were automatically being pruned despite not opting into `--prune`.
    - https://github.com/solidiquis/erdtree/issues/185
- Fixed issue where searching via globs or regex doesn't work when user has a config file.

### Other changes:
- Added exit codes to `erdtree`.
- Progress indicator now acquires a lock to `stdout` reduce likelihood of printing the progress-text being a bottleneck.
- Some code-cleanup.

### Pull Requests
- https://github.com/solidiquis/erdtree/pull/183
- https://github.com/solidiquis/erdtree/pull/186
- https://github.com/solidiquis/erdtree/pull/187

### Contributors
- [KP64](https://github.com/KP64)

**Full Changelog**: https://github.com/solidiquis/erdtree/compare/v3.0.0...v3.0.1

## [3.0.0] - 2023-05-25

### Bug fixes
- [Completions now properly reference the new binary name](https://github.com/solidiquis/erdtree/commit/c4ff0dacb137f1007af27f692f1e5ee66bf47cb6), `erd`
    - https://github.com/solidiquis/erdtree/issues/141
    - https://github.com/solidiquis/erdtree/issues/143
    - https://github.com/solidiquis/erdtree/issues/150
- [Symlink target now shows full path rather than just file name](https://github.com/solidiquis/erdtree/pull/162)
    - https://github.com/solidiquis/erdtree/issues/148
- `--dir-order <DIR_ORDER>` [added to address ordering ambiguity with arg names](https://github.com/solidiquis/erdtree/pull/161)
    - https://github.com/solidiquis/erdtree/issues/149
- [Fixed NetBSD build error associated with xattrs](https://github.com/solidiquis/erdtree/commit/33d918f2acc68f2782d1fada8384aeadec57f3fe)
    -  https://github.com/solidiquis/erdtree/issues/157
- [Fixed issue where `--level` would omit files from the total count](https://github.com/solidiquis/erdtree/pull/169)
- [Added support for %APPDATA% config folder for Windows](https://github.com/solidiquis/erdtree/pull/154)
    - https://github.com/solidiquis/erdtree/issues/152

### Additions
- `-C, --color` https://github.com/solidiquis/erdtree/pull/147
- Support for owner and group, changed default columns for `l, --long`, and added optional long-view arguments https://github.com/solidiquis/erdtree/pull/174
- Support customizing time output format https://github.com/solidiquis/erdtree/pull/165
- Support for sorting files by modified time, access time, and created time [55783ff](https://github.com/solidiquis/erdtree/pull/155/commits/55783ff7957a1e42c6d06dfd84682188b74b2cec)
- Support for reporting block usage, line-count, and word-count https://github.com/solidiquis/erdtree/pull/172 
- Added progress indicator https://github.com/solidiquis/erdtree/pull/176

### New Contributors
- [iquiw](https://github.com/iquiw)
- [KP64](https://github.com/KP64)
- [m4rch3n1ng](https://github.com/m4rch3n1ng)
- [demilich1](https://github.com/demilich1)

For a more comprehensive overview of `erdtree` v3.0.0, please refer to the [README.md](README.md).

**Full Changelog**: https://github.com/solidiquis/erdtree/compare/v2.0.0...v3.0.0

## [2.0.0] - 2023-04-26

### What's Changed

`erdtree` v2.0.0 introduces numerous breaking changes as well as a plethora of new features. Most breaking changes are predicated on the fact that
arguments were either renamed, removed, or fundamentally modified. The following is a list of all the PRs that document these changes and feature additions:

- https://github.com/solidiquis/erdtree/pull/130
- https://github.com/solidiquis/erdtree/pull/132
- https://github.com/solidiquis/erdtree/pull/135
- https://github.com/solidiquis/erdtree/pull/136
- https://github.com/solidiquis/erdtree/pull/137
- https://github.com/solidiquis/erdtree/pull/138
- https://github.com/solidiquis/erdtree/pull/139
- https://github.com/solidiquis/erdtree/pull/131

Perhaps the most important change to note is that the compiled binary has been renamed from `et` to `erd` in order to address the following issue
regarding name collisions with other programs: https://github.com/solidiquis/erdtree/issues/23

For a more comprehensive overview of `erdtree` v2.0.0, please refer to the [README.md](README.md).

**Full Changelog**: https://github.com/solidiquis/erdtree/compare/v1.8.1...v2.0.0

## [1.8.1] - 2023-04-11

### What's Changed
* Fix some typos by @goggle in https://github.com/solidiquis/erdtree/pull/110
* add clap requires to flags that depent on --report by @jhscheer in https://github.com/solidiquis/erdtree/pull/111
* refactor tests: move --no-config to mod::run_cmd by @jhscheer in https://github.com/solidiquis/erdtree/pull/112
* Prevent panic when `--prune` is used with `--glob` which results in empty match set by @solidiquis in https://github.com/solidiquis/erdtree/pull/116
* Add ability to take glob patterns from stdin by @jhscheer in https://github.com/solidiquis/erdtree/pull/114
* Refactor/node and support hard link detection on Windows by @solidiquis in https://github.com/solidiquis/erdtree/pull/118
* Support colorless output when redirecting/piping stdout; also provide `--no-color` option by @solidiquis in https://github.com/solidiquis/erdtree/pull/120
* remove ansi escapes for default icon by @solidiquis in https://github.com/solidiquis/erdtree/pull/122

### New Contributors
* @goggle made their first contribution in https://github.com/solidiquis/erdtree/pull/110
* @jhscheer made their first contribution in https://github.com/solidiquis/erdtree/pull/111

**Full Changelog**: https://github.com/solidiquis/erdtree/compare/v1.7.1...v1.8.1

## [1.7.1] - 2023-03-30

### What's Changed
* fix issue where not-existent directory as cli arg causes infinite loop by @solidiquis in https://github.com/solidiquis/erdtree/pull/107

**Full Changelog**: https://github.com/solidiquis/erdtree/compare/1.7.0...v1.7.1

## [1.7.0] - 2023-03-30

### What's Changed
* Fix windows can not recognize the tag version when publishing by @Tlntin in https://github.com/solidiquis/erdtree/pull/91
* Fix the problem that test actions cannot upload windows binary files, add the function of custom form to set tag version. by @Tlntin in https://github.com/solidiquis/erdtree/pull/94
* Reduce default thread count by @solidiquis in https://github.com/solidiquis/erdtree/pull/99
* option for plain text disk usage reporting by @solidiquis in https://github.com/solidiquis/erdtree/pull/102
* Prune bug by @solidiquis in https://github.com/solidiquis/erdtree/pull/103
* dirs-only by @solidiquis in https://github.com/solidiquis/erdtree/pull/104
* Feature/file count by @solidiquis in https://github.com/solidiquis/erdtree/pull/105

### New Contributors
* @CosmicHorrorDev made their first contribution in https://github.com/solidiquis/erdtree/pull/93
* @KP64 made their first contribution in https://github.com/solidiquis/erdtree/pull/100
* @Masynchin made their first contribution in https://github.com/solidiquis/erdtree/pull/98

**Full Changelog**: https://github.com/solidiquis/erdtree/compare/v1.6.0...1.7.0


## [1.6.0] - 2023-03-20

### What's Changed
* Add NetBSD instructions by @0323pin in https://github.com/solidiquis/erdtree/pull/77
* Add repology badge by @jubalh in https://github.com/solidiquis/erdtree/pull/76
* fix issue where level wasn't being read from config by @solidiquis in https://github.com/solidiquis/erdtree/pull/78
* add scoop manifest by @fawni in https://github.com/solidiquis/erdtree/pull/80
* default to num logical cpus rather than 4 threads by @solidiquis in https://github.com/solidiquis/erdtree/pull/81
* Add support for generating shell completions by @Brezak in https://github.com/solidiquis/erdtree/pull/82
* Fix miscoloration of directories that have extension by @fawni in https://github.com/solidiquis/erdtree/pull/83
* [Optimization] - Upgraded heap-based tree data structure to an index-tree by @solidiquis in https://github.com/solidiquis/erdtree/pull/86
* Replace tempdir crate with tempfile crate by @Brezak in https://github.com/solidiquis/erdtree/pull/87
* fix issue where ansi escapes were being printed raw on windows by @solidiquis in https://github.com/solidiquis/erdtree/pull/90

### New Contributors
* @0323pin made their first contribution in https://github.com/solidiquis/erdtree/pull/77
* @jubalh made their first contribution in https://github.com/solidiquis/erdtree/pull/76
* @Brezak made their first contribution in https://github.com/solidiquis/erdtree/pull/82

**Full Changelog**: https://github.com/solidiquis/erdtree/compare/v1.5.2...1.6.0

## [1.5.2] - 2023-03-15

### Bug Fixes
- Stray print

## [1.5.1] - 2023-03-15

### Bug Fixes
- Fixed issue where globbing didn't work when user uses a config file https://github.com/solidiquis/erdtree/pull/75

## [1.5.0] - 2023-03-14

### Additions
- Added `--size-left` to print disk usage to the left of the tree https://github.com/solidiquis/erdtree/pull/61
- Added more paths the `erdtree` config could be placed at https://github.com/solidiquis/erdtree/pull/70

### Bug Fixes
- Fixed issue where `--dirs-first` wouldn't work unless `-s, --sort` was specified https://github.com/solidiquis/erdtree/pull/67
- Fixed isse where arguments from the `erdtree` config were being completely disregarded https://github.com/solidiquis/erdtree/pull/74

### Contributors

- [bryceberger](https://github.com/bryceberger)
- [fawni](https://github.com/fawni)

## [1.4.1] - 2023-03-12

### Bug Fixes
- [config file boolean options not working](https://github.com/solidiquis/erdtree/pull/60)

## [1.4.0] - 2023-03-12

### Bug Fixes
- [--ignore-git didn't ignore .git](https://github.com/solidiquis/erdtree/pull/59)

### Additions
- [sanders41](https://github.com/sanders41): [Added --suppress-size to suppress printing disk usage](https://github.com/solidiquis/erdtree/pull/47)
- [Added ability to use a config file to override erdtree defaults](https://github.com/solidiquis/erdtree/pull/52)
- [Added -P, --prune option to prevent printing of empty branches](https://github.com/solidiquis/erdtree/pull/55)
- [Added -p, --prefix to toggle between binary and SI prefixes when reporting disk usage](https://github.com/solidiquis/erdtree/pull/54)

### Contributors
- [sanders41](https://github.com/sanders41)

## [1.3.0] - 2023-03-04

### Bug Fixes
- [Fixed panic when file names contain non Unicode sequences](https://github.com/solidiquis/erdtree/pull/32)
- [Fixed panic when running from root directory](https://github.com/solidiquis/erdtree/pull/33)

### Additions
- [bryceberger](https://github.com/bryceberger): [Allow multiple uses of the same option for override](https://github.com/solidiquis/erdtree/pull/35)
- [bryceberger](https://github.com/bryceberger): [--dirst-first sorting option](https://github.com/solidiquis/erdtree/pull/38)
- [Added option to toggle logical vs. physical size and changed size sorting default so that largest is on bottom](https://github.com/solidiquis/erdtree/pull/39)
- [If multiple hardlinks in the same file-tree only one is taken into account](https://github.com/solidiquis/erdtree/pull/40)
- [Can now adjust scale of disk usage report](https://github.com/solidiquis/erdtree/pull/41)

### Major Changes
- [Binary prefixes have now replaced SI prefixes for reporting disk usage](https://github.com/solidiquis/erdtree/commit/b118006640a53e8083977d393beb1eca1c239e15)

### Special thanks

Thank you to all of the folks from [this Reddit thread](https://www.reddit.com/r/rust/comments/11ioq1n/erdtree_v120_a_modern_multithreaded_alternative/) who helped shape `erdtree` with their very valuable feedback as well as contributors!


## [1.2.0] - 2023-03-04

### What changed
- [Icon support](https://github.com/solidiquis/erdtree/pull/24)
- [--ignore-git](https://github.com/solidiquis/erdtree/pull/25)
- [Better UI for symlinks](https://github.com/solidiquis/erdtree/pull/26)

## [1.1.0] - 2023-02-14

### What changed
- `-S, --follow-links` added to give option to traverse symlinks to directories. If enabled the disk usage of the target directory is considered; additionally, descendents of symlink target directory have different color branches.
- CLI options in help text alphabetized with the exception of `-h, --help` and `-V, --version`.
- Minor refactors for clarity and organization.
- More comprehensive test coverage.

### Contributors
- [jprochazk](https://github.com/jprochazk): [Filtering functionality via glob options](https://github.com/solidiquis/erdtree/pull/12)
- [tintin](https://github.com/Tlntin): [Cross-compilation and CI](https://github.com/solidiquis/erdtree/pull/18)

## [1.0.0] - 2023-02-07

Did a complete rewrite with emphasis on an intuitive interface and performance. Notable changes:
- Binary renamed to `et` for brevity.
- Respects `.gitignore` and hidden file rules.
- Parallel filesystem traversal.
- Completely new CLI. `$ erdtree -h` for usage info.
- Uses `LS_COLORS` environment variable for file coloring.

## [0.1.0] - 2022-05-08

First release.
