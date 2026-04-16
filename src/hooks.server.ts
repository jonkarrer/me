import { initDb } from '$lib/server/db';

export async function init() {
	await initDb();
}
