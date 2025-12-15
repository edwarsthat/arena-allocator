use arena_allocator::{Arena, ArenaError};

#[test]
fn test_arena_creation() {
    let arena = Arena::new(1024).expect("Failed to create arena");
    assert_eq!(arena.remaining(), 1024);
    assert_eq!(arena.used(), 0);
}

#[test]
fn test_basic_allocation() {
    let arena = Arena::new(1024).unwrap();
    let x = arena.alloc(42u32).unwrap();
    assert_eq!(*x, 42);
}

#[test]
fn test_alloc_out_of_capacity_returns_error() {
    // Arena de solo 8 bytes
    let arena = Arena::new(8).unwrap();

    // Primera asignación (consume 8 bytes)
    let result1 = arena.alloc(1u64);
    assert!(result1.is_ok());

    // Segunda asignación (excede la capacidad) -> DEBE DEVOLVER ERROR
    let result2 = arena.alloc(2u64);

    assert_eq!(
        result2.unwrap_err(),
        ArenaError::NotEnoughCapacity,
        "El error devuelto debe ser OutOfCapacity."
    );
}
