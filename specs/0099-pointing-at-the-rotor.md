# 0099 Pointing at the rotor

**Status:** implemented
**Date:** 2026-11-16

## Goal

Spec 0098 let the sight find a ball on the cradle rather than the table it
stands on, and said the rest of the toys would come one at a time. Jake: the
gyro. This is the second.

## Behavior

**Its parts are the rotor and the spindle, and only one of them is a thing
you reach for.** The dial is not a part at all: it is a readout, `Spin 3 of
5`, with nothing in the room to point at. The spindle is an axle twelve
millimetres across that the arrows already lean on from the bench. What is
left is the rotor, which is the thing that spins and the thing a hand would
go to.

**Boxed on its sweep and not on the disc.** The rotor turns about an axle the
whole frame walks round, so which face it shows depends on where the
demonstration has got to. A box of its own diameter is the ball it sweeps and
is the same answer whichever way it is pointing.

**It answers as its own bench, found by name rather than copied.** Pointing
at the wheel instead of the table is the whole of what changed, so Return
spins it up or stops it exactly as the bench always has, the arrows lean on
the spindle, shift and up or down still turn the dial, and the plate says
what the bench's plate says. No new copy: pointing at the wheel does not
change what the wheel is.

**`leaning_on` is its own function now.** Which bench a held arrow leans on
was a `if let Some(Seen::Bench(n))` inline, and the rotor is a second thing
that answers to one bench, so the lookup has a name. The lean is taken off
the bench rather than written down as the nook's axes, which is a rule this
building already learned when the benches moved to the other wall and left
and right turned round.

## Acceptance criteria

- The sight finds the rotor, pressing it is the bench's own press, and a held arrow at it leans on that bench rather than turning you. — `tests::the_sight_finds_the_rotor_and_it_answers_as_its_bench`

Two controls were put back. Never asking about the rotor gives "pointing at
the wheel finds None". A rotor that does not look up its bench gives "an
arrow at the rotor leans on None rather than its own bench", which is the
quiet failure this shape has: the keys are taken and nothing happens with
them.

### Verified by hand

Not yet, beyond the test, which stands the camera at the wheel and points at
it more precisely than a pose would.

## Out of scope

The spindle as a thing of its own. The arrows already lean on it from
anywhere the gyroscope answers, and an axle twelve millimetres across is not
a thing anybody aims at.

The dial, which would want something drawn before it could be pointed at.

The remaining toys: the globe, the metronome's weight, the puzzle's cubies
and the wrecker's ball. Still one at a time.
