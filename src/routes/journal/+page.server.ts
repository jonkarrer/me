import { getAllBlogs } from '$lib/server/db';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const blogs = await getAllBlogs();
	return { blogs };
};
