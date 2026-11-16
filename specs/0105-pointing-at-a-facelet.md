# 0105 Pointing at a facelet

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake: cubies should get extra special treatment, controls to manipulate it
manually, as well as hints and a step by step full solution as an option. The
last of the six toys, after specs 0098, 0099, 0102, 0103 and 0104.

## Behavior

**The face and not the cubie**, though a cubie is what you point at. What you
can do to a cube is turn a face, and which face a facelet is on is the whole
of what pointing at it tells you; a corner shows three and you have named
one. `face_at` reads it off the hit point by the axis it stands furthest
along, taken from the cube's own middle and matched against `face_way`, so
the answer is in the same list the turns are written in.

One box and then that arithmetic, rather than twenty six boxes and a search.

**The hand is on the arrows.** Left and right turn the face you are pointing
at, a quarter each way. A turn goes on the same list the solver's moves do
and is swept over `TURNS_AT` like any other, so a hand's turn and a solver's
look the same and spec 0050's one account of where the pieces are still
holds. A cube in the middle of something is left alone, the way `worked`
leaves it alone.

**The hint is the first move of a solution, worked out and not kept.** A
solve of a scrambled cube measures 58 microseconds cold and 7 warm, a third
of one per cent of a frame, so the thing that keeps this honest is that there
is nothing to hold in step: ask the cube. Nothing while it is moving, because
the answer would be about a state halfway to somewhere else.

It is written the way a cuber writes one, `R'` and `F2`, and that is not
copy: the five letters are the notation, the same in every language this
might be read in, and a hint saying "turn the right face anticlockwise" would
be a sentence where `R'` is the thing itself.

**Shift and up is the step**, which makes the whole solution something you
walk rather than watch. `worked` at the bench still plays all of it.

**And `worked` is not a way home**, which is the fourth toy in a row to say
so. It solves a scrambled cube by queueing every move and playing them, and
that takes as long as it takes; `home` is solved at once. Spec 0104 said the
next toy should be assumed to have this gap until shown otherwise, and this
one had it.

## Acceptance criteria

- The sight names the face you point at, the arrows turn it, a step gets the cube nearer, and R solves it at once. — `tests::the_sight_finds_a_face_and_the_arrows_turn_it`

Three controls. Never asking about the cube gives "pointing at the cube finds
None". A `face_at` that always says up gives "the face toward +x is Up". An R
that plays the solution rather than jumping to it gives "R left it unsolved".

**A step gets nearer and does not solve**, which is what the test says and is
not what it said first. It asserted that one step from one turn out would
leave the cube solved, and that failed: the solver answers a single `R` with
`R` then `R2`, which is two moves to undo one and is a valid solution rather
than the shortest. `away` is the length of a solution, and a step making it
shorter is the claim that is true.

**And one test elsewhere had to learn the new name.**
`the_sight_finds_what_each_work_in_the_clip_works` aims the clip at the
puzzle's bench and expects `Seen::Bench`. The sight now finds a facelet
first, and pressing a facelet presses what the bench presses, so what moved
is the name and not the work. It takes a part as its bench, for all six
toys rather than this one.

### Verified by hand

Not by eye yet.

## Out of scope

Dragging a face round with the mouse, which is the other way a hand turns a
cube and wants a grip rather than a key.

The copy. The plate carries `[COPY - Jake]` on both of its lines.
