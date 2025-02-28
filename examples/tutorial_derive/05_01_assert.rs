use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Network port to use
    #[clap(parse(try_from_str))]
    port: usize,
}

fn main() {
    let cli = Cli::parse();

    println!("PORT = {}", cli.port);
}

#[test]
fn verify_app() {
    use clap::IntoApp;
    Cli::into_app().debug_assert()
}
