# Writing Standards That Agents Read

I run about a dozen projects, and a coding agent touches most of them most days. For a while each one had its own idea of how to log an error, where secrets came from, how a migration was applied. The agent would do whatever the nearest file suggested. The nearest file was often wrong.

So I wrote it down once. Headquarters, my ops dashboard, now hosts a library of engineering standards, and every project points at it.

## What a standard is

One markdown file per topic: Turso, Doppler, Fly, errors, telemetry, auth, Apple payments, iOS deploys, how an app talks to a model. Each has a version, a date, and a short reason it exists. Each ends in a checklist. A runbook called "New project" walks the checklists in the order that matters: secrets before code, database before migrations, cert before DNS, deploy before domain, and nothing is done until Headquarters can see it.

Every standard has a sibling with an `.llm.md` suffix. That's the same content trimmed for an agent: fewer paragraphs, more rules, the exact file paths and table shapes. The site serves it under `/docs/<name>/llm`, and the index of all of them is at `/llms.txt`. A project's `CLAUDE.md` says: read the standards from there.

## The one I'd keep if I could keep one

The LLM standard. Its doctrine fits on a line:

> A model is an untrusted, brilliant intern. Everything it reads was written by someone you don't trust, and everything it writes is treated like user form input.

From that line fall most of the rules:

- **Prompts are code.** One `ai/prompts.ts` per app, exported constants, reviewed in diffs. Never inline at a call site.
- **Schema first.** Every generation call passes a JSON Schema and gets back a parsed object. Groq takes it as strict `json_schema`; Anthropic takes the same object as a strict tool. One schema, both providers, so the fallback is drop-in.
- **A sanitize boundary.** One module where model output becomes database rows. Numbers that fail a CHECK go to null, strings are capped, enum mismatches get a safe default, array items missing a required field are dropped and the ordinals renumbered.
- **An explicit ladder.** Primary provider, one retry on transient errors, fallback provider with the same schema, then a logged failure. Never an infinite retry, never a silent degradation. A fallback leaves a telemetry breadcrumb.
- **Two-layer injection defense.** A regex pre-screen that skips the model call for most writes, and a small classifier that fails closed on a hit.
- **Evals decide the ladder.** Prompt tests run offline for free. A second suite hits real providers and checks that the injection classifier tags both benign and hostile inputs correctly. A prompt change without a green live eval is a guess.

## Why it works better than a README

A README describes one project. A standard describes a decision, and the decision is what the agent needs. When Lernt was brought up to the standards, the diff was mostly about things the agent had quietly improvised the first time: a hand-copied test schema that had drifted from the migrations, a session refresh that wrote on every request past day fifteen, an email-verification link that mail clients prefetched and burned. The standard had a rule for each. The second project got them for free.

The other thing: standards disagree with each other, and you only find out when you write them down. The errors standard said `user_id INTEGER`, the telemetry standard said `TEXT`, and the Turso standard said nothing. A real app's schema broke the tie, and the standards were corrected. That conversation can't happen inside a README.

## What it costs

Maintenance. Each standard has a changelog, and when a project learns something the standard has to learn it too, or it starts lying. I treat a standard like code with one consumer who never asks questions and always does what it says. That consumer is the agent, and it's a demanding one.
