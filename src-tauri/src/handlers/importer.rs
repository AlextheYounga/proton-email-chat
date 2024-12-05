use std::fs;
use std::path::Path;
use mail_parser::*;
use rusqlite::{Connection, Result};


struct EmailMetadata {
    pub proton_id: String,
    pub data: String,
}
struct Email {
    pub proton_id: String,
    pub subject: String,
    pub sender_name: String,
    pub sender_address: String,
    pub content: String,
    pub timestamp: String,
	pub metadata: EmailMetadata,
}

fn db_session() -> Result<Connection> {
    let conn = Connection::open("sqlite:database.db")?;
    Ok(conn)
}

fn parse_email(path: &Path) -> Email {
    let data = fs::read(path).unwrap();
    let proton_id = path.file_stem().unwrap().to_str().unwrap();
    let message = MessageParser::default().parse(&data).unwrap();

    let email = Email {
        proton_id: String::from(proton_id),
        subject: String::from(message.subject().unwrap()),
        sender_name: String::from(message.from().unwrap().first().unwrap().name().unwrap()),
        sender_address: String::from(message.from().unwrap().first().unwrap().address().unwrap()),
        content: String::from(message.body_html(0).unwrap()),
        timestamp: message.date().unwrap().to_rfc3339(),
		metadata: parse_email_metadata(path),
    };

    return email;
}

fn parse_email_metadata(path: &Path) -> EmailMetadata {
	let metadata_file_path = path.with_extension(".metadata.json");
	if metadata_file_path.is_file() {
		let data = fs::read(metadata_file_path).unwrap();
		let proton_id = path.file_stem().unwrap().to_str().unwrap();
		let json_string = String::from_utf8(data).unwrap();
	
		let metadata = EmailMetadata {
			proton_id: String::from(proton_id),
			data: json_string,
		};

		return metadata;
	} else {
		return EmailMetadata {
			proton_id: String::from(""),
			data: String::from(""),
		};
	}
}

#[tauri::command]
pub fn import(folder_path: String) -> Result<(), String> {
	let db = db_session().unwrap();
    let path = Path::new(&folder_path);
    if !path.is_dir() {
        return Err("Provided path is not a directory".into());
    }

    let entries = fs::read_dir(path).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() && path.extension() == Some("eml".as_ref()) {
			let email = parse_email(&path);

			let email_record: std::result::Result<usize, rusqlite::Error> = db.execute(
				"INSERT INTO email (proton_id, subject, sender_name, sender_address, content, timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
				(&email.proton_id, &email.subject, &email.sender_name, &email.sender_address, &email.content, &email.timestamp),
			);

			println!("{:?}", email_record);

			db.execute(
				"INSERT INTO email_metadata (email_id, proton_id, data) VALUES (?1, ?2, ?3)",
				(&email_record.unwrap(), &email.metadata.proton_id, &email.metadata.data),
			).unwrap();
        }
    }
    Ok(())
}
