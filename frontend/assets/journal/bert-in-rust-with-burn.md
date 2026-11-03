# Training a BERT Classifier in Rust With burn

My Magic card search app tags every card with what it does: ramp, removal, sac outlet, dig, stax, and 45 others. Those tags come from a multi-label text classifier I trained in Rust on [burn](https://burn.dev), with the Metal backend on a Mac. This is how it went.

## The unit is an ability, not a card

A card is several paragraphs, and each paragraph does something different. So the row is one ability, a single paragraph of rules text with its cost or trigger attached, plus the coarse type of the card it sits on:

```text
artifact | {T}: Add {C}.           -> ramp
land     | {T}: Add {C}.           -> (no tag)
instant  | Destroy target artifact. -> artifact hate, removal
```

The kind matters. 422 cards print `{T}: Add {C}.` and it means different things on a land and on a mana rock. The text alone can't decide it.

Pulling `card_ability` out of the card database and collapsing identical text on the same kind gives 33,412 distinct rows. Keywords like flying are left out on purpose: a regex finds those, and they'd crowd out what needs learning.

## Labels, and the two times the rules were the bug

The tag list is 50 lines in a markdown file, each with a definition, an example, and the Scryfall Tagger tag it corresponds to. A label outside the list is rejected, not quietly accepted.

Labeling was done in batches with a written procedure, then audited: invariant checks, four blind audits, and a 5,000-row deep pass. Twice the audit found that the labeling rules themselves were wrong at scale, and the rules were rewritten and the batches relabeled. The procedure now has two rules near the top that say, in effect, "read these before you label anything, because each was got wrong first."

11,388 abilities labeled when the model started working.

## The model

One ability in, 50 probabilities out. Each tag is its own yes/no question, so the output layer is 50 logits through a sigmoid, not a softmax. Softmax would force the 50 to sum to one, which says "exactly one tag," and that's wrong here. Loss is binary cross-entropy per tag, averaged. Prediction is every tag over a threshold.

Scoring is micro-averaged precision, recall and F1 over tag decisions. Plain accuracy is useless: 97% of the label matrix is zeros, so predicting nothing scores 97%.

Two architectures, measured:

| | from scratch | pretrained BERT |
|---|---|---|
| encoder | 2 layers, 128 wide | bert-base-cased, 12 layers, 768 wide |
| best test F1 | 0.829 | **0.893** |
| with tuned threshold | 0.825 | **0.902** at 0.35 |
| epochs to pass 0.82 | 35 | 4 |
| wall clock | ~15 s/epoch | ~4.7 min/epoch |

The from-scratch model had to discover from 8,000 abilities that "destroy" and "exile" are related. The pretrained one already knew, and only had to learn which of what it knew matters here.

Threshold tuning helped only the pretrained model. The small model's probabilities were badly enough calibrated that trading precision for recall was a wash.

## Loading BERT into burn by hand

burn doesn't ship BERT. Its `TransformerEncoder` is post-norm, which is BERT's layout exactly, so the weights map one to one once you know the names. I wrote the loader: read the safetensors, walk the twelve layers, assign query, key, value, output, the two feed-forward matrices and the two layer norms, plus token, position and segment embeddings. The tokenizer is bert-base-cased's, with padding per batch and an attention mask.

It's the kind of thing you do once and never want to do again. It's also why the F1 went from 0.83 to 0.90.

## What I'd do next

The levers not yet pulled are the usual ones: more labeled rows, a learning-rate schedule, and per-tag thresholds instead of one. The bigger question is whether to keep hand-labeling or let the model propose labels for the remaining 22,000 and audit those. The audit history says the second path works if the audit is real.
