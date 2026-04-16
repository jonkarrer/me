import { createClient } from '@libsql/client';

const url = process.env.TURSO_DATABASE_URL;
if (!url) throw new Error('TURSO_DATABASE_URL not set');

const db = createClient({ url, authToken: process.env.TURSO_AUTH_TOKEN });

await db.executeMultiple(`
	CREATE TABLE IF NOT EXISTS blogs (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		slug TEXT NOT NULL UNIQUE,
		title TEXT NOT NULL,
		description TEXT NOT NULL DEFAULT '',
		content TEXT NOT NULL,
		published_at TEXT NOT NULL DEFAULT (datetime('now')),
		updated_at TEXT NOT NULL DEFAULT (datetime('now'))
	);
	CREATE INDEX IF NOT EXISTS idx_blogs_slug ON blogs(slug);
	CREATE INDEX IF NOT EXISTS idx_blogs_published_at ON blogs(published_at DESC);
`);

const tables = await db.execute(
	"SELECT name FROM sqlite_master WHERE type='table' AND name='blogs'"
);
console.log(`Ensured tables: ${tables.rows.map((r) => r.name).join(', ')} @ ${url}`);
