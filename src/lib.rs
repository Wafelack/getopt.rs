//! Parse options from command line arguments

/// Get long command line options.
///
/// This function works as [`getopt`](fn.getopt.html), but it also handles long options (e.g.,
/// `--help`).
///
/// The arguments are the same as `getopt`, but with a short - long reference array, containing
/// tuples with the short option and its long correspondance, such as `&[('a', "all"), ('h',
/// "help"), ('v', "version")]`.
///
/// ### Example
///
/// ```no_run
/// use std::env;
/// use getopt_rs::getopt_long;
///
/// fn main() {
///     let mut args = env::args().collect();
///     while let Some(opt) = 
///         getopt_long(&mut args, "hv", &[('h', "help"), ('v', "version")]) 
///     {
///         match opt {
///             ('\0', _) => return, /* An error occured. */
///             ('h', _) => println!("Insert help here."),
///             ('v', _) => println!("long-example 0.1.0."),
///             _ => break,
///         }
///     }
/// }
/// ```
pub fn getopt_long(
    args: &mut Vec<String>,
    optstring: &str,
    long_opts: &[(char, &str)],
) -> Option<(char, Option<String>)> {
    *args = args
        .into_iter()
        .map(|a| {
            let a = a.to_string();
            if let Some(idx) = long_opts.iter().position(|(_, s)| a == format!("--{}", s)) {
                format!("-{}", long_opts[idx].0)
            } else {
                a
            }
        })
        .collect();
    getopt(args, optstring)
}

/// Get command line options.
///
/// This function only gets the "small" command line options (i.e. the one-char-long ones, like
/// `-n`), to parse "long" options, see [`getopt_long`](fn.getopt_long.html)
///
/// `args` is simply the vector containing the command line arguments.  
/// `optstring` is a string reference, like `ab:c?`, `ab` or `hVv`.
/// > Each alphanumeric character is an option, and the following `:` and `?` respectively mean
/// > that it takes a mandatory or optional value.
///
/// If no matching option is found, `None` is returned.  
/// If a mandatory value is not given, an error message is displayed and `Some(('\0', None))` is
/// returned.  
/// If the option doesn't take an argument or if an optional argument is not given, `Some((opt,
/// None))` is returned.
///
/// ### Example
///
/// ```no_run
/// use std::env;
/// use getopt_rs::getopt;
///
/// fn main() {
///     let mut args = env::args().collect();
///     while let Some(opt) = getopt(&mut args, "ab?c:") {
///         match opt {
///             (('\0', _)) => return, /* An error occured. */
///             ('a', _) => println!("Found option 'a' that takes no argument."),
///             ('b', val) => println!("Found option 'b' that takes an optional argument: {:?}.", val),
///             ('c', val) => println!("Found option 'c' that takes a mandatory argument: {:?}", val.unwrap()),
///             _ => break,
///         }
///     }
/// }
/// ```
pub fn getopt(args: &mut Vec<String>, optstring: &str) -> Option<(char, Option<String>)> {
    let mut optchars = optstring.chars();
    while let Some(c) = optchars.next() {
        let with_prefix = format!("-{}", c);
        if let Some(idx) = args.iter().position(|a| a == &with_prefix) {
            args.remove(idx);
            let value = if let Some(n) = optchars.next() {
                let available = idx >= args.len();
                if available {
                    if n == ':' {
                        eprintln!("{}: Option '{}' requires an argument.", args[0], c);
                        return Some(('\0', None));
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
    }
    None
}
