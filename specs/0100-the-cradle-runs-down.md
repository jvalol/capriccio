# 0100 The cradle runs down

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake, playing: gravity doesn't slow things like the cradle down
realistically.

## Behavior

**Gravity was not the fault.** The swing's period measures 1.4547 seconds
against an ideal pendulum's 1.4466 at this rope and this amplitude, which is
six parts in a thousand. What was wrong is where the energy went.

**It was losing almost all of it in six seconds and then losing none at
all.** Measured as the widest any ball reaches over a second and a half, from
a lift at `LIFTED`:

```
          was              is
   0s   0.294 m         0.294 m
   3s   0.165           0.247
   6s   0.061           0.217
  10s   0.061           0.123
  16s   0.061           0.040
  24s   0.061           0.033
```

The old column stops falling. Six centimetres at ten seconds, at sixteen, at
twenty four: the same number for ever. That is not a cradle running down, it
is a cradle buzzing, and the buzz is the swing spread across all five balls
as a jitter none of them can lose.

**`BOUNCE` was the brake and should not have been.** Nothing else took speed
off a ball, so the only way to stop a row that would otherwise drift for ever
was to make every blow lossy, and the constant carried exactly that note: not
one, or the thing never stops. At 0.92 a blow throws away 15 per cent of the
energy, and a steel ball gives back 0.95 to 0.99 of one.

`KEEPS` is the brake now, at 0.955 a second, which is where the loss belongs:
a swing dies to the air over its whole arc, not to the one instant it touches
the next ball. `BOUNCE` is 0.99. Drag also takes the buzz out faster than it
takes the swing, which is the shape of the thing: a jitter at a tenth of the
amplitude is a tenth of the speed and loses the same share of it every step.

**`step` is one function now.** The drag went into the game's loop first, and
the toy's own tests stepped the solver directly, so the row they watched was
not the row the building draws. `it_runs_down_rather_than_on_forever` failed
the moment `BOUNCE` went up, which is how that was found. A thing that knows
how it is strung should know what it loses.

**Measured as a swing and not as `stirring`.** That function reads one
instant, and says 0.048 at three seconds and 0.405 at ten, because the answer
depends on where in the arc it was asked.

## Acceptance criteria

- It is still worth watching at six seconds, and the swing keeps falling rather than settling on a buzz. — `cradle::tests::it_still_swings_and_keeps_running_down`
- And it is quiet by half a minute, which `cradle::tests::it_runs_down_rather_than_on_forever` already held and still does.

Both halves were put back as controls. The lossy blow gives "six seconds in,
the widest swing is 0.053 m of a 0.294 start". No drag gives "it swings 0.256
at six seconds and 0.256 at sixteen, which is a buzz and not a run down",
which is the old fault in one line.

### Verified by hand

Not by eye yet. The numbers above are the measurement, taken the same way for
both columns.

## Out of scope

Why the energy spreads into a buzz at all, which is the solver's contact
handling and not this toy's. The drag covers it rather than curing it: a
cradle that settles looks right whether or not the jitter it settles out of
should have been there.

The other toys on the same solver. The wrecker and the gyroscope have their
own losses and nobody has stood and watched them not stop.
