# 0069 Asking the arcade for something

**Status:** implemented
**Date:** 2026-11-13

## What this is for

Every option this building takes is a flag, and `--help` prints the list.
Jake, 2026-11-13, on the line he had to type to look at it:

> this is cumbersome, the arguments seem to belong in different places, it's
> just not ergonomic: `WHERE=1 cargo run --release -- --done 2> /tmp/arcade.log`
> there's also no --help for any of it

Eight of its options were environment variables and seven were flags, with
nothing at all deciding which was which. The engine's spec 0100 has the cause:
`one_argument` was private, so a game that wanted an option reached for
`std::env::var`, which needs no parser. Whichever a given option was is the
order the two were written in, and nothing else.

## What it does

**Every one of them is a flag.** `--pose`, `--where`, `--at`, `--day`,
`--sky`, `--fly`, `--open-case` and `--dump-mirrors` were `POSE`, `WHERE`,
`AT`, `DAY`, `SKY`, `FLY`, `OPENCASE` and `DUMP`. The variables are gone
rather than kept as aliases: two ways to say one thing is the split this spec
is about, written down twice.

**And every one of them is read by the engine's parser**, so `--name value`
and `--name=value` both work, which they did not before: `--walk` and `--film`
took only the second and `--done` took neither.

**`says::OPTIONS` is the list**, with the engine's printed under it by
`Game::options`. It was one block of copy with one marker on a line of its
own, which is the form `sign-copy` leaves nothing behind in.

**And a list rather than that block, since engine spec 0109, 2026-11-19.**
The columns were typed into the text and the whole thing printed as it
stood, so nothing could ask it what is *not* an option and `--flyy` opened
the building and walked. Each option is an `Asked` now, carrying its name,
what its value is called and one sentence; where it all sits on the line is
worked out where it is printed. The words are the same words. The building
refuses an option it does not take before the window opens.

**Except the ones answered before the engine sees them.** `--screenshots`,
`--walk`, `--tour` and `--film` are read in this game's own `main` and
answered there, so a typo of one of those still falls through to the window.
Engine spec 0109 leaves that where it is: moving them behind the engine is a
change to how this game starts.

**Nothing says a trail exists.** Spec 0036 is plain that somebody who never
finds the trail must not be able to tell it is there, and `--done` is now in a
list anybody can print. It is a list of what the command line takes and the
rule is about the game, the same argument the flag itself was given under; the
line here says what it does without saying there is anything to find.

## Acceptance criteria

- Every option the arcade reads is in its list. — `tests::the_help_names_every_option_the_arcade_takes`

### Verified by hand

- `cargo run --release -- --help` prints both lists and opens no window,
  the two sharing one column.
- `cargo run --release -- --flyy` says it is not an option and opens no
  window, where it used to open one and walk.
- `cargo run --release -- --where --done 2> /tmp/arcade.log` does what the old
  line did.

## Out of scope

- **Keeping the variables working.** Two ways to say one thing is the fault,
  not the fix. Anything of Jake's that sets one is a line he types, and this
  spec is the answer to his having to type it.
- **Rejecting an option nobody knows.** A typo is still as quiet as it was.
  Engine spec 0100 has the reason.
