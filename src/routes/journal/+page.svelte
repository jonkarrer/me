<script lang="ts">
	let { data } = $props();
	const blogs = $derived(data.blogs);
</script>

<svelte:head>
	<title>Journal</title>
</svelte:head>

<section class="section">
	<article class="grid gap-2">
		<h2>Journal</h2>
		<p>
			I like to journal while coding. It helps organize ideas and keep track of progress. Also a great
			tool for remembering context when switching between projects. Here is a collection of some of
			those entries.
		</p>
	</article>
	<img class="pt-8" src="/assets/image/space_car.jpg" alt="" />
</section>

<section class="section">
	<h2 class="pb-5">Entries</h2>

	{#each blogs as blog}
		<a class="entry" href="/journal/{blog.slug}">
			<h3 class="pb-2">{blog.title}</h3>
			{#if blog.description}
				<p>{blog.description}</p>
			{/if}
		</a>
	{/each}

	<img class="pt-8" src="/assets/image/level_one.jpg" alt="" />
</section>
