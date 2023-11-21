
//       LOCALE
//-------------------------------------------------------

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
i18n!("locales");

// Or just use `i18n!`, default locales path is: "locales" in current crate.
//
// i18n!();

// Config fallback missing translations to "en" locale.
// Use `fallback` option to set fallback locale.
//
// i18n!("locales", fallback = "en");

//-------------------------------------------------------

pub mod vm;
pub mod assembler;

#[cfg(test)]
mod tests {
   // use super::*;


}
