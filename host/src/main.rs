use cp_shell::run;
use embedded_io_adapters::std::FromStd;

fn main() {
    let mut stdout = FromStd::new(std::io::stdout());
    if let Err(e) = run(&mut stdout) {
        panic!("Fatal error: {e}");
    }
}
