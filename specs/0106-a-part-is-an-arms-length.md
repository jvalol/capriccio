# 0106 A part is an arm's length

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake, playing: the game feels hard to control, it overlaps with movement in
the room.

## Behavior

**Six parts went in at a bench's reach and each one takes the arrows.** Specs
0098 to 0105 gave the nook's six toys a part you can point at, and every one
of them was sighted with the same `BENCH_REACH` the bench uses, 1.8 metres.
Each also binds the arrows to its toy, because that is what pointing at a
thing means here.

Two of those toys never wanted the arrows at all. Nothing in `wanted_by`
matches the cradle or the puzzle, so until today you could walk the row
holding an arrow down and keep turning. Now a ball or a facelet takes the key
off you from nearly two metres away, in a room four metres across, and the
toy that reacts is somewhere behind your shoulder.

**The building had already written this fault down, one room along.**
`looking_at`'s own comment: the arrows work the toy the sight is on and turn
you when it is on nothing, so crossing the nook they stop turning you, for no
reason you can see. That was about the benches and it was answered by pulling
their reach in. The six parts reintroduced it, wider, by inheriting the
number the benches had settled on.

**So a part answers at an arm's length, `PART_REACH` of 1.0.** You read what
a thing is from a step back and you touch it when you are at it. A bench is
untouched: `BENCH_REACH` is still 1.8 and no sight that uses it was changed,
so every toy behaves exactly as it did before 0098, and the part is the
nearer half of the same gesture rather than a wider one.

It keeps the plates honest too. A part's plate promises what the arrows do,
and at 1.8 it promised it from places where pressing an arrow walked you
instead. A part is now only ever named when its keys actually work.

## Acceptance criteria

- The globe answers at 0.8 and not at 1.2. — `tests::a_part_answers_at_arms_length_and_no_further`
- A part's reach is shorter than its bench's. — `const _: () = assert!(PART_REACH < BENCH_REACH)` in `room`

Two controls. Widening `globe_i_see`'s own filter to `BENCH_REACH` gives "a
step back from the globe the sight still has it". Setting `PART_REACH` to 1.8
fails the build: `evaluation panicked: assertion failed: PART_REACH <
BENCH_REACH`.

**The comparison is a const assertion because a test could not hold it.** It
was a third `assert!` in the test above first, and the test's aim distances
were written as `PART_REACH` either side, so moving the constant moved the
camera with it and both ray assertions held at any value; the comparison was
the only thing in there that noticed. Then clippy refused it, correctly: the
comparison is constant, so a run-time assertion is one that cannot fail.
Saying it to the compiler is the stronger version of what the test was
reaching for, since the only way for it to go wrong is somebody editing one
of the two numbers.

**And its first draft asserted a handover that the room cannot stage.** A
step back from the globe along its own axis stands you inside a cabinet in
the hall, and a ray level with the ball passes over the bench top into that
cabinet rather than into the bench. The probe said `Cabinet(9)` at 0.0 away,
which is a camera inside the furniture. The claim was dropped rather than
staged, since what it was reaching for is the comparison of the two numbers.

### Verified by hand

Not by eye yet. Jake was in the game when he reported it.

## Out of scope

The benches themselves, which keep 1.8. If an arm's length turns out to be
right for a part it is worth asking whether a bench wants it too, but that
is a change to how the nook has worked since it was built and not a fix to
what went in today.
