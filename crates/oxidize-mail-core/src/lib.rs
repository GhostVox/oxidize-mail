//! Core utilities and shared functionality for the Oxidize Mail application.
//!
//! This crate provides core functionality that is shared across different
//! components of the Oxidize Mail email client. It contains fundamental
//! utilities, data structures, and algorithms that support the main
//! application logic.
//!
//! # Features
//!
//! - Core data processing utilities
//! - Shared algorithms and helper functions
//! - Common functionality used across multiple crates
//! - Performance-critical operations
//!
//! # Example
//!
//! ```rust
//! use oxidize_mail_core::add;
//!
//! let result = add(2, 3);
//! assert_eq!(result, 5);
//! ```
//!
//! # Note
//!
//! This crate is currently in early development and contains placeholder
//! functions that will be replaced with actual core implementation as
//! the application evolves.

/// Adds two unsigned 64-bit integers together.
///
/// This is a placeholder function in the core library that demonstrates
/// basic arithmetic operations. It will be replaced with actual core
/// functionality as the Oxidize Mail application develops.
///
/// # Arguments
///
/// * `left` - The first number to add
/// * `right` - The second number to add
///
/// # Returns
///
/// The sum of `left` and `right` as a `u64`
///
/// # Examples
///
/// ```rust
/// use oxidize_mail_core::add;
///
/// let result = add(5, 3);
/// assert_eq!(result, 8);
///
/// let large_sum = add(u64::MAX - 1, 1);
/// assert_eq!(large_sum, u64::MAX);
/// ```
///
/// # Panics
///
/// This function will panic if the addition causes integer overflow.
/// Use `checked_add()` if overflow detection is needed.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
