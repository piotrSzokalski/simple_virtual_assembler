// pub mod flag;
// pub mod opcodes;
// pub mod virtual_machine;
// pub mod instruction;
// pub mod operand;
// pub mod assembler;

//       TEMP LOCALE
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


   #[test]
   fn test() {
      println!("_____________________________________--");
      println!("{}", t!("hello"));

      rust_i18n::set_locale("pl");

      println!("_____________________________________--");
      println!("{}", t!("hello"));
   }



}
