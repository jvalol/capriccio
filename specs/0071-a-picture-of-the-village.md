# 0071 A picture of the village

**Status:** implemented
**Date:** 2026-11-14

## What this is for

Everything built into this building over the last week is outside it: the
grain on the boards, the people on their rounds, the damped yard fires, the
lean, the horses. All of it is in the village out in the field and in the
steading at the top of the bridge, and nothing published shows either.

The obvious answer was the clip, and the numbers rule it out. The village
stands 56 metres west and 46 south of the garden's way out, the route is one
continuous walk at 4.2 metres a second, and `tour::ways` cannot lay places in
the field at all: it says so itself, that a stop two metres past the wall
plans no way. Reaching the village is a walkable patch of field plus 34
seconds of round trip across it, on a route that is 54.2 seconds long. That
is the same arithmetic that cut the solarium's sun and the imaginarium off
this route, both of them Jake's call with the take in front of him.

So the picture goes where the pictures already are. xanadu's entry on the
engine's games page carries four stills and all four are the solar system
behind one wall. Jake, 2026-11-14, picking it off three: stills on the games
page.

## What it does

**A fifth staged shot, `--screenshot=village`.** `shots()` already answers
`refresh-screenshots` with a slug and a file apiece, so this is one more
line, and the photographer takes it with the other four and leaves it alone
when it has not changed.

**The pose is shares of the street, not metres.** A third of its length out
from the middle, an eighth of its width off the centre line, looking back
down it from inside the end you would arrive at. Written as shares because a
picture framed in metres comes apart the day a cabin moves, which is the
fault `village::around` already exists to answer.

A quarter of the width off was the first framing, and it was wrong in the way
Jake has named once before on the arcade's own still: the near cabin stood
across a third of the frame, and the picture's job is the street.

**It names an hour and a sky.** This is the only staged shot in this
building that is out of doors; every other one is in the hall or behind its
wall, where the hour decides nothing. Mid-morning, so the fronts down one
side are lit and the other side is in shade, under a fair sky, found with
`garden::under` the way every outdoor picture test finds one. That function
was `cfg(test)` on the grounds that nothing which runs wants it. Something
does now: the first day this building ever has is a storm, and the first
three takes of this shot were that storm.

**And a long day while it is taken.** The photographer runs a game for five
seconds before it draws, which at `garden::DAY` is a quarter of a day: the
first take asked for mid-morning and got the middle of the night. A day of
twenty minutes makes those five seconds a two-hundredth of a day. The film
solves the same problem the same way with `tour::FILM_DAY`, and this has its
own number rather than borrowing that one, because how long a film may take
to cross an evening and how little five seconds may move a sky are two
questions that happen to have the same answer.

## What it does not do

- **It does not change the clip.** The route, its length and its still are
  untouched.
- **It says nothing about a trail.** Spec 0036 is plain that somebody who
  never finds the trail must not be able to tell it is there, so the page
  gains a village and not a bridge.
- **The steading is still unpublished.** It is only reachable with `--done`,
  which `refresh-screenshots` does not pass, and that is a separate job.

## Acceptance criteria

- The staged shot stands in the street and looks along it. — `village::tests::the_staged_shot_looks_down_the_street`
- Its hour is one the sun is up for. — `village::tests::the_shot_is_taken_in_daylight`
- Every shot the game offers has a file on disc. — `tests::every_staged_shot_has_a_picture`

### Verified by hand

- The picture shows a street rather than a cabin. — `./refresh-screenshots
  xanadu` and look at `xanadu/media/village.png`.
