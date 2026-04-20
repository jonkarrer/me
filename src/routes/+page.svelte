<svelte:head>
	<title>Me</title>
</svelte:head>

<section class="section">
	<header class="section-head">
		<p class="eyebrow">About</p>
		<h2>Hi, I'm Jon.</h2>
	</header>
	<p>
		Another programmer on another slice of the internet. Probably solving another problem, or
		creating another thing. I love it, and will do it in a loop while alive is true. Take a look
		around at some of my stuff, or just say hi.
	</p>
	<img class="mt-5 pixel-img" src="/assets/image/looking_out.jpg" alt="pixel art of a robot" />
</section>

<section id="projects" class="section">
	<header class="section-head">
		<p class="eyebrow">Projects</p>
		<h2>Things I've built</h2>
	</header>

	<p class="pb-5">Some frontend focused projects.</p>
	<div class="grid gap-8 pb-6">
		<article class="project grid gap-2">
			<h3>Using AI</h3>
			<p>
				The central hub for my suite of AI-focused products. The purpose is to provide a landing
				page for marketing and education. My goal is to provide every-day tools that leverage AI,
				and wrap them in an easy-to-use interface.
			</p>
			<div class="pt-2">
				<span class="underline"><a href="https://www.usingai.app/">Website</a></span>
			</div>
		</article>

		<article class="project grid gap-2">
			<h3>Landing Pages</h3>
			<p>
				My first tool under the Using AI ecosystem. Generates a landing page template based on a few
				buttons and a prompt. Clean and simple interface for creating landing-page templates, with
				HTML export for further customization.
			</p>
			<div class="pt-2">
				<span class="underline"><a href="https://landingpages.usingai.app/">Website</a></span>
			</div>
		</article>

		<article class="project grid gap-2">
			<h3>Auidophile</h3>
			<p>
				My first large project with a framework back in 2019 — a challenge project from
				<a class="underline" href="https://www.frontendmentor.io/">frontend mentors</a>. Built with
				Next.js, TypeScript, and Tailwind. Still use these things today.
			</p>
			<div class="pt-2">
				<span class="underline"><a href="https://github.com/jonkarrer/audiophile">Github</a></span>
				<span class="underline"><a href="https://silly-dubinsky-a8c901.netlify.app/">Website</a></span>
			</div>
		</article>
		<img class="pixel-img" src="/assets/image/saturn.jpg" alt="pixel art of saturn" />
	</div>

	<p class="pb-5">Some backend focused projects.</p>
	<div class="grid gap-8 pb-6">
		<article class="project grid gap-2">
			<h3>Prep App</h3>
			<p>
				A fullstack application in Rust, my favorite language. Went as minimalistic as possible and
				had a ton of fun. Was hosted on an Amazon EC2 instance, but the cost wasn't worth it.
			</p>
			<div class="pt-2">
				<span class="underline"><a href="https://github.com/jonkarrer/prep">Github</a></span>
			</div>
		</article>

		<article class="project grid gap-2">
			<h3>Recipe Api</h3>
			<p>
				Explored PHP and Docker. PHP is an appealing scripting language for backend applications,
				specifically APIs that service web applications. Docker is a great modern tech that
				simplifies deployment.
			</p>
			<div class="pt-2">
				<span class="underline"><a href="https://github.com/jonkarrer/prep_api">Github</a></span>
			</div>
		</article>

		<article class="project grid gap-2">
			<h3>Yafa</h3>
			<p>
				The stock market is full of data that can be used in various ways. This one uses a Rust API
				to pull data and transform it into views. Got a chance to use Bun.js with TypeScript.
			</p>
			<div class="pt-2">
				<span class="underline"><a href="https://github.com/jonkarrer/yafa">Github</a></span>
			</div>
		</article>
		<img class="pixel-img" src="/assets/image/ody.jpg" alt="random space parts" />
	</div>

	<p class="pb-5">Some open source focused projects.</p>
	<div class="grid gap-8 pb-6">
		<article class="project grid gap-2">
			<h3>Brize Auth</h3>
			<p>
				A simple auth server in Rust. Work in progress, but functions really well as is. Built for
				MySQL and SurrealDB. A good project for understanding authentication at a low level.
			</p>
			<div class="pt-2">
				<span class="underline"><a href="https://github.com/xbrize/brize_auth">Github</a></span>
				<span class="underline"><a href="https://crates.io/crates/brize_auth">Crates.io</a></span>
			</div>
		</article>

		<article class="project grid gap-2">
			<h3>Alpaca Api</h3>
			<p>
				<a class="underline" href="https://alpaca.markets/">Alpaca</a> provides endpoints for stock
				market data. They don't have an official Rust SDK, so I scrapped one together.
			</p>
			<div class="pt-2">
				<span class="underline"
					><a href="https://github.com/jonkarrer/alpaca_api_client">Github</a></span
				>
				<span class="underline"
					><a href="https://crates.io/crates/alpaca_api_client">Crates.io</a></span
				>
			</div>
		</article>

		<article class="project grid gap-2">
			<h3>Tindi</h3>
			<p>
				A simple technical analysis indicator library in Rust. Uses various algorithms to generate
				technical indicators — stock market signals to buy and sell.
			</p>
			<div class="pt-2">
				<span class="underline"><a href="https://github.com/jonkarrer/tindi">Github</a></span>
				<span class="underline"><a href="https://crates.io/crates/tindi">Crates.io</a></span>
			</div>
		</article>
		<img class="pixel-img" src="/assets/image/space_ship_interior.jpg" alt="pixel art of a space base" />
	</div>

	<p class="pb-5">Some machine learning focused projects.</p>
	<div class="grid gap-8 pb-6">
		<article class="project grid gap-2">
			<h3>Tabular Binary Classification</h3>
			<p>
				Tabular binary classification using the classic Titanic dataset. My first attempt, and I'm
				happy with it. Used the Burn crate from the Rust ecosystem and submitted to
				<a class="underline" href="https://www.kaggle.com/">Kaggle</a>.
			</p>
			<div class="pt-2">
				<span class="underline"
					><a href="https://github.com/jonkarrer/titanic_classifier">Github</a></span
				>
			</div>
		</article>

		<article class="project grid gap-2">
			<h3>Natural Language</h3>
			<p>
				An attempt to train an NLP to classify patent types. Didn't create an accurate model, but
				it was a great experience using the Transformer neural network architecture.
			</p>
			<div class="pt-2">
				<span class="underline"
					><a href="https://github.com/jonkarrer/patent_classifier">Github</a></span
				>
			</div>
		</article>

		<article class="project grid gap-2">
			<h3>MNIST Digit Classifier</h3>
			<p>
				A case study of the classic MNIST dataset — 70,000 images of handwritten numbers. A good
				lesson for creating a neural network from scratch, and playing with custom loss functions.
			</p>
			<div class="pt-2">
				<span class="underline"
					><a href="https://github.com/jonkarrer/digit_classifier_rs">Github</a></span
				>
			</div>
		</article>
		<img class="pixel-img" src="/assets/image/island.jpg" alt="pixel art of a space base" />
	</div>
</section>

<section class="section">
	<header class="section-head">
		<p class="eyebrow">Stack</p>
		<h2>Experience with</h2>
	</header>
	<div class="chip-list">
		<span class="chip">TypeScript</span>
		<span class="chip">React</span>
		<span class="chip">Rust</span>
		<span class="chip">Electron</span>
		<span class="chip">SQL</span>
		<span class="chip">Git</span>
		<span class="chip">Bash</span>
		<span class="chip">Linux</span>
		<span class="chip">Node</span>
		<span class="chip">Docker</span>
		<span class="chip">Prompt Design</span>
		<span class="chip">PHP</span>
		<span class="chip">Nginx</span>
		<span class="chip">AWS</span>
		<span class="chip">Sentry</span>
		<span class="chip">Logrocket</span>
		<span class="chip">PWAs</span>
		<span class="chip">Vite</span>
		<span class="chip">Next</span>
		<span class="chip">Voiceflow</span>
		<span class="chip">Svelte</span>
		<span class="chip">Twilio</span>
		<span class="chip">ElevenLabs</span>
	</div>
</section>

<section class="section">
	<header class="section-head">
		<p class="eyebrow">Contact</p>
		<h2>Say hi</h2>
	</header>
	<menu class="grid gap-4 md-flex justify-between flex-wrap">
		<div class="flex gap-4 flex-wrap">
			<span class="underline"><a href="https://www.linkedin.com/in/jonkarrer">LinkedIn</a></span>
			<span class="underline"><a href="https://github.com/jonkarrer">GitHub</a></span>
		</div>
		<span class="underline"
			><a href="mailto:karrerdigital@gmail.com">karrerdigital@gmail.com</a></span
		>
	</menu>

	<img class="mt-5 pixel-img" src="/assets/image/partners.jpg" alt="" />
</section>
