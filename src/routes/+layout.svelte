<script lang="ts">
	import '../app.css';

	let { children } = $props();
</script>

<main>
	<header>
		<a href="/">Jon Karrer</a>
		<nav>
			<a href="/journal">Journal</a>
			<a href="/#projects">Projects</a>
		</nav>
	</header>

	{@render children()}

	<footer>
		<p>© 2026 Jon Karrer</p>
	</footer>
</main>
