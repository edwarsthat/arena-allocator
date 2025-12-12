# Arena Allocator

A simple, safe, and efficient arena allocator implementation in Rust.

## Description

`arena-allocator` provides a memory arena that allows for fast allocation of objects with a fixed capacity. It is designed to be efficient by reducing the overhead of individual allocations and simplifying memory management through collective deallocation.

## Features

- **Fixed Capacity**: Initialize with a pre-defined memory size.
- **Fast Allocation**: Bump pointer allocation with proper alignment handling.
- **Memory Reset**: Reuse the allocated memory block by resetting the offset.
- **Safe Abstraction**: Encapsulates unsafe raw pointer operations.
- **Resource Management**: Automatically deallocates memory when the arena is dropped.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
arena-allocator = { path = "." } # Or your git repository/version
```

## Usage

Here's a quick example of how to use the arena allocator:

```rust
use arena_allocator::Arena;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new arena with 1024 bytes of capacity
    let arena = Arena::new(1024)?;

    // Allocate values
    let x = arena.alloc(42u32);
    let y = arena.alloc(100u64);

    println!("x: {}", *x);
    println!("y: {}", *y);

    // Check usage
    println!("Used: {} bytes", arena.used());
    println!("Remaining: {} bytes", arena.remaining());

    // Reset the arena to reuse memory
    arena.reset();

    Ok(())
}
```

## API Documentation

### `Arena::new(capacity: usize) -> Result<Self, ArenaError>`
Creates a new `Arena` with the specified capacity in bytes. Returns an error if the capacity is zero or if allocation fails.

### `Arena::alloc<T>(&self, value: T) -> &mut T`
Allocates a value of type `T` in the arena and returns a mutable reference to it. Panics if there is not enough capacity.

### `Arena::reset(&self)`
Resets the arena's offset to zero, allowing the memory to be reused. Note that this does not drop the objects previously allocated; it just resets the pointer.

### `Arena::used(&self) -> usize`
Returns the number of bytes currently used in the arena.

### `Arena::remaining(&self) -> usize`
Returns the number of bytes remaining in the arena.

## License

[MIT](LICENSE)
