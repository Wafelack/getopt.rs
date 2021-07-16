//! Parse options from command line arguments

/// Get command line options.
///
/// `args` is the vector containing the command line arguments.  
/// `optstring` is a string reference, like `ab:c?`, `ab` or `hVv`.
/// > Each alphanumeric character is an option, and the following `:` and `?` respectively mean
/// > that it takes a mandatory or optional value.  
///
/// `long_opts` is the short - long options reference table, containing the long equivalent for
/// short options. An option can be present only in the short form.
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
/// use std::env;
/// use getopt_rs::getopt;
///
/// fn main() {
///     let mut args = env::args().collect();
///     while let Some(opt) = getopt(&mut args, "ab?c:", &[('a', "all"), ('b', "byte")]) {
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
    long_opts: &[(char, &str)],
) -> Option<(char, Option<String>)> {
    let mut optchars = optstring.chars();
    while let Some(c) = optchars.next() {
        let short_prefix = format!("-{}", c);
        let long_prefix = if let Some(idx) = long_opts.iter().position(|(ch, _)| c == *ch) {
            let opt = long_opts[idx].1;
            Some((format!("--{}", opt), format!("-{}", opt)))
        } else {
            None
        };
        if let Some(idx) = args.iter().position(|a| a == &short_prefix) {
            args.remove(idx);
            return procopt(args, c, idx, optchars.next());
        } else if let Some(idx) = args
            .iter()
            .position(|a| a.contains(c) && a.starts_with('-') && a.len() > 1)
        {
            let cidx = if let Some(idx) = args[idx].chars().position(|a| a == c) {
                idx
            } else {
                unreachable!()
            };
            args[idx].remove(cidx);
            return procopt(args, c, idx, optchars.next());
        } else if let Some((short, long)) = long_prefix {
            if let Some(idx) = args.iter().position(|a| a == &long) {
                args.remove(idx);
                return procopt(args, c, idx, optchars.next());
            } else if let Some(idx) = args.iter().position(|a| a == &short) {
                args.remove(idx);
                return procopt(args, c, idx, optchars.next());
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
