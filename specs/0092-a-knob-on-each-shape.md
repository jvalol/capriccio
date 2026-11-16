# 0092 A knob on each shape

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake: menger and hilbert and sierpinski could have the size of the holes
changed, and the klein bottle and teapot could be switched between solid and
transparent.

## Behavior

Every shape in the row gets one number of its own, and shift with up or down
winds it. Spec 0091 had just given the plain arrows to turning the shape in
your hands, so the pair with shift on is what was left, and it is already
what this building means by that gesture: at the Mandelbrot panel, spec 0039
has shift and up or down change the scale while the plain arrows move about
the plane.

**One number and not one kind of knob each.** Three of the six are built at a
depth or a thickness, so for those it is that number. The teapot and the
Klein bottle are one mesh each however you ask for them, and the only thing
left to change is whether you can see through them, so for those it is nought
or one. The tesseract has neither: it is a door.

**What the three fractals' knob actually is, said plainly.** Only the hilbert
tube has a hole size: its radius, against a curve on a lattice of 0.125, so
the holes close as it approaches 0.06. The Sierpinski tetrahedron and the
Menger sponge take a depth and nothing else, and a depth is not a hole size,
it is how many times the hole is cut again. Jake chose that over adding a
hole ratio to the engine's menger, which would have made one of the three
different from the other two.

The runs stop where the shape stops being worth looking at rather than where
the arithmetic stops:

- Sierpinski 1 to 5, resting at 4. Six was tried: sixteen thousand tetrahedra over half a metre is a few pixels each, and it reads as grey speckle on a sheet rather than as a solid.
- Menger 1 to 3, resting at 2. Three is 36 thousand triangles and a millisecond and a half to build. Four is 673 thousand and 29 milliseconds, which is a frame and three quarters spent on one keypress.
- Hilbert 1 to 4, resting at 2, a step of 0.015.

**A step a press.** A depth is a whole number and three of them is a whole
run, so there is nothing between two positions to ease through, and a repeat
held down would walk the sponge to its most expensive depth before you let
go. `input.repeat` is what says a press from a repeat, the way the list in
spec 0018 already uses it.

**See-through is an alpha and nothing else.** The engine puts any instance
under a full alpha into the translucent runs, which draw the far side before
the near one. 0.38, further over than the sauna's glass at 0.26, because what
is worth seeing here is the shape's own far side rather than the room behind
it. On the Klein bottle it shows the handle going in through the wall
standing still, which `Spin`'s own doc calls the one thing worth seeing and
which until now needed turning it to find.

**A knob rebuilds the mesh, so the display is measured again.** `all_of_them`
sized this row by measuring each mesh, once, at the start. A hilbert tube
wound thicker is a wider object than the one the row was laid out around, and
`ACROSS` is `APART` over the root of three exactly so that a turning shape
fails to reach its neighbour, so a scale left at what it was would draw it
over the sponge and a `half` left at what it was would sink it into its own
plinth. `Display::fitted` is that measurement, called by both.

**`meshes()` is six calls no longer.** It was the six shapes written out, and
a knob on three of them would have been a second list saying the same thing
differently. There is one `mesh_at`, and the row at rest is that function at
each shape's resting knob.

## Acceptance criteria

- Shift and an arrow wind the knob, do not tilt the shape, rebuild its mesh, measure it again, and stop at both ends of the run. — `tests::shift_and_an_arrow_wind_the_shapes_own_knob`
- A plain arrow still turns the shape rather than winding the knob, which is the control, in the same test.
- Every step of every run builds a shape with a surface. — `tests::every_step_of_every_knob_builds_something`

Three controls were put back and each failed on its own assertion: the
keypress not reaching the knob, the display not being measured again, and a
hilbert radius of nought.

That last one is why the surface test counts area. It counted triangles
first, and a tube of no radius keeps every ring and every index it had and
puts them all on the curve, so it has its full count, its full box and no
surface at all. The control passed and the test was worthless until it was
measured as area.

### Verified by hand

Driven with played logs. The key is spelled `shift`, not `LShift`: the
engine's parser knows a short list of names, prints `is not an event` on
stderr for the rest, and a run with stderr thrown away looks exactly like a
run where nothing happened.

```
printf '0.500 press Left\n1.000 down Return\n1.033 up Return\n1.500 down shift\n1.700 down Up\n1.733 up Up\n2.300 up shift\n' > /tmp/k.log
./target/release/capriccio --pose "3.345 0 -13.0 1.55 0.0 -5.7 120.0" \
  --play /tmp/k.log --picture-for 3 --picture-to /tmp/a.png
```

- The hilbert tube at 4 is fat enough to close its own holes, and still stands inside its plinth and clear of the sponge.
- The sponge at 3 has structure inside each of the eight small holes that the one at 2 does not.
- The teapot and the Klein bottle at 1 are glass, and the bottle's handle can be seen entering its own wall without turning it.

## Out of scope

Saying on the plate what the knob is at. It reads "Click or press enter to
turn it" and says nothing about shift, which is the same silence every
inspection key in this building keeps. It is copy, so it is Jake's.

A knob on the tesseract. It turns itself in the fourth, per spec 0090, and
that is the thing it has.
