use std::fs::File;
use std::io::{self, Write};
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

#[tauri::command]
pub fn get_key() -> Result<String, String> {
    fs::read_to_string("./key.bin").map_err(|e| e.to_string()) // Reads the file content as a String
}

fn generate_secure_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
}

pub fn generate_key_file() -> io::Result<()> {
	if fs::metadata("./key.bin").is_ok() {
		return Ok(());
	}

	let key = generate_secure_string(16);
    let mut file = File::create("./key.bin")?; // Creates or overwrites the file
    file.write_all(key.as_bytes())?;    // Writes the content to the file
    Ok(())
}

