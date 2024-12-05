import Database from "@tauri-apps/plugin-sql";
import { readEml } from "eml-parse-js";
import { encryptContent } from "./encryption";

const DB = await Database.load("sqlite:database.db");

export async function saveEmail(protonId, email) {
	try {

		const encryptedContent = await encryptContent(email.html);

		return await DB.execute(
			`INSERT INTO emails 
			(proton_id, subject, sender_name, sender_address, receiver_name, receiver_address, content, headers, timestamp) 
			VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)`,
			[
				protonId,
				email?.subject?.toString(),
				email?.from?.name?.toString(),
				email?.from?.email?.toString(),
				email?.to?.name?.toString(),
				email?.to?.email?.toString(),
				encryptedContent,
				JSON.stringify(email.headers),
				new Date(email?.date)?.toISOString(),
			]
		);
	} catch (e) {
		console.error("Error saving email:", e);
		return false;
	}
}

export async function saveEmailMetadata(emailId, protonId, metadata) {
	try {
		return await DB.execute(
			`INSERT INTO email_metadata (email_id, proton_id, data) VALUES (?1, ?2, ?3)`,
			[emailId, protonId, metadata]
		);
	} catch (e) {
		console.error("Error saving email metadata:", e);
	}
}

export function parseEmail(emlContent) {
	try {
		return readEml(emlContent);
	} catch (e) {
		console.error("Error parsing email:", e);
		return null;
	}
}

async function importEmailMetadata(emailId, file) {
	const reader = new FileReader();
	reader.onload = async (event) => {
		const protonId = file.name.split(".")[0];
		const metadataContent = event.target.result;
		const metadata = JSON.parse(metadataContent);
		return await saveEmailMetadata(emailId, protonId, metadata);
	}

	return reader.readAsText(file);
}

export async function importEmail(file, metadataFile) {
	const reader = new FileReader();
	reader.onload = async (event) => {
		const protonId = file.name.split(".")[0];
		const emlContent = event.target.result;
		const parsedEmail = parseEmail(emlContent);
		if (!parsedEmail) return false;

		const emailRecord = await saveEmail(protonId, parsedEmail);
		if (metadataFile && emailRecord && emailRecord.lastInsertId) {
			await importEmailMetadata(emailRecord.lastInsertId, metadataFile)
		}

		return emailRecord;
	};
	return reader.readAsText(file);
}




