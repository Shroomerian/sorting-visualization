# sorting-visualization

[![CI Status](https://github.com/dmitmel/sorting-visualization/workflows/CI/badge.svg)](https://github.com/dmitmel/sorting-visualization/actions?query=workflow:CI)

A [Rust](https://www.rust-lang.org/) program for visualizing sorting algorithms which uses [Piston](http://www.piston.rs/) for graphics. Inspired by [**Hopson97/Sort-Algorithm-Visualiser**](https://github.com/Hopson97/Sort-Algorithm-Visualiser).

[![Demo](https://i.imgur.com/jyPDiWX.gif)](https://gist.github.com/dmitmel/f8664421b547577065912c3246f4c1e9)

## Setup

```bash
git clone https://github.com/dmitmel/sorting-visualization
cd sorting-visualization
cargo build --release
```

## Usage

```bash
# see 'Features' section for the list of supported algorithms and their IDs
# and use the --help flag to print all available animation options
cargo run <algorithm>
# list all available algorithms
cargo run -- --list
```

## Features

- CLI
- Supports different algorithms (IDs are in brackets):
  - [Double Sort](https://github.com/Shroomerian/double_sort) \[`double_sort`\]
  - [Bubble sort](https://en.wikipedia.org/wiki/Bubble_sort) \[`bubble`\]
  - [Cocktail sort](https://en.wikipedia.org/wiki/Cocktail_shaker_sort) \[`cocktail`\]
  - [Cycle sort](https://en.wikipedia.org/wiki/Cycle_sort) \[`cycle`\]
  - [Gnome sort](https://en.wikipedia.org/wiki/Gnome_sort) \[`gnome`\]
  - [Heap sort](https://en.wikipedia.org/wiki/Heapsort) \[`heap`\]
  - [Insertion sort](https://en.wikipedia.org/wiki/Insertion_sort) \[`insertion`\]
  - [Merge sort](https://en.wikipedia.org/wiki/Merge_sort) \[`merge`\]
  - [Quicksort](https://en.wikipedia.org/wiki/Quicksort) \[`quicksort`\]
  - [Selection sort](https://en.wikipedia.org/wiki/Selection_sort) \[`selection`\]
  - [Shellsort](https://en.wikipedia.org/wiki/Shellsort) \[`shellsort`\]
- Animation controls:
  - <kbd>Space</kbd> - pause/resume
  - <kbd>&uparrow;</kbd> - 2x faster
  - <kbd>&downarrow;</kbd> - 2x slower
- Easy-to-use algorithm API
- Algorithms can highlight important array elements
- Code is well-documented

## Building docs

```bash
./docs.sh
```

**Nightly Rust is required for building docs** because it has [infra doc link resolution](https://github.com/rust-lang/rust/issues/43466) which is currently unstable.

## TODO

1. User-friendly GUI
2. More algorithms
3. Sound?

## Contributing

[**Documentation**](https://dmitmel.github.io/sorting-visualization)

PRs are appreciated!

## License

[MIT](https://github.com/dmitmel/sorting-visualization/blob/master/LICENSE) © [Dmytro Meleshko](https://github.com/dmitmel)
