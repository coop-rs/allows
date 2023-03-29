//#![deny(unknown_lints)]
//#![deny(unused)]

/// BEWARE: If you forget the exclamation mark, like:
///
/// `#[deny(unused)]`
///
/// instead of
///
/// `#![deny(unused)]`
///
/// then:
/// 1. such a #[deny(...)] applies only to the first item (function), and
/// 2. even worse: Even if you do add an appropriate `#[allows::...]` in front of that first item,
///    that `#[allows::...]` will NOT apply - because it will be "overriden" by the previous
///    (mistaken) `#[deny(...)]`.
/// 3. The rest of the code will not get that lint checked (of course).

// NO need to mark functions as `#[test]`, since all we check is compilation.
// But we do have `#[test]`, for peace of mind.

#[test]
fn test_unused() {
    _unused();
}

// The following two together have triggered an ICE.
#[allows::unused]
//#[allows::unused_braces]

// The following two together trigger an ICE.
// #[allows::array_into_iter]
// #[allows::clippy_assign_ops]

//#[allows::clippy_almost_swapped] // <--- problem

// Repeating the same fails, too - with #[thread_local]
#[allows::clippy_assign_ops]
//#[allows::clippy_assign_ops]

// Repeating the same fails, too - with #[thread_local]
//#[allows::unused]
//#[allows::unused]
fn _unused() {
    let unused = ();
}
