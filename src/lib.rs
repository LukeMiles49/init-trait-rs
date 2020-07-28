//! A helper trait to inialise data structures using a function applied to each 'index'.
//!
//! This is intended to simplify the initialisation of 'indexable' data structures to non-default
//! values. For example, if you wanted to initialise a long or arbitrary length array, you would
//! need to first initialise it to some default value, then modify each element to the value you
//! want:
//!
//! ```rust
//! // struct needs to implement either Copy or Default to initialise the array
//! #[derive(Copy, Clone)]
//! struct House { number: usize }
//!
//! // Need to first initialise road to some dummy value to avoid the error:
//! //     road[i] = House { number: i };
//! //     ^^^^^^^ use of possibly-uninitialized `road`
//! let mut road = [House { number: 0 }; 3];
//!
//! for i in 0..3 {
//!     road[i] = House { number: i + 1 };
//! }
//!
//! assert_eq!(road[0].number, 1);
//! assert_eq!(road[1].number, 2);
//! assert_eq!(road[2].number, 3);
//! ```
//!
//! This would be difficult for a generic type or a type with no default.
//!
//! With `Init` you can instead provide a function to generate the element given the index:
//!
//! ```rust
//! use init_trait::Init;
//!
//! struct House { number: usize }
//!
//! // [T; N]: Init<T, usize>
//! let road = <[House; 3]>::init(|i| House { number: i + 1 });
//!
//! assert_eq!(road[0].number, 1);
//! assert_eq!(road[1].number, 2);
//! assert_eq!(road[2].number, 3);
//! ```
//!
#![cfg_attr(feature = "std", doc = r##"
This also works for types which need some additional information to initialise, such as a
run-time length:

```rust
use init_trait::Init;

struct House { number: usize }

// Vec<T>: Init<T, usize, usize>
let road = Vec::<House>::init_with(3, |i| House { number: i + 1 });

assert_eq!(road[0].number, 1);
assert_eq!(road[1].number, 2);
assert_eq!(road[2].number, 3);
```
"##)]

#![no_std]

#![feature(const_generics)]

#![doc(html_root_url = "https://docs.rs/init_trait/0.1.0")]

// FIXME (#20041): Replace this workaround with real type equality constraints
mod type_equals;
use type_equals::TypeEquals;

use core::marker::Sized;
use core::mem::{MaybeUninit, transmute_copy, forget};

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use std::vec::Vec;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::vec::Vec;

/// Types which can be initialised by applying a function to each 'index' of the type.
pub trait Init<T, I, V = ()>: Sized {
	/// Initialise an instance of this type using `value` by applying `elem` to each 'index' of the type.
	///
	/// # Examples
	///
	#[cfg_attr(feature = "std", doc = r##"
	Constructing a Vec containing the values 0 to 4:
	
	```rust
	use init_trait::Init;
	
	let vec = Vec::<usize>::init_with(5, |i| i);
	
	assert_eq!(vec, vec![0, 1, 2, 3, 4]);
	```
	"##)]
	fn init_with<F: FnMut(I) -> T>(value: V, elem: F) -> Self;
	
	/// Initialise an instance of this type by applying `elem` to each 'index' of the type.
	///
	/// This is syntax sugar for `init_with((), elem)`.
	///
	/// # Examples
	///
	/// Constructing an array containing the values 0 to 4:
	///
	/// ```rust
	/// use init_trait::Init;
	///
	/// let arr = <[usize; 5]>::init(|i| i);
	///
	/// assert_eq!(arr, [0, 1, 2, 3, 4]);
	/// ```
	fn init<F: FnMut(I) -> T>(elem: F) -> Self where V: TypeEquals<()> {
		Self::init_with(().into(), elem)
	}
}

impl<T, const N: usize> Init<T, usize> for [T; N] {
	fn init_with<F: FnMut(usize) -> T>(_: (), mut elem: F) -> Self {
		let mut contents: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
		
		for i in 0..N {
			contents[i] = MaybeUninit::new(elem(i));
		}
		
		// FIXME: Replace with transmute once it works with const generic array sizes
		let res = unsafe { transmute_copy(&contents) };
		forget(contents);
		res
	}
}

impl<T, const N1: usize, const N2: usize> Init<T, [usize; 2]> for [[T; N1]; N2] {
	fn init_with<F: FnMut([usize; 2]) -> T>(_: (), mut elem: F) -> Self {
		Self::init(|i2| <[T; N1]>::init(|i1| elem([i1, i2])))
	}
}

impl<T, const N1: usize, const N2: usize, const N3: usize> Init<T, [usize; 3]> for [[[T; N1]; N2]; N3] {
	fn init_with<F: FnMut([usize; 3]) -> T>(_: (), mut elem: F) -> Self {
		Self::init(|i3| <[[T; N1]; N2]>::init(|[i1, i2]: [usize; 2]| elem([i1, i2, i3])))
	}
}

impl<T, const N1: usize, const N2: usize, const N3: usize, const N4: usize> Init<T, [usize; 4]> for [[[[T; N1]; N2]; N3]; N4] {
	fn init_with<F: FnMut([usize; 4]) -> T>(_: (), mut elem: F) -> Self {
		Self::init(|i4| <[[[T; N1]; N2]; N3]>::init(|[i1, i2, i3]: [usize; 3]| elem([i1, i2, i3, i4])))
	}
}

impl<T, const N1: usize, const N2: usize, const N3: usize, const N4: usize, const N5: usize> Init<T, [usize; 5]> for [[[[[T; N1]; N2]; N3]; N4]; N5] {
	fn init_with<F: FnMut([usize; 5]) -> T>(_: (), mut elem: F) -> Self {
		Self::init(|i5| <[[[[T; N1]; N2]; N3]; N4]>::init(|[i1, i2, i3, i4]: [usize; 4]| elem([i1, i2, i3, i4, i5])))
	}
}

impl<T, const N1: usize, const N2: usize, const N3: usize, const N4: usize, const N5: usize, const N6: usize> Init<T, [usize; 6]> for [[[[[[T; N1]; N2]; N3]; N4]; N5]; N6] {
	fn init_with<F: FnMut([usize; 6]) -> T>(_: (), mut elem: F) -> Self {
		Self::init(|i6| <[[[[[T; N1]; N2]; N3]; N4]; N5]>::init(|[i1, i2, i3, i4, i5]: [usize; 5]| elem([i1, i2, i3, i4, i5, i6])))
	}
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> Init<T, usize, usize> for Vec<T> {
	fn init_with<F: FnMut(usize) -> T>(length: usize, mut elem: F) -> Self {
		let mut value = Vec::with_capacity(length);
		
		for i in 0..length {
			value.push(elem(i));
		}
		
		value
	}
}
