#![deny(anonymous_parameters,
        const_err,
        dead_code,
        deprecated,
        deprecated_attr,
        exceeding_bitshifts,
        extra_requirement_in_impl,
        fat_ptr_transmutes,
        illegal_floating_point_literal_pattern,
        improper_ctypes,
        invalid_type_param_default,
        late_bound_lifetime_arguments,
        legacy_constructor_visibility,
        legacy_directory_ownership,
        legacy_imports,
        missing_copy_implementations,
        missing_docs,
        missing_fragment_specifier,
        mutable_transmutes,
        no_mangle_const_items,
        no_mangle_generic_items,
        non_camel_case_types,
        non_shorthand_field_patterns,
        non_snake_case,
        non_upper_case_globals,
        overflowing_literals,
        parenthesized_params_in_types_and_modules,
        path_statements,
        patterns_in_fns_without_body,
        plugin_as_library,
        private_in_public,
        private_no_mangle_fns,
        private_no_mangle_statics,
        pub_use_of_private_extern_crate,
        renamed_and_removed_lints,
        resolve_trait_on_defaulted_unit,
        safe_extern_statics,
        stable_features,
        trivial_numeric_casts,
        unconditional_recursion,
        unions_with_drop_fields,
        unknown_crate_types,
        unknown_lints,
        unreachable_code,
        unreachable_patterns,
        unsafe_code,
        unstable_features,
        unused_allocation,
        unused_assignments,
        unused_attributes,
        unused_comparisons,
        unused_doc_comment,
        unused_extern_crates,
        unused_features,
        unused_import_braces,
        unused_imports,
        unused_macros,
        unused_must_use,
        unused_mut,
        unused_parens,
        unused_qualifications,
        unused_unsafe,
        unused_variables,
        while_true)]
#![warn(unknown_lints)]
#![allow(box_pointers,
        missing_debug_implementations,
        trivial_casts,
        unused_results,
        variant_size_differences,
        warnings)]
#![cfg_attr(feature="clippy", feature(plugin))]

//! # shell2batch
//!
//! Coverts simple basic shell scripts to windows batch scripts.
//!
//! While it is not really possible to take every shell script and automatically convert it to a windows batch file,
//! this library provides a way to convert simple basic shell commands to windows batch commands.<br>
//! The original goal of this library is to provide users of [cargo-make](https://sagiegurari.github.io/cargo-make/) a
//! way to write simple tasks with shell scripts without duplicating their code for each platform.
//!
//! # Examples
//!
//! ```
//! extern crate shell2batch;
//!
//! fn main() {
//!     let script = shell2batch::convert("rm ./myfile.txt");
//!
//!     assert_eq!(script, "del ./myfile.txt");
//!     println!("Script: {}", script);
//! }
//! ```
//!
//! # Installation
//! In order to use this library, just add it as a dependency:
//!
//! ```ini
//! [dependencies]
//! shell2batch = "*"
//! ```
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/shell2batch/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/shell2batch/blob/master/LICENSE) open source license.
//!

#[cfg(test)]
#[path = "./lib_test.rs"]
mod lib_test;

mod converter;

/// Converts the provided shell script and returns the windows batch script text.
///
/// # Example
///
/// ```
/// extern crate shell2batch;
///
/// fn main() {
///     let script = shell2batch::convert("rm ./myfile.txt");
///
///     assert_eq!(script, "del ./myfile.txt");
///     println!("Script: {}", script);
/// }
/// ```
pub fn convert(script: &str) -> String {
    converter::run(script)
}
