import { getBlogBySlug } from '$lib/server/db';
import { error } from '@sveltejs/kit';
import { marked } from 'marked';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
	const post = await getBlogBySlug(params.slug);
	if (!post) throw error(404, 'Not Found');
	const html = marked.parse(post.content) as string;
	return { post, html };
};
