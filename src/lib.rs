#![deny(missing_docs)]
//! Parse options from command line arguments

#[derive(PartialEq, Eq)]
/// An option long form rule.
///
/// `DoubleDash` allows `--foo`.
/// `Both` allows both `-foo` and `--foo`.
pub enum LongForm {
    /// Allows the `-foo` long option form.
    SimpleDash,
    /// Allows the `--foo` long option form.
    DoubleDash,
    /// Allows both the `--foo` and `-foo` long option form.
    Both,
}

/// A macro used to generate `Opt`s.
///
/// Usage:
/// - `opt!('c' /* Short form and identifier */)`
/// - `opt!('c', "ctrl" /* Long form */)`
/// - `opt!('c', "ctrl", LongForm::SimpleDash /* Long rule */)`
/// - `opt!('c', "ctrl", LongForm::SimpleDash, true /* No short flag */)`
///
/// If not specified, the fields are set to their defaults.
#[macro_export]
macro_rules! opt {
    ($short:literal) => {
        $crate::Opt {
            long_rule: $crate::LongForm::DoubleDash,
            short: $short,
            long: None,
            no_short: false,
        }
    };
    ($short:literal, $long:literal) => {
        $crate::Opt {
            long_rule: $crate::LongForm::DoubleDash,
            short: $short,
            long: Some($long),
            no_short: false,
        }
    };
    ($short:literal, $long:literal, $long_rule:expr) => {
        $crate::Opt {
            long_rule: $long_rule,
            short: $short,
            long: Some($long),
            no_short: false,
        }
    };
    ($short:literal, $long:literal, $long_rule:expr, $no_short:expr) => {
        $crate::Opt {
            long_rule: $long_rule,
            short: $short,
            long: Some($long),
            no_short: $no_short,
        }
    };
}

/// A command line option rule.
///
/// At least one option form must be present, `short` and `long` cannot both be `None`.
pub struct Opt<'a> {
    /// The long-option rule, telling how long options should be parsed.
    ///
    /// Default: `LongForm::DoubleDash`.
    pub long_rule: LongForm,
    /// The short form and identifier for the option.
    pub short: char,
    /// The long form for the option.
    ///
    /// Default: `None`.
    pub long: Option<&'a str>,
    /// Disallow the short form.
    ///
    /// Default: `false`.
    pub no_short: bool,
}
impl<'a> Opt<'a> {
    pub(crate) fn gen_long(&self) -> Vec<String> {
        let short = format!("-{}", self.short);
        if let Some((slong, dlong)) = self
            .long
            .and_then(|s| Some((format!("-{}", s), format!("--{}", s))))
        {
            let mut rules = vec![];

            if self.long_rule == LongForm::Both {
                rules.push(slong);
                rules.push(dlong);
            } else if self.long_rule == LongForm::SimpleDash {
                rules.push(slong);
            } else {
                rules.push(dlong);
            }

            if !self.no_short {
                rules.push(short);
            }

            rules
        } else {
            vec![short]
        }
    }
}

/// Get command line options.
///
/// `args` is the vector containing the command line arguments.  
/// `optstring` is a string reference, like `ab:c?`, `ab` or `hVv`.
/// > Each alphanumeric character is an option, and the following `:` and `?` respectively mean
/// > that it takes a mandatory or optional value.
///
/// `opts` are the option rules.
///
/// If no matching option is found, `None` is returned.
/// If a mandatory value is not given, an error message is displayed and `Some(('?', None))` is
/// returned.  
/// If the option doesn't take an argument or if an optional argument is not given, `Some((opt,
/// None))` is returned.
///
/// ### Example
///
/// ```no_run
/// #[macro_use]
/// extern crate getopt_rs;
///
/// use std::env;
/// use getopt_rs::{getopt, LongForm};
///
/// fn main() {
///     let mut args = env::args().collect();
///     while let Some(opt) = getopt(&mut args, 
///                                  "ab?c:", 
///                                  &[
///                                     opt!('a'), 
///                                     opt!('b', "bar"), 
///                                     opt!('c', "ctrl", LongForm::SimpleDash)]) 
///     {
///         match opt {
///             ('a', _) => println!("Found option 'a' that takes no argument."),
///             ('b', val) => println!("Found option 'b' that takes an optional argument: {:?}.", val),
///             ('c', val) => println!("Found option 'c' that takes a mandatory argument: {:?}", val.unwrap()),
///             _ => return, /* An error occured, Some(('?', None)) is returned. */
///         }
///     }
/// }
/// ```
pub fn getopt(
    args: &mut Vec<String>,
    optstring: &str,
    opts: &[Opt],
) -> Option<(char, Option<String>)> {
    opts.iter()
        .map(|opt| assert!(!(opt.no_short && opt.long.is_none())))
        .count();

    let mut optchars = optstring.chars();
    while let Some(c) = optchars.next() {
        if let Some(forms) = opts
            .iter()
            .find(|opt| opt.short == c)
            .and_then(|opt| Some(opt.gen_long()))
        {
            for form in forms {
                if let Some(idx) = args.iter().position(|a| a == &form) {
                    args.remove(idx);
                    return procopt(args, c, idx, optchars.next());
                }
            }
        }
    }
    None
}

fn procopt(
    args: &mut Vec<String>,
    c: char,
    idx: usize,
    next: Option<char>,
) -> Option<(char, Option<String>)> {
    let value = if let Some(n) = next {
        let available = idx < args.len();
        if !available {
            if n == ':' {
                eprintln!("{}: Option '{}' requires an argument.", args[0], c);
                return Some(('?', None));
            } else {
                None
            }
        } else if n == ':' || (n == '?' && available) {
            let v = args[idx].clone();
            args.remove(idx);
            Some(v)
        } else {
            None
        }
    } else {
        None
    };
    return Some((c, value));
}
