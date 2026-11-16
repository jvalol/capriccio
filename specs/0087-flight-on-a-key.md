# 0087 Flight on a key

**Status:** implemented
**Date:** 2026-11-15

## Goal

Jake: in `--fly` mode, make there a way to toggle it on and off. Falling to
the ground is fine.

## Behavior

`--fly` was a decision taken before the window opened. Spec 0041 built flight
as a tool for getting to where a fault can be seen, and the thing you actually
want is to walk a room, go up to look at the roof of it, and come back down
without starting the game again.

`G` toggles it. The letter nearest the word belongs to spec 0056's frame
counter, and the rest of this family is already arbitrary: `O` outlines the
colliders, `L` swings the solar system out, `M` says where you are, `P` finds
coplanar faces, `R` puts you back at the door. One more undocumented
inspection key is this building's convention rather than an exception to it,
which is also why nothing in `says.rs` mentions it. Say the word and it gets a
line of help, at the cost of a signing pass on the whole copy file.

Flight is now state on `Arcade` rather than a function asking the command
line, because a function asking the command line can only give one answer for
the whole run. `flying()` is read once at construction and nothing else asks
it.

**Nothing is done about where you are when it goes off**, which is what Jake
asked for. The next frame calls `gone` instead of `flown`, and `gone` is the
one that knows about gravity, so you fall. Measured: from twenty metres the
drop runs 20.00, 17.42, 12.92, 8.42, 3.92 and settles at 3.77, which is the
hall's roof. Turning it back on mid-fall stops you where you are: off at one
second and on again at one and a half leaves you hovering at 17.27 for the
rest of the run.

## Acceptance criteria

- Flight goes where the walk cannot, which is the branch the toggle picks between. — `tests::flying_goes_through_what_walking_does_not`
- Flight follows the pitch, which is the other half of that branch. — `tests::flying_follows_the_pitch`

**And the toggle itself has none**, which is the honest position rather than
an oversight. It is `self.flying = !self.flying`, and the two tests above
already hold both sides of what it chooses between; a test of the flip would
be a test of `!`. What is genuinely uncovered is that `--fly` still seeds the
state, and that needs an `Arcade` built with an argv, which nothing in this
crate can do.

### Verified by hand

The engine plays a key log, so this was driven rather than pressed:

```
printf '1.0000 down G\n1.0333 up G\n1.5000 down G\n1.5333 up G\n' > /tmp/midair.log
./target/release/capriccio --fly --pose "0 20 -2 21.55 0.0 -10.0 120.0" \
  --play /tmp/midair.log --picture-for 4 --picture-to /tmp/a.png --where
```

- Pressed once, the height falls away. — the run above with only the first pair of lines.
- Pressed twice, the fall stops in the air. — the run above entire, which holds 17.27.

## Out of scope

Help text, for the reason above.

Anything about the rooms popping. `flying`'s own doc lists seven things
decided by which room the building thinks you are in, and flight breaks that
assumption on purpose. Spec 0085 took one of the seven off a half space
earlier today; the rest stand, and a key that turns flight off does not change
any of it.
