import { createClient } from '@libsql/client';

const url = process.env.TURSO_DATABASE_URL;
if (!url) throw new Error('TURSO_DATABASE_URL not set');

const db = createClient({ url, authToken: process.env.TURSO_AUTH_TOKEN });

const ACRONYMS = new Set(['aws', 'cli', 'sdk', 'css', 'svg', 'dns', 'https', 'sql']);
const MIXED: Record<string, string> = {
	sqlx: 'SQLx',
	sqlite: 'SQLite',
	svgs: 'SVGs'
};
const SMALL = new Set(['for', 'to', 'of', 'and', 'in', 'on', 'the', 'a', 'an']);

function titleCase(slug: string): string {
	return slug
		.split('-')
		.map((w, i) => {
			if (MIXED[w]) return MIXED[w];
			if (ACRONYMS.has(w)) return w.toUpperCase();
			if (i > 0 && SMALL.has(w)) return w;
			return w.charAt(0).toUpperCase() + w.slice(1);
		})
		.join(' ');
}

const rows = (await db.execute('SELECT slug, title FROM blogs')).rows as unknown as {
	slug: string;
	title: string;
}[];

console.log(`Reviewing ${rows.length} blog(s)`);

let updated = 0;
for (const row of rows) {
	const next = titleCase(row.slug);
	if (next === row.title) continue;
	await db.execute({
		sql: "UPDATE blogs SET title = ?, updated_at = datetime('now') WHERE slug = ?",
		args: [next, row.slug]
	});
	console.log(`  ${row.slug}: "${row.title}" → "${next}"`);
	updated++;
}

console.log(`\nUpdated ${updated} title(s)`);
