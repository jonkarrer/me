import { getBlogBySlug } from '$lib/server/db';
import { error } from '@sveltejs/kit';
import { marked } from 'marked';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
	const post = await getBlogBySlug(params.slug);
	if (!post) throw error(404, 'Not Found');
	// Strip a leading "# Title" heading if present — the page header renders the title.
	const content = post.content.replace(/^\s*#\s+.+\n+/, '');
	const html = marked.parse(content) as string;
	return { post, html };
};
