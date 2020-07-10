/// Distributed cache system written in Rust.
/// ************
/// Caches are distributed in such a way that requests are routed to the appropriate machine based on the requested key.
/// Example:
/// [
///  A -> machine X,
///  B -> machine Y,
///  C -> machine Z
/// ]
/// By keeping a hashmap in memory, we can map request A to machine X.
/// ************
/// The following eviction policies are able to be used:
/// - LRU cache: Once cache is full, the next item to be discarded is the least recently used one.
/// - MRU cache: Discard the most recently used item.
/// - Random replacement: Discard a randomly picked item.
/// - Least frequently used: Keep a count of how frequently used each item is and discard the least frequently used one.
/// ************
/// Concurrency: Many different processes may be changing a cache at any given moment in time.
/// - Split the cache into multiple shards, however, some shards may be more used than others.
/// - Use commit logs to store the mutations inside logs rather than updating the data immediately. Then update asynchronously at another point in time.
fn main() {
    println!("Hello, world!");
}
