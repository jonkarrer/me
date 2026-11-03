# Ternt: A Browser and a Terminal in One Window

I live in two apps: a browser with a sidebar of tabs grouped into spaces, and a terminal with a coding agent in it. I kept wanting them to be one window. So I built it. Ternt is an Arc-style browser and terminal for macOS, in Tauri v2 (Rust) and Svelte 5, and the first release shipped a week after the first commit.

## What it is

The sidebar is Arc's, top to bottom: favorites as app-icon tiles, pinned tabs per space with folders, today's tabs under a divider, and terminals at the bottom. Spaces each have an icon, a color, and a profile. A profile is the identity sites see: cookies, logins, history. One Gmail for work in one space, another for side projects in the next.

Then the parts Arc doesn't have:

- **Terminal tabs.** A real PTY with xterm.js and flow control. ⌘D splits a pane to the right, ⇧⌘D stacks one below, panes nest, and the layout is saved with the session. Programs that copy for you (tmux, vim, Claude Code, even over ssh) reach the clipboard through OSC 52.
- **Split view.** A page beside a page, or a page beside a terminal. Drag the gap to resize.
- **A command bar.** ⌘T opens a URL, searches, switches tabs or spaces, finds pages in history, and runs commands. Initials work: "nt" is New Terminal. `> cmd` runs `cmd` in a new terminal.
- **Explore mode.** ⌘F, then j/k through your tabs, Space to peek, Enter to open.
- **Little Ternt.** Make it the default browser and links from other apps open in a small window signed in with your profile.

## Why Tauri

The same reason as last time I tried a desktop app: a Rust process doing filesystem work, PTYs and navigation state feels different from a Node process doing the same. Lighter, and the binary is small. WebKit comes from the OS, so there's no browser engine to bundle.

The cost is that the webview is macOS's, with macOS's rules about popups, profiles and downloads. A good chunk of the first week was learning which of those rules bend.

## Self-updating

The part I'm happiest with is the part users never notice. Ternt checks for a release at launch, every fifteen minutes, and when you come back to it. When one is out, a card appears at the bottom of the sidebar. Restart Now downloads the update, swaps the app in place and relaunches with your tabs. Later hides it for that version. The whole thing rides on GitHub releases and Tauri's updater.

I did this on day two, before most of the features, because I wanted every later day's work to reach the machine I actually use without me thinking about it. It did.

## The week

Day one was the shell, native navigation state, popups and tab shortcuts. Then the command bar, terminals, sessions and pinned tabs. Then spaces, favorites, folders and profiles. Then explore mode, history, a test suite with unit tests for ranking and parsing plus e2e coverage, and the updater. Then a rename, because the first name was a placeholder.

Seventy commits and a 0.10.0 so far. It's the browser I use now, and the terminal, which is the whole point.

[Releases →](https://github.com/jonkarrer/ternt/releases/latest)
