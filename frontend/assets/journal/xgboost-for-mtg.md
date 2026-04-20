# XGBoost for MTG

Magic: The Gathering has a secondary market where a single card can sit anywhere between five cents and several hundred dollars. The price depends on gameplay, scarcity, nostalgia, art treatment, format legality, tribal demand, and whatever the EDHREC crowd is currently brewing around. Predicting that number is a classic supervised regression problem wrapped in a lot of domain knowledge.

I trained an XGBoost regressor that settles around a 0.96 R² on held-out data. Here's the shape of how I got there without giving away the kitchen.

## Data shape

The training set is a merge of three tables — card attributes, legality per format, and current market prices — keyed on each printing's UUID. Card attributes are mostly categorical or boolean (type, rarity, border color, frame version, is this a reprint, is this on the Reserved List). Prices are *heavily* skewed — most printings are cheap, a small long tail drives the chaos.

Two moves at ingest:

- **Drop extreme outliers.** Anything over $200 gets excluded. Including them muddies the model, and the use case for this service isn't "predict a Lotus."
- **`log1p` the target.** Prices are not normally distributed. `log1p(price)` gives the regressor a better-behaved loss surface. Predictions get inverse-transformed at the end.

## Feature engineering, honestly

This was most of the project. The model only sees what I hand it, and raw card data is not where the signal lives. A few categories of engineered features:

- **Gameplay stats.** Power + toughness, mana value, pip density, efficiency ratios — turning raw numbers into "is this creature statted aggressively for its cost."
- **Text complexity.** Parsing the oracle text for keyword abilities, modal choices, ETB triggers, activated abilities. A wall of text isn't always more valuable, but *the right wall* correlates with price.
- **Format legality and pressure.** Aggregating across ~20 formats into a legality score, with a separate weight on the high-power formats (Legacy, Vintage, Modern, Commander).
- **Collectibility.** Reserved List × old frame, promo types, premium treatments, serialized/galaxy/surge foils, and similar. These stack.
- **Popularity priors.** EDHREC rank and saltiness, age since first printing, reprint frequency. A card reprinted every set doesn't behave like one printed once in 1998.
- **Tribal and archetype signals.** Popular tribes, combo-potential word patterns, build-around indicators.

Everything categorical — rarity, finish, border color, price provider — goes through one-hot encoding. The model ends up with a wide feature matrix.

## Model

XGBoost regressor, `tree_method="hist"`, moderate regularization, a large number of estimators at a small learning rate, depth tuned so individual trees can capture interactions between the collectibility, format, and popularity features. Cross-validated with three folds for evaluation, then refit on the full training split. The held-out test R² settles around 0.96 with an RMSE that makes sense for the price band the model is scoped to.

Feature importance gets dumped to a ranked CSV after every run. That's how I find out whether my latest "clever" feature is actually doing anything — most of the time it isn't, and I delete it.

## What I'd do differently

The biggest honest gain from here would come from sourcing, not modeling — price history over time instead of a single snapshot, booster-opening probabilities, print-run leaks, movement around set releases. The model is good. The data could be better. That's usually the answer.
