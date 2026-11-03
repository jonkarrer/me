# Cardsift: Searching 34,000 Cards Offline

Cardsift is a Magic: The Gathering card search app for iPhone. The whole catalogue lives on the phone. You type what you remember of a card's text, and every face of every card is searched as you type, with no signal and no account. It takes a regular expression, which as far as I can tell makes it the only card app on a phone that does.

It started life as something else. The repo has 636 commits, and most of the first 500 were an attempt to predict card prices. This post is about what shipped, and a little about why the price predictor didn't.

## What's in the box

- **The database.** MTGJSON, imported weekly by a scheduled machine on Fly into a Rust API, then packed into a SQLite file the app downloads. About 28 MB for a weekly update. The finder on the phone answers in a millisecond.
- **Tags for what a card does.** Sac outlet, draw engine, removal, ramp, pump. Fifty of them, read from the rules text by a classifier rather than typed in by hand, so every card has them and not only the ones somebody got round to. More on that in [the classifier post](/journal/bert-in-rust-with-burn).
- **Sorting that means something.** Price, how often EDHREC sees a card played, and how salty a table finds it.
- **A scanner.** Point the camera at a card and the app names the printing out of 109,632, by matching the picture against an index on the device. Nothing is uploaded.
- **Pictures, offline too.** Every card's art, about 1.3 GB over Wi-Fi, in a bucket I own. Optional. Without it, art loads from Scryfall as you browse.
- **Nothing leaves the phone.** No account, no analytics, no tracking. The App Privacy questionnaire says "Data Not Collected" in every category, and the privacy manifest says the same thing.

Paid once, $2.99, new cards every week.

## The price that is actually a price

A card's "price" on most sites is the cheapest listing anywhere, which is often a foreign-language printing or a damaged copy. Cardsift's price is the cheapest English printing you could actually buy and play, refreshed weekly, and the app says which printing that is. It's a small thing. It took a surprising amount of ETL to get right.

## What didn't ship

The original goal was to understand what a card does well enough to say what it's worth. That became a language for writing a card's text as a program, 35 operations with a selector and an expression grammar, compiled to vectors. A thousand cards were written out by hand. A checker validated each program against two independent game engines, and 888 of 1,001 agreed with both.

It was finished work, and none of it reached the app. Hand-modelling didn't scale, and everything downstream inherited the sample. Value turned out not to be price, and price was contaminated by scarcity and nostalgia in ways no feature could separate. So the product was reduced back to what it ships: a landing page, the weekly ETL, and the app. The research was archived in its own repo with notes on the nine things that were genuinely hard, so the next attempt argues from them rather than rediscovering them.

That's the part I'm most pleased with, honestly. Knowing when to stop, and leaving the trail.
