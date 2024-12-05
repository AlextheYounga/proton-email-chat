use tauri_plugin_sql::Migration;
use tauri_plugin_sql::MigrationKind;

pub struct EmailMetadata;

impl EmailMetadata {
    pub fn migration() -> Migration {
        return Migration {
            version: 2,
            description: "create email_metadata table",
            sql: "CREATE TABLE IF NOT EXISTS email_metadata (
				id INTEGER PRIMARY KEY,
				email_id INTEGER NOT NULL,
				proton_id TEXT NOT NULL,
				data TEXT NOT NULL,
				FOREIGN KEY(email_id) REFERENCES emails(id)
			);",
            kind: MigrationKind::Up,
        };
    }
}
