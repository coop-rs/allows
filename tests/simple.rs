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

// NO need to mark functions as #[test], since all we check is compilation.
//#[test]
#[allows::prefix_attrs_before_item(
    #[allow(unused)]
)]
fn _prefix_attrs_before_item() {
    let unused = ();
}

#[allows::unused]
fn _unused() {
    //#[allow(unused)]
    let unused = ();
}
