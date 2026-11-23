# 0093 The row catches up with the example

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake, with a screen recording of the gallery: rotating the object myself they
appear to bounce on the plinths, they used to rotate smoothly with the
centroid staying in the same place and no collisions with the plinth; there
used to be accurate shadows on the plinths; the plaques don't accurately
reflect what keys will do what; menger used to have a deeper depth possible.

## Behavior

**Four complaints and one source.** The engine's own `menger` example does
all four: it turns the shape about its middle with the floor well below it,
it has a floor for the shadow to land on and its own comment calls that the
point of the example, its readout reads `depth 3 of 4   8000 cubes   up and
down to change`, and `MAX_DEPTH` is 4. The gallery had none of it. Nothing
was taken away: the row has been at depth 2 since the day it went in, and the
only casting lamp in this building is the middle fire in Valhalla.

**The bounce.** Spec 0005 lifted each shape by how far its own corners
reached that frame, so that a tipped one would not sink a seventh of a unit
into its plinth. That is right standing still and wrong under a hand: the
height depends on the facing, so a shape being turned rises and falls twice a
revolution. Nobody saw it until spec 0091 gave the arrows to turning, which
is the first time a shape could be tipped far and fast without a mouse.

The lift is the sweep now: the half diagonal of the shape's own box, which is
the radius of the ball it sweeps and so the one number that holds for every
orientation at once. The middle stays where it is however the shape is
turned, and the lowest corner reaches the plinth's top at exactly the one
orientation that stands the diagonal upright. Every other way up it clears.

That is the tightest lift that never collides, and it is still air: a shape
at rest now stands about two tenths of a metre over its plinth, which is what
a shape that can be turned to any angle costs. Which is the other half of why
the shadow matters.

**The shadows.** This room's own comment already said a shape is lit from in
front and above so that it throws its shadow back onto the wall. It threw
nothing, because no lamp in the gallery asked to cast and the engine's lamps
do not by default. The two lamps over the ends of the row ask now.

Two and not three, and named rather than left to the engine. It carries two
shadowing lamps and takes the first two it is given, so three asking is a
draw settled by push order, which is what Valhalla's fires already say in
their own comment. The middle lamp is over the panel, which has nothing
standing in front of it.

They are pushed before the ranked lamps, so what the engine drops when the
building outgrows its cap of sixty four is never the lamp a room is built
around, and they take the same frustum cull as the rest, so a room you cannot
see costs no shadow map. Measured headless in the gallery looking down the
whole row: twenty nine seconds of play goes from 7.27 seconds to 7.65, which
is five per cent.

What it buys is not only the plinths. The sponge's holes were as bright
inside as its faces and now they are not, and the hilbert tube's own lacework
lands on the plinth under it.

**The depth.** `MAX_DEPTH` is the engine's number and the gallery reads it
now rather than carrying one of its own. Four is 673 thousand triangles and
29 milliseconds, paid once on the keypress that asks for it. Spec 0092 had
capped it at three for that cost, which was a cost worth naming and not a
cost worth refusing.

Sierpinski keeps the room's cap of five rather than the library's nine, and
that is a different kind of number: at six, sixteen thousand tetrahedra over
half a metre is a few pixels apiece and the shape reads as grey speckle
rather than as a solid. The engine's cap is about what the arithmetic will
build and the room's is about what can be seen.

**The plates.** They read "Click or press enter to turn it" whether or not
the shape was in your hands, and said nothing about the arrows, the shift
pair, or where the knob stood. The one plate in this room that did say its
keys was the panel's, which is the shape the new ones take: what the keys do,
then the state.

- Nothing in hand: click or press enter to take hold.
- In hand: the arrows, the shift pair with the knob's own word and where it stands, and enter to let go.

The knob's own word, because a depth and a thickness are not the same thing
and a plate saying depth over a tube that has none is the fault this is
fixing.

**The copy was drafted and not his**, so every line of it carried
`[COPY - Jake]` and the marker audit held the gate. The half sentence
`knob_says` returns had no marker of its own: it is never drawn alone, and
two markers in one line of copy is the mistake of 2026-11-09.

He took them later the same day and wrote the lines himself. The draft said
"Arrows turn it. Shift and up or down: {}. Enter lets go"; the plate says
"Use arrow keys to rotate it. Hold Shift and press up or down: {}. Press
Enter to let go". One word of that is not his: he wrote hold against the
arrows, and holding an arrow does nothing after the first step, because the
key repeat is swallowed on purpose so that one press is one step of depth.
Hold moved to Shift, which is the key you do hold.

**And the arm for a shape with no knob went with them.** It could not fire:
every shape you can take hold of has one, and the one that does not is the
tesseract, which Return takes you inside of rather than holding. What stands
there now is `unwrap_or_default`, so the impossible case is a gap in the line
rather than a panic.

## Acceptance criteria

- No turn moves a shape, and no turn puts it through its plinth, and it does come down to touch. — `display::tests::no_turn_moves_a_shape_or_puts_it_through_its_plinth`

That test replaces spec 0004's `a_turned_one_still_rests_on_its_plinth` and
spec 0002's `each_one_sits_on_its_plinth`, which between them said the old
rule. Both halves of the new one were put back as controls: lifting by the
rest reach puts the teapot 3.6 centimetres into its plinth when it is tipped
a quarter turn, and lifting by twice the sweep leaves it hanging 35
centimetres clear and never touching.

### Verified by hand

```
./target/release/xanadu --pose "2.395 0 -12.6 1.55 0.0 -12.0 120.0" \
  --picture-for 2 --picture-to /tmp/a.png
```

- The sponge's plinth has the sponge's shadow on its top, and the tube's plinth has the tube's.
- The sponge's own holes are darker inside than its faces, which they were not.
- The plate under a held shape reads its keys and its knob, and wraps to two lines.

## Out of scope

A shadow anywhere else in the building. Every other room is lit by lamps
that cast nothing, and whether that is right is a question per room rather
than one answer. Only the gallery was asked about.

Caching a mesh per depth, which the engine's example does. It holds one
`MeshId` per depth and never builds the same one twice; the gallery writes
over one mesh, so it would need a slot per step of every knob. 29
milliseconds once, on a keypress, is not yet worth that.
