//! A compatibility layer for `rand` and `rand_core` providing adaptation between traits for each version
//!
//! ## Forward compatibility (using `rand/std` for `rand_0_7::OsRng`)
//!
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
#![cfg_attr(feature = "std", doc = "```")]
//! use rand_0_7::rngs::OsRng;
//! use rand_core_0_6::{RngCore, CryptoRng};
//! use rand_compat::ForwardCompat;
//!
//! // RngCore + CryptoRng from rand_core@0.6.x
//! fn something<R: RngCore + CryptoRng>(r: &mut R) -> u32 {
//!     r.next_u32()
//! }
//!
//! let mut rng = OsRng;    // OsRng from rand@0.7.x (rand_core@0.5.x)
//!
//! let n = something(&mut rng.forward());
//! ```
//!
//! ## Backward compatibility (using `rand/std` for `rand_0_8::OsRng`)
//!
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
#![cfg_attr(feature = "std", doc = "```")]
//! use rand_0_8::rngs::OsRng;
//! use rand_core_0_5::{RngCore, CryptoRng};
//! use rand_compat::BackwardCompat;
//!
//! // RngCore + CryptoRng from rand_core@0.5.x
//! fn something<R: RngCore + CryptoRng>(r: &mut R) -> u32 {
//!     r.next_u32()
//! }
//!
//! let mut rng = OsRng;    // OsRng from rand@0.8.x (rand_core@0.6.x)
//!
//! let n = something(&mut rng.backward());
//! ```
//!

#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt::Debug;

/// Re-export of rand_core@0.5.x
pub use rand_core_0_5;

/// Re-export of rand_core@0.6.x
pub use rand_core_0_6;

/// Re-export of rand@0.7.x
pub use rand_0_7;

/// Re-export of rand@0.8.x
pub use rand_0_8;

/// Forward compatibility container object
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Forward<T>(pub T);

/// Helper trait to convert a type for forward compatibility
pub trait ForwardCompat<T> {
    fn forward(self) -> Forward<T>;
}

impl<T: rand_core_0_5::RngCore> ForwardCompat<T> for T {
    /// Call `.forward()` on an 0.5.x [`rand_core_0_5::RngCore`] to receive a 0.6.x [`rand_core_0_6::RngCore`] compatible instance
    fn forward(self) -> Forward<T> {
        Forward(self)
    }
}

/// Implementation of [`rand_core_0_6::RngCore`] for forward compatibility
impl<T: rand_core_0_5::RngCore> rand_core_0_6::RngCore for Forward<T> {
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core_0_6::Error> {
        // Attempt to fill bytes
        let e = match self.0.try_fill_bytes(dest) {
            Ok(_) => return Ok(()),
            Err(e) => e,
        };

        // Map errors via code if available
        if let Some(c) = e.code() {
            return Err(rand_core_0_6::Error::from(c));
        }

        // Otherwise we have to return an unknown error
        let c = unsafe { core::num::NonZeroU32::new_unchecked(getrandom::Error::CUSTOM_START) };
        Err(rand_core_0_6::Error::from(c))
    }
}

/// Forward [`rand_core_0_6::CryptoRng`] marker for [`rand_core_0_5::CryptoRng`] types
impl<T: rand_core_0_5::RngCore + rand_core_0_5::CryptoRng> rand_core_0_6::CryptoRng
    for Forward<T>
{
}

/// Backward compatibility container object
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Backward<T>(pub T);

/// Convert a type into a forward compatibility wrapper object
pub trait BackwardCompat<T> {
    fn backward(self) -> Backward<T>;
}

impl<T: rand_core_0_6::RngCore> BackwardCompat<T> for T {
    /// Call `.backward()` on an 0.6.x [`rand_core_0_6::RngCore`] to receive a 0.5.x [`rand_core_0_5::RngCore`] compatible instance
    fn backward(self) -> Backward<T> {
        Backward(self)
    }
}

/// Implementation of [`rand_core_0_5::RngCore`] for backward compatibility
impl<T: rand_core_0_6::RngCore> rand_core_0_5::RngCore for Backward<T> {
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core_0_5::Error> {
        // Attempt to fill bytes
        let e = match self.0.try_fill_bytes(dest) {
            Ok(_) => return Ok(()),
            Err(e) => e,
        };

        // Map errors via code if available
        if let Some(c) = e.code() {
            return Err(rand_core_0_5::Error::from(c));
        }

        // Otherwise we have to return an unknown error
        let c = unsafe { core::num::NonZeroU32::new_unchecked(getrandom::Error::CUSTOM_START) };
        Err(rand_core_0_5::Error::from(c))
    }
}

/// Backward [`rand_core_0_5::CryptoRng`] marker for [`rand_core_0_6::CryptoRng`] types
impl<T: rand_core_0_6::RngCore + rand_core_0_6::CryptoRng> rand_core_0_5::CryptoRng
    for Backward<T>
{
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
