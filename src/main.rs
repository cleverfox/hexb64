use std::env;
use std::io::{self, Read};
use std::path::Path;
use std::process;

use base64::{engine::general_purpose, Engine as _};

#[derive(Debug, Clone, Copy)]
enum Mode {
    B64ToHex,
    HexToB64,
}

#[derive(Debug, Clone, Copy)]
enum HexCase {
    Lower,
    Upper,
}

fn main() {
    // 1. Detect mode from executable name
    let exe = env::args()
        .next()
        .unwrap_or_else(|| "hexb64".to_string());

    let exe_name = Path::new(&exe)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("hexb64");

    let mode = match exe_name {
        "b64hex" => Mode::B64ToHex,
        "hexb64" => Mode::HexToB64,
        other => {
            eprintln!(
                "Unknown mode for executable name: {other}\n\
                 Use hardlinks named 'b64hex' or 'hexb64'."
            );
            process::exit(1);
        }
    };

    // 2. Parse flags and positional data arg
    let mut hex_case = HexCase::Lower;
    let mut b64_urlsafe = false;
    let mut data_arg: Option<String> = None;

    let mut args_iter = env::args().skip(1);

    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-low" => {
                hex_case = HexCase::Lower;
            }
            "-up" => {
                hex_case = HexCase::Upper;
            }
            "-url" => {
                b64_urlsafe = true;
            }
            _ if !arg.starts_with('-') && data_arg.is_none() => {
                data_arg = Some(arg);
            }
            _ => {
                eprintln!("Unknown or misplaced argument: {arg}");
                print_usage(mode);
                process::exit(1);
            }
        }
    }

    // 3. Get input: from arg or stdin
    let input_raw = match data_arg {
        Some(s) => s,
        None => match read_stdin() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to read stdin: {e}");
                process::exit(1);
            }
        },
    };

    // Normalize: drop all whitespace
    let input: String = input_raw.split_whitespace().collect();

    if input.is_empty() {
        eprintln!("No input data provided.");
        print_usage(mode);
        process::exit(1);
    }

    // 4. Do conversion based on mode
    let result = match mode {
        Mode::B64ToHex => b64_to_hex(&input, hex_case),
        Mode::HexToB64 => hex_to_b64(&input, b64_urlsafe),
    };

    match result {
        Ok(out) => {
            println!("{out}");
        }
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
    }
}

/// Print short usage depending on mode.
fn print_usage(mode: Mode) {
    match mode {
        Mode::B64ToHex => {
            eprintln!(
                "Usage: b64hex [-low|-up] [data]\n\
                 -low   Hex output lowercase (default)\n\
                 -up    Hex output uppercase\n\
                 data   Base64 input (classic or URL-safe). If omitted, read from stdin."
            );
        }
        Mode::HexToB64 => {
            eprintln!(
                "Usage: hexb64 [-url] [data]\n\
                 -url   Use URL-safe base64 output\n\
                 data   Hex input (0x prefix allowed, any case). If omitted, read from stdin."
            );
        }
    }
}

/// Read all of stdin to a String.
fn read_stdin() -> io::Result<String> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

/// Convert base64 (classic or URL-safe) to hex.
fn b64_to_hex(input: &str, hex_case: HexCase) -> Result<String, String> {
    // Try classic base64 first
    let bytes = match general_purpose::STANDARD.decode(input) {
        Ok(b) => b,
        Err(_) => {
            // Fallback to URL-safe
            general_purpose::URL_SAFE
                .decode(input)
                .map_err(|e| format!("Failed to decode as classic or URL-safe base64: {e}"))?
        }
    };

    Ok(bytes_to_hex(&bytes, hex_case))
}

/// Convert hex (0x prefix allowed) to base64.
fn hex_to_b64(input: &str, urlsafe: bool) -> Result<String, String> {
    let bytes = parse_hex(input)?;
    let encoded = if urlsafe {
        general_purpose::URL_SAFE.encode(bytes)
    } else {
        general_purpose::STANDARD.encode(bytes)
    };
    Ok(encoded)
}

/// Parse hex string into bytes. Supports:
/// - optional 0x/0X prefix
/// - upper/lowercase
/// - ignores whitespace
fn parse_hex(input: &str) -> Result<Vec<u8>, String> {
    let mut s = input.trim();

    if s.starts_with("0x") || s.starts_with("0X") {
        s = &s[2..];
    }

    // Remove any remaining whitespace
    let s: String = s.split_whitespace().collect();

    if s.is_empty() {
        return Err("Empty hex string".to_string());
    }

    if s.len() % 2 != 0 {
        return Err("Hex string must have even length".to_string());
    }

    let mut bytes = Vec::with_capacity(s.len() / 2);
    let chars: Vec<char> = s.chars().collect();

    for i in (0..chars.len()).step_by(2) {
        let hi = chars[i];
        let lo = chars[i + 1];
        let pair: String = [hi, lo].iter().collect();

        let byte = u8::from_str_radix(&pair, 16)
            .map_err(|e| format!("Invalid hex pair '{pair}': {e}"))?;
        bytes.push(byte);
    }

    Ok(bytes)
}

/// Convert bytes to hex string in chosen case.
fn bytes_to_hex(bytes: &[u8], hex_case: HexCase) -> String {
    match hex_case {
        HexCase::Lower => bytes.iter().map(|b| format!("{:02x}", b)).collect(),
        HexCase::Upper => bytes.iter().map(|b| format!("{:02X}", b)).collect(),
    }
}

