# 0118 The stone says what is left

**Status:** implemented
**Date:** 2026-11-18

## Goal

Jake, 2026-11-18, after reading the trail back: there is no way for a user to
know how to complete the bridge, hints would be good. Asked which of two he
wanted, he took both.

Spec 0117 put the unfinished bridge in front of you from the first minute, and
that is the whole of what the building currently tells you: there is a thing,
it is not done. What it has never said is what would finish it. Six questions
are asked in six rooms and nothing anywhere connects any of them to the gap in
the sky.

## What this must not become

Spec 0036 rules out most of what "hint" usually means, and those rules stand:

- **No panel of ticks and no "4 of 7".** A count is a checklist with the
  numerals filed off.
- **No signposting.** A clue that says GO LEFT is a sign.
- **Nobody who never finds the trail may be able to tell it is there.**

The model 0036 gives instead is spec 0020's worn lane in the carpet: a thing
that happened rather than a thing somebody put up. Both halves of this are
built to that.

## Behavior

### The stone stands from the beginning

`bridge::runestone` returns the stone only once `whole(trail)`, so today the
one object out there that says what any of this is arrives after you no longer
need telling. It stands from the first minute instead, like the deck.

This is the argument `shades` already made for the clearing in the trees, one
object along: a stone that appears the moment somebody finishes the building is
a stone popping into existence, which is worse to watch than a stone that was
always there. Spec 0117 made the same call about the deck's foot.

The loose rubble in its footing follows it, for the same reason.

### Its face says what is still held

The carved face carries the rune ribbon and the name. Under the name it now
carries one more line: what the building is still holding, named one at a time.

- **One, never a list.** The first station in `Asked::every` order that has not
  been given. When every one has been given it says nothing more, and the stone
  is the stone it was.
- **In the voice of the thing undone, never an instruction.** "The water has
  not had you" is a fact about the building. "Swim in the baths" is a sign.
  Each line is drafted copy and carries `[COPY - Jake]` until Jake has read it.
- **Recut rather than hung.** `Renderer::write_texture` replaces the face's
  contents in place, so this is one carved face as it was, and not the second
  plate that spec 0054 spent a change getting rid of. It is recut when the
  trail changes and at no other time, which over a whole run is six times.

Nobody who never walks out past the garden's west wall ever sees it. The rule
about not being able to tell the trail is there survives, because the stone is
already out there and already saying this is Bifröst.

### Each span carries the room that paid for it

`Trail` records what was given **in the order it was given**, and nothing has
ever read that order. The bridge's kerbs do now: the piece of kerb over span
`k` takes the colour of the room that gave the `k`th answer.

- **The kerbs and not the deck.** Bifröst is a rainbow. A span of it in
  mahogany is not a rainbow bridge, and the deck is the one surface here that
  cannot carry this. The kerb is already a row of separate pieces down either
  side, already drawn in one flat colour, and is the thing you look along for
  the whole climb.
- **The first span is nobody's.** It is there before anything is answered, per
  spec 0117, so it keeps the kerb's own colour.
- **What it teaches.** Answer the cellar and a span of cellar-coloured kerb
  appears. You are not told the rule; you watch it happen twice and you have
  it. From then on the bridge is a record of your own walk in the order you
  walked it, which is the worn lane in the carpet at a hundred metres.

Between them the two halves answer the question without a checklist: the gap
says how many are left and the stone says which one is next.

### And a joint between one span and the next

2026-11-28. Jake, looking up the finished bridge: switching from orange to
blue and back as you go up seems arbitrary. Asked what the bands were, he
had it straight away: countable because they represent each step of the
trail. They do, five answers and the dark one at the foot that was there
before any of them, and a count wants edges.

Walked mid-trail the bands have edges for free, since the end of the colour
is the end of the bridge. Finished, five mid-tones abut and there is nothing
to count. So `JOINT` is a metre of the bar's own colour at every boundary,
which is the colour the span at the foot wears, so the joints and that first
span are one thing and the bar reads as a bar with colours laid along it.

**And the gradient this spec said it did not have.** "Whole slots, never a
gradient" was the requirement and one sweep with the slot written into each
vertex is not that: the quad between the last vertex of one span and the
first of the next has its `u` running from one slot's middle to the other's,
which walks the strip through whatever lies between. Two metres of fade at
every boundary, five times up the climb. The kerb is swept a run at a time
now, a run being a span or a joint, and nothing is shared across a boundary,
so the change is where the change is.

## Acceptance criteria

- The stone and its footing are there before anything has been answered. —
  `bridge::tests::the_stone_stands_before_the_first_answer`
- The face names the first station not given, and names nothing once every one
  has been. — `bridge::tests::the_stone_names_what_is_still_held`
- Every station has a line of its own, checked inside the test above rather
  than in `says`, a line being nothing without the station it belongs to.
- It names what is left and not what was last done, which is the one a
  `given().last()` would pass. —
  `bridge::tests::it_names_what_is_left_and_not_what_was_last_done`
- A kerb piece takes the colour of the room that gave that span, in the order
  given. — `bridge::tests::a_span_wears_the_room_that_paid_for_it`
- The first span wears nobody's colour. — `bridge::tests::the_first_span_is_nobodys`
- The spans are counted: no triangle of the kerb runs from one slot to
  another, and a joint of the bar's own colour stands at every boundary. —
  `bridge::tests::the_spans_are_counted_and_not_blended`
- Two runs that answer the same rooms in different orders paint the kerbs
  differently. — `bridge::tests::the_kerbs_are_a_record_of_your_own_walk`
- There is a slot in the strip for every span the deck grows in. —
  `bridge::tests::there_is_a_slot_for_every_span`
- Nothing is asked for before the room it is in can be reached. —
  `trail::tests::nothing_is_asked_for_before_its_room_can_be_reached`
- The cube counts only when it went round and came back by hand. —
  `kociemba::tests::the_cube_has_to_go_round_and_come_back`

### Verified by hand

- The line is readable from where you stand at the foot. — run the arcade,
  walk out to the stone, read it.
- The kerb colours read as rooms rather than as a progress bar from halfway up
  the climb. — run the arcade with a part-given trail and look along the deck.
  **At walking height and not from the air**: the colours were measured
  arriving correctly down the whole ribbon while being invisible from a
  hundred metres up, a kerb at that distance being two pixels. They were also
  lifted to one brightness to get there, the rooms' own materials running from
  (10, 10, 25) to (26, 31, 42) out of 255 when taken straight.

## Out of scope

**Saying which room a colour belongs to.** Nothing names the mapping. You learn
it by doing a room and watching a span arrive, or you never learn it and the
bridge is a bridge with coloured kerbs.

**Heimdall.** He still asks nothing, per spec 0063. Whether the end of the
trail should want something from you is a separate question and a better one.

**The order the stations are asked in**, which was out of scope for about two
minutes. The stone reads `Asked::every` order and nobody had designed that
order, so the first thing it would ever have named was riding to the sun,
which happens inside the room `GotBehindTheWall` is about. It is a curve now
and the two orderings the building fixes are held by
`trail::tests::nothing_is_asked_for_before_its_room_can_be_reached`.

**Making the tesseract intelligible.** It came out of the requirements on the
same day, Jake not being able to write a line for it he understood himself.
The room is spec 0021's and the work on it is that spec's. What this one
records is that three lines were tried and the one that would have worked is
about the shape changing rather than about the other five.
