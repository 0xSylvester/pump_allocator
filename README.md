# pump_allocator

An early-stage bump allocator written in Rust. A bump allocator typically hands
out memory from a preallocated block by moving a cursor forward, making individual
allocations inexpensive.

This project is for learning and fun. It is not intended for production use.

## Current status

The project currently creates and frees the backing block; allocating individual
values from it is not yet implemented. `BumpAllocator` is private to the binary
and is not yet a library API.

- `BumpAllocator::new(capacity)` allocates a block of `capacity` bytes and starts
  the cursor at zero.
- Zero or invalid capacities return `AllocatorError::InvalidCapacity`; a failed
  allocation returns `AllocatorError::AllocationFailed`.
- The backing block is freed automatically when the allocator is dropped.
- The backing block requests an alignment of one byte. Future allocation methods
  must account for each value's alignment and any necessary padding.

## Getting started

Install a Rust toolchain that supports the 2024 edition (Rust 1.85 or newer),
then run:

```sh
cargo run
```

The example creates a 1024-byte backing block and prints its size and cursor.

Run the tests with:

```sh
cargo test
```

## Future work

To become a usable bump allocator, the project still needs allocation methods
that advance the cursor, honor requested alignments, and reject requests that do
not fit in the backing block.
