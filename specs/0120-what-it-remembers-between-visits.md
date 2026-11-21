# 0120 What it remembers between visits

**Status:** implemented
**Date:** 2026-11-20

## Goal

The trail survives quitting. You come back and the bridge is as far along as
you left it, and the stone still says what you did.

## Why

Jake, 2026-11-20, asked whether the building should remember the trail between
runs: yes, and as its own spec.

Spec 0051 built the record and called itself plumbing. Spec 0117 gave it a
bridge that grows, 0118 a stone that says what is left, and 0119 a note when a
span goes up and a ledger of what was answered. All four are about a run, and
the arcade has no save of any kind: quit and the bridge is a stub again and the
stone is back to its first line.

That is the hole under everything just built. The trail is five questions in
five rooms spread over eleven, and nobody walks all of it in one sitting.
Answer three, stop for the night, and the building has forgotten you.

## What the copy already decided

**The ledger says "somebody", not "you".** That register was settled on
2026-11-18 for a different reason: three lines were thrown out for sounding
predatory, and what worked was a fact about the building with the player
nowhere in the sentence.

It is also, exactly, the voice of a thing that outlasts a visit. A stone
recording that somebody has been in the water reads as a record left in the
world. A stone saying "you have been in the water" would be a save file with a
face on it, and it would be a lie the first time anybody else plays on this
machine.

So nothing in the copy has to change, and the one thing that would have made
this feel wrong was ruled out a fortnight before it was needed.

## Behavior

### What is kept

**The trail, in the order it was given, and nothing else.** Not where you
stood, not the hour, not the cube, not the hotel, not which doors are open.

The trail is the only state this building holds that is about the player rather
than about the building. Everything else is a toy, and a toy you come back to
is a toy as it was built: that is what a room in this arcade is. Spec 0051
named the order as the only thing in the record that could ever mean anything,
and the kerbs and the ledger both read it, so the order is kept and not a set
of flags.

### Where

A file, in the platform's own place for a program's data, holding one station
per line by name.

- **Readable, and a line at a time.** A file you can open and read is a
  building that says plainly what it remembers, which is the only register this
  one has. It also makes a part-answered trail something Jake can set up by
  hand, which is a better `--done` than `--done`.
- **An unknown name is skipped, not fatal.** `trail::Asked` has changed three
  times in a fortnight and will change again. A file written by an older
  arcade has to open.
- **A missing file is somebody who has not been here. So is an unreadable
  one.** Nothing about this may ever stop the building opening.

### When it is written

**When the trail changes, which is five times in a run.** Not on quitting: the
way out of this building is Escape or a window closed, and a save that needs a
clean exit is a save that is not there the one time it mattered.

### What never touches the disk

**`Arcade::new`.** This is the whole of the risk in the spec and it is worth
more than the feature. A thousand tests build an `Arcade`, two of them compare
pictures of the field against references, and `new` reading a save would make
every one of them depend on what Jake last played. The gate would pass or fail
by accident.

So the building is handed its memory rather than fetching it:

- `Arcade` holds where it remembers, and that is `None` until something sets
  it. Nothing in `new` sets it, and nothing writes while it is `None`.
- `main` is what sets it, next to where `--done` and `--pose` are already read.
  No test reaches `main`.
- Which also gives the round trip a test: hand an arcade a path of its own and
  read it back.

### Starting again

`--forget` opens the building with nothing answered, and the run then saves
over what was there as it goes.

- **Ignored rather than deleted.** A flag that deletes is a flag that deletes
  when you fat-finger it.
- **And a run given `--done` writes nothing at all.** It is a diagnostic, like
  `--pose` and `--fly`, and a diagnostic that overwrites a real save is a trap.

### What this costs, and it is worth saying

**The building only ever answers you back once.** Spec 0119's note sounds five
times in a life rather than five times a run, and the deck only winds in once.
That is the price of the thing being remembered at all, and it is the right
trade: a promise you can finish beats a receipt you can hear again.

It does mean `--forget` is the way back to a building that still has something
to tell you, and whether starting again should be a thing inside the building
rather than a flag is a real question. It is not this spec's.

## Acceptance criteria

- A trail written out and read back is the same trail, in the same order, and
  not in `every` order. —
  `trail::tests::what_is_written_comes_back_in_the_order_it_went`
- A name nothing answers to is skipped and the rest of the file is kept. —
  `trail::tests::a_file_from_an_older_arcade_still_opens`
- Nothing written in the file can break it: blank lines, space, a name twice,
  JSON, and bytes that are not text. —
  `trail::tests::nothing_written_in_it_can_break_it`
- Every station can be written down and read back, and no two share a name. —
  `trail::tests::every_station_has_a_name_of_its_own`
- What goes to the file comes back off it. —
  `kept::tests::what_goes_to_the_file_comes_back_off_it`
- A missing file and an unreadable one are both somebody who has not been
  here. — `kept::tests::nothing_to_read_is_a_first_visit`
- It is kept in a folder of the arcade's own and not in somebody's home. —
  `kept::tests::it_remembers_somewhere_of_its_own`
- `Arcade::new` remembers nowhere, so nothing a test builds can read or write
  a save. — `tests::a_building_made_in_a_test_remembers_nowhere`
- A room answering writes the file, a frame where nothing was answered does
  not, and a building that remembers nowhere writes nothing at all. —
  `tests::the_file_is_written_when_a_room_answers`
- A trail read off the file brings the deck and the notes with it, so opening
  the building does not sound the whole run at you. —
  `tests::a_walked_trail_brings_the_deck_with_it`

  **`--done` has no test of its own.** What it does is leave `remembers`
  unset, and that it writes nothing once unset is the control in the test
  above. Whether the flag leaves it unset is `main`'s, which no test reaches
  without an argv to hand it, and inventing one for this is more machinery
  than the thing. It is checked by hand instead.

### Verified by hand

- A trail put in the file by hand opens a building that is that far along. —
  **done 2026-11-20**: `opened-the-case` and `waded-in-the-pool` written to
  the file, and the arcade opened with three sixths of a deck and a ledger
  reading both entries and then "Nobody has played with the cube and then
  solved it". `--forget` on the same file opened a stub and one line, and
  neither run touched what was in it, nothing having been answered in either.
- Answer something, quit, open it again, and the bridge is where you left it
  and the stone still says it. — play it twice.
- A run given `--done` leaves the file as it was. — play it, then look at the
  file's time.
- The file reads as a list of what you did. — open it.

## Out of scope

**Anything else the building could remember.** Named above and refused: the
rooms are toys and a toy resets.

**The browser.** blitzkit builds for it and `std::fs` is not there. The
building forgets there, which is what it does everywhere today.

**Putting this in the engine.** Where a game keeps its state is an engine-shaped
question and the platform arithmetic would be written once instead of once per
game. It is here because capriccio is the only one of fifteen that has any
state worth keeping, and a shared answer built for one caller is a guess about
the other fourteen. **The trigger is the second game that wants it**, and then
this moves rather than being copied.

**A save menu, slots, or a continue.** There is one building and one record of
what has been done in it.
