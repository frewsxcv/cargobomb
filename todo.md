# not next

- download-crates-for-ex should refresh git repos
- test that docker works before running tests
- record durations tests take to run
- record and report on remaining work to go
- move lockfiles to non-ex data area
- make generate-lockfiles skip existing lockfiles unless --all
- fix single-threaded logging redirecting
- load crates into experiment
- single define-experiment step
- support local toolchains
- log to experiment directory
- write files atomically
- unstable feature analysis
- load crates into experiment as a discrete step
- build toolchains
- incrementalize discovery
- load gh-apps from fallback
- allow prepare-crates to update repos
- resident memory tests, compiler and tests
- instruction count tests, compiler and tests
- cache miss tests, compiler and tests
- broken blacklist
- run the rust test suite
- --cap-lints=allow
- reliable checkpoints
- aggregate benchmarking
- crate-platform compatibility matrix
- test --debug and --release
- test debug + optimizations
- use cargo-vendor for caching libraries
- fix docker file system permissions
- associate crates with github repos
- analyze contributors to crates
- what kind of analysis can you do with
  - full source of all crate revisions
  - full dependency graph of all revisions
  - full source history of all crates
  - full source history of all rust project on github
  - compile and run sandboxed rust on all platforms
- declared dependency tester.
  for a single crate, for each of its deps, modify the lockfile while running
  that crate's tests, looking for combinations that don't pass
- crusader testing
- check for working docker before running tests

# Apps I didn't find through search :-/

https://github.com/kaj/chord3
https://github.com/dzamlo/treeify
https://github.com/azdle/virgil
https://github.com/farseerfc/ydcv-rs

# Data size

5.5 GB for the most recent crates
10M lines Rust in most recent crates
8.6 GB after fetching deps
3152 deps cached
13 GB after building up to bytestool

# Data model

cargobomb works with a lot of data, and wants to coordinate that between
distributed workers

- master/
  - state.json - atomic replace
- local/
  - cargo-home/ - mutable
  - rustup-home/
  - crates.io-index/
  - gh-clones/
  - target-dirs/
  - test/
  - custom-tc/
- shared/
  - crates/
  - gh-mirrors/
  - fromls/
  - lockfiles/
  - lists/
    - recent.txt
    - second.txt
    - hot.txt
    - gh-repos.txt
    - gh-apps.txt
- ex/
  - $ex-name/
    - config.json
    - crates.txt
    - lockfiles/
    - custom-tc/ - ?
      - $host/
    - run/
      - $run-name/
        - c/
	  - $crate-id
            - result.txt
            - log.txt
        - g/
          - $gh-org/
            - $gh-name/
              - $sha/
                - result.txt
                - log.txt
- report/
  - index.html, etc.
  - ex
    - $ex-name
      - info.json
      - index.html
      - report.md
      - todo

# Work flow

- update-data
  - prepare-stable-toolchain
  - create-lists
    - create-recent-list
    - create-second-list
    - create-hot-list
    - create-gh-candidate-list-from-cache
    - create-gh-app-list-from-cache
- define-ex - sets up experiment definition and per-experiment crate list
- prepare-ex
  - prepare-ex-global
    - download-crates
    - capture-shas
    - frob-tomls
    - generate lockfiles (w/ stable)
  - Prepare local state for experiment (prepare-ex-local)
    - Fetch deps (w/ stable)
    - Prepare toolchains
- run
- gen-report

```
cargobomb update-data
cargobomb define-ex
cargobomb prepare-ex
cargobomb run
cargobomb report
```