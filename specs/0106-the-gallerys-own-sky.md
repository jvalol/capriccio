# 0106 The gallery's own sky

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake, standing still in the gallery: can you see the sweeping light across the
floor and into the wall, that looks like a bug. And, when it was found: a
windowless room should not be affected by the outdoor fill. If the light
cannot reach a room, it shouldn't make it to that room.

## Behavior

**A whole day was crossing the floor of a room with no window.**
`garden::DAY` is twenty seconds, which its own comment calls a working
setting and says to put a longer one back when the building is being played.
Jake wants it kept. The sun was already shut out of a roofed room, three
lines above where the ambient was handed to every room alike, so the fill
swung from `(0.15, 0.18, 0.29)` at noon to `(0.03, 0.04, 0.09)` at midnight
every twenty seconds, indoors and out. Nine seconds of standing still moved
32 per cent of the pixels in the frame; pinning the hour moved 1 per cent.

**The root was one room missing from a list.** `noise::Room` had the hall,
the nook, the stair, the cellar, the baths, the garden, the field and the
space behind the wall, and no gallery, so the room at the end of the aisle
answered as the hall. The hall has a sky: its door looks into the garden.
This is spec 0011's fault one room along, and that spec's own words for it
were "It was also given the hall's sky."

So the gallery is a room. Everything about it is the hall's deliberately, the
carpet underfoot and what it hears, and the only question it answers
differently is the sky.

**`garden::indoors` is `filled` at the hour the sky is level**, and not a
grey somebody picked. It is built from the same two colours, so it cannot
drift from the sky's palette, and it is the hour every indoor picture in the
suite was blessed under, so the rooms look like they already did. Twelve
references moved by about a tenth of their pixels and none of them changed
shape.

**Wound and not switched**, because a doorway is not a shutter. The sun's
gate above can switch: a roofed room has no sun either way and nothing moves
when it flips. The fill is on every surface, so a step through a door would
be a flash. Six tenths of a second.

**And the first frame takes the answer whole.** A wind has to start
somewhere and nought was the wrong place: every picture in the suite renders
inside six tenths of a second, so a garden starting at nought was a garden
lit like a cellar. Eleven pictures failed that way before this was added,
which is how it was found.

## Acceptance criteria

- The gallery is its own room and has no sky, and the hall still has both. — `noise::tests::the_gallery_is_its_own_room_and_has_no_sky`

### Verified by hand

At Jake's own position, nine seconds apart, before and after:

```
./target/release/capriccio --pose "1.61 0.00 -12.76 1.55 59.1 -45.8 2288.7" \
  --picture-for <t> --picture-to /tmp/a.png
```

- The gallery goes from 32.3 per cent of pixels moving to 1.0, and what is left is the exhibits turning.
- The garden holds at 68.7 per cent, unchanged, because it is outdoors.
- The hall holds at 65.7 per cent, unchanged, because its door looks into the garden.

## Out of scope

The length of a day, which Jake kept at twenty.

Whether the hall should take the whole sky through one door, which is the
next room along the same question and nobody has stood in it and complained.
