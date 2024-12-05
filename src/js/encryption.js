import { createMessage, encrypt, readMessage, decrypt } from 'openpgp';

const decryptionKey = 'my-secret-key';

export async function encryptContent(content) {
	const message = await createMessage({ text: content });
	const encrypted = await encrypt({
		message,
		passwords: [decryptionKey],
		format: 'armored'
	});
	return encrypted;
}

export async function decryptContent(encryptedData) {
	const message = await readMessage({
		armoredMessage: encryptedData // parse armored message
	});
	const { data: decrypted } = await decrypt({
		message,
		passwords: [decryptionKey],
		format: 'utf8'
	});

	return decrypted
}