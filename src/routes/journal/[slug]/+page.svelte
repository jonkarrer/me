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

<article>
	<header class="entry-header">
		<p class="eyebrow"><a href="/journal" class="back">← Journal</a></p>
		<h1>{post.title}</h1>
		{#if post.description}
			<p class="lede">{post.description}</p>
		{/if}
	</header>

	<div class="entry-content">
		{@html html}
	</div>
</article>

<style>
	article {
		padding-top: 8px;
	}

	.entry-header {
		padding: 8px 0 16px;
		border-bottom: 1px dashed var(--border);
		margin-bottom: 20px;
	}

	.back {
		color: var(--text-dim);
		transition: color 0.2s ease;
	}

	.back:hover {
		color: var(--accent);
	}

	.lede {
		margin-top: 12px;
		color: var(--text-dim);
		font-family: 'italic-font', 'regular-font', serif;
		font-style: italic;
		font-size: 1.05rem;
		max-width: 60ch;
	}

	/* Prose */
	.entry-content {
		font-size: 1.02rem;
		line-height: 1.7;
		color: var(--text-dim);
	}

	.entry-content :global(h1),
	.entry-content :global(h2),
	.entry-content :global(h3),
	.entry-content :global(h4) {
		color: var(--text);
		padding-top: 1.2rem;
		padding-bottom: 0.35rem;
		letter-spacing: -0.005em;
	}

	.entry-content :global(p) {
		padding-bottom: 0.75rem;
	}

	.entry-content :global(strong) {
		color: var(--text);
	}

	.entry-content :global(ul),
	.entry-content :global(ol) {
		padding-left: 1.5rem;
		padding-bottom: 1rem;
	}

	.entry-content :global(li) {
		padding-bottom: 0.25rem;
	}

	.entry-content :global(li::marker) {
		color: var(--accent);
	}

	.entry-content :global(a) {
		display: inline;
		color: inherit;
		text-decoration: underline;
		text-decoration-color: var(--border);
		text-underline-offset: 0.25rem;
		text-decoration-thickness: 1px;
		transition:
			color 0.2s ease,
			text-decoration-color 0.2s ease;
	}

	.entry-content :global(a:hover) {
		color: var(--accent);
		text-decoration-color: var(--accent);
	}

	.entry-content :global(blockquote) {
		border-left: 2px solid var(--accent);
		padding: 0.4rem 1rem;
		margin: 1rem 0;
		background: var(--accent-soft);
		border-radius: 0 4px 4px 0;
		color: var(--text);
	}

	.entry-content :global(blockquote p:last-child) {
		padding-bottom: 0;
	}

	.entry-content :global(code) {
		font-family: 'fira-code', ui-monospace, monospace;
		font-size: 0.9em;
		background: var(--accent-soft);
		color: var(--accent);
		padding: 2px 6px;
		border-radius: 4px;
	}

	.entry-content :global(pre) {
		border: 1px solid var(--border);
		border-radius: 6px;
		padding: 0.85rem 1rem;
		overflow-x: auto;
		margin: 0.75rem 0 1rem;
		background: rgb(17, 17, 20);
	}

	.entry-content :global(pre code) {
		background: none;
		color: inherit;
		padding: 0;
		font-family: 'fira-code', ui-monospace, monospace;
		font-size: 0.88rem;
		line-height: 1.55;
	}

	.entry-content :global(img) {
		border: 1px solid var(--border);
		border-radius: 4px;
		margin: 1.25rem 0;
	}

	.entry-content :global(hr) {
		border: none;
		border-top: 1px dashed var(--border);
		margin: 2rem 0;
	}

	.entry-content :global(table) {
		width: 100%;
		border-collapse: collapse;
		margin: 1rem 0 1.5rem;
		font-size: 0.92rem;
	}

	.entry-content :global(th),
	.entry-content :global(td) {
		border: 1px solid var(--border);
		padding: 8px 12px;
		text-align: left;
	}

	.entry-content :global(th) {
		background: var(--border-soft);
		color: var(--text);
		font-weight: 400;
		letter-spacing: 0.02em;
	}

	.entry-content :global(tbody tr:nth-child(even)) {
		background: rgba(248, 250, 252, 0.02);
	}
</style>
