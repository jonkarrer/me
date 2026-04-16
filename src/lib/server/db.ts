import { createClient, type Client } from '@libsql/client';

let _db: Client | null = null;
function db(): Client {
	if (_db) return _db;
	_db = createClient({
		url: process.env.TURSO_DATABASE_URL!,
		authToken: process.env.TURSO_AUTH_TOKEN
	});
	return _db;
}

export async function initDb() {
	await db().executeMultiple(`
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
}

export interface Blog {
	id: number;
	slug: string;
	title: string;
	description: string;
	content: string;
	published_at: string;
	updated_at: string;
}

export type BlogSummary = Omit<Blog, 'content'>;

export async function getAllBlogs(): Promise<BlogSummary[]> {
	const result = await db().execute(
		'SELECT id, slug, title, description, published_at, updated_at FROM blogs ORDER BY published_at DESC'
	);
	return result.rows as unknown as BlogSummary[];
}

export async function getBlogBySlug(slug: string): Promise<Blog | null> {
	const result = await db().execute({
		sql: 'SELECT * FROM blogs WHERE slug = ?',
		args: [slug]
	});
	if (result.rows.length === 0) return null;
	return result.rows[0] as unknown as Blog;
}

export async function createBlog(post: {
	slug: string;
	title: string;
	description?: string;
	content: string;
}): Promise<Blog> {
	const result = await db().execute({
		sql: 'INSERT INTO blogs (slug, title, description, content) VALUES (?, ?, ?, ?) RETURNING *',
		args: [post.slug, post.title, post.description || '', post.content]
	});
	return result.rows[0] as unknown as Blog;
}
