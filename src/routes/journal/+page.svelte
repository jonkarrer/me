<script lang="ts">
	let { data } = $props();
	const blogs = $derived(data.blogs);
</script>

<svelte:head>
	<title>Journal</title>
</svelte:head>

<section class="section">
	<header class="section-head">
		<p class="eyebrow">Journal</p>
		<h2>Notes from the bench</h2>
	</header>
	<p>
		I keep notes on the things I figure out so others can maybe find clues for themselves. Each
		entry is a snapshot — a problem I was stuck on, an idea I was chasing, a tool I was poking at.
		If any of it helps, I'm glad it's here.
	</p>
	<img class="mt-5 pixel-img" src="/assets/image/space_car.jpg" alt="" />
</section>

<section class="section">
	<header class="section-head">
		<p class="eyebrow">{blogs.length} {blogs.length === 1 ? 'entry' : 'entries'}</p>
		<h2>Entries</h2>
	</header>

	<div class="reveal">
		{#each blogs as blog}
			<a class="entry" href="/journal/{blog.slug}">
				<h3>{blog.title}</h3>
				{#if blog.description}
					<p>{blog.description}</p>
				{/if}
			</a>
		{/each}
	</div>

	<img class="mt-5 pixel-img" src="/assets/image/level_one.jpg" alt="" />
</section>
