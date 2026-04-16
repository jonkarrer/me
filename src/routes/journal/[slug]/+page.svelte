<script lang="ts">
	import { onMount } from 'svelte';

	let { data } = $props();
	const post = $derived(data.post);
	const html = $derived(data.html);

	onMount(() => {
		// @ts-expect-error — Prism is loaded via CDN script tags
		if (typeof window !== 'undefined' && window.Prism) window.Prism.highlightAll();
	});
</script>

<svelte:head>
	<title>{post.title}</title>
	<link
		href="https://cdn.jsdelivr.net/npm/prismjs@v1.x/themes/prism-tomorrow.css"
		rel="stylesheet"
	/>
	<script src="https://cdn.jsdelivr.net/npm/prismjs@v1.x/components/prism-core.min.js"></script>
	<script src="https://cdn.jsdelivr.net/npm/prismjs@v1.x/plugins/autoloader/prism-autoloader.min.js"
	></script>
</svelte:head>

<section>
	<h1 class="pb-8 pt-8">{post.title}</h1>
	<div class="entry-content">
		{@html html}
	</div>
</section>

<style>
	.entry-content :global(h1),
	.entry-content :global(h2),
	.entry-content :global(h3),
	.entry-content :global(h4) {
		padding-top: 1.5rem;
		padding-bottom: 0.5rem;
	}

	.entry-content :global(p) {
		padding-bottom: 1rem;
	}

	.entry-content :global(ul),
	.entry-content :global(ol) {
		padding-left: 1.5rem;
		padding-bottom: 1rem;
	}

	.entry-content :global(li) {
		padding-bottom: 0.25rem;
	}

	.entry-content :global(a) {
		display: inline;
		text-decoration: underline;
		text-underline-offset: 0.25rem;
	}

	.entry-content :global(blockquote) {
		border-left: 2px solid var(--text);
		padding: 0.25rem 1rem;
		margin: 1rem 0;
		opacity: 0.8;
	}

	.entry-content :global(code) {
		font-family: 'fira-code', ui-monospace, monospace;
		font-size: 0.95rem;
		background: rgba(255, 255, 255, 0.08);
		padding: 2px 6px;
		border-radius: 4px;
	}

	.entry-content :global(pre) {
		border-radius: 8px;
		padding: 1rem;
		overflow-x: auto;
		margin: 1rem 0;
	}

	.entry-content :global(pre code) {
		background: none;
		padding: 0;
		font-family: 'fira-code', ui-monospace, monospace;
		font-size: 0.9rem;
	}

	.entry-content :global(img) {
		border-radius: 4px;
		margin: 1rem 0;
	}

	.entry-content :global(hr) {
		border: none;
		border-top: 1px dashed #e5e7eb;
		margin: 1.5rem 0;
	}
</style>
