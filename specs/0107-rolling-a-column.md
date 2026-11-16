# 0107 Rolling a column

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake, playing: i can only rotate a face left or right, not a slice forward
or backward.

## Behavior

**Pointing at a face gave you that face and no other.** Spec 0105 read the
sighted face off the hit point and bound left and right to a quarter of it
each way. That is one axis of the three, and which one you get is decided by
where you happen to be standing: in front of the cube you can turn the front
face and, by looking down, the up face. Nothing rolls a layer away from you.

**Up and down roll the layer the crosshair stands in.** The axis is the cube
axis nearest your own right hand, so pushing up on a column sends that column
over the top, away from you. This is the gesture every cube on a screen uses
and the reason it is taken off the camera rather than off the cube: which way
is right depends on where you are standing, not on which face this is. Turn
your head without moving the crosshair off a facelet and the answer changes,
which is why `rolling` is asked at the keypress and nothing is kept in `Seen`.

From one place you now have three faces rather than one: the face you point
at, and the two layers either side of the column you point at.

**The middle column rolls nothing, and this cube cannot be taught to.** A
`Cube` is corner and edge places with the six centres implied, and the
eighteen entries in `TURNS` are the whole vocabulary. A middle slice turns
four centres, which that representation has nowhere to put. Nor is it a
permutation somebody could simply add: a slice is a four cycle of edges and
nothing of corners, where every face turn is a four cycle of both, so a slice
leaves the cube at a parity no sequence of face turns can reach and the
solver would be asked to solve something unsolvable.

So in the middle column the two keys are left alone and walk you, exactly as
they did before today, and the plate does not mention them there. That is the
rule spec 0106 went in under, one toy along: a key is named where it works.

Giving the cube real slice moves means modelling the centres, and through
them the solver's input. It is a change to the toy and not to the sight, and
it is not in this spec.

## Acceptance criteria

- Pushing up on the right hand column of the front face puts the front colour on top, and pushing down brings the up colour round to the front. — `kociemba::tests::pushing_up_on_a_column_rolls_it_away_from_you`
- The middle column offers no layer, and every face turn moves four corners and four edges, which is the parity a slice would break. — `kociemba::tests::the_middle_layer_has_no_face_to_turn`
- Up rolls a layer that is not the face the sight is on, and in the middle of a face it is still the key that walks you. — `tests::up_rolls_the_column_and_the_middle_of_a_face_does_not`

**The direction is checked by where a colour went, not by the move it
named.** The quarters come out of a cross product and a sign, so asserting
that the move is `R` asserts the formula against itself. Flipping that sign
puts the back face's colour on top instead of the front's, which is what the
control reports.

Three more controls. Using the sighted face for the up arrow instead of the
rolled one gives "up did not roll the back layer". Dropping the middle layer
guard gives "the middle column of a face offered a layer to roll" in one test
and "the middle of a face offered a roll" in the other, the second of which
is the walking half.

**One naming trap, found while writing the control.** The match arm bound the
rolled face as `face`, which shadows the face the sight is on, in a block
where both are in scope and either compiles. It is `rolled` now, with a
comment saying why.

### Verified by hand

Not by eye yet. Jake was in the game when he reported it.

## Out of scope

Real slice moves, for the reason above. And the sight itself, which still
names one face: what changed is what the keys do with it.
