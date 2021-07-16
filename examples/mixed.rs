fn main() {
    let mut args = std::env::args().collect();
    while let Some(opt) = getopt_rs::getopt(&mut args, "hva:", &[('h', "help"), ('v', "version")]) {
        match opt {
            ('?', _) => return, /* An error occured. */
            ('h', _) => println!("Insert help here."),
            ('v', _) => println!("long-example 0.1.0."),
            ('a', v) => println!("a => {}", v.unwrap()),
            _ => break,
        }
    }
}
