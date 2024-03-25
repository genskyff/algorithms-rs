# Algorithms implemented in Rust

<a href="https://github.com/genskyff/algorithms-rs/actions/workflows/rust.yml">
  <img src="https://github.com/genskyff/algorithms-rs/actions/workflows/rust.yml/badge.svg" height="20" alt="Build workflow">
</a>

## Overview
This is a repository implemented in Rust for various fundamental algorithms and data structures, intended for learning and recording purposes. It is licensed under the GPLv3 and is designed to be modular, making it easy to import into other code.

## Features
- Only use stable Rust
- Use Unsafe features only in certain data structures
- Includes integration tests
- Includes documentation

## Usage
You need to install Rust, and then run the following commands.

To perform integration tests:

```shell
cargo test
```

To view the documentation:

```shell
cargo doc --open
```

## Roadmap
- [ ] Sorting
  - [x] Bubble sort
  - [x] Cocktail sort
  - [ ] Heap sort
  - [x] Insertion sort
  - [x] Binary insertion sort
  - [x] Shell sort
  - [x] Merge sort (recursion version)
  - [x] Merge sort (iteration version)
  - [x] Quick sort
  - [x] Selection sort
  - [ ] Bucket sort
  - [ ] Counting sort
  - [ ] Radix sort
- [ ] Data structures
- [ ] Vector
  - [x] Linear lists
    - [x] Sequential list
    - [x] Static linked list
    - [x] Linked list
  - [ ] Queue
    - [ ] Array queue
    - [ ] Linked queue
  - [ ] Stack
    - [ ] Array stack
    - [ ] Linked stack
  - [x] Hash table
  - [ ] Tree
  - [ ] Heap
  - [ ] Graph
- [ ] Searching
- ...
