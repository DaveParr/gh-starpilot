use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hi {}!", args.name);
    }
}

#[cfg(test)]
// test main with dave input
mod tests {
    use assert_cmd::Command;
    use predicates::str::contains;

    #[test]
    fn test_main() {
        let mut cmd = Command::cargo_bin("gh-starpilot").unwrap();
        cmd.arg("--name").arg("Dave").arg("--count").arg("3");

        cmd.assert()
            .success()
            .stdout(contains("Hi Dave!\nHi Dave!\nHi Dave!\n"));
    }
}
