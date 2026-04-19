//! # Generic Containers Library
//!
//! This library provides reusable generic data structures
//! for different use cases.
//!
//! ## Available Containers
//!
//! - [`Queue`] - FIFO (First In, First Out) Queue
//! - [`Deque`] - Double-ended Queue
//! - [`Bounded`] - Fixed capacity container
//! - [`Cache`] - Simplified LRU Cache

pub mod cache;
pub mod cola;
pub mod deque;
pub mod limitado;

pub use cache::Cache;
pub use cola::Queue;
pub use deque::Deque;
pub use limitado::Bounded;
