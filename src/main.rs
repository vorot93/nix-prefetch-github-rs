use docopt::Docopt;
use serde::Deserialize;

const USAGE: &'static str = "
Prefetch sources from github for nix build tool.

Usage:
  nix-prefetch-github [--rev=<rev>] <owner> <repo>
  nix-prefetch-github (-h | --help)
  nix-prefetch-github --version

Options:
  -h --help     Show this screen.
  --version     Show version.
  --rev=<rev>   Revision to download.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_owner: String,
    arg_repo: String,
    flag_rev: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {}
