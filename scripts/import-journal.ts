import { createClient } from '@libsql/client';
import { readdirSync, readFileSync } from 'node:fs';
import { resolve, parse } from 'node:path';

const url = process.env.TURSO_DATABASE_URL;
if (!url) throw new Error('TURSO_DATABASE_URL not set');

const db = createClient({ url, authToken: process.env.TURSO_AUTH_TOKEN });

const journalDir = resolve('frontend/assets/journal');
const files = readdirSync(journalDir)
	.filter((f) => f.endsWith('.md'))
	.sort();

console.log(`Found ${files.length} journal file(s)`);

function toTitle(slug: string): string {
	return slug
		.split('-')
		.map((w) => w.charAt(0).toUpperCase() + w.slice(1))
		.join(' ');
}

let inserted = 0;
let skipped = 0;

for (const file of files) {
	const slug = parse(file).name;
	const title = toTitle(slug);
	const content = readFileSync(resolve(journalDir, file), 'utf8');
	const result = await db.execute({
		sql: `INSERT OR IGNORE INTO blogs (slug, title, content) VALUES (?, ?, ?)`,
		args: [slug, title, content]
	});
	if (result.rowsAffected > 0) {
		inserted++;
		console.log(`  + ${slug}`);
	} else {
		skipped++;
		console.log(`  = ${slug} (already exists)`);
	}
}

const count = await db.execute('SELECT COUNT(*) as n FROM blogs');
console.log(`\nInserted: ${inserted}, skipped: ${skipped}, total in DB: ${count.rows[0].n}`);
