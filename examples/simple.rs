fn main() {
    let mut args = std::env::args().collect();
    while let Some(opt) = getopt_rs::getopt(&mut args, "ab?c:") {
        match opt {
            ('\0', _) => return, /* An error occured. */
            ('a', _) => println!("Found option 'a', that takes no argument."),
            ('b', val) => println!(
                "Found option 'b', that takes an optional argument: {:?}.",
                val
            ),
            ('c', val) => println!(
                "Found option 'c', that takes a mandatory argument: {}.",
                val.unwrap()
            ),
            _ => break,
        }
    }
}
