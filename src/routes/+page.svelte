<svelte:head>
	<title>Jon Karrer — Lead AI Engineer</title>
	<meta
		name="description"
		content="Jon Karrer is a Lead AI Engineer and the founder of Using AI, a software studio that ships AI products on iOS and the web."
	/>
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

	<p class="pb-5">Products shipped through Using AI.</p>
	<div class="grid gap-8 pb-6">
		<article class="project grid gap-2">
			<h3>Bernt</h3>
			<p>
				An AI kitchen companion for iOS. It knows what's in your pantry, writes recipes from what
				you already have, and plans the week through plain conversation. Underneath: SwiftUI, a
				Hono server on Bun, Turso, Groq with Claude as the fallback, Sign in with Apple, and App
				Store subscriptions with the webhook edge cases handled.
			</p>
			<div class="pt-2 flex gap-4">
				<span class="underline"><a href="https://bernt.app">Website</a></span>
				<span class="underline"><a href="/journal/shipping-bernt">Journal</a></span>
			</div>
		</article>

		<article class="project grid gap-2">
			<h3>Lernt</h3>
			<p>
				Describe what you want to learn and Lernt builds the course: structured lessons,
				auto-scored quizzes, progress tracking, and a tutor whose answers are grounded in the
				lesson you're reading. iOS, free to start, with community courses you can publish and
				enroll in.
			</p>
			<div class="pt-2">
				<span class="underline"><a href="https://lernt.app">Website</a></span>
			</div>
		</article>

		<article class="project grid gap-2">
			<h3>Market Runners</h3>
			<p>
				A daily pre-market brief, written by a multi-pass pipeline: a gather step with live web
				search, a writer, a critic that reviews every draft, and an editorial pass. Live equities,
				macro and news data in; one honest email out, every morning before the open.
			</p>
			<div class="pt-2">
				<span class="underline"><a href="https://market-runners.com">Website</a></span>
			</div>
		</article>

		<article class="project grid gap-2">
			<h3>Cardsift</h3>
			<p>
				Search every Magic: The Gathering card by what it says, on your phone, offline. All
				34,000 cards live in a database on the device, regex included. Each card carries tags for
				what it does, read from the rules text by a classifier I trained. Point the camera at a
				card and it names the printing out of 109,632, without uploading anything.
			</p>
			<div class="pt-2 flex gap-4">
				<span class="underline"><a href="/journal/cardsift-offline-search">Journal</a></span>
				<span class="underline"><a href="/journal/bert-in-rust-with-burn">Classifier</a></span>
			</div>
		</article>
		<img class="pixel-img" src="/assets/image/saturn.jpg" alt="pixel art of saturn" />
	</div>

	<p class="pb-5">Infrastructure and tools I built to ship those.</p>
	<div class="grid gap-8 pb-6">
		<article class="project grid gap-2">
			<h3>Headquarters</h3>
			<p>
				An ops dashboard that pulls Fly, Cloudflare, Turso, Stripe, Resend, Anthropic and the
				domain registrars into one database and derives the links between them, so it can answer
				questions no vendor dashboard can. It also hosts the engineering standards every project
				follows, served as plain text so coding agents read the same rules I do.
			</p>
			<div class="pt-2">
				<span class="underline"><a href="/journal/standards-for-agents">Journal</a></span>
			</div>
		</article>

		<article class="project grid gap-2">
			<h3>Ternt</h3>
			<p>
				An Arc-style browser and terminal for macOS, built with Tauri and Svelte 5. Spaces,
				profiles, split views, nested terminal panes, a command bar, and an updater that swaps the
				app in place. The first release shipped a week after the first commit.
			</p>
			<div class="pt-2 flex gap-4">
				<span class="underline"><a href="https://github.com/jonkarrer/ternt">GitHub</a></span>
				<span class="underline"><a href="/journal/ternt-browser-and-terminal">Journal</a></span>
			</div>
		</article>

		<article class="project grid gap-2">
			<h3>Tempo</h3>
			<p>
				A quant research stack. Agents take a market hypothesis through data curation, feature
				engineering, strategy design and a single-shot backtest, with a reviewer at every station.
				A human approves what's viable. Deterministic code trades it. No model in the live loop,
				on purpose.
			</p>
		</article>

		<article class="project grid gap-2">
			<h3>Teli, Simbridge, Forest</h3>
			<p>
				Smaller tools from the same workshop. Teli launches coding agents inside a bubblewrap
				sandbox with Doppler-scoped secrets. Simbridge drives the iOS Simulator from a browser over
				SSH, screen stream and real taps included. Forest is a multilingual voice agent in the
				browser where everything but the LLM runs locally.
			</p>
		</article>
		<img class="pixel-img" src="/assets/image/ody.jpg" alt="random space parts" />
	</div>

	<p class="pb-5">Client work and open source.</p>
	<div class="grid gap-8 pb-6">
		<article class="project grid gap-2">
			<h3>Global Guardians</h3>
			<p>
				A site for an author and their illustrator, promoting the Global Guardians children's book
				series: middle-grade adventure mysteries where young operatives protect the world's
				landmarks, with real history smuggled into the fun.
			</p>
			<div class="pt-2">
				<span class="underline"><a href="https://global-guardians.fly.dev">Website</a></span>
			</div>
		</article>

		<article class="project grid gap-2">
			<h3>Alpaca API client</h3>
			<p>
				<a class="underline inline" href="https://alpaca.markets/">Alpaca</a> provides stock market
				data and has no official Rust SDK, so I wrote one.
			</p>
			<div class="pt-2">
				<span class="underline"
					><a href="https://github.com/jonkarrer/alpaca_api_client">GitHub</a></span
				>
			</div>
		</article>

		<article class="project grid gap-2">
			<h3>Tindi</h3>
			<p>A technical analysis indicator library in Rust.</p>
			<div class="pt-2">
				<span class="underline"><a href="https://github.com/jonkarrer/tindi">GitHub</a></span>
			</div>
		</article>

		<article class="project grid gap-2">
			<h3>Brize Auth</h3>
			<p>A small auth server in Rust, built for MySQL and SurrealDB.</p>
			<div class="pt-2">
				<span class="underline"><a href="https://github.com/xbrize/brize_auth">GitHub</a></span>
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
		<span class="chip">Claude API</span>
		<span class="chip">Groq</span>
		<span class="chip">Tool use</span>
		<span class="chip">Structured outputs</span>
		<span class="chip">Prompt injection defense</span>
		<span class="chip">Evals</span>
		<span class="chip">Voice agents</span>
		<span class="chip">Twilio</span>
		<span class="chip">Deepgram</span>
		<span class="chip">ElevenLabs</span>
		<span class="chip">Claude Code</span>
		<span class="chip">Machine Learning</span>
		<span class="chip">BERT</span>
		<span class="chip">XGBoost</span>
		<span class="chip">TypeScript</span>
		<span class="chip">Rust</span>
		<span class="chip">Swift</span>
		<span class="chip">Python</span>
		<span class="chip">SQL</span>
		<span class="chip">Svelte</span>
		<span class="chip">React</span>
		<span class="chip">SwiftUI</span>
		<span class="chip">Tauri</span>
		<span class="chip">Electron</span>
		<span class="chip">Bun</span>
		<span class="chip">Hono</span>
		<span class="chip">axum</span>
		<span class="chip">Turso</span>
		<span class="chip">Postgres</span>
		<span class="chip">Redis</span>
		<span class="chip">Fly.io</span>
		<span class="chip">Cloudflare</span>
		<span class="chip">Docker</span>
		<span class="chip">Nix</span>
		<span class="chip">Doppler</span>
		<span class="chip">Linux</span>
		<span class="chip">App Store Connect</span>
		<span class="chip">StoreKit</span>
		<span class="chip">Stripe</span>
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
			<span class="underline"><a href="https://usingai.app">Using AI</a></span>
		</div>
		<span class="underline"
			><a href="mailto:karrerdigital@gmail.com">karrerdigital@gmail.com</a></span
		>
	</menu>
</section>
