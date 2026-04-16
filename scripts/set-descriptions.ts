import { createClient } from '@libsql/client';

const url = process.env.TURSO_DATABASE_URL;
if (!url) throw new Error('TURSO_DATABASE_URL not set');

const db = createClient({ url, authToken: process.env.TURSO_AUTH_TOKEN });

const DESCRIPTIONS: Record<string, string> = {
	'algo-trading-bot':
		'Overview of building a Rust-based stock trading bot that ingests live market data and executes bracket orders.',
	'alpaca-rust-sdk':
		'Designing an open-source Rust SDK for the Alpaca trading API using a Serde-backed builder pattern.',
	'aws-certificate-manager':
		'Step-by-step guide to serving an ECS service over HTTPS with an ALB and an ACM certificate.',
	'aws-cli-auth':
		'Installing the AWS CLI on macOS and configuring IAM credentials to authenticate against AWS services.',
	'aws-elastic-container-registry':
		'CLI reference for creating, pushing to, pulling from, and deleting Elastic Container Registry repositories.',
	'aws-elastic-container-service':
		'Walkthrough for deploying a Docker image from ECR to a Fargate ECS cluster via the console and CLI.',
	'clap-cli-tool':
		'Building a small Clap-based CLI in Rust for managing journal entries in a Turso SQLite database.',
	'claude-prompting-guide':
		"Anthropic's tips for effective prompting, with concrete before-and-after examples for common writing and analysis tasks.",
	'css-svgs-basics':
		'Reference of the seven basic SVG shapes and the attributes that control their geometry and styling.',
	'daily-linear-classifier':
		'Running thirteen hyperparameter experiments trying to train a baseline linear classifier to predict next-day stock direction.',
	'digit-classifier':
		'Building a baseline MNIST digit classifier in Rust using Candle and tensor-averaged "ideal" images before moving to SGD.',
	'docker-basics':
		'The minimum commands to write a Dockerfile, build an image, run a container, and clean up after it.',
	'docker-vite':
		'Short recipe for containerizing a Vite app with a multi-stage Dockerfile and serving the build via nginx.',
	'https-nginx-certbot':
		"Setting up Nginx as a reverse proxy with Let's Encrypt certificates via Certbot, including a Docker Compose variant.",
	'image-classifier':
		'Training an image classifier in Rust using tch-rs and transfer learning on top of a pre-trained ResNet18.',
	'language-classifier':
		'Rolling a custom transformer-based NLP classifier in Rust with the Burn framework for the US patent phrase-matching task.',
	'patent-classifier':
		'Preparing, tokenizing, and batching patent phrase-matching data for a Rust-built transformer classifier with padding masks and embeddings.',
	'raspberry-pi-dns':
		'Turning a Raspberry Pi into a LAN-wide DNS server and ad blocker using Pi-hole inside Docker.',
	'shuttle-portfolio':
		'Notes on moving the portfolio site to Shuttle with a markdown-powered journal page backed by Turso.',
	'sqlx-for-sqlite':
		'Using SQLx in Rust to run SQLite migrations, manage a connection pool, and wire up repository traits.',
	'titanic-binary-classification':
		'Walkthrough of building a Rust neural network from scratch to predict Titanic survival and iterating on its hyperparameters.'
};

let updated = 0;
let skipped = 0;

for (const [slug, description] of Object.entries(DESCRIPTIONS)) {
	const result = await db.execute({
		sql: "UPDATE blogs SET description = ?, updated_at = datetime('now') WHERE slug = ?",
		args: [description, slug]
	});
	if (result.rowsAffected > 0) {
		console.log(`  + ${slug}`);
		updated++;
	} else {
		console.log(`  ! ${slug}: no matching row`);
		skipped++;
	}
}

console.log(`\nUpdated: ${updated}, skipped: ${skipped}`);
