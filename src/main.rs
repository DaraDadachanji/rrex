use clap::Parser;
use std::io::{self, BufRead, Write};
use regex::Regex;


// Simple regex capture groups extraction tool
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // Regex pattern
    pattern: String,
    
    // String to separate multiple capture groups
    #[arg(short, long, default_value=" ")]
    delimiter: String,

    // Include unmatched lines in the output
    #[arg(short, long, default_value="false")]
    include_unmatched_lines: bool,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let re = Regex::new(args.pattern.as_str()).unwrap();

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    while let Some(Ok(line)) = lines.next() {
        let Some(caps) = re.captures(&line) else {
            if args.include_unmatched_lines {
                writeln!(handle,"{}", line)?;
            };
            continue;
        };
        let groups: Vec<&str> = caps.iter()
            .skip(1)  // Skip the full match
            .map(|m| m.map_or("", |v| v.as_str()))
            .collect();
        let output = groups.join(&args.delimiter);
        writeln!(handle,"{}", output)?;
    }

    Ok(())
}

