# 0119 The building answers back

**Status:** draft
**Date:** 2026-11-20

## Goal

When a room gives up its answer, the building makes a sound, wherever you are
standing. And the runestone says what has been answered as well as what is
still held, so the trail can be read rather than only guessed at.

## Why

Jake, 2026-11-20, having ridden out to the sun: there should be some
indication one of the pieces of the trail was reached, and an indication
somewhere of what has and hasn't yet been done.

**It had happened and he could not tell.** Put back at the camera his
screenshot was taken from, the deck reaches three sixths along rather than
one: getting behind the wall and riding to the sun had both been recorded and
both had put a span up. Nothing was broken. What the building has never had is
a way of saying so.

The reason is where you are when it happens. Every one of the five questions
is answered deep inside a room, and the only thing that changes is a hundred
metres away in the field, through at least one wall. By the time you walk out
to look at it you have no earlier bridge to compare it against, so a bridge
half built looks exactly like a bridge.

**And the one object that does speak to you said nothing had happened.** Spec
0118's stone names the first station not given, which after the longest thing
in the building was still the first line it says at minute zero. A hint about
what is next is not a receipt for what is done, and the stone was carrying only
the first.

## What this must not become

The rules from specs 0036, 0117 and 0118 stand and none of this needs them
bent:

- **No panel of ticks and no "2 of 5".** The sound carries no number you can
  read off and the stone names stations one sentence at a time, in the voice it
  already uses.
- **No signposting.** Nothing here says where to go.
- **Nobody who never finds the trail may be able to tell it is there.** The
  stone is still out past the garden's west wall and nobody is sent to it. The
  sound is a sound the building makes, in a building that already thunders.

**And it is not a notification.** Spec 0117 ruled that out in as many words,
about a span that popped rather than grew. The line it was drawing is between
the building doing something and the game telling you something, and a sound
can be either: a bell with a flash on the screen is the second, and something
low that rolls through the place and that you could miss is the first. This is
written to be the first, which is why it is a note and not a chime, why it
takes as long as the span takes, and why nothing is drawn with it.

## Behavior

### The span winds in, which spec 0117 said and the code never did

`bridge::reach` is a step function of the trail and [`main.rs`] rebuilds the
deck, soffit, bank and kerbs in the single frame the count changes. 0117's own
words are that a span that popped would be a notification and that it should
take a few seconds, the way spec 0019's lanterns wind up, and `stations`' doc
already reasons about rounding "while a span winds in". That winding does not
exist.

- `Arcade` keeps the reach it is drawing and walks it toward `reach(&trail)`
  over `bridge::WINDS`, which is 6 seconds, eased the way everything else here
  is wound rather than switched.
- The meshes are rebuilt each frame that it moves and not otherwise, which is
  six seconds of rebuilds five times in a run.
- Nothing else reads the walked value: what holds you up and what carries you
  read it too, so you cannot walk out onto a span that is still arriving.

### A note, wherever you are

- One low struck note with a long tail, made in `noise` beside the building's
  other sounds, played with `sound.play` the way thunder is. Mixed and not
  queued: the engine queues `play_at` and a room's voice ahead of it would land
  the note after the span had finished arriving.
- **Heard in every room, which is what separates it from thunder.** Spec 0016's
  thunder plays only under the sky on purpose. This one is about the building
  rather than about the weather, and it is heard indoors because indoors is
  where you always are when it fires.
- **It climbs.** The note for the `n`th span given is a step up from the one
  before it, five steps over the run. Nothing says how many are left and
  nothing needs to: the fifth note is the top of what you have been hearing,
  and somebody who has heard three knows roughly where they are without a
  numeral anywhere. It is one multiply off `trail::given().len()`.
- It runs as long as the span takes, so the sound and the thing it is about are
  one event.
- **Not Gjallarhorn.** Spec 0055 built the horn as the one thing in the myth
  that is only ever about to be used, held through the whole of the world's
  history and sounded once at the end. A horn blown five times on the way up is
  that spec undone.

### The stone says both

Spec 0118 gave the face one line under the name, for the first station not
given. It carries what has been answered as well.

- **The same sentence with the negative taken out.** `says::still_held` is in
  one register, "nobody has done the thing yet", settled on 2026-11-18 after
  three lines were thrown out for sounding predatory. Its mirror is the same
  sentence about somebody who has, and that is the whole of the new copy: the
  cellar's "A book is a handle, and nobody has pulled it" becomes "A book was a
  handle, and somebody pulled it".
- **Which teaches the rule without stating it.** You read a line, you do the
  thing, you walk back out and the same line has flipped. Nothing anywhere says
  that is what the bridge is for.
- Given stations first, in the order they were given, which is the order the
  kerbs are already painted in and the only thing in `Trail` that means
  anything. Then the one still held, as now.
- When every one has been given the stone is the stone it was, per 0118, and
  the held line is gone while the given ones stay.
- Recut with `write_texture` when the trail changes and at no other time, which
  is what 0118 already does and is five times in a run.
- Every new line is drafted copy and carries `[COPY - Jake]` until he has read
  it.

## Acceptance criteria

- The deck's drawn reach walks toward the trail's rather than jumping to it,
  and arrives. — `bridge::tests::a_span_takes_time_to_arrive`
- Nothing holds you up past where the deck is drawn while a span is arriving. —
  `bridge::tests::you_cannot_walk_onto_a_span_that_is_still_coming`
- A station given makes a note, and the same station given twice makes one. —
  `main::tests::a_room_answering_sounds_once`
- The note for a later span is higher than the one before it. —
  `noise::tests::each_span_is_a_step_up`
- The face names every station given, in the order given, and the first one not
  given after them. — `bridge::tests::the_stone_says_what_was_done_and_what_is_left`
- Every station has a line for having been given as well as for being held. —
  checked inside the test above, a station being half-written otherwise.
- With everything given the face carries five lines and no held line. —
  `bridge::tests::a_finished_trail_leaves_nothing_held`

### Verified by hand

- The note is a thing the building did and not a thing the game said. — answer
  a question from inside the room it is in, with the window not looking at the
  field, and listen.
- It is audible in the cellar, which is the deepest room and behind two doors.
  — open the case and listen.
- The span is watchable from the garden's west opening while it winds in. —
  stand there and answer something.
- The face is readable with five lines on it at the height the one line is read
  from. — walk out to the stone with the trail finished.

## Out of scope

**Remembering any of it between runs.** The arcade saves nothing and a note you
hear once is spent the moment you quit. That is its own spec and is the larger
question of the two.

**Saying which room a colour belongs to**, which 0118 left out and this does not
put back.

**Anything under the sight.** The plate stays what spec 0109 made it.
