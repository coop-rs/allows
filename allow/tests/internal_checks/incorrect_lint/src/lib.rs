#![forbid(unknown_lints)]

use allow_internal::generate_allow_attribute_macro_definition_standard;

#[macro_use]
mod wrapper_macros;

#[allow(unused_macros)]
macro_rules! generate_allow_attribute_macro_definition_internal {
    ( $lint_path:path, $new_macro_name:ident ) => {
    };
}

//standard_lint!(non_existing_lint_without_prefix);
//standard_lint!(unused);