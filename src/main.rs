use std::process;

fn main() {
    loop {
        you_dont_know::ask_to_crypt_one_file().unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1);
        });
    }
}
