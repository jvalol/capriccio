# 0081 A moon built in bands

**Status:** implemented
**Date:** 2026-11-15

## What this is for

Jake, 2026-11-15: the gate. It is slow, and the reason written down for it
was that every picture test stands up its own GPU device. That is false.
Standing one up is 39 milliseconds of a picture test that takes 1.43
seconds: an instance, an adapter and every pipeline together.

Where the time goes, measured rather than guessed:

| | |
| --- | --- |
| `Arcade::load` | 1130 ms |
| of which GPU upload | 69 ms |
| `moon::face` | 527 ms |
| the orrery's worlds | 229 ms |
| the garden's clouds | 77 ms |
| `valhalla::night_sky` | 74 ms |
| the galaxied skies | 71 ms |
| the starred walls | 25 ms |
| drawing the frame | 35 ms |
| holding it against the file | 0.7 ms |

Six builders are 89 percent of a load and the moon is near half of it on
its own. Every picture test pays for all of them, and most of them show no
sky at all: the cellar, the nook, the hall inside, the glass room and the
village all build a forty-five thousand crater moon that is not in frame.

## What it does

**The ground is built in bands of rows, one to a core.** `Band` is a run of
rows with its own slice of the height and the colour, and `laid` is
everything that ever happened to one of them, in the order it happened.
`std::thread::scope` and `chunks_mut` hand each thread its own rows, so
there is no sharing to get wrong and no dependency to add.

**Rows and not craters.** The craters are laid in the order they fell and
each one blends with what is under it, so dealing them out to threads would
change the picture. Every band walks all forty-five thousand in the same
order and writes only inside its own rows, which is why what comes out is
what one thread wrote, to the bit.

**And the clip is on the loop rather than the write.** A crater that misses
a band walks none of that band's picture, which is what stops eight threads
each doing the whole job.

## What it bought, and what it did not

A picture test on its own: 1.43 seconds to 0.99. The run of twenty-two
picture tests one at a time: 25.6 seconds to 18.1. A load: 1130
milliseconds to about 690, which is the same four tenths of a second off
the black screen the game opens with.

**The whole suite is unchanged**, at 62 seconds either way. `cargo test`
runs its tests across every core already, so the picture tests were
overlapping each other before any of this and the machine was saturated.
Threading inside a test and threading across tests are the same cores.
What that means for the gate is that the win here is the game's own
startup and a single test, not the wall clock of a full run. Doing less
work is the only thing that moves a saturated run, and six builders that
nothing in frame can see are where that is.

## The shading pass is not banded, and was

`moon::face` works out a slope and a light at every step of the picture
after the ground is finished. It was banded too, and it bought five
milliseconds of a second and a half, because the slope and the light are a
fraction of what laying forty-five thousand craters is. It came back out.
Threading pays where the work is, and twenty-five lines that buy nothing
are worse than the loop they replaced.

## What it does not do

- **It does not make the moon cheaper**, only wider. The same craters are
  laid and the same arithmetic is done.
- **It does not touch the other five builders.** They are independent of
  each other and could be run together, which has the same ceiling as this
  one: nothing, on a saturated run.
- **Nothing is built lazily.** The moon is still made before the first
  frame whether or not anybody looks up.

## Acceptance criteria

- Dividing the picture between threads does not show in it, including a band count that leaves a ragged end. — `moon::tests::the_moon_is_the_same_however_it_is_divided`
- The moon still looks like the moon. — `tests::the_moon_is_drawn`
- And its seas, its craters and its edge are where they were. — `moon::tests::the_seas_are_darker_than_the_highlands`, `moon::tests::a_crater_leaves_a_hole_where_it_was_put`, `moon::tests::the_sun_leaves_a_dark_edge`

### Verified by hand

- The picture is byte-for-byte the one that was there before. — the
  reference file did not move.
