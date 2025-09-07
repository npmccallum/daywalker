//! # nightly - Conditional Nightly Code Inclusion
//!
//! This crate enables the sharing of code between nightly and stable Rust by
//! providing conditional inclusion syntax. It is small and lightweight. It works
//! on a simple principle: use `++[...]` to include code only on nightly with the
//! `nightly` feature enabled, and `--[...]` to include code only on stable
//! without the feature. That's it!
//!
//! When the nightly features you're using are stabilized, you can remove the
//! conditional prefixes and remove the use of this crate.
//!
//! ## Example
//!
//! This is the canonical example of the const trait syntax, adapted to use this
//! crate. At the time of this writing, the const trait syntax is only available
//! on nightly. This feature requires a syntax change, which makes it difficult
//! to share code between nightly and stable. Using this crate, however, we can
//! write the same codebase for both nightly and stable by using the
//! conditional inclusion syntax.
//!
//! ```rust
//! #![cfg_attr(feature = "nightly", feature(const_trait_impl))]
//!
//! nightly::nightly! {
//!     pub ++[const] trait Default {
//!         fn default() -> Self;
//!     }
//!
//!     impl ++[const] Default for () {
//!         fn default() -> Self {}
//!     }
//!
//!     pub struct Thing<T>(pub T);
//!
//!     impl<T: ++[[const]] Default> ++[const] Default for Thing<T> {
//!         fn default() -> Self {
//!             Self(T::default())
//!         }
//!     }
//!
//!     pub ++[const] fn default<T: ++[[const]] Default>() -> T {
//!         T::default()
//!     }
//!
//!     #[allow(unused_braces)]
//!     pub fn compile_time_default<T: ++[const] Default>() -> T {
//!         ++[const] { T::default() }
//!     }
//! }
//! ```

extern crate proc_macro;

use proc_macro::{Group, Spacing, TokenStream, TokenTree};

trait Process {
    fn process(self) -> TokenStream;
}

impl Process for Group {
    fn process(self) -> TokenStream {
        let mut grp = Group::new(self.delimiter(), self.stream().process());
        grp.set_span(self.span());
        TokenTree::Group(grp).into()
    }
}

impl Process for TokenStream {
    fn process(self) -> TokenStream {
        let mut stream = TokenStream::new();
        let mut prev = [None, None];

        for token in self {
            match (prev[0].take(), prev[1].take(), token) {
                // Save the first '+' or '-' if it is joint.
                (None, None, TokenTree::Punct(p))
                    if "+-".contains(p.as_char()) && p.spacing() == Spacing::Joint =>
                {
                    prev[1] = Some(TokenTree::Punct(p));
                }

                // Save the second '+' or '-' if it is alone.
                (None, Some(TokenTree::Punct(p)), TokenTree::Punct(q))
                    if p.as_char() == q.as_char() && q.spacing() == Spacing::Alone =>
                {
                    prev[0] = Some(TokenTree::Punct(p));
                    prev[1] = Some(TokenTree::Punct(q));
                }

                // If we have a '+' or '-' pair followed by a group, conditionalize it.
                (Some(TokenTree::Punct(p)), Some(TokenTree::Punct(_)), TokenTree::Group(grp)) => {
                    if (p.as_char() == '+') == cfg!(feature = "nightly") {
                        stream.extend(grp.stream());
                    }
                }

                // Otherwise, just emit what we have and continue.
                (p, q, tt) => {
                    for tt in [p, q, Some(tt)] {
                        match tt {
                            // If we see a group, recurse into it.
                            Some(TokenTree::Group(grp)) => stream.extend(grp.process()),
                            Some(tt) => stream.extend(TokenStream::from(tt)),
                            None => {}
                        }
                    }
                }
            }
        }

        stream
    }
}

/// Emits conditionally included code based on nightly feature availability.
///
/// - `++[...]` includes content only when `feature = "nightly"` is enabled
/// - `--[...]` includes content only when `feature = "nightly"` is disabled
///
/// The macro processes the input token stream and conditionally includes or
/// excludes bracketed content based on the feature flag. This enables writing
/// code that uses nightly features when available but falls back to stable
/// alternatives when not.
#[proc_macro]
pub fn nightly(input: TokenStream) -> TokenStream {
    input.process()
}
