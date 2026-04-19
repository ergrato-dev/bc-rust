//! Demonstration of the generic containers library

use proyecto_contenedor::{Bounded, Cache, Deque, Queue};

fn main() {
    println!("=== Generic Containers Library ===\n");

    demo_queue();
    demo_deque();
    demo_bounded();
    demo_cache();

    println!("✅ All demos completed!");
}

fn demo_queue() {
    println!("📦 Demo: Queue<T>");
    println!("{}", "-".repeat(40));

    let mut queue: Queue<&str> = Queue::new();
    queue.enqueue("first");
    queue.enqueue("second");
    queue.enqueue("third");

    println!("Queue created with 3 elements");
    println!("Front: {:?}", queue.front());
    println!("Dequeuing: {:?}", queue.dequeue());
    println!("New front: {:?}", queue.front());
    println!("Length: {}\n", queue.len());
}

fn demo_deque() {
    println!("📦 Demo: Deque<T>");
    println!("{}", "-".repeat(40));

    let mut deque: Deque<i32> = Deque::new();
    deque.push_back(2);
    deque.push_front(1);
    deque.push_back(3);

    println!("Deque: [1, 2, 3]");
    println!("Front: {:?}", deque.front());
    println!("Back: {:?}", deque.back());
    println!("Pop front: {:?}", deque.pop_front());
    println!("Pop back: {:?}", deque.pop_back());
    println!("Remaining length: {}\n", deque.len());
}

fn demo_bounded() {
    println!("📦 Demo: Bounded<T, 3>");
    println!("{}", "-".repeat(40));

    let mut bounded: Bounded<char, 3> = Bounded::new();

    println!("Capacity: {}", bounded.capacity());
    println!("Inserting 'a': {:?}", bounded.insert('a'));
    println!("Inserting 'b': {:?}", bounded.insert('b'));
    println!("Inserting 'c': {:?}", bounded.insert('c'));
    println!("Is full?: {}", bounded.is_full());
    println!("Inserting 'd' (should fail): {:?}", bounded.insert('d'));
    println!("Length: {}\n", bounded.len());
}

fn demo_cache() {
    println!("📦 Demo: Cache<K, V>");
    println!("{}", "-".repeat(40));

    let mut cache: Cache<&str, i32> = Cache::new(2);

    cache.insert("one", 1);
    cache.insert("two", 2);
    println!("Cache with 'one' and 'two'");

    println!("Getting 'one': {:?}", cache.get(&"one"));

    cache.insert("three", 3); // Should remove 'two' (LRU)
    println!("Inserted 'three' (capacity 2)");

    println!("Contains 'one'?: {}", cache.contains(&"one"));
    println!("Contains 'two'?: {}", cache.contains(&"two")); // Should be false
    println!("Contains 'three'?: {}", cache.contains(&"three"));
    println!("Length: {}\n", cache.len());
}
