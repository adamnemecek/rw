use clap::Parser;

fn open_file(path: &str, append: bool) -> std::io::Result<std::fs::File> {
    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(!append)
        .append(append)
        .open(path)
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Append to file instead of overwriting
    #[arg(short)]
    append: bool,

    /// File to write into, uses stdout if unspecified
    path: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut writer: Box<dyn std::io::Write> = match args.path {
        Some(ref path) => Box::new(open_file(path, args.append)?),
        None => Box::new(std::io::stdout()),
    };

    std::io::copy(&mut std::io::stdin(), &mut writer)?;
    Ok(())
}
