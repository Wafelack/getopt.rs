fn main() {
    let mut args = std::env::args().collect();
    while let Some(opt) =
        getopt_rs::getopt_long(&mut args, "hv", &[('h', "help"), ('v', "version")])
    {
        match opt {
            ('\0', _) => return, /* An error occured. */
            ('h', _) => println!("Insert help here."),
            ('v', _) => println!("long-example 0.1.0."),
            _ => break,
        }
    }
}