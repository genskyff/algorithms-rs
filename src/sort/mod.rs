/* -------------------------- */
/*  Comparison-based Sorting  */
/* -------------------------- */

/// # Bubble Sort
pub mod bubble;
pub use bubble::Bubble;

/// # Heap Sort
pub mod heap;
pub use heap::Heap;

/// # Insertion Sort
pub mod insertion;
pub use insertion::Insertion;

/// # Merge Sort
pub mod merge;
pub use merge::Merge;

/// # Quick Sort
pub mod quick;
pub use quick::Quick;

/// # Selection Sort
pub mod selection;
pub use selection::Selection;

/// # Tim Sort
pub mod tim;
pub use tim::Tim;

/* ------------------------------ */
/*  Non-comparison-based Sorting  */
/* ------------------------------ */

/// # Bucket Sort
pub mod bucket;
pub use bucket::Bucket;

/// # Counting Sort
pub mod counting;
pub use counting::Counting;

/// # Radix Sort
pub mod radix;
pub use radix::Radix;
