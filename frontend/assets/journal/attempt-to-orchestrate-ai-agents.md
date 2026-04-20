# Attempt to Orchestrate AI Agents

AI agents have been on my mind for a while. Not the demo-reel kind where an agent writes a snake game end to end — the real question: can a human *direct* an agent well enough that the whole operation feels like collaboration instead of a roll of the dice?

I started a desktop IDE called `muve` to try answering that. The bet is simple: flip the traditional IDE. The chat is the workspace. The developer talks, the agent drives the files.

## Idea

In most IDEs, you open a file, edit it, save it, and the AI helper sits in a side panel. In muve, I wanted to invert that:

- You tell the agent what you want
- The agent reads, creates, modifies, and deletes files
- Every action streams back to the chat as a first-class object with diffs and file references

The editor panel is secondary. You can open a file to look at it, but the primary surface is the conversation.

## Architecture

Three layers:

```text
Svelte UI  →  Tauri / Rust backend  →  Claude Code CLI
```

- **Frontend**: Svelte 5 with runes, Tailwind v4, CodeMirror 6 for file views.
- **Backend**: Tauri 2 with Rust and Tokio. Spawns Claude Code as a subprocess in stream-json mode and forwards events to the frontend via Tauri's event system.
- **Agent**: the Claude Code CLI, doing the actual work.

I picked Tauri over Electron for native performance. A Rust process handling filesystem ops and agent streams feels very different from a Node process doing the same — lighter, faster, smaller binary.

## Streaming as a design choice

The agent emits NDJSON events as it works. Each one — file read, diff, shell command — renders as a card in the chat. You watch the agent work as it works. No spinner, no "thinking…", no waiting for the full plan.

That visibility is the point. If the agent is about to do something wrong, you want to see it early enough to interrupt. "Radical transparency" is how I've been framing it — every tool call visible, every file write showing its diff, every shell command showing its output.

## Multi-session

Development is iterative. A side-task interrupts the main task; you come back an hour later. Muve keeps each project as a separate session with its own conversation, file tree, and agent context. Switching between them shouldn't cost you the thread.

## What I learned

The hybrid model — human director, agent executor — feels right for the current generation of models. Fully autonomous agents hit walls; agents that can't be steered hit them harder. A UI that makes steering cheap is what I wanted to explore.

Still early. Twelve commits in and it runs, but there's a long tail of "what happens when the agent does *this* weird thing" to sand down. Also: the moment you start thinking about session persistence and restore-after-restart, you realize how much state an agent accumulates between turns.

[Repo →](https://github.com/jonkarrer/muve)
