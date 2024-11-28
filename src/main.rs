use xibao_gen::program_entry;

fn main() {
    if let Err(err) = program_entry() {
        eprintln!("Fatal error: {}", err);
    }
}
