// operator-sugar
// Copyright (C) SOFe
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This crate provides simple macros that serve as syntactic sugar to make overloading operators
//! in Rust easier.
//!
//! The basic syntax for binary operators is in this format:
//!
//! ```
//! # use operator_sugar::*;
//! struct Left(i32);
//! struct Right(i32);
//! struct Answer(i32);
//!
//! operator!(Left, Right: a + b -> Answer {
//!     // Implementation of the operator function here
//!     Answer(a.0 + b.0)
//! });
//!
//! ```
//!
//! # Meta Attributes
//! Attributes can be applied to the `impl` block (which implements e.g. `Add`) and the `fn` block respectively:
//!
//! ```
//! # use operator_sugar::*;
//! struct Left(i32);
//! struct Right(i32);
//! struct Answer(i32);
//!
//! operator!(
//!     #[doc("This attribute will be applied on the `impl` block")] Left, Right:
//!     #[doc("This attribute will be applied on the `fn` block")] a + b -> Answer {
//!         Answer(a.0 + b.0)
//!     });
//! ```
//!
//! # Generics
//! Generics can be used on the three types and on the `impl` block.
//!
//! Due to disambiguation, generic parameters for the `impl` block must be written in `{}` rather
//! than `<>`.
//!
//! ```
//! # use operator_sugar::*;
//! use core::ops::Add;
//! struct Left<T>(T) where T: Add<i32, Output = i32>;
//! struct Right(i32);
//! struct Answer(i32);
//!
//! operator!(
//!     {T: Add<i32, Output = i32>}
//!     Left<T>, Right: a + b -> Answer {
//!         Answer(a.0 + b.0)
//!     });
//! ```
//!
//! # List of operators
//! For conciseness, these definitions are defined for each of the following examples:
//! ```no_run
//! use operator_sugar::*;
//! #[derive(Debug)] struct Left(i32);
//! #[derive(Debug)] struct Right(i32);
//! #[derive(Debug, Eq, PartialEq)] struct Answer(i32);
//! ```
//!
//! ## Addition
//! ```
//! # use operator_sugar::*;
//! # #[derive(Debug)] struct Left(i32);
//! # #[derive(Debug)] struct Right(i32);
//! # #[derive(Debug, Eq, PartialEq)] struct Answer(i32);
//! #
//! operator!(Left, Right: a + b -> Answer {
//!     Answer(a.0 + b.0)
//! });
//!
//! fn main() {
//!     assert_eq!(Left(1) + Right(2), Answer(3));
//! }
//! ```
//!
//! ## Subtraction
//! ```
//! # use operator_sugar::*;
//! # #[derive(Debug)] struct Left(i32);
//! # #[derive(Debug)] struct Right(i32);
//! # #[derive(Debug, Eq, PartialEq)] struct Answer(i32);
//! #
//! operator!(Left, Right: a - b -> Answer {
//!     Answer(a.0 - b.0)
//! });
//!
//! fn main() {
//!     assert_eq!(Left(1) - Right(2), Answer(-1));
//! }
//! ```
//!
//! ## Multiplication
//! ```
//! # use operator_sugar::*;
//! # #[derive(Debug)] struct Left(i32);
//! # #[derive(Debug)] struct Right(i32);
//! # #[derive(Debug, Eq, PartialEq)] struct Answer(i32);
//! #
//! operator!(Left, Right: a * b -> Answer {
//!     Answer(a.0 * b.0)
//! });
//!
//! fn main() {
//!     assert_eq!(Left(3) * Right(2), Answer(6));
//! }
//! ```
//!
//! ## Division
//! ```
//! # use operator_sugar::*;
//! # #[derive(Debug)] struct Left(i32);
//! # #[derive(Debug)] struct Right(i32);
//! # #[derive(Debug, Eq, PartialEq)] struct Answer(i32);
//! #
//! operator!(Left, Right: a / b -> Answer {
//!     Answer(a.0 / b.0)
//! });
//!
//! fn main() {
//!     assert_eq!(Left(8) / Right(2), Answer(4));
//! }
//! ```
//!
//! ## Remainder
//! ```
//! # use operator_sugar::*;
//! # #[derive(Debug)] struct Left(i32);
//! # #[derive(Debug)] struct Right(i32);
//! # #[derive(Debug, Eq, PartialEq)] struct Answer(i32);
//! #
//! operator!(Left, Right: a % b -> Answer {
//!     Answer(a.0 % b.0)
//! });
//!
//! fn main() {
//!     assert_eq!(Left(9) % Right(5), Answer(4));
//! }
//! ```
//!
//! ## Bitwise AND
//! ```
//! # use operator_sugar::*;
//! # #[derive(Debug)] struct Left(i32);
//! # #[derive(Debug)] struct Right(i32);
//! # #[derive(Debug, Eq, PartialEq)] struct Answer(i32);
//! #
//! operator!(Left, Right: a & b -> Answer {
//!     Answer(a.0 & b.0)
//! });
//!
//! fn main() {
//!     assert_eq!(Left(5) & Right(6), Answer(4));
//! }
//! ```
//!
//! ## Bitwise OR
//! ```
//! # use operator_sugar::*;
//! # #[derive(Debug)] struct Left(i32);
//! # #[derive(Debug)] struct Right(i32);
//! # #[derive(Debug, Eq, PartialEq)] struct Answer(i32);
//! #
//! operator!(Left, Right: a | b -> Answer {
//!     Answer(a.0 | b.0)
//! });
//!
//! fn main() {
//!     assert_eq!(Left(5) | Right(6), Answer(7));
//! }
//! ```
//!
//! ## Bitwise XOR
//! ```
//! # use operator_sugar::*;
//! # #[derive(Debug)] struct Left(i32);
//! # #[derive(Debug)] struct Right(i32);
//! # #[derive(Debug, Eq, PartialEq)] struct Answer(i32);
//! #
//! operator!(Left, Right: a ^ b -> Answer {
//!     Answer(a.0 ^ b.0)
//! });
//!
//! fn main() {
//!     assert_eq!(Left(5) ^ Right(6), Answer(3));
//! }
//! ```
//!
//! ## Shift-left
//! ```
//! # use operator_sugar::*;
//! # #[derive(Debug)] struct Left(i32);
//! # #[derive(Debug)] struct Right(i32);
//! # #[derive(Debug, Eq, PartialEq)] struct Answer(i32);
//! #
//! operator!(Left, Right: a << b -> Answer {
//!     Answer(a.0 << b.0)
//! });
//!
//! fn main() {
//!     assert_eq!(Left(5) << Right(3), Answer(40));
//! }
//! ```
//!
//! ## Shift-right
//! ```
//! # use operator_sugar::*;
//! # #[derive(Debug)] struct Left(i32);
//! # #[derive(Debug)] struct Right(i32);
//! # #[derive(Debug, Eq, PartialEq)] struct Answer(i32);
//! #
//! operator!(Left, Right: a >> b -> Answer {
//!     Answer(a.0 >> b.0)
//! });
//!
//! fn main() {
//!     assert_eq!(Left(43) >> Right(3), Answer(5));
//! }
//! ```
//!
//! ## Index
//! ```
//! # use operator_sugar::*;
//! #[derive(Debug)] struct Left(Vec<i32>);
//! #[derive(Debug)] struct Right(usize);
//!
//! operator!(Left, Right: a[b] -> &i32 {
//!     // The & is required to remind developers that a reference is to be returned.
//!     &a.0[b.0]
//! });
//!
//! fn main() {
//!     // We check for 6 not &6, because while the impl returns &6, the [] operator from Rust dereferences it.
//!     assert_eq!(Left(vec![5, 6, 7])[Right(1)], 6);
//! }
//! ```

#![no_std]

#[macro_export]
macro_rules! operator {
    (
        $(#[$impl_attr:meta])* $({ $($generics:tt)* })? $A:ty, $B:ty :
        $(#[$fn_attr:meta])* $a:ident + $b:ident -> $C:ty
        { $($body:tt)* }
    ) => {
        $(#[$impl_attr])*
        impl $(< $($generics)* >)? ::core::ops::Add<$B> for $A {
            type Output = $C;

            $(#[$fn_attr])*
                fn add(self, $b: $B) -> Self::Output {
                    let $a = self;
                    $($body)*
                }
        }
    };

    (
        $(#[$impl_attr:meta])* $({ $($generics:tt)* })? $A:ty, $B:ty :
        $(#[$fn_attr:meta])* $a:ident - $b:ident -> $C:ty
        { $($body:tt)* }
    ) => {
        $(#[$impl_attr])*
        impl $(< $($generics)* >)? ::core::ops::Sub<$B> for $A {
            type Output = $C;

            $(#[$fn_attr])*
                fn sub(self, $b: $B) -> Self::Output {
                    let $a = self;
                    $($body)*
                }
        }
    };

    (
        $(#[$impl_attr:meta])* $({ $($generics:tt)* })? $A:ty, $B:ty :
        $(#[$fn_attr:meta])* $a:ident * $b:ident -> $C:ty
        { $($body:tt)* }
    ) => {
        $(#[$impl_attr])*
        impl $(< $($generics)* >)? ::core::ops::Mul<$B> for $A {
            type Output = $C;

            $(#[$fn_attr])*
                fn mul(self, $b: $B) -> Self::Output {
                    let $a = self;
                    $($body)*
                }
        }
    };

    (
        $(#[$impl_attr:meta])* $({ $($generics:tt)* })? $A:ty, $B:ty :
        $(#[$fn_attr:meta])* $a:ident / $b:ident -> $C:ty
        { $($body:tt)* }
    ) => {
        $(#[$impl_attr])*
        impl $(< $($generics)* >)? ::core::ops::Div<$B> for $A {
            type Output = $C;

            $(#[$fn_attr])*
                fn div(self, $b: $B) -> Self::Output {
                    let $a = self;
                    $($body)*
                }
        }
    };

    (
        $(#[$impl_attr:meta])* $({ $($generics:tt)* })? $A:ty, $B:ty :
        $(#[$fn_attr:meta])* $a:ident % $b:ident -> $C:ty
        { $($body:tt)* }
    ) => {
        $(#[$impl_attr])*
        impl $(< $($generics)* >)? ::core::ops::Rem<$B> for $A {
            type Output = $C;

            $(#[$fn_attr])*
                fn rem(self, $b: $B) -> Self::Output {
                    let $a = self;
                    $($body)*
                }
        }
    };

    (
        $(#[$impl_attr:meta])* $({ $($generics:tt)* })? $A:ty, $B:ty :
        $(#[$fn_attr:meta])* $a:ident & $b:ident -> $C:ty
        { $($body:tt)* }
    ) => {
        $(#[$impl_attr])*
        impl $(< $($generics)* >)? ::core::ops::BitAnd<$B> for $A {
            type Output = $C;

            $(#[$fn_attr])*
                fn bitand(self, $b: $B) -> Self::Output {
                    let $a = self;
                    $($body)*
                }
        }
    };

    (
        $(#[$impl_attr:meta])* $({ $($generics:tt)* })? $A:ty, $B:ty :
        $(#[$fn_attr:meta])* $a:ident | $b:ident -> $C:ty
        { $($body:tt)* }
    ) => {
        $(#[$impl_attr])*
        impl $(< $($generics)* >)? ::core::ops::BitOr<$B> for $A {
            type Output = $C;

            $(#[$fn_attr])*
                fn bitor(self, $b: $B) -> Self::Output {
                    let $a = self;
                    $($body)*
                }
        }
    };

    (
        $(#[$impl_attr:meta])* $({ $($generics:tt)* })? $A:ty, $B:ty :
        $(#[$fn_attr:meta])* $a:ident ^ $b:ident -> $C:ty
        { $($body:tt)* }
    ) => {
        $(#[$impl_attr])*
        impl $(< $($generics)* >)? ::core::ops::BitXor<$B> for $A {
            type Output = $C;

            $(#[$fn_attr])*
                fn bitxor(self, $b: $B) -> Self::Output {
                    let $a = self;
                    $($body)*
                }
        }
    };

    (
        $(#[$impl_attr:meta])* $({ $($generics:tt)* })? $A:ty, $B:ty :
        $(#[$fn_attr:meta])* $a:ident << $b:ident -> $C:ty
        { $($body:tt)* }
    ) => {
        $(#[$impl_attr])*
        impl $(< $($generics)* >)? ::core::ops::Shl<$B> for $A {
            type Output = $C;

            $(#[$fn_attr])*
                fn shl(self, $b: $B) -> Self::Output {
                    let $a = self;
                    $($body)*
                }
        }
    };

    (
        $(#[$impl_attr:meta])* $({ $($generics:tt)* })? $A:ty, $B:ty :
        $(#[$fn_attr:meta])* $a:ident >> $b:ident -> $C:ty
        { $($body:tt)* }
    ) => {
        $(#[$impl_attr])*
        impl $(< $($generics)* >)? ::core::ops::Shr<$B> for $A {
            type Output = $C;

            $(#[$fn_attr])*
                fn shr(self, $b: $B) -> Self::Output {
                    let $a = self;
                    $($body)*
                }
        }
    };

    (
        $(#[$impl_attr:meta])* $({ $($generics:tt)* })? $A:ty, $B:ty :
        $(#[$fn_attr:meta])* $a:ident[$b:ident] -> & $C:ty
        { $($body:tt)* }
    ) => {
        $(#[$impl_attr])*
        impl $(< $($generics)* >)? ::core::ops::Index<$B> for $A {
            type Output = $C;

            $(#[$fn_attr])*
                fn index(&self, $b: $B) -> &Self::Output {
                    let $a = self;
                    $($body)*
                }
        }
    };
}
