# Algorithms implemented in Rust

<a href="https://github.com/genskyff/algorithms-rs/actions/workflows/rust.yml">
  <img src="https://github.com/genskyff/algorithms-rs/actions/workflows/rust.yml/badge.svg" height="20" alt="Build workflow">
</a>

## Overview
This is a repository implemented in Rust for various fundamental algorithms and data structures, intended for learning and recording purposes. It is licensed under the MIT and is designed to be modular, making it easy to import into other code.

## Features
- Only use stable Rust
- Uses unsafe in only a few implementations
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
  - [x] Vector
  - [x] Linear lists
    - [x] Sequential list
    - [x] Static linked list
    - [x] Linked list
  - [x] Queue
    - [x] Array queue
    - [x] Linked queue
  - [x] Stack
    - [x] Array stack
    - [x] Linked stack
  - [x] Hash table
  - [ ] Tree
  - [ ] Heap
  - [ ] Graph
- [ ] String
  - [x] Brute force
  - [ ] KMP
- [ ] Searching
- ...
