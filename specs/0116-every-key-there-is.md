# 0116 Every key there is

**Status:** implemented
**Date:** 2026-11-17

## Goal

H opens a list of every key the arcade binds, in two groups, growing out of
the line already in the corner.

## Why

Twenty keys were bound and the corner named five. The rest were learned by
reading `process_keyboard`, and the comment on the newest of them says so in
as many words: "One more undocumented inspection key is the convention here
rather than an exception to it." A convention that a key is a secret is worth
ending rather than extending.

Jake, playing: there should be at least a mode where a user can see what any
key action is possible. Asked which keys belonged in it he said everything,
with the inspection keys as a second group, and asked where it should go he
said it should grow out of the corner line.

## The thing that decides the whole of it

**A table and a test, rather than a dispatch driven by the table.**

The obvious shape is one list of bindings that both the handler and the screen
read, so that neither can name a key the other does not. That shape does not
survive contact with what these keys do. They reach into `Arcade` twenty
different ways: a field toggled, a method called, a value wound, two of them
conditional on where you are standing. As data each row would carry a closure
over `&mut self`, and twenty of those is a worse `match` than the one already
there, written twice.

So `keys.rs` holds the names and `process_keyboard` keeps its arms, and what
stops them drifting is a test that reads the dispatch as source text and
fails if anything is bound that the list does not name. That is the same
shape as `./check-listings`, which holds a readme against the disk rather
than generating one from the other, and it is the shape this project reaches
for whenever two accounts of one thing cannot be collapsed into one.

**The test needs a test.** A source scan that finds nothing reports that
nothing is unnamed, which is the one failure here that looks exactly like
success. `the_dispatch_is_actually_being_read` asks that the slice holds at
least ten keys. This was not hypothetical: the first control written for it
passed, because breaking the search widened the slice to most of the file
instead of emptying it, and both tests sailed through on a region that
happened to contain only named keys.

## Behavior

- **H** toggles the list. A toggle and not a hold, like the four inspection
  keys beside it: a list you have to keep a key down to read is a list you
  cannot walk away from while reading.
- The lines are appended to the corner's own plate, under whatever that
  corner already says, and wrapped to the window the same way, because the
  plate is measured off what is in `lines`.
- Two groups. **Getting about** is what the building is played with.
  **Looking at the building itself** is the five keys each added for one
  afternoon's hunt and then kept: O, F, P, M and G.
- A row is a label, the keys it actually is, and what it does. The label is
  the reading form, so WASD is one row and not four.
- G is in the second group rather than the first. Its own comment already
  called it an inspection key, and flight is a way of looking at a building
  rather than a way of being in one.
- The sixteen strings are signed copy. `src/keys.rs` is in `.copy-files`, so
  changing any of them un-signs the file.

## Acceptance criteria

- Nothing `process_keyboard` answers to is missing from the list. — `keys::tests::every_key_the_arcade_binds_is_in_the_list`
- The dispatch is actually being read, so the check above cannot pass on an
  empty slice. — `keys::tests::the_dispatch_is_actually_being_read`
- Every row names something, and no key is in two rows. — `keys::tests::each_key_is_named_once`

### Verified by hand

- H opens and closes it, and it lands under the corner line rather than in a
  panel of its own. — run the arcade and press H.

## Out of scope

**The dispatch is not driven by the table**, for the reason above. The test
is what holds them together, and a test is a weaker guarantee than a single
account: it catches a key bound without a name, and it cannot catch a name
whose description has gone stale.

**The headings do not read as headings.** They sit in the same column and
weight as the rows, so the list reads as fourteen similar lines with two odd
ones in it. The row labels are ragged too, so the second column never lines
up. Both are layout rather than wording and neither is answered here.

**Nothing is said about what a key does while it is unavailable.** Tab lists
the solar bodies only out past the door and L swings only the system that is
there, and the list names both without that condition.
