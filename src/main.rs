use std::env;
use std::fs;
use std::io::{self};
use base64::{engine::general_purpose, Engine as _};

fn xor_encrypt(data: &[u8], key: &str) -> Vec<u8> {
    let key_bytes = key.as_bytes();
    let mut result = Vec::with_capacity(data.len());

    for (i, byte) in data.iter().enumerate() {
        result.push(byte ^ key_bytes[i % key_bytes.len()]);
    }

    result
}

fn process_encrypt(file_path: &str, key: &str) -> io::Result<()> {
    let contents = fs::read(file_path)?;
    let encrypted = xor_encrypt(&contents, key);
    let encoded = general_purpose::STANDARD.encode(encrypted);

    let out_path = format!("{}.enc", file_path);
    fs::write(out_path.clone(), encoded)?;
    println!("Encrypted and ASCII-safe. Saved to {}", out_path);
    Ok(())
}

fn process_decrypt(file_path: &str, key: &str) -> io::Result<()> {
    let encoded = fs::read_to_string(file_path)?;
    let decoded = general_purpose::STANDARD.decode(encoded.trim())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    let decrypted = xor_encrypt(&decoded, key); // same function, works both ways

    let out_path = if file_path.ends_with(".enc") {
        file_path.trim_end_matches(".enc").to_string()
    } else {
        format!("{}_decrypted", file_path)
    };

    fs::write(&out_path, decrypted)?;
    println!("Decryption complete. Output: {}", out_path);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <encrypt|decrypt> <file_path> <key>", args[0]);
        return;
    }

    let cmd = &args[1];
    let file_path = &args[2];
    let key = &args[3];

    let result = match cmd.as_str() {
        "encrypt" => process_encrypt(file_path, key),
        "decrypt" => process_decrypt(file_path, key),
        _ => {
            eprintln!("Invalid command. Use 'encrypt' or 'decrypt'");
            return;
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}
