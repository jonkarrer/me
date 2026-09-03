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
			<h3>Bernt</h3>
			<p>
				An iOS app that plans your meals around the ingredients you already have. Talk to it in
				plain English, tell it what's in your pantry, and it writes the week's recipes and
				grocery list for you. Less thinking, less food waste, more cooking.
			</p>
			<div class="pt-2">
				<span class="underline"><a href="https://bernt.app">Website</a></span>
			</div>
		</article>

		<article class="project grid gap-2">
			<h3>Global Guardians</h3>
			<p>
				A landing page I built for an author and their illustrator. It promotes the Global
				Guardians children's book series — middle-grade adventure mysteries where a network of
				young operatives race to protect the world's famous landmarks from those who'd erase
				them, smuggling real history into the fun along the way.
			</p>
			<div class="pt-2">
				<span class="underline"><a href="https://global-guardians.fly.dev">Website</a></span>
			</div>
		</article>
		<img class="pixel-img" src="/assets/image/saturn.jpg" alt="pixel art of saturn" />
	</div>

	<p class="pb-5">Some machine learning focused projects.</p>
	<div class="grid gap-8 pb-6">
		<article class="project grid gap-2">
			<h3>Market Runners</h3>
			<p>
				AI-driven market intelligence delivered through Discord. Multi-model pipelines ingest live
				equities, crypto, and macro data to emit signals, sentiment, volatility flags, and daily
				briefs. Built for retail traders who want machine-graded context, not buy/sell calls.
			</p>
			<div class="pt-2">
				<span class="underline"><a href="https://market-runners.com">Website</a></span>
			</div>
		</article>

		<article class="project grid gap-2">
			<h3>IPO Case Study</h3>
			<p>
				An exploratory data analysis of IPO returns. Jupyter notebooks walk through cleaning,
				feature engineering, hypothesis testing, and visualization, with the findings written up
				in a formal report.
			</p>
			<div class="pt-2">
				<span class="underline"
					><a href="https://github.com/jonkarrer/ipo-case-study">Github</a></span
				>
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
			</div>
		</article>
		<img class="pixel-img" src="/assets/image/space_ship_interior.jpg" alt="pixel art of a space base" />
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
		<span class="chip">PHP</span>
		<span class="chip">Nginx</span>
		<span class="chip">AWS</span>
		<span class="chip">Sentry</span>
		<span class="chip">Logrocket</span>
		<span class="chip">PWAs</span>
		<span class="chip">Vite</span>
		<span class="chip">Next</span>
		<span class="chip">Svelte</span>
		<span class="chip">Twilio</span>
		<span class="chip">ElevenLabs</span>
		<span class="chip">iOS</span>
		<span class="chip">Fly.io</span>
		<span class="chip">Cloudflare</span>
		<span class="chip">Doppler</span>
		<span class="chip">Turso</span>
		<span class="chip">Machine Learning</span>
		<span class="chip">AI Systems</span>
		<span class="chip">Prompt Engineering</span>
	</div>
</section>

<section class="section">
	<header class="section-head">
		<p class="eyebrow">Certifications</p>
		<h2>Credentials</h2>
	</header>
	<ul class="cert-list">
		<li>
			<a
				href="https://coursera.org/share/51c0ab211acee606c898fdfda4a3f0be"
				target="_blank"
				rel="noopener"
			>
				<span class="cert-title">IBM Machine Learning Specialist</span>
				<span class="cert-meta">IBM · Jun 2026</span>
			</a>
		</li>
		<li>
			<a
				href="https://coursera.org/share/125abde7969d87559154a5192d37fb52"
				target="_blank"
				rel="noopener"
			>
				<span class="cert-title">Deep Learning and Reinforcement Learning</span>
				<span class="cert-meta">IBM · Jun 2026</span>
			</a>
		</li>
		<li>
			<a
				href="https://www.coursera.org/account/accomplishments/verify/P8FIOG1NXNBL"
				target="_blank"
				rel="noopener"
			>
				<span class="cert-title">Unsupervised Machine Learning</span>
				<span class="cert-meta">IBM · Apr 2026</span>
			</a>
		</li>
		<li>
			<a
				href="https://www.coursera.org/account/accomplishments/verify/CRPRYBP4H10N"
				target="_blank"
				rel="noopener"
			>
				<span class="cert-title">Supervised Machine Learning: Classification</span>
				<span class="cert-meta">IBM · Mar 2026</span>
			</a>
		</li>
		<li>
			<a
				href="https://www.coursera.org/account/accomplishments/verify/VQSHGAHKWJVL"
				target="_blank"
				rel="noopener"
			>
				<span class="cert-title">Supervised Machine Learning: Regression</span>
				<span class="cert-meta">IBM · Jan 2026</span>
			</a>
		</li>
		<li>
			<a
				href="https://www.coursera.org/account/accomplishments/verify/WK52ASY320DB"
				target="_blank"
				rel="noopener"
			>
				<span class="cert-title">Exploratory Data Analysis for Machine Learning</span>
				<span class="cert-meta">IBM · Dec 2025</span>
			</a>
		</li>
	</ul>

	<img class="mt-5 pixel-img" src="/assets/image/partners.jpg" alt="" />
</section>

<section class="section">
	<header class="section-head">
		<p class="eyebrow">Contact</p>
		<h2>Say hi</h2>
	</header>
	<menu class="grid gap-4 md-flex justify-between flex-wrap">
		<div class="flex gap-4 flex-wrap">
			<span class="underline"><a href="https://www.linkedin.com/in/jonkarr">LinkedIn</a></span>
			<span class="underline"><a href="https://github.com/jonkarrer">GitHub</a></span>
		</div>
		<span class="underline"
			><a href="mailto:karrerdigital@gmail.com">karrerdigital@gmail.com</a></span
		>
	</menu>
</section>
