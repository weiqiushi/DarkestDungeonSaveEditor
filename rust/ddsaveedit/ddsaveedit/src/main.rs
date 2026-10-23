use ddsavelib::{File, Unhasher};
use std::{
    io::{BufReader, BufWriter, Write},
    path::Path,
};

const USAGE: &str = "Usage: ddsaveedit decode|encode <input_file_path> <output_file_path>";

enum Command {
    Decode,
    Encode,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("ddsaveedit: {}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() == 1 && (args[0] == "--help" || args[0] == "-h") {
        println!("{}", USAGE);
        return Ok(());
    }
    if args.len() != 3 {
        return Err(USAGE.to_owned());
    }
    let command = match args[0].to_str() {
        Some("decode") => Command::Decode,
        Some("encode") => Command::Encode,
        _ => {
            return Err(format!(
                "Unknown command: {}\n{}",
                args[0].to_string_lossy(),
                USAGE
            ))
        }
    };
    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);
    let input = std::fs::File::open(input_path)
        .map_err(|e| format!("Cannot open input '{}': {}", input_path.display(), e))?;
    let reader = BufReader::new(input);
    // Parse the entire input before creating or truncating the output file.
    let file = match command {
        Command::Decode => File::try_from_bin(reader)
            .map_err(|e| format!("Cannot decode '{}': {}", input_path.display(), e))?,
        Command::Encode => File::try_from_json(reader)
            .map_err(|e| format!("Cannot encode '{}': {}", input_path.display(), e))?,
    };

    let output = std::fs::File::create(output_path)
        .map_err(|e| format!("Cannot create output '{}': {}", output_path.display(), e))?;
    let mut writer = BufWriter::new(output);
    let result = match command {
        Command::Decode => file.write_to_json(&mut writer, true, &Unhasher::empty()),
        Command::Encode => file.write_to_bin(&mut writer),
    };
    result
        .and_then(|_| writer.flush())
        .map_err(|e| format!("Cannot write output '{}': {}", output_path.display(), e))
}
