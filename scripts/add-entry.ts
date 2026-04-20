// Usage: bun run scripts/add-entry.ts <slug> "<title>" "<description>"
// Reads markdown from frontend/assets/journal/<slug>.md
// Inserts or updates the blogs row for the given slug.
import { createClient } from '@libsql/client';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const [, , slug, title, description] = process.argv;
if (!slug || !title || typeof description !== 'string') {
	console.error('Usage: bun run scripts/add-entry.ts <slug> "<title>" "<description>"');
	process.exit(1);
}

const url = process.env.TURSO_DATABASE_URL;
if (!url) throw new Error('TURSO_DATABASE_URL not set');

const db = createClient({ url, authToken: process.env.TURSO_AUTH_TOKEN });

const content = readFileSync(resolve('frontend/assets/journal', `${slug}.md`), 'utf8');

const existing = await db.execute({
	sql: 'SELECT id FROM blogs WHERE slug = ?',
	args: [slug]
});

if (existing.rows.length === 0) {
	await db.execute({
		sql: `INSERT INTO blogs (slug, title, description, content) VALUES (?, ?, ?, ?)`,
		args: [slug, title, description, content]
	});
	console.log(`+ ${slug} inserted`);
} else {
	await db.execute({
		sql: `UPDATE blogs SET title = ?, description = ?, content = ?, updated_at = datetime('now') WHERE slug = ?`,
		args: [title, description, content, slug]
	});
	console.log(`= ${slug} updated`);
}

const count = await db.execute('SELECT COUNT(*) as n FROM blogs');
console.log(`Total blogs: ${count.rows[0].n}`);
