# 0098 Pointing at a ball

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake: newton's cradle and the others should be interactable without having to
point at the table. If I'm pointing at a ball, I should be able to pull or
push it. For newton's cradle it could turn wildly chaotic, so some mechanism
to keep the intent and purpose controllable should be in place, but making it
chaotic isn't bad either.

## Behavior

**The sight asks about the balls before it asks the room.** `Room` holds the
places this building keeps, and a ball is a body on a rope that has moved by
the time the question is answered, so it cannot be in that list. The
watchman is the precedent one room along: he is sighted in
`Arcade::what_i_see` before the room is, with a comment saying he is not in
the building's list and cannot be. `ball_i_see` does the same, nearest ball
first so that pointing between two names the one you would touch.

Boxed rather than a sphere, because everything else here is sighted against a
box and a ball of 85 millimetres is inside the slop of that either way.

**Pull is controlled and push is chaotic, and which you get is which key you
pressed.** That is the answer to both halves of what Jake asked at once, and
it needed no choosing between them.

- Return, or a click: `set_going` on the ball you are pointing at. It is placed on its arc and let go, which is the demonstration everybody knows, and it is exactly what the bench already did except that the ball is yours rather than always the end one.
- Left or right: `shove`, a velocity along the row where the ball hangs. `set_going`'s own note says what that does in a row already touching: the ball spends it on the contact in front of it. What comes out rings rather than clicks.

`SHOVE` is 0.7 metres a second, a little under the 0.9 a ball arrives at
falling from `LIFTED`, so a shove is a hand on a ball and not a bat.

**`away` is its own function now.** Which side of the row a ball swings out
to was worked out inline for the one ball `set_going` could reach, as "nought
goes one way and everything else the other". Two verbs want it and five balls
have an answer, so it is a function and it says the real rule: toward the end
it is nearer. Nothing in the building changes, because the only ball anything
asked for was nought.

**Pressing at the bench still works.** The bench answers as it always did and
lifts the end ball, so the toy is the same from a step back.

## Acceptance criteria

- The sight finds a ball, names the one you point at, and pressing it pulls that ball and no other. — `tests::the_sight_finds_a_ball_on_the_cradle`

Two controls were put back. Never asking about the balls gives "pointing at
the far ball finds None". Pressing the end ball whatever you point at gives
"pressing the ball moved it 0.000".

### Verified by hand

Not yet, beyond the test: the camera work to stand at the cradle and point at
one ball is what the test does, and it does it more precisely than a pose.

## Out of scope

The other toys, which is the rest of what Jake asked for. The mechanism
generalises and the geometry does not: the gyro's rings, the globe, the
metronome's arm, the puzzle's cubies and the wrecker's ball each want their
own parts, and guessing five sets at once is how a sweep gets a room wrong.
One at a time.

A ball you can hold and drag, which is a different verb from either of
these and wants the mouse rather than a key.
