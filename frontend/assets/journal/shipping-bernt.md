# Shipping Bernt

Bernt is an iOS app that knows what's in your pantry and writes the week's recipes from it. The AI part was the fun part. Getting it onto the App Store, with accounts and subscriptions and nothing that could embarrass me in review, was most of the work. This is what that work looked like.

## The shape of it

- **iOS**: SwiftUI, Swift 6, XcodeGen. The `.xcodeproj` is generated and gitignored; `project.yml` is the truth.
- **Server**: Hono on Bun, one container on Fly. It serves the app's API, the marketing and legal pages, and Apple's billing webhooks.
- **Database**: Turso. Hand-written SQL migrations applied by geni. No ORM.
- **Models**: Groq's `gpt-oss-120b` for recipes, meal plans and edits. Claude as the fallback, with the same JSON schema on both rungs so the swap is drop-in.

That last line is the one I'd underline. Every generation call passes a schema and gets back a parsed object. When Groq is down, the call goes to Anthropic with the same schema object, and nothing downstream notices.

## Subscriptions are where the bugs live

Apple sends server notifications for every subscription event: renewals, refunds, grace periods, billing retries. They arrive out of order. A delayed `DID_RENEW` can land after a `REFUND`, and if you apply it naively the refunded user gets Pro back.

The fix was a watermark. Each subscription row records the `signedDate` of the newest transaction applied to it, and the webhook skips anything signed earlier. Forward-only, via `MAX`.

The second bug was mine. The webhook acknowledged every notification with a 200 and marked it processed before the work was done. A transient database error mid-apply lost the notification forever, because Apple's retry was dismissed as a duplicate. Now a failure releases the dedup marker and returns a 500, and Apple redelivers on its own schedule.

The third was a status mapping. `REFUND` wrote `cancelled/free`, but `cancelled` was in the set of statuses that keep Pro access, because a user who merely turns off auto-renew keeps what they paid for until it lapses. Refunded users kept Pro until their expiry date. Refunds now write `expired`.

None of this is clever. It's the kind of thing you only find by reading your own webhook handler slowly, as if someone else wrote it.

## Sign in with Apple

Native Sign in with Apple is a token exchange. The server verifies the identity token against Apple's JWKS with `jose`: signature, issuer, audience and expiry. The client generates a nonce, sends its SHA-256 in the request, and posts the raw nonce back with the credential, so the token is bound to the exact authorization request.

One endpoint handles signup and login, keyed on Apple's stable `sub`. There's no auto-linking: an existing account with the same email gets a 409 and a pointer to password login. Apple-only accounts carry an empty password hash, and every password-shaped endpoint has to know that. Delete-account in particular must work without a password, because App Review requires deletion to work for every kind of account.

The tests sign real ES256 tokens against a local JWKS, so each rejection path runs for real instead of through a mock.

## The model is an untrusted intern

A recipe title saved today is a prompt injection read tomorrow, because it gets fed back into a later generation. So stored content goes through two layers at write time: a regex pre-screen for known-hostile shapes, and, on a hit, a classifier call to Claude Haiku with `max_tokens: 8` that answers SAFE or UNSAFE. It fails closed.

The classifier stays on Haiku even though Groq is the primary everywhere else, because the eval caught Groq's 20B model marking clear jailbreaks as SAFE. That's recorded as a decision with the eval that backs it, not as a preference.

## What I'd tell myself at the start

- Write the sanitize boundary first. One module where AI output becomes database rows, with length caps, enum defaults and renumbered ordinals. Everything downstream can then trust its input.
- Read Apple's notification docs twice, then write the out-of-order test before the handler.
- Ship the telemetry on day one. The retention sweeps, the error log, the cleanup heartbeat. You will want them the first time something goes quiet.

Bernt is free on iOS 17 and up at [bernt.app](https://bernt.app).
