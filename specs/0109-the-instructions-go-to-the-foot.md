# 0109 The instructions go to the foot

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake, playing: the instructions could be a lot more readable, both visually
and in copy. It covers the actual thing it describes, it could be at the
bottom of the screen with more distinct colours and spacing. Even the font
could be better.

## Behavior

**Under the sight is on top of the thing.** Spec 0003 put what to press under
the sight and said so in those words, 34 below the middle. What is under the
sight is also what the sight is on, so a plate there lands on the toy it is
naming: a cube 0.3 across, read from under a metre, had its bottom row behind
its own instructions.

The sight keeps the middle and the words go to the foot. `PROMPT_FOOT` is 64
off the bottom, which put the plate 54 physical pixels clear of the window
edge in the shot it was set from.

**A block pinned at its foot cannot be laid out a line at a time**, because
where the first line goes depends on how many rows the last one wraps to. So
`prompt_at` is gone and `said` measures the whole block before placing any of
it. Two accounts of where the words are is the fault this building keeps
finding; one function now answers it.

**Two inks, because they are two things.** The name is what you are pointing
at and the line under it is what you can do to it, and at one colour and two
sizes the pair read as one paragraph that started big. The name takes the
warm marquee colour this building already puts on a lit sight; the keys take
a cool white, being the half you read every time rather than once.

**And the words sat against the edge of their own shade.** `PLATE_PAD` goes
from 7 to 14, with `PROMPT_GAP` of 12 between the name and the line under it.
Padding is what makes a plate read as something the text sits on rather than
a box drawn round it.

**The copy is three short sentences of one shape** instead of two clauses and
a comma: left or right turns this face, up or down rolls the column, shift
and up plays the next move. It carries its marker until Jake has it.

## Acceptance criteria

- The prompt ends near the foot of the window, is centred, stacks downwards, and its plate does not reach the sight. — `aim::tests::the_prompt_sits_at_the_foot_and_clear_of_the_sight`

The control is the old anchor: putting the block back under the middle gives
"at (1280, 800) the plate ends at 505 of 800".

**The gap is the claim and not the anchor.** A test that pins only the foot
passes for a block tall enough to reach the middle again, which is exactly
how this would come back, so the plate's top is measured against the sight as
well.

### Verified by hand

By eye, on shots of the puzzle's plate before and after, taken at the same
pose.

## Out of scope

**The font.** It is `PressStart2P` for every word in every game, loaded once
in the engine, whose own stencilling test names two fonts in one building as
the fault it guards against. Changing it
is an engine change that moves six games and every screenshot of them, and
which typeface is Jake's to pick. The case for it is that a pixel face is
right for a marquee and wrong for a sentence, which is what this plate is.

The help line along the top, which describes the keys rather than the room
and so is not over anything it names.

The other toys' plates, whose words come from `about` and are Jake's already.
