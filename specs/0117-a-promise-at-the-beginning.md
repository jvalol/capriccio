# 0117 A promise at the beginning

**Status:** implemented
**Date:** 2026-11-17

## Goal

Bifröst is visible from the first minute, unfinished, reaching out over the
field and stopping in mid-air. Each question the building asks adds a span.
Finishing the trail finishes the bridge.

## Why

Jake, 2026-11-17: the concept of a trail in its current form is weak, there
needs to be a mission or goal. Asked which way to take it, he said a promise
at the beginning is better.

**The cause is in spec 0036's own rules.** It forbids a checklist and it
forbids letting anybody who missed the trail know it is there. Those two
protect the rooms and they are right. Together they also remove the one
thing a goal needs, which is knowing you have one, so what was left was five
things you would do anyway and a bridge appearing at the end for reasons you
cannot connect to anything you did.

**A half-built bridge is not a sign.** 0036's worry was a clue saying GO
LEFT. This is spec 0020's worn lane at the scale of the building: a thing
that is, rather than a thing that tells. Nobody has to be told the rule to
want the far end, and somebody who never works it out has still spent the
whole time looking at a strange unfinished thing on the skyline, which is
more than the secret they never knew about gave them.

## The thing that decides the whole of it

**`shows` becomes `reach`.** Today `bridge::shows(trail)` is all five
questions or nothing, and `landing`, `piers`, `parapet_runs`, `treads` and
`carries` each ask it and return nothing. One predicate becomes one
fraction, and every one of those asks it instead, so there is one account of
how far the bridge has got rather than six.

**The foot is always there.** `reach` is `(given + 1) / (Asked::every() + 1)`
rather than `given / every`, because a bridge that is nothing until the
first question is answered is not a promise, it is the same surprise moved
one step earlier. At nothing given it is a sixth of the run, which at
`RUN` 100 and `RISE` 26 is a stub standing about four metres out of the
field: enough to see from the garden, not enough to read as a way anywhere.

## Behavior

- The bridge is drawn whenever the field is, at every count including none.
- Five questions and six states, so a span is a fifth of what is left after
  the foot. `high(along)` is unchanged and still the one curve: what moves
  is where the deck stops, not its shape.
- **It grows rather than appears.** The garden's lanterns wind up over a
  long minute rather than switching, per spec 0019, and the carving on the
  bridge already breathes for the same reason. A span that popped would be
  a notification. A span that takes a few seconds is the building doing
  something. You will usually be in another room when a question is
  answered, and that is fine: you find it longer next time you look.
- **The cut end is a cut end.** No parapet across it, no rail. You can walk
  up what exists and run out of bridge, and the drop at the end is real.
  That is the want made physical and it is the whole point of the shape.
  Jake, 2026-11-17, asked about exactly this: the fall makes sense.
- `carries` carries you up the spans that exist and no further.
- Nothing inside any room changes. The bridge is in the field.

## Its edge was sampled once a wave

2026-11-28. `bank_rim` wobbles the bank's edge with four sines, the
quickest at 9.1 radians a metre, which is a wave 0.69 metres long.
`BANK_ALONG` was forty steps over the bank's own stretch, so the surface
read that edge every 0.48 metres: barely once a cycle. Neighbouring steps
took their spread from opposite sides of the wave, the bank zigzagged in
plan, and faceted normals off a zigzag swing hard. Against a low sun every
other facet blew out and the bank wore pale plates laid up its slope.

**Found by painting it.** Four guesses missed — the surface's own ripple,
the section's resolution, the loose stones lying on it, the angle of the
slope — and each cost a build and a look. Painting the bank bright red
settled it in one: the plates went pale red, so they were the bank and not
anything lying on it, and their boundaries were its along-steps.

A hundred and twenty steps is 0.16 metres, four samples to the shortest
wave, and it costs nothing: this is one mesh built twice a run.

**It took a change that was reverted to make it visible.** The bank was
briefly banked up against the deck's side, which turned its flank towards
the sun; that is gone, per spec 0054, and at the height the bank actually
stands these plates were not something Jake would have seen. The
under-sampling was real either way.

## The stones lie on the bank

Turned about their own up alone, the loose stones kept a level top however
steep the ground under them. On a flat bed that is right and on a slope it
is not: a level face is the one that catches a low sun, and two hundred of
them read as plates. Each is now tipped onto the bank it sits in, off
`bank_up`, and spun on that. `bank_up` is worked off `bank_at` rather than
written out, which is the only way it stays a normal to the thing it is a
normal to.

## Acceptance criteria

- With nothing given, the bridge is drawn and its deck stops a sixth of the
  way along. — `bridge::tests::the_foot_is_there_before_anything_is_asked`
- Each question given puts the end one span further out, and five puts it at
  the top. — `bridge::tests::a_question_adds_a_span`
- What holds you up stops where what is drawn stops, so there is no
  invisible deck past the end. — `bridge::tests::you_cannot_walk_past_the_end`
- And no gap the other way: every drawn piece has tread under it. —
  `bridge::tests::every_piece_drawn_can_be_stood_on`
- The foot never moves when a span is added, since only the far end grows. —
  `bridge::tests::it_grows_from_the_far_end`
- One account of how far it has got: a question given puts the end one span
  further out, and the foot never moves. —
  `bridge::tests::a_question_adds_a_span` and
  `bridge::tests::it_grows_from_the_far_end`

  **This said that nothing asks `Asked` directly but `reach`, and spec 0118
  made that false on purpose.** `bridge::still_held` reads the list to find
  the first question not given, and `bridge::span_colours` reads it in the
  order it was given. Neither is counting how far the bridge has got, which
  is what the rule was about; both are asking what the trail says, which is
  a different question and is the whole of what 0118 is for.

### Verified by hand

- It can be seen from the garden at nothing given, and reads as a thing
  rather than as scenery. — stand at the garden's west opening and look.
- The drop off the cut end is a fall worth having rather than an
  annoyance. — walk up an unfinished bridge and off it.

## Out of scope

**The surprise.** It is spent on purpose. Jake took that trade when he
chose the promise, and 0054's ending is now a completion rather than a
reveal.

**What is on the far side.** Still open, as 0036 left it, and this does not
touch it.

**The nook's and the baths' questions.** Five questions means five spans;
if a sixth is added the arithmetic follows `Asked::every()` and nothing
here is written down twice. Which act each of those rooms asks for is spec
0036's open question and is Jake's.

**A counter anywhere.** No panel, no ticks, no "3 of 5". The bridge is the
only thing that says how far you have got, and it says it by being that
long.
