//! The bridge at the end of the trail: a rainbow out of the field, going up.
//! Spec 0054.
//!
//! **Nothing here exists until the building has been done.** Every room that
//! asks you something has to have given up its answer, which is what
//! `trail::Asked` lists and what [`shows`] reads. Until then this draws
//! nothing and stands nothing up, and spec 0036's rule holds: somebody who
//! never finds the trail cannot tell it is there.
//!
//! **A crossing and not a room.** Spec 0036 warns that inventing a fourth
//! room as the prize is how you get a room nobody wanted, and the warning is
//! why there is nothing at the top of this. What the climb gives you is
//! height and the only sight of the building from outside it: eleven rooms
//! walked and nobody has ever seen the arcade whole.

use blitzkit::collision::Aabb;
use glam::{vec3, Vec3, Vec4};

use crate::trail::{Asked, Trail};

/// How far west of the garden's ground the foot of it stands.
///
/// Far enough to see the whole arc from the gap you come out of, near
/// enough to walk to without the building going behind you.
pub const OUT: f32 = 18.0;

/// How far west it runs from there, and how high it climbs over that run.
///
/// **A hundred and sixty metres is the field's own reach**, so a hundred of
/// run with eighteen of standing-off leaves the top well inside the ground
/// rather than hanging over the edge of it. Twenty-six up is six storeys of
/// the hall.
///
/// The two are a pair and not two numbers: the rise over the run is what
/// decides whether this can be walked at all, and at these it is steepest
/// in the middle at a shade under half, which is twenty-five degrees.
pub const RUN: f32 = 100.0;
pub const RISE: f32 = 26.0;

/// How wide the deck is.
///
/// **Six metres, which is deliberately more than it needs.** A body is 0.9
/// across and every walkway in this building has been built too narrow
/// first: the nook went 3.0 to 6.4, the baths 9 to 11 and then wide enough
/// for walkways. A bridge you edge along is not a prize.
pub const WIDE: f32 = 6.0;

/// How many colours lie along it, and how thick the deck is.
pub const BANDS: usize = 7;
pub const DEEP: f32 = 0.35;

/// How many pieces the drawn deck is cut into.
///
/// Only the drawing. What holds you up is [`treads`], which is a different
/// and much finer list for a different reason, and both come out of
/// [`high`] so there is one curve rather than two.
pub const PIECES: usize = 48;

/// How long one tread of the collider is.
///
/// **This is what makes it walkable or not.** Colliders here are boxes that
/// cannot tilt, so a slope is a staircase to the thing walking up it, and a
/// riser taller than `walk::STEP` is a wall. At the steepest part of the
/// arc, a tread of 0.6 rises 0.28, and the walk steps 0.42.
///
/// **Walkable is not the same as smooth, which is what 0.6 missed.**
/// Jake: the walk up is jittery. Every riser at the steep part was most
/// of a foot, taken at seven metres a second, which is a dozen
/// twenty-eight centimetre hops a second and reads exactly as a jitter.
/// Clearing `walk::STEP` only says the stairs are climbable.
///
/// At 0.12 the riser at the steepest point is under six centimetres,
/// which is a kerb rather than a stair, and the staircase is close
/// enough to the ramp it is standing in for. It costs eight hundred
/// boxes where there were a hundred and sixty seven, which is nothing:
/// they are axis aligned and the walk tests them with six comparisons
/// each.
pub const TREAD: f32 = 0.12;

/// Where the last of the climb is done, as a share of the run.
///
/// **It ends level.** Arriving somewhere is standing on it, and a deck
/// still climbing at its last piece is a thing you walk off the end of.
pub const LEVELS_AT: f32 = 0.86;

/// The seven, in order from the foot.
///
/// Past one on purpose. Colour is an unclamped multiplier in this engine
/// and anything over one glows with no light on it, which is what a rainbow
/// is: light rather than paint. Neon in the cellar is 3.2 and this is
/// quieter than that, because the field has a sky behind it and a sun on it.
pub const COLOURS: [Vec3; BANDS] = [
    vec3(2.0, 0.42, 0.42),
    vec3(2.0, 1.02, 0.36),
    vec3(1.90, 1.74, 0.40),
    vec3(0.46, 1.86, 0.62),
    vec3(0.42, 1.10, 2.0),
    vec3(0.52, 0.56, 1.95),
    vec3(1.20, 0.52, 1.90),
];

/// What every piece of the deck is multiplied by, and so what the colours
/// in [`rainbow`] are divided by to get there.
///
/// **Because a picture cannot hold a colour past one and this needs to.**
/// Colour is an unclamped multiplier in this engine and anything over one
/// glows with no light on it, which is what a rainbow is. A texture is
/// eight bits a channel and stops at one. So the picture carries the hue
/// and this carries the glow, and `COLOURS` is still the one place the
/// seven are written.
pub const GLOW: f32 = 2.0;

/// How many texels across the rainbow's picture is drawn.
///
/// **Enough that the blend is a blend.** Seven flat bands is what this was
/// and Jake said the colours should run together the way a real one's do.
/// At 192 across six metres a texel is three centimetres, which is finer
/// than the eye resolves at the distance you stand from it, and the step
/// between two neighbouring texels is about a fortieth of the step between
/// two of the seven.
pub const RAINBOW_WIDE: u32 = 192;

/// The rainbow across the deck, as a picture that blends.
///
/// **The seven are stops and no longer bands.** Each sits at the middle of
/// the width it used to fill and the picture runs between them, so the
/// colours at those seven places are exactly what they always were and
/// everything between them is new. Outside the first and last stop it holds
/// the end colour rather than fading, because a rainbow's edges are where
/// it stops being a rainbow and not where it stops being bright.
pub fn rainbow() -> blitzkit::texture::TextureData {
    let mut pixels = vec![0u8; (RAINBOW_WIDE * 4) as usize];

    for at in 0..RAINBOW_WIDE {
        // where this texel falls among the stops, which sit at the middles
        // of the seven bands
        let across = (at as f32 + 0.5) / RAINBOW_WIDE as f32 * BANDS as f32 - 0.5;
        let below = across.floor().clamp(0.0, (BANDS - 1) as f32) as usize;
        let above = (below + 1).min(BANDS - 1);
        let share = (across - below as f32).clamp(0.0, 1.0);

        let colour = COLOURS[below] + (COLOURS[above] - COLOURS[below]) * share;
        let at = (at * 4) as usize;

        for (n, one) in [colour.x, colour.y, colour.z].iter().enumerate() {
            pixels[at + n] = ((one / GLOW).clamp(0.0, 1.0) * 255.0).round() as u8;
        }
        pixels[at + 3] = 255;
    }

    blitzkit::texture::TextureData::from_pixels(RAINBOW_WIDE, 1, pixels)
}

/// Whether the building has given up everything it asks.
///
/// **Every variant rather than a count.** A number here would go stale the
/// first time a room grows a question, and spec 0036 is plain that this
/// must never become "3 of 8 found". `Asked::every` is the one list.
///
/// This is the destination's gate and no longer the bridge's. The gate at
/// the top and what is through it are a finished bridge's, per spec 0117;
/// how far the bridge itself has got is [`reach`].
pub fn whole(trail: &Trail) -> bool {
    Asked::every().iter().all(|one| trail.given().contains(one))
}

/// How far along its run the bridge has got, as a share of it. Spec 0117.
///
/// **One fraction where there were six predicates.** Everything that makes
/// up the bridge asked `whole` separately and returned nothing, which is
/// six accounts of the same question and the reason a growing bridge could
/// not be written. There is one now and the deck, what holds you up, and
/// what carries you all read it.
///
/// **The foot is there before anything is answered**, which is why this is
/// one more than the questions over one more than their number rather than
/// given over every. A bridge that is nothing until the first question is
/// answered is not a promise; it is the same surprise moved one step
/// earlier. At nothing given that is a sixth of the run.
///
/// Counted off `Asked::every` for the reason `whole` gives: a sixth
/// question added anywhere makes this a seventh of the run with nothing
/// here touched.
pub fn reach(trail: &Trail) -> f32 {
    let every = Asked::every();
    let given = every
        .iter()
        .filter(|one| trail.given().contains(one))
        .count();

    (given + 1) as f32 / (every.len() + 1) as f32
}

/// How long one span takes to arrive. Spec 0119.
///
/// **Because spec 0117 said it took a few seconds and it took one frame.**
/// That spec's own words are that a span which popped would be a
/// notification, and `stations`' doc reasons about rounding "while a span
/// winds in", written as though the winding were there. It was not: `reach`
/// is a step function of the trail and the deck, soffit, bank and kerbs
/// were rebuilt in the single frame the count changed.
///
/// Six seconds, which is long enough to catch from the garden's opening if
/// you happen to be looking and short enough to be over before you could
/// walk out to it.
pub const WINDS: f32 = 6.0;

/// The deck's reach a moment later, walking toward what the trail owes it.
///
/// **At one pace and not eased.** Everything else in this building is wound
/// on rather than switched and most of it is eased, because most of it is a
/// light coming up or a view swinging round. A deck is laid, and a thing
/// being laid goes at the speed it goes at: an eased span would slide into
/// place, which is a different thing happening.
///
/// **One span's worth over `WINDS`**, counted off `Asked::every` the way
/// [`reach`] is, so a sixth question makes the run longer rather than each
/// span faster.
///
/// It lands exactly on what is owed rather than near it, so a finished
/// bridge reads 1.0 and the landing, which asks for that, is not kept out
/// by a millionth.
pub fn winding(drawn: f32, owed: f32, dt: f32) -> f32 {
    if drawn >= owed {
        return owed;
    }

    let span = 1.0 / (Asked::every().len() + 1) as f32;

    (drawn + span / WINDS * dt.max(0.0)).min(owed)
}

/// The name under the runes, which lives in [`crate::says`] with the
/// rest of the drafted copy so that signing it off is one command.
pub use crate::says::BIFROST as SAYS;

/// How hard the carving is burning, which breathes rather than holding
/// still.
///
/// **A steady glow is a lamp and a slow one is a thing that is alive.**
/// Jake asked for mystical, and what reads as that is not the brightness
/// but that it will not sit still: the garden's lanterns already come up
/// and go out over a long minute rather than switching, for the same
/// reason.
///
/// Two breaths crossed at lengths that do not divide, so it never repeats
/// on a count anybody could feel. Never down to nothing, because a rune
/// that goes out is a rune that was turned off.
pub fn burning(since: f32) -> f32 {
    let slow = (since * 0.41).sin();
    let under = (since * 0.17 + 1.3).sin();

    1.0 + slow * 0.16 + under * 0.07
}

/// How hard it burns while the sun is up, as a share of the night's.
///
/// Jake: the glyphs should especially glow at night. Which is also the
/// only time it can be seen to: a thing that glows is a thing brighter
/// than what is round it, and at noon nothing on a field is. Never
/// nought, because a rune that goes out is a rune that was turned off.
pub const BY_DAY: f32 = 0.30;

/// What it is giving off now: the hour and the flicker together.
pub fn glows(since: f32) -> f32 {
    burning(since) * (BY_DAY + (1.0 - BY_DAY) * nightly(since))
}

/// How much of the night it is: nought while the sun is up, one at the
/// bottom of it, and squared so the two twilights are the day's side of
/// halfway rather than the night's.
pub fn nightly(since: f32) -> f32 {
    let dark = 1.0 - (crate::garden::sun_up(since) * 0.5 + 0.5).clamp(0.0, 1.0);

    dark * dark
}

/// The colour the carved face is drawn in, glow and all.
///
/// **Divided by the light, because the engine multiplies by it.** A
/// colour past one is a thing lit with nothing lighting it, which is
/// true of the arithmetic and not of this one: what the shader works out
/// is the colour times the ambient plus the sun, so a multiplier on a
/// dark ambient is a dark mark. The fill over this field at the bottom
/// of the night is three hundredths, and the carving was four point two
/// red against that, which comes to a third of white. Jake, 2026-11-06:
/// it reads like a gentle orange, not burning bright like from myth.
///
/// Worse than gentle, it was backwards. Every number above says the
/// carving burns hardest at night, and the fill it was multiplied by
/// falls faster than they rise, so the one hour it was built for was the
/// one hour it was dimmest.
///
/// So `aim::RUNE_CUT` is now what the carving comes to on screen rather
/// than what it is multiplied by, and `garden::gives_off` divides it by
/// the fill the engine is about to multiply it back by. That is the same
/// function `main` hands the scene, so the two cannot drift, and it is
/// shared with the painted night under Valhalla's ridge.
/// The sun is on top of the fill and not divided out, which is why
/// `BY_DAY` is a third: by day this is an ember in daylight and nothing
/// is asked to read as burning.
///
/// **Alpha held under one on purpose**, per `aim::RUNE_GLAZE`: at one
/// the face goes through the engine opaque and is not drawn at all.
pub fn alight(since: f32) -> glam::Vec4 {
    let lit = crate::garden::gives_off(crate::aim::RUNE_CUT.truncate() * glows(since), since);

    glam::vec4(lit.x, lit.y, lit.z, crate::aim::RUNE_GLAZE)
}

/// How far the carving's own light stands off the face, how far it
/// carries, and how hard it burns at the bottom of the night.
///
/// **Because bright is not the same as glowing.** Jake, 2026-11-06: it
/// looks more neon than glowing; glowing means it is shining brightly
/// from within what holds it, like the stone. A line that is past white
/// and throws nothing is a neon tube, which is a thing that is its own
/// light and lights nothing. What makes a glow read is the surface
/// round it, so the carving puts a lamp in front of its own face and
/// the rock takes it.
///
/// **This was tried once and taken out again**, and it is worth saying
/// why it comes back rather than pretending it is new. It washed the
/// face pale and took the contrast the glyphs were living on, because
/// the glyphs then were a multiplier on a dark ambient and a lamp on
/// the same stone could out-shine them. They are four times past white
/// now, per `alight`, so nothing a lamp does to the rock can reach
/// them.
///
/// Short and weak against the room lamps: this is a groove, not a
/// lantern, and the thing to light is the stone it is cut in and a
/// little of the rubble at its foot.
pub const LAMP_OUT: f32 = 0.26;
/// **Shortened from five, because five lit the heap.** Jake, with a
/// recording: there is banding from the lights. The face is 0.26 off the
/// lamp and the rubble is one to three metres out, and at a range of five
/// those stones took enough of it to matter: `garden::rock_mesh` is
/// faceted on purpose, so a facet turned toward a near point light flares
/// while the one beside it stays dark, and the heap came out in hard
/// orange patches that moved as the lamp pulsed.
///
/// A range that falls off inside the heap keeps what the note above asks
/// for, which is the stone it is cut in and a little of the rubble, and
/// not the heap picked out stone by stone.
pub const LAMP_RANGE: f32 = 1.8;
pub const LAMP_LIT: f32 = 0.42;

/// Where that light stands, what colour it is, how hard and how far,
/// for a stone standing at `at` and scaled by `size`.
///
/// **Off the face's own middle rather than a guess at it.** The dressed
/// panel is the stone's own skin at `u` of a half, lifted by
/// `FACE_PROUD`, so the one place the light can stand and be in front of
/// every part of it is on that normal. Taken off `stone_point` and the
/// same lean the stone is drawn with, so a knobblier seed moves the
/// lamp with the rock instead of burying it.
pub fn carving_lamp(at: Vec3, size: Vec3, since: f32) -> (Vec3, Vec3, f32, f32) {
    let middle = (FACE_FROM + FACE_TO) * 0.5;
    let local = stone_point(STONE_SEED, 0.5, middle) + Vec3::X * (FACE_PROUD + LAMP_OUT);
    let stood = at + glam::Quat::from_rotation_z(RUNE_LEANS) * (local * size);

    (
        stood,
        crate::aim::RUNE_EMBER.normalize_or(Vec3::X),
        LAMP_LIT * glows(since),
        LAMP_RANGE,
    )
}

/// How fine the carved face is drawn.
pub const CARVED: u32 = 1024;

/// The runes of the name, in younger futhark, as the strokes each is made
/// of in its own square.
///
/// **Because Latin capitals on a grey slab are a gravestone.** Jake,
/// 2026-11-05: it looks like a random grey slab, it does not land as a
/// nordic runestone. The lettering was the arcade's own pixel font, which
/// says arcade and not Norse however it is coloured.
///
/// Runes are strokes and nothing else, which is why they were cut into
/// stone and why they can be drawn here without a font: a stave down the
/// middle and branches off it. These are b, i, f, r, o, s, t, the name
/// spelled the way the sound goes rather than the way the Latin does.
///
/// Each is a list of lines in its own square, nought to one, y up.
const RUNES: [&[(f32, f32, f32, f32)]; 7] = [
    // bjarkan, b
    &[
        (0.3, 0.0, 0.3, 1.0),
        (0.3, 1.0, 0.72, 0.78),
        (0.72, 0.78, 0.3, 0.55),
        (0.3, 0.55, 0.72, 0.33),
        (0.72, 0.33, 0.3, 0.1),
    ],
    // iss, i
    &[(0.5, 0.0, 0.5, 1.0)],
    // fe, f. **Its branches rise and oss's fall, which is the whole of
    // what tells the two apart.** Both were drawn falling: a stave with
    // two branches angling down to the right, a few hundredths of a
    // square apart, so the name read b i f r f s t. Jake, 2026-11-18, of
    // the fifth rune: it looks like an f. It was one.
    &[
        (0.3, 0.0, 0.3, 1.0),
        (0.3, 0.62, 0.74, 0.86),
        (0.3, 0.34, 0.74, 0.58),
    ],
    // reid, r
    &[
        (0.3, 0.0, 0.3, 1.0),
        (0.3, 1.0, 0.72, 0.8),
        (0.72, 0.8, 0.3, 0.56),
        (0.34, 0.52, 0.74, 0.05),
    ],
    // oss, o, which is fe turned over
    &[
        (0.3, 0.0, 0.3, 1.0),
        (0.3, 0.9, 0.72, 0.68),
        (0.3, 0.66, 0.72, 0.44),
    ],
    // sol, s
    &[
        (0.68, 1.0, 0.32, 0.72),
        (0.32, 0.72, 0.68, 0.44),
        (0.68, 0.44, 0.32, 0.14),
    ],
    // tyr, t
    &[
        (0.5, 0.0, 0.5, 1.0),
        (0.5, 1.0, 0.22, 0.68),
        (0.5, 1.0, 0.78, 0.68),
    ],
];

/// How wide a cut stroke is, as a share of the carved face: the ribbon
/// round the inscription, and the runes inside it.
///
/// **Thin, and fattening it was the wrong fix.** At 0.026 a stroke was
/// half the width of the rune it belonged to, so every branch ran into
/// its own stave and consecutive runes touched: the chisel was wider than
/// the gaps. What the writing needed was to be bigger, not bolder.
///
/// **And the two are not one chisel.** They were, at `STROKE` for both,
/// and Jake read it on the stone: the band is right and the lettering is
/// heavy. That is what a real stone looks like anyway. The ribbon is the
/// outline somebody cut first and deepest, and the writing inside it is
/// finer work.
///
/// **A chisel is a whole number of texels wide.** `carving` walks a disc
/// of `ceil(thick * CARVED / 2)` along each stroke, so every width
/// between two of those rounds to the same line. The first thinner rune
/// was 0.0085 against the band's 0.0115, which is 1.09 against 1.47 and
/// both ceil to 2: the carving came out byte for byte what it was and
/// was reported as thinner off two renders at different scales. Jake
/// read the stone and said the glyphs were still too thick, which they
/// were, because nothing had moved.
///
/// So the steps are what there is to choose from, not the numbers. At
/// 256 across, the band is five texels and the writing is three.
pub const STROKE: f32 = 0.0115;
pub const RUNE_STROKE: f32 = 0.0070;

/// How finely the stone itself is cut, and how far its skin wanders at
/// each scale.
///
/// **Jake, 2026-11-05: it needs to be impressive, and the kind of
/// texturing that would take would be worth the vertices.** This is the
/// start of the last part of the trail. The lump the garden's set stones
/// are cut from is eleven round by seven up, which is right for something
/// the size of a loaf lying in gravel and is a smooth egg at two metres
/// tall: the granite picture gave it the look of rock and none of the
/// shape of one.
///
/// Twelve thousand quads for one object, which this building would not
/// spend anywhere else and spends here.
pub const STONE_ROUND: u32 = 176;
pub const STONE_STEPS: u32 = 224;
pub const KNOBBLE: f32 = 0.235;

/// How far the stone wanders from a ball at `u`, `v`.
///
/// **Five scales, each a whole number of waves.** Whole, because a wave
/// that does not come back to itself leaves a crack up the seam and a
/// tear at the poles, which is the rule `garden::wandered` already works
/// to; this one just goes finer, and the fine scales are what rock is.
pub fn knobble(seed: u32, u: f32, v: f32) -> f32 {
    use std::f32::consts::{PI, TAU};

    let mut state = seed | 1;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;

        state
    };

    let mut out = 0.0;
    let mut weight = 0.0;

    // **Wave counts that are not multiples of each other.** Doubling
    // them, 1 2 4 8 16, lines every scale's crests up on the same
    // meridians and the stone comes out fluted. These share no factors,
    // so the scales never agree twice round.
    //
    // And not past about a quarter of the steps there are, or the finest
    // scale has two samples a wave and draws a shimmer rather than a
    // bump.
    for (waves, share) in [
        (1.0, 1.0),
        (3.0, 0.62),
        (7.0, 0.40),
        (13.0, 0.30),
        (23.0, 0.24),
        (41.0, 0.19),
        (67.0, 0.14),
    ] {
        let round = waves + (next() % 2) as f32;
        let up = waves + (next() % 3) as f32;
        let phase = (next() % 360) as f32 / 360.0 * TAU;

        out += (u * TAU * round + phase).sin() * (v * PI * up).sin() * share;
        weight += share;
    }

    // **The fine scales keep most of their weight.** Halving each one
    // against the last is how a landscape is made, and it draws a boulder
    // with gentle angles, which is what Jake called this. Real stone is
    // mostly small bumps with a shape somewhere under them.
    //
    // And the written part is taken back down to smooth, because a thin
    // line over that surface breaks up and stops being readable, which it
    // did. A runestone's face is dressed before it is cut.
    out / weight * KNOBBLE * (1.0 - 0.74 * dressed_by(u, v))
}

/// How far the face has been dressed flat at `u`, `v`: one in the middle
/// of the written part and nothing outside it.
///
/// **A rough stone and legible writing are one question.** Jake asked for
/// both in the same breath, and they pull against each other everywhere
/// except here: the shape of the stone carries on under the face and only
/// the small stuff comes off it.
pub fn dressed_by(u: f32, v: f32) -> f32 {
    // **Flat across the whole panel, and falling off outside it.** This
    // measured out from the middle of the face, so the dressing was
    // strongest at the centre and half gone at the edges. That was
    // survivable while the panel was a thin strip. Widened to a proper
    // dressed face, its sides sat where the stone is still half as
    // bumpy as it is anywhere, the bumps came through the carving, and
    // the ribbon vanished at every hour of the day: Jake's night
    // picture of a blank stone is this and not the glow.
    let across = over(1.0, (u - 0.5).abs() / face_wide(v));
    let middle = (FACE_FROM + FACE_TO) * 0.5;
    let along = over(0.5, (v - middle).abs() / (FACE_TO - FACE_FROM));
    let off = across.max(along);

    (1.0 - off * off).max(0.0)
}

/// Nought up to `edge`, one by half again past it: how far outside a
/// thing you are, for something that has to be wholly flat inside it.
fn over(edge: f32, at: f32) -> f32 {
    ((at - edge) / (edge * 0.6)).clamp(0.0, 1.0)
}

/// Where the stone's skin is at `u` round and `v` up, in its own unit.
///
/// **One account of the surface, which is the whole point.** The carving
/// is built from this too, so it lies on the rock instead of over it: it
/// was a flat slab standing proud of a lump, and you could see the plate.
/// How broad the stone is a given way up itself.
///
/// **A slab and not a column.** An egg is as narrow at the top as it is
/// at the foot, and a tall narrow lump with two round stones either side
/// of it looks like exactly what Jake said it looked like. A runestone is
/// a broad flat slab that holds its width nearly to the top and is
/// shouldered off above that.
///
/// Full width from the grass to better than two thirds up, then an arc
/// over. The foot closes to a point under the ground, where nobody sees
/// it.
pub fn girth(v: f32) -> f32 {
    // **Blunt at the crown, not pointed.** Smoothstepped off over the
    // top third it came out a cone, and a cone on a field is a haystack.
    // A stone that has stood a thousand years has a weathered top, so
    // this holds full width to near the head and then rounds over fast.
    let crown = ((0.995 - v) / 0.17).clamp(0.0, 1.0).powf(0.38);
    // and loses a little going up, the way a split block does
    let waist = 1.0 - 0.18 * (v - 0.25).max(0.0);
    let foot = (v / 0.04).clamp(0.0, 1.0).sqrt();

    crown * waist * foot
}

/// How much wider than its body the stone is at a height `v`: the
/// swelling at its foot.
///
/// **Jake: it needs to be a lot thicker at the bottom.** The first go at
/// that multiplied the whole profile by a term falling off over the
/// stone's entire height, which is the shape of a cone and read as one
/// from thirty metres. Weight low down is not a taper from top to
/// bottom; it is a body of roughly one width with a plinth swelling out
/// of the ground under it. This is the swelling, and it is gone by
/// `SWELLS_TO`.
pub const SWELLS: f32 = 1.05;
pub const SWELLS_TO: f32 = 0.62;

pub fn flare(v: f32) -> f32 {
    let low = ((SWELLS_TO - v) / SWELLS_TO).clamp(0.0, 1.0);

    1.0 + SWELLS * low * low
}

/// How much of the swelling the broad face gets.
///
/// **Less than the thin one.** The stone is a slab: 0.92 through and 1.6
/// across, and a swelling applied evenly to both keeps that ratio, so
/// the foot is a bigger slab rather than a heavier stone. Given mostly
/// to the thickness, the base reads as a block and the head still reads
/// as a face to carve.
pub const BROADENS: f32 = 0.45;

/// The stone's half-extents in metres at a height `v`, through the face
/// and across it. The shape as the eye reads it, which is the body, the
/// swelling and the two scales together and not any one of them.
pub fn half_at(v: f32) -> (f32, f32) {
    let body = girth(v) * 0.5;
    let swell = flare(v);

    (
        RUNE_THICK * body * swell,
        RUNE_WIDE * body * (1.0 + (swell - 1.0) * BROADENS),
    )
}

/// And where it meets the grass, which is what the heaped footing banks
/// against.
pub fn foot_half() -> (f32, f32) {
    half_at(RUNE_SUNK / RUNE_TALL)
}

/// Which stone this is.
///
/// **Named once because three things are the same lump.** The stone's
/// own mesh, the dressed face laid on it and the granite it is
/// textured with all take it, and a fourth now reads it to work out
/// where the carving's light stands. Four copies of a magic number is
/// three chances to carve one rock and light another.
pub const STONE_SEED: u32 = 0x51A7;

pub fn stone_point(seed: u32, u: f32, v: f32) -> Vec3 {
    use std::f32::consts::TAU;

    // the face looks along +x at `u` of a half, which keeps the seam round
    // the back where nothing is written
    let round = -(u - 0.5) * TAU;
    let out = (0.5 + knobble(seed, u, v)) * girth(v);
    let lean = crate::garden::tip(seed, v);
    let swell = flare(v);

    vec3(
        round.cos() * out * swell + lean.x,
        v - 0.5,
        round.sin() * out * (1.0 + (swell - 1.0) * BROADENS) + lean.y,
    )
}

/// The stone.
pub fn stone_mesh(seed: u32) -> blitzkit::mesh::MeshData {
    crate::garden::faceted(blitzkit::mesh::MeshData::surface(
        STONE_ROUND,
        STONE_STEPS,
        |u, v| stone_point(seed, u, v),
    ))
}

/// How much of the stone the carved face takes: half its width in `u`,
/// and the two `v` it runs between.
/// **A dressed face, with the ribbon a narrow thing down the middle of
/// it.** The band used to be drawn at the edges of this, which made the
/// frame three times wider than the seven runes it framed: a chain hung
/// on a hoarding. Seven runes stacked down a metre and a fifth cannot be
/// made much bigger than a hand, so what had to change was the frame,
/// not the lettering. The panel is wider than it was and the ribbon
/// hugs the text, leaving plain dressed stone either side of it, which
/// is what the carved stones actually look like.
pub const FACE_WIDE: f32 = 0.105;
pub const FACE_FROM: f32 = 0.52;
pub const FACE_TO: f32 = 0.86;
/// And how far the carving stands off the skin it is laid on.
/// **Proud enough to clear what is left of the stone's grain.** A
/// hundredth was the number while the panel sat in the flattest part of
/// the face. It is a dressed panel standing off a rough stone, and a
/// real one stands off it by more than a millimetre.
pub const FACE_PROUD: f32 = 0.03;

/// How much of the way round the stone the dressed face takes, at a
/// height `v`.
///
/// **Widening as the stone narrows, so the panel does not keystone.**
/// `FACE_WIDE` is a share of the way round, and a share of the way round
/// a tapering stone is a narrowing strip: once the foot was flared the
/// band's bottom row was twice the width of its top, and the name read
/// as if seen from below. Held to a constant arc instead, by giving back
/// exactly what the taper takes, the panel is a straight-sided board
/// with the stone falling away behind it.
pub fn face_wide(v: f32) -> f32 {
    let middle = (FACE_FROM + FACE_TO) * 0.5;

    FACE_WIDE * girth(middle) / girth(v).max(0.05)
}

/// The carved face: the stone's own surface over the written part of it,
/// lifted off itself by a hair.
///
/// **Built from `stone_point`, so every bump under it is the bump the
/// stone has.** A quad laid across a rock is a plate stuck on a rock, and
/// that is what Jake saw: the panel's own edge was visible against the
/// lump. This one is the rock.
pub fn carved_mesh(seed: u32) -> blitzkit::mesh::MeshData {
    use std::f32::consts::TAU;

    let mut mesh = blitzkit::mesh::MeshData::surface(56, 72, |s, t| {
        // down the stone as the picture goes down, because the picture's
        // row nought is its top and `v` nought is this lump's foot.
        // Taken the other way the band stood on its head and the name
        // read along the crown.
        let v = FACE_TO - t * (FACE_TO - FACE_FROM);
        let u = 0.5 + (s - 0.5) * face_wide(v) * 2.0;
        let round = -(u - 0.5) * TAU;
        let at = stone_point(seed, u, v);

        at + vec3(round.cos(), 0.0, round.sin()) * FACE_PROUD
    });
    mesh.compute_normals();

    mesh
}

/// One stroke of the carving: where it runs in the face's own square,
/// nought to one, and how wide the chisel was.
///
/// **The chisel travels with the stroke.** The band and the runes are cut
/// at different widths, and the only thing that says which a stroke is
/// is what it is. Read off the front of the list instead, the first four
/// being the band is a fact in two places.
pub type Cut = ((f32, f32), (f32, f32), f32);

/// Every stroke of the carving, in the face's own square, nought to one.
///
/// **One account, because the stone is cut and not printed.** Jake,
/// 2026-11-05: there could be geometrical texture on which the glyphs are
/// laid, so they do not look so flat. A picture on a rock is a decal
/// however it is lit: what makes a carving read is that its edges take the
/// light differently from the face round them, and that wants geometry.
///
/// So this is the lines, once, and both the picture and the cut bars are
/// stood along them.
/// `room` is how much of the face to clear at the foot, which the whole
/// column rises by.
///
/// **Nought everywhere, and kept because it was earned.** Spec 0118 cut
/// what the building is still holding into this face and the serpent
/// pulled its tail up to make room: the whole column and not the tail
/// alone, the uprights running to 0.065 with the lowest rune's foot at
/// 0.085, so a tail lifted on its own leaves runes hanging outside the
/// ribbon they are written in. The line moved off this panel onto its own
/// band of stone below, that panel being 0.6 metres wide and a sentence
/// needing more, so nothing asks for room now. The parameter stays
/// because the next thing to want room will want it the same way.
pub fn strokes(room: f32) -> Vec<Cut> {
    // **The ribbon's own numbers, not the runes'.** Its top used to be
    // `far`, which was also its width, and the two agreed only because
    // the band happened to be as wide as it was tall. Narrowed to the
    // lettering, `far` fell to 0.615 and took the top down with it: the
    // column ran out of the bottom of the picture and three of the
    // seven runes were drawn off the stone.
    let (near, far) = (0.385, 0.615);
    let (foot, shoulder, apex) = (0.065 + room, 0.885 + room, 0.945 + room);
    // **Each stroke carries its own chisel.** The band and the runes are
    // cut at different widths and the only thing that says which is which
    // is what it is, so it is said here rather than counted off the front
    // of the list by whoever draws it.
    let mut out = vec![
        // the band, round the face and open at the foot the way a
        // serpent's body leaves its own tail
        ((near, foot), (near, shoulder), STROKE),
        ((near, shoulder), (0.5, apex), STROKE),
        ((0.5, apex), (far, shoulder), STROKE),
        ((far, shoulder), (far, foot), STROKE),
    ];

    // and the runes down the middle of it, reading from the top. Taller
    // than wide, and the strokes run from 0.22 to 0.78 inside their own
    // square, so the width factor is near one to get a rune half as wide
    // as it is high.
    // **Drawn squat, because the face they land on is not square.** The
    // picture is a square and the patch it is laid on is about twice as
    // tall as it is wide, so anything drawn square comes out stretched
    // into a smear: the column of runes merged into one wiggling line and
    // Jake could not read a letter of it.
    //
    // So a rune is written about twice as wide as it is high here, to
    // arrive about twice as high as it is wide out there. The strokes run
    // from 0.22 to 0.78 inside their own square, which is in the factor.
    //
    // **And filling the ribbon.** At 2.05 against the old wide band the
    // column used a fifth of it. The ribbon moved rather than the
    // runes; this is the width that fills the ribbon and still arrives
    // about twice as high as it is wide out on the stone.
    let tall = 0.0778;
    let wide = tall * 1.3;
    let gap = 0.035;
    let top = 0.875 + room;

    for (n, lines) in RUNES.iter().enumerate() {
        let foot = top - (n as f32 + 1.0) * (tall + gap);

        for (x0, y0, x1, y1) in lines.iter() {
            out.push((
                (0.5 + (x0 - 0.5) * wide, foot + y0 * tall),
                (0.5 + (x1 - 0.5) * wide, foot + y1 * tall),
                RUNE_STROKE,
            ));
        }
    }

    out
}

/// The carved face of the stone: a band round the edge with the runes of
/// the name standing in it.
///
/// **The band is what says runestone more than the runes do.** An Uppland
/// stone is an inscription inside a looping ribbon that runs round the
/// face, and the eye reads that outline before it reads a single
/// character. Without it this is strokes on a rock.
///
/// Cut away rather than painted on: the alpha is nought everywhere the
/// chisel did not go, so the stone shows between the strokes and the
/// engine's cutout throws the rest away.
/// Cuts a line of letters into a face that is already being carved: what
/// it says, how tall its capitals are as a share of the face, and how far
/// its foot is up from the bottom.
///
/// **One account of compositing text, because there are two lines now.**
/// Spec 0118 put what the building is still holding under the name, and
/// the name's own block was written inline: a second copy of it would be
/// two ways of laying type on one stone, which is the fault this project
/// keeps finding in a new costume.
fn cut_in(pixels: &mut [u8], wide: u32, high: u32, says: &str, tall: f32, foot: f32, stretch: f32) {
    let said = blitzkit::text::stencilled(says, RUNE_TEXELS);
    let (was_wide, was_tall) = (said.width() as f32, said.height() as f32);
    // **Sized by its height, not its width.** It was laid out to a
    // fixed share of the picture across, with the height following from
    // the aspect, which means the fewer letters there are the taller
    // they get. The marker came off when Jake signed the copy, the
    // string went from twenty one characters to seven, and the block
    // trebled in height and ran up into the rune ribbon. A cap height
    // is the thing that should not move when the words change.
    // **Drawn wider than it should be, to arrive the right shape.** The
    // patch this is laid on is about twice as tall as it is wide, so a
    // square picture is squeezed by two across and every letter with it:
    // the O in the name lost its bowl until its two uprights merged and
    // Jake read it as an F. The runes have compensated for this since they
    // went in, their own note saying a rune is written twice as wide here
    // to arrive twice as high as it is wide out there; the Latin never did.
    let want = (
        high as f32 * tall * stretch * was_wide / was_tall,
        high as f32 * tall,
    );
    // **And taken down when it is too wide for the face.** A cap height
    // that does not move is right for a name of one word and wrong for a
    // sentence: spec 0118's lines run to sixty characters, and a long one
    // set at the name's own height would run off both edges of the stone.
    let room = wide as f32 * CUT_WIDE;
    let shrink = (room / want.0).min(1.0);
    let want = (want.0 * shrink, want.1 * shrink);
    let (at_x, at_y) = ((wide as f32 - want.0) * 0.5, high as f32 * foot);

    for down in 0..want.1 as u32 {
        for across in 0..want.0 as u32 {
            let from = (
                (across as f32 / want.0 * was_wide) as u32,
                (down as f32 / want.1 * was_tall) as u32,
            );
            let took = ((from.1.min(said.height() - 1) * said.width()
                + from.0.min(said.width() - 1))
                * 4) as usize;

            // **How much of a letter, not whether there is one.** This
            // took anything over a half and wrote it solid, which throws
            // away the font's own antialiasing and leaves the name as
            // blocky as the strokes were.
            let cover = said.levels[0].pixels[took + 3];

            if cover == 0 {
                continue;
            }

            // the picture's row nought is its top and so is the text's,
            // so the block is placed by its top row and read forwards.
            // Walking the destination upwards instead turned every
            // letter over.
            let (to_x, to_y) = (
                at_x as u32 + across,
                high.saturating_sub(at_y as u32 + want.1 as u32) + down,
            );
            let at = ((to_y.min(high - 1) * wide + to_x.min(wide - 1)) * 4) as usize;

            for c in 0..3 {
                pixels[at + c] = 255;
            }
            pixels[at + 3] = pixels[at + 3].max(cover);
        }
    }
}

/// How much of the face across a cut line may use.
pub const CUT_WIDE: f32 = 0.92;

/// How much taller than wide the dressed panel is, which is how much a
/// square picture laid on it squeezes what is drawn.
///
/// Two, which is the same observation `strokes` records about the runes:
/// the patch is about twice as tall as it is wide. Named here because two
/// things now have to undo it.
pub const FACE_SQUEEZE: f32 = 2.0;

/// How tall spec 0118's line stands and how far up the face its foot is.
///
/// **Smaller than the name and above it.** The name is what the stone is
/// and does not change; this is what it happens to be saying today, and a
/// sentence set as large as a title reads as the title.
/// **Asked for larger than it can have.** `cut_in` takes a line down to
/// fit `CUT_WIDE` of the picture, and on a band this shape every one of
/// spec 0118's lines is wider than it is tall: what sets the size is the
/// number of characters, not this. It is written large so the short lines
/// get everything the band will give them.
///
/// **And since spec 0119 it is a ceiling rather than a height.** The
/// ledger cuts every row at one size, which is the smallest any of the
/// lines can have, so what this sets is how large a row may ever be: see
/// `ledger_tall`. `HELD_FOOT` went with it, a block of rows being placed
/// by `ledger_foot` off its own count.
pub const HELD_TALL: f32 = 0.60;

/// Where the held line's own band of stone runs, and how far round the
/// stone it takes.
///
/// **Its own patch of rock, under the dressed panel rather than on it.**
/// The line was cut into the panel with the name, which is 1.22 metres
/// tall and 0.6 wide: a square picture squeezed onto that halves every
/// letter across, so forty five characters came out 3.7 centimetres tall
/// and 1.1 wide and Jake could not read them from more than a few paces.
/// Width is what binds, not height, and the panel had none to give.
///
/// This band is wider than the panel and short, and its picture is cut to
/// the same shape, so the letters arrive the shape they were drawn. It is
/// built from `stone_point` like the panel above it, so it is the rock
/// rather than a plate on the rock, which is what spec 0054 spent a whole
/// change learning.
///
/// Below the panel and clear of it, reading as something added to the
/// stone rather than part of its making.
/// The ledger stone at the runestone's foot: how far out from its middle
/// it lies, how wide it is across the approach, how far it runs away from
/// you, and how far it is tipped up to face somebody standing at it.
///
/// **Flat, because the stone is not.** Spec 0118 cut the held line into
/// the lump itself and then into a band round it, and both failed the same
/// way: to be big enough for a sentence the line has to run a fifth of the
/// way round, and that far round the rock is turning away, so the line
/// bends at both ends and the letters skew with the surface. Narrow it
/// until it stays on the flat front and it is back to the panel's width.
/// Jake, 2026-11-18, looking at the band: impossible to read. Then, asked
/// where it should go: somewhere other than the stone, but still part of
/// it.
///
/// A ledger laid in the footing is that. It is a real thing to find at the
/// foot of a raised stone, it is as wide as the heap will take with no
/// curve anywhere on it, and the line gets half as many characters per
/// metre again as the band could give it.
///
/// Tipped up rather than laid flat, because a slab read from standing
/// height at a pace away is seen at forty degrees and a tilt buys most of
/// that back.
pub const SLAB_OUT: f32 = 1.15;
/// **Widened to 2.1 for spec 0119 and put straight back.** The ledger
/// cuts every row at the smallest line it could ever say, so the shortest
/// entry came down from the 3.2 centimetres it had alone to the 2.2 the
/// longest has always had, and width is what binds a line here. The
/// arithmetic said the footing would take it: at `SLAB_OUT` out from the
/// middle a heap of `RUBBLE_OUT` still runs 1.8 either way and 2.1 needs
/// 1.05. The picture said otherwise at once. A mound falls away from its
/// middle, so what the rim can hold is not what the slope can carry, and
/// at 2.1 the slab stood out past the rubble on both sides and read as a
/// black board laid on the grass: which is the exact thing `slab_at` was
/// written to avoid, in that constant's own words. What buys the size
/// back is a shorter longest line, not a wider stone.
pub const SLAB_WIDE: f32 = 1.5;
pub const SLAB_LONG: f32 = 0.62;
/// **Propped rather than laid.** At a quarter turn it was near enough
/// flat, and a face turned up at a noon sun takes the full of it: at an
/// albedo of 0.25 it still came out paler than the heap beside it at
/// 0.70, because the heap's slopes are turned away from the light. Stood
/// up it takes less sun and faces the reader at once, and a slab propped
/// against the foot of a raised stone is a thing that happens.
pub const SLAB_TIPS: f32 = 0.78;

/// How far the inscription stands off the slab it is cut into.
///
/// **Two faces in one plane is the fault this building has found eight
/// times**, and a flat slab with a flat inscription on it is the easiest
/// way yet to make a ninth. The same answer `FACE_PROUD` gives on the
/// lump above: the words stand off the stone.
pub const SLAB_PROUD: f32 = 0.004;

/// How thick the ledger is.
///
/// **Because a slab with no sides is a smooth patch.** Laid as one face
/// it was the same granite as the heap with no edge anywhere on it, so
/// what you saw was words lying on the ground rather than words on a
/// stone. A hand's thickness gives it four sides and a shadow under its
/// lower edge, which is the whole of what says there is a stone there.
pub const SLAB_THICK: f32 = 0.11;

/// The ledger as a box: where its middle sits, how far it is tipped, and
/// how big it is.
///
/// **A box and not a mesh of its own.** It is a cuboid, the building has
/// a cube, and a sixth mesh of six quads would be six quads nobody needs.
/// What it cannot be is the inscription as well, a cube's texture running
/// nought to one on every face it has.
pub fn slab_box() -> (Vec3, f32, Vec3) {
    let (top, lean) = slab_at();
    let up = vec3(-lean.sin(), lean.cos(), 0.0);

    (
        top - up * SLAB_THICK * 0.5,
        lean,
        vec3(SLAB_LONG, SLAB_THICK, SLAB_WIDE),
    )
}

/// How wide its picture is, and how tall, which is that slab's own shape.
///
/// **Its own shape and not near it.** `said_mesh` lays this on the slab
/// corner to corner, so a picture of another aspect stretches every
/// letter: that is what `cut_in`'s `stretch` undoes for the panel above,
/// and what this avoids by matching. Held by
/// `tests::the_ledgers_picture_is_the_slabs_own_shape`.
///
/// **Enough across for five rows rather than one.** At 1024 a row's
/// capitals came out thirteen pixels, which is a letter you can count the
/// steps on: one line could be cut large and five are cut at the smallest
/// of them. This is 1024 pixels a metre and a capital of about twenty
/// two, and it costs 3.9 megabytes rewritten five times in a run.
pub const SAID_ACROSS: u32 = 1536;
pub const SAID_DOWN: u32 = 635;

/// The ledger stone, as the face you read.
///
/// Built flat and then tipped, and set into the heap rather than resting
/// on it: a slab lying on a mound is a slab lying on a mound, which is
/// what `rubble`'s own note says about pebbles on grass.
/// **In metres about the stone's foot, like `mound_mesh`.** The panel and
/// the band were both built in the stone's own unit space and drawn
/// scaled by `size`; this is not, so it takes the transform the mound
/// takes rather than the one the carving takes.
pub fn said_mesh(proud: f32) -> blitzkit::mesh::MeshData {
    let (at, lean) = slab_at();
    let up = vec3(-lean.sin(), lean.cos(), 0.0) * proud;
    let mut mesh = blitzkit::mesh::MeshData::surface(2, 2, |s, t| {
        // **Both the way round they are, and neither by guess.** You read
        // this standing east of the stone looking west, so screen right is
        // `-z` and the picture's own row nought is its top, which has to
        // land at the far edge. Written the other way the line came out
        // turned through half a circle, which is the one thing a flat slab
        // can get wrong.
        let across = (0.5 - s) * SLAB_WIDE;
        let along = (t - 0.5) * SLAB_LONG;

        at + up + vec3(along * lean.cos(), along * lean.sin(), across)
    });
    mesh.compute_normals();

    mesh
}

/// Where the middle of the ledger sits, in the runestone's own terms, and
/// how far it is tipped.
///
/// **Measured off the heap and not guessed.** `mound_high` says how high
/// the footing stands a share of the way out, so the slab lies on the heap
/// wherever the heap happens to be rather than at a number that goes stale
/// the first time `RUBBLE_UP` moves.
pub fn slab_at() -> (Vec3, f32) {
    let share = SLAB_OUT / RUBBLE_OUT;
    // its near edge rather than its middle, the heap falling away as it
    // goes out: a slab set by its middle digs its far edge in
    let on = mound_high(share - SLAB_LONG * 0.5 / RUBBLE_OUT);

    (vec3(SLAB_OUT, on, 0.0), -SLAB_TIPS)
}

/// And what is cut into it, or nothing once every room has answered.
pub fn said_cut(lines: &[&str]) -> blitzkit::texture::TextureData {
    let mut pixels = vec![0u8; (SAID_ACROSS * SAID_DOWN * 4) as usize];
    let tall = ledger_tall();

    for (row, line) in lines.iter().take(rows()).enumerate() {
        // **No stretch here**, the picture already being cut to the slab's
        // own shape rather than square.
        cut_in(
            &mut pixels,
            SAID_ACROSS,
            SAID_DOWN,
            line,
            tall,
            ledger_foot(row, tall),
            1.0,
        );
    }

    blitzkit::texture::TextureData::from_pixels(SAID_ACROSS, SAID_DOWN, pixels)
}

/// How many rows the ledger has, which is how many lines it can ever show.
///
/// **Five, and it cannot be six.** A row is filled by a question answered
/// and the last row by the one still held, so a trail with four given has
/// four done and one held, and a finished trail has five done and nothing
/// held. The count never passes the number of questions. Held by
/// `tests::the_ledger_never_wants_more_rows_than_it_has`.
///
/// **A function, `Asked::every` not being one you can call in a
/// constant.** Which is the right way round anyway: a sixth question
/// gives the stone a sixth row with nothing here touched.
pub fn rows() -> usize {
    Asked::every().len()
}

/// How far apart the rows sit, as a share of a row's own cap height.
///
/// Set by eye against a ledger rather than a page: wider than a
/// paragraph's leading, because these are entries and not prose, and
/// there is stone to spare.
pub const LEDGER_PITCH: f32 = 2.6;

/// The cap height every row of the ledger is cut at.
///
/// **One size for the lot, set by the longest line there could ever be.**
/// `cut_in` takes a line down on its own to fit the face, so left to
/// itself a short entry is cut larger than a long one: "Nobody has been
/// in the water" would stand half again as tall as the line about the
/// room you started in, and five entries at five sizes is not a ledger,
/// it is a ransom note.
///
/// **And over every line the stone could ever say, not the ones showing.**
/// Sized to what is on it today, the type would change size as rows filled
/// in, which is an entry you already read moving and growing while your
/// back is turned.
fn ledger_tall() -> f32 {
    Asked::every()
        .iter()
        .flat_map(|one| [crate::says::given(*one), crate::says::still_held(*one)])
        .map(|line| fits_in(SAID_ACROSS, SAID_DOWN, line, HELD_TALL, 1.0))
        .fold(HELD_TALL, f32::min)
}

/// Where the foot of row `row` sits, counting from the top.
///
/// **Laid out for all five however many are on it.** A block that was
/// centred on what it holds would move every entry down the stone each
/// time one was added, and an entry already written is the one thing on a
/// ledger that does not move.
fn ledger_foot(row: usize, tall: f32) -> f32 {
    let rows = rows();
    let block = (rows - 1) as f32 * LEDGER_PITCH * tall + tall;
    let bottom = (1.0 - block) * 0.5;

    bottom + (rows - 1 - row.min(rows - 1)) as f32 * LEDGER_PITCH * tall
}

/// What a line would have to come down to, to fit `CUT_WIDE` of the face
/// at `tall`.
///
/// **The same arithmetic `cut_in` does to shrink one**, asked ahead of
/// time so that a block of lines can all be cut at the smallest of them.
/// Written here rather than taken out of `cut_in`, because that one has to
/// shrink whatever it is handed whether or not anybody asked first: the
/// name on the panel above has nothing to be sized against.
fn fits_in(wide: u32, high: u32, says: &str, tall: f32, stretch: f32) -> f32 {
    let said = blitzkit::text::stencilled(says, RUNE_TEXELS);
    let want = high as f32 * tall * stretch * said.width() as f32 / said.height() as f32;
    let room = wide as f32 * CUT_WIDE;

    tall * (room / want).min(1.0)
}

/// Every line the ledger shows: what has been answered, in the order it
/// was, and then the one still held. Spec 0119.
///
/// **Done first and held last, which is a ledger and not a list.** Spec
/// 0036 forbids a list of what is left and spec 0118 answered that with
/// one held line at a time; that rule is untouched here, there still being
/// exactly one. What is added is the other half, which is not a list of
/// what is left but a record of what happened, and spec 0020's worn lane
/// in the carpet is the same thing one room along.
///
/// Nothing held once every question has been given, per spec 0118, and
/// then the stone is five entries and no question.
pub fn ledger(trail: &Trail) -> Vec<&'static str> {
    let mut out: Vec<&'static str> = trail
        .given()
        .iter()
        .map(|one| crate::says::given(*one))
        .collect();

    out.extend(held_line(trail));

    out
}

pub fn carving() -> blitzkit::texture::TextureData {
    let n = CARVED;
    let mut pixels = vec![0u8; (n * n * 4) as usize];

    // **How much of a texel the chisel covered, not whether it touched
    // it.** The alpha was nought or 255, so every edge was a pixel step
    // and a stroke running at any angle but a right one came out as a
    // staircase. Jake read the stone and said the blockiness was
    // distracting. The carved face is drawn in the blended pass, per
    // `aim::RUNE_GLAZE`, so a texel can be part covered and say so.
    //
    // The most of what is already there, because strokes meet: two
    // overlapping at a corner must take the deeper cut rather than
    // adding up to a bright spot at the joint.
    let mut ink = |at_x: i32, at_y: i32, cover: f32| {
        if at_x < 0 || at_x >= n as i32 || at_y < 0 || at_y >= n as i32 {
            return;
        }

        let at = ((at_y as u32 * n + at_x as u32) * 4) as usize;
        let now = (cover.clamp(0.0, 1.0) * 255.0).round() as u8;

        if now <= pixels[at + 3] {
            return;
        }

        for c in 0..3 {
            pixels[at + c] = 255;
        }
        pixels[at + 3] = now;
    };

    // **Measured from the line rather than stamped along it.** This
    // walked a disc of `ceil(thick * n / 2)` texels along the stroke,
    // which quantised every width to a whole number: 0.0085 and 0.0115
    // were the same line, so splitting the chisel in two changed nothing
    // and was reported as done. A width is a distance now and any value
    // of it means something.
    let mut line = |from: (f32, f32), to: (f32, f32), thick: f32| {
        let across = n as f32;
        let (ax, ay) = (from.0 * across, (1.0 - from.1) * across);
        let (bx, by) = (to.0 * across, (1.0 - to.1) * across);
        let half = thick * across * 0.5;
        let pad = half + 1.5;
        let (dx, dy) = (bx - ax, by - ay);
        let along = (dx * dx + dy * dy).max(1e-6);

        for at_y in (ay.min(by) - pad).floor() as i32..=(ay.max(by) + pad).ceil() as i32 {
            for at_x in (ax.min(bx) - pad).floor() as i32..=(ax.max(bx) + pad).ceil() as i32 {
                // the middle of the texel, against the nearest point of
                // the segment: a stroke has ends and a distance to an
                // endless line rounds its corners off into the stone
                let (mid_x, mid_y) = (at_x as f32 + 0.5, at_y as f32 + 0.5);
                let on = (((mid_x - ax) * dx + (mid_y - ay) * dy) / along).clamp(0.0, 1.0);
                let off = (mid_x - (ax + dx * on)).hypot(mid_y - (ay + dy * on));

                ink(at_x, at_y, half + 0.5 - off);
            }
        }
    };

    // the band, round the face and open at the foot the way a serpent's
    // body leaves its own tail
    for (from, to, thick) in strokes(0.0) {
        line(from, to, thick);
    }

    // and the name in letters along the foot, composited into the same
    // picture rather than hung on a second quad. **One carved face.** A
    // separate plate for the Latin is the plate this whole change is
    // getting rid of.
    // **The ribbon is haloed and the name is not, and the order here is
    // the whole of how.** `HALO_OUT` throws twenty four texels of glow
    // round every cut, which is what the runes want: nothing their
    // strokes enclose is small. The name's capitals are `LATIN_TALL` of
    // the picture, so the hole inside an O is about six texels with
    // strokes haloing inward from all four sides, and twenty four beats
    // six: the counter floods and the O reads as an F. Jake saw that and
    // I twice told him it was fixed.
    //
    // Cut after the halo rather than before it, so the name keeps its
    // holes. What it gives up is a glow skirt of its own, which it can
    // afford: it is the one thing on this face that is letters, and
    // letters are read rather than looked at.
    halo(&mut pixels, n);

    cut_in(
        &mut pixels,
        n,
        n,
        SAYS,
        LATIN_TALL,
        LATIN_FOOT,
        FACE_SQUEEZE,
    );

    blitzkit::texture::TextureData::from_pixels(n, n, pixels)
}

/// How far the halo reaches round a cut, in texels, and how strongly.
pub const HALO_OUT: i32 = 24;
pub const HALO: f32 = 0.50;

/// Spreads a soft skirt of the same colour round everything already cut,
/// in the picture's own alpha.
///
/// **Because this engine has no bloom.** A colour past one is a thing
/// lit with nothing lighting it, and that is all it is: it does not
/// bleed into the pixels round it. So turning the carving up for the
/// night only made a brighter mark, and adding white to it only made a
/// paler one. Jake's two night pictures are those two answers.
///
/// What a glow actually is on screen is the thing plus light around the
/// thing, and here the cheapest place to put the light around it is in
/// the same picture: the glyph at full alpha, and a soft skirt at part
/// alpha spreading off it in the ember's own colour. One texture, one
/// draw, and it is the glyph's own shape rather than a disc behind it.
///
/// The skirt goes under the cut and never over it: a glyph whose own
/// edge has been softened is a blurry glyph, and legibility is the one
/// thing this carving has cost the most rounds.
fn halo(pixels: &mut [u8], n: u32) {
    // **How far the nearest cut is, rather than a search round every
    // texel.** This read a disc of `HALO_OUT` either way at every one of
    // them, which is n squared times the kernel: fine at 256 across with
    // a reach of 4, and eleven hundred million operations at 1024 with a
    // reach of 16. The skirt only ever depended on the distance to the
    // nearest cut, so that is what is worked out, in two passes over the
    // picture and no kernel at all.
    //
    // Chamfered rather than exact: a step to a neighbour counts one and a
    // step to a corner counts the diagonal, which is within a few per
    // cent of the true distance and is a soft glow.
    let far = (n * n) as f32;
    let mut off: Vec<f32> = (0..(n * n) as usize)
        .map(|i| if pixels[i * 4 + 3] > 0 { 0.0 } else { far })
        .collect();
    let corner = std::f32::consts::SQRT_2;

    // down and to the right, then back up and to the left
    for down in 0..n as i32 {
        for across in 0..n as i32 {
            let at = (down * n as i32 + across) as usize;

            for (dx, dy, step) in [
                (-1, 0, 1.0),
                (0, -1, 1.0),
                (-1, -1, corner),
                (1, -1, corner),
            ] {
                let (sx, sy) = (across + dx, down + dy);

                if sx < 0 || sy < 0 || sx >= n as i32 || sy >= n as i32 {
                    continue;
                }

                off[at] = off[at].min(off[(sy * n as i32 + sx) as usize] + step);
            }
        }
    }

    for down in (0..n as i32).rev() {
        for across in (0..n as i32).rev() {
            let at = (down * n as i32 + across) as usize;

            for (dx, dy, step) in [(1, 0, 1.0), (0, 1, 1.0), (1, 1, corner), (-1, 1, corner)] {
                let (sx, sy) = (across + dx, down + dy);

                if sx < 0 || sy < 0 || sx >= n as i32 || sy >= n as i32 {
                    continue;
                }

                off[at] = off[at].min(off[(sy * n as i32 + sx) as usize] + step);
            }
        }
    }

    let reach = HALO_OUT as f32;
    let ember = crate::aim::RUNE_EMBER;

    for i in 0..(n * n) as usize {
        let near = 1.0 - (off[i] * off[i]) / (reach * reach);

        if near <= 0.0 {
            continue;
        }

        // **Laid under what is already cut rather than beside it.** The
        // skirt used to skip any texel that had alpha at all, and a cut
        // with a soft edge has alpha at all: the outermost texel of every
        // stroke sat at a fifth of a chisel with no skirt beneath it, so
        // between each glyph and its glow ran a ring of nearly nothing,
        // one texel wide and following every jag of the rasteriser.
        // Composited instead, the cut over the skirt, which is the
        // arithmetic the engine would do if they were two draws.
        //
        // One of the two things wrong with the edge, not the whole of
        // it. The other was the engine throwing away every alpha under
        // its cutout, which is spec 0047 amended the same day, and
        // which of the two Jake's picture was showing has not been
        // taken apart.
        let cut = pixels[i * 4 + 3] as f32 / 255.0;
        let skirt = near * near * HALO * (1.0 - cut);
        let both = cut + skirt;

        for (c, tint) in ember.to_array().iter().copied().enumerate() {
            // white where the chisel went and the ember's own colour
            // where the glow spreads, per `aim::RUNE_EMBER`: the canvas
            // clips at one, so a glow that is hot in the middle and
            // amber at the edge has to say so in the picture.
            pixels[i * 4 + c] = (255.0 * (cut + tint * skirt) / both).round() as u8;
        }

        pixels[i * 4 + 3] = (255.0 * both).round() as u8;
    }
}

/// How tall the lettering is asked of the font.
///
/// **The size it is going to be drawn at.** It was a flat sixty four
/// texels, laid into a block of forty seven, so the name was point
/// sampled down by a third before anything else happened to it and
/// came out of that as blocky as the strokes were. A font that is
/// asked for the right size antialiases it there.
pub const RUNE_TEXELS: f32 = CARVED as f32 * LATIN_TALL;

/// How tall the Latin name at the foot is, as a share of the carved
/// picture, and how far it sits off the bottom of it. Held rather than
/// derived, so signing the copy cannot move it.
pub const LATIN_TALL: f32 = 0.046;
pub const LATIN_FOOT: f32 = 0.012;
/// The whole length of the stone, foot and all, and how much of that is
/// under the grass.
///
/// **Jake: it has no reason to be standing that way.** A lump the shape
/// of an egg with its bottom pole exactly at ground level is an egg
/// balanced on its tip, and nothing says whether there is more of it
/// down there. The garden's set stones are the same shape and get away
/// with it because a third of each is under the gravel.
///
/// Better than a quarter of this one is, which is what a standing stone
/// needs to stay standing and is why the ones still up are still up. The
/// width at the grass line is what says so.
pub const RUNE_TALL: f32 = 3.6;
pub const RUNE_SUNK: f32 = 1.0;
pub const RUNE_WIDE: f32 = 1.6;
/// And how deep, which is the one that decides whether it looks like it
/// is about to fall over.
///
/// **Jake: it is still a really thin slab, and rocks around the base do
/// not make it look like it is not about to fall over.** Both true. At
/// 0.34 against two and a half metres standing it is a blade seen edge
/// on, and no amount of spoil round the foot buys what mass buys. The
/// stones that are still up after a thousand years are the thick ones.
pub const RUNE_THICK: f32 = 0.92;
/// How far from the stone's middle the heaped footing reaches, how high
/// it banks where it meets the stone, how many pieces it is broken into,
/// and how much grass there is between its rim and the bridge.
pub const RUBBLE_OUT: f32 = 2.15;
pub const RUBBLE_UP: f32 = 0.62;
pub const RUBBLE: usize = 160;
pub const RUBBLE_CLEARS: f32 = 0.5;

pub const RUNE_LEANS: f32 = 0.06;
/// How far the stone stands off the deck, measured from the deck's edge
/// on one side and from the deck's foot on the other.
///
/// **Which is the footing's reach and not the stone's.** It was 1.4,
/// back when the footing was a handful of stones that could be put
/// wherever there was room. A heaped bank has a footprint of its own,
/// and at 1.4 its edge came within two centimetres of the ramp's
/// corner. None of it is solid, so touching the ramp is rubble you walk
/// through. The clearance the stone needs is the clearance the footing
/// needs, and it is written here once and used for both directions.
///
/// Conservative, because the sum is measured along one axis while the
/// thing it has to miss is a half strip: the shortest line to the ramp
/// is a diagonal, so this leaves about 1.7 metres of grass where
/// `RUBBLE_CLEARS` only asks for half of one. Half a metre is the
/// requirement and `the_bank_keeps_off_the_bridge` is what holds it;
/// this is one way of satisfying it and not the tightest.
pub const RUNE_ASIDE: f32 = RUBBLE_OUT + RUBBLE_CLEARS;

/// How high the heaped footing stands a share `share` of the way from
/// the stone's foot to its rim, and how ragged its edge is at `round`.
///
/// One function because the bank and the loose pieces lying on it have
/// to agree: a stone floating a hand over the gravel is the fault that
/// comes of working them out twice.
pub fn mound_high(share: f32) -> f32 {
    RUBBLE_UP * (1.0 - share.clamp(0.0, 1.0)).powf(1.6)
}

pub fn mound_rim(round: f32) -> f32 {
    // **No one wave allowed to carry it.** At 0.11 the three-wave did,
    // and three big lobes round a low bank is the outline of a spill
    // rather than a heap. Spread over more waves, none of them large,
    // the edge wanders without taking a shape.
    //
    // The shares add to one, so the whole wander is `RUBBLE_RAGGED` and
    // the furthest the bank can reach is a number rather than a guess.
    // It has to be: the clearance from the bridge is worked out from it.
    let wobble = (round * 3.0).sin() * 0.26
        + (round * 7.0 + 1.9).sin() * 0.33
        + (round * 13.0 + 0.4).sin() * 0.24
        + (round * 23.0 + 2.6).sin() * 0.17;

    1.0 + wobble * RUBBLE_RAGGED
}

/// How far past `RUBBLE_OUT` the bank's edge can wander.
pub const RUBBLE_RAGGED: f32 = 0.21;

/// And so the furthest anything in the footing can be from the stone.
pub fn rubble_reaches() -> f32 {
    RUBBLE_OUT * (1.0 + RUBBLE_RAGGED)
}

/// The bank of spoil the stone is set into, as one skin.
///
/// **Because a scatter of pebbles on flat grass is a scatter of pebbles
/// on flat grass.** The first go at this was ninety loose stones in a
/// ring, and from thirty metres it read as a ring: you could see the
/// grass between them and the ground under them was dead level. What
/// says a stone was dug in and packed is that the ground itself is
/// raised round it. So the ground is raised, as a surface, and the loose
/// pieces lie on that rather than standing in for it.
///
/// Built in metres about the stone's middle, with its rim wandering, and
/// faceted, because a smooth dome of earth is a smooth dome of earth.
pub fn mound_mesh() -> blitzkit::mesh::MeshData {
    use std::f32::consts::TAU;

    let (hug_x, hug_z) = foot_half();

    crate::garden::faceted(blitzkit::mesh::MeshData::surface(
        MOUND_ROUND,
        MOUND_STEPS,
        |u, v| {
            let round = -u * TAU;
            let (cos, sin) = (round.cos(), round.sin());
            // from under the stone's own foot, so there is no seam of
            // grass showing between the bank and what it banks against
            let inner = (hug_x * cos).hypot(hug_z * sin) * 0.86;
            let rim = RUBBLE_OUT * mound_rim(round);
            let out = inner + v * (rim - inner).max(0.0);
            // and it thins to nothing at the rim rather than ending in a
            // step, with the surface itself broken up on the way down
            let rough = ((u * TAU * 9.0).sin() * (v * 11.0).sin()) * 0.035 * (1.0 - v);

            vec3(cos * out, mound_high(v) + rough, sin * out)
        },
    ))
}

pub const MOUND_ROUND: u32 = 112;
pub const MOUND_STEPS: u32 = 14;

/// The loose pieces lying on that bank: where each sits, how big it is,
/// and which way round.
///
/// Finest in close where it was packed and trodden, coarsest at the rim
/// where it was thrown clear, and every one of them sitting down into
/// the bank rather than resting on top of it.
///
/// A hundred and sixty pieces, which is cheap: these are the garden's
/// own eleven by seven lump, not the hundred and seventy six by two
/// hundred and twenty four the standing stone is. Seven of *those* put
/// more geometry in the footing than in the monument.
pub fn rubble(reaches: f32, _trail: &Trail) -> Vec<(Vec3, Vec3, f32)> {
    // **And it is there from the first minute, like the stone it packs.**
    // Spec 0118. The footing appearing the moment somebody finishes the
    // building is the fault `shades` already names one object along: a
    // clearing that arrives when you finish is trees vanishing.

    let (at, _) = runestone_at(reaches);
    let (hug_x, hug_z) = foot_half();

    let mut state = 0x5EED_u32;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;

        state
    };
    let roll = |v: u32| (v % 10_000) as f32 / 10_000.0;
    let mut out = Vec::with_capacity(RUBBLE);

    let (slab, _) = slab_at();

    // **Drawn again rather than dropped.** A piece that lands on the
    // ledger is not thrown away: the heap is `RUBBLE` pieces and
    // `the_rubble_banks_against_the_stone_and_thins_outward` says so,
    // which is what catches a generator quietly making fewer. The bound
    // is there because a roll that can never land clear would otherwise
    // spin for ever, and it cannot happen: the ledger is a tenth of the
    // heap's area.
    let mut tries = 0;

    while out.len() < RUBBLE && tries < RUBBLE * 20 {
        tries += 1;

        let round = roll(next()) * std::f32::consts::TAU;
        let (cos, sin) = (round.cos(), round.sin());

        let inner = (hug_x * cos).hypot(hug_z * sin) * 0.86;
        // **At its own direction, not the opposite one.** Written
        // `-round` the pieces read a mirrored rim, so round half the
        // heap they lay out past the bank's edge and round the other
        // half they stopped short of it.
        let rim = RUBBLE_OUT * mound_rim(round);
        let share = roll(next()).powf(1.2);
        let off = inner + share * (rim - inner).max(0.0);

        // **Coarse at the collar and fine at the rim, which is the
        // way round it was not.** Thrown-clear spoil at the edge and
        // gravel in close is one real thing, but the thing that ties a
        // raised stone to the ground is the packing: the wedges driven
        // in round the collar to hold it plumb. Those are the biggest
        // pieces on the site and they are the ones against the stone.
        let across = (0.30 - share * 0.20) * (0.7 + roll(next()) * 0.8);
        // sitting into the bank at that radius, by about a third of
        // itself, which is what stops them reading as dropped
        let up = mound_high(share) - across * 0.34;

        // **And not on the ledger.** Spec 0118 laid a slab in this heap
        // and the heap did not know: a piece came to rest on it and sat
        // across the middle of a word. The same fault as grass through
        // the bank's gravel, which `field::cover` learned the same day.
        let (from_x, from_z) = (cos * off - slab.x, sin * off);

        if from_x.abs() < SLAB_LONG * 0.5 + across && from_z.abs() < SLAB_WIDE * 0.5 + across {
            continue;
        }

        out.push((
            vec3(at.x + cos * off, up, at.z + sin * off),
            vec3(
                across,
                across * (0.45 + roll(next()) * 0.4),
                across * (0.8 + roll(next()) * 0.6),
            ),
            roll(next()) * std::f32::consts::TAU,
        ));
    }

    out
}

/// Gjallarhorn, in the watchman's hand. Spec 0055.
///
/// **The horn that has not been blown.** It is the one thing in the
/// myth that is only ever about to be used: Heimdall holds it through
/// the whole of the world's history and sounds it once, at the end. So
/// it hangs at his side with the bell forward and down rather than
/// being raised to his lips. A horn at the mouth is a warning already
/// given, and nothing about the walk up here should say the end has
/// come.
///
/// Built as a circle swept along an arc, widening from the mouthpiece
/// to the bell. Flat shaded with the rest of him, so it reads as a
/// carved horn rather than a brass instrument.
/// **Short and heavy, not long and thin.** At three quarters of a
/// metre of spine with an eighth of that at the bell it came out a
/// blade: from the side it read as a scythe hanging off his hand. A
/// horn is a stubby thing. Shorter, fatter at the mouth and curled
/// harder, it is a horn from any angle.
pub const HORN_LONG: f32 = 0.52;
pub const HORN_CURLS: f32 = 1.5;
pub const HORN_TIP: f32 = 0.028;
pub const HORN_BELL: f32 = 0.125;
/// How big it is in his hand. **Bigger than life**, because an ox horn
/// at true size on a figure at the top of a hundred metre bridge is a
/// stick, and this is the object the last of the trail is named for.
pub const HORN_HELD: f32 = 1.35;
pub const HORN_ROUND: u32 = 11;
pub const HORN_STEPS: u32 = 20;

/// Where the horn's spine is a share `along` of the way to the bell,
/// which way it is pointing there, and how thick it is.
pub fn horn_at(along: f32) -> (Vec3, Vec3, f32) {
    let turn = along * HORN_CURLS;
    let spine = vec3(
        HORN_LONG * turn.sin() / HORN_CURLS,
        HORN_LONG * (turn.cos() - 1.0) / HORN_CURLS,
        0.0,
    );
    let ahead = vec3(turn.cos(), -turn.sin(), 0.0);
    // **Cubed rather than tapered straight.** An ox horn is nearly a
    // point for its first third and nearly all bell in its last
    // quarter, and a straight cone between the two ends is a traffic
    // cone whichever way round you put it.
    let thick = HORN_TIP + (HORN_BELL - HORN_TIP) * along.powf(2.3);

    (spine, ahead, thick)
}

pub fn horn_mesh() -> blitzkit::mesh::MeshData {
    use std::f32::consts::TAU;

    crate::garden::faceted(blitzkit::mesh::MeshData::surface(
        HORN_ROUND,
        HORN_STEPS,
        |u, v| {
            let (spine, ahead, thick) = horn_at(v);
            let round = -u * TAU;
            // the circle standing across the spine. `ahead` turns in
            // the x y plane, so across it is that turned a quarter,
            // and z is the one axis the arc never leaves.
            let side = vec3(-ahead.y, ahead.x, 0.0);

            spine + (side * round.cos() + Vec3::Z * round.sin()) * thick
        },
    ))
}

/// The stone at the foot of the bridge: where it stands and how big.
///
/// **Always, and the `Option` is what is left of when it was not.** The
/// comment here said "empty until the bridge is there" and "a stone
/// naming a bridge that does not exist is a grave marker", which spec
/// 0118 overturned in as many words: the stone stands from the first
/// minute, like the deck's foot, because a stone appearing the moment
/// somebody finishes the building is worse to watch than one that was
/// always there. The code has returned `Some` unconditionally since, and
/// the reason given for the opposite sat above it for two days.
///
/// The trail goes unread for the same reason. Kept rather than taken out
/// because `tests::the_stone_stands_before_the_first_answer` is spec
/// 0118's own acceptance criterion and asks this with two different
/// trails: dropping the argument would make that test true by its own
/// shape and prove nothing.
pub fn runestone(reaches: f32, _trail: &Trail) -> Option<(Vec3, Vec3)> {
    Some(runestone_at(reaches))
}

/// What the building is still holding, which is the first question in
/// `Asked::every` order that has not been given. Spec 0118.
///
/// **One, never a list.** Spec 0036 forbids a panel of ticks and forbids
/// "3 of 8 found", and a list of what is left is the same thing written
/// out in words. The order is the one `trail::Asked` is written in, which
/// as of 2026-11-18 is a curve rather than an accident: see the note on
/// that enum, and `trail::tests::nothing_is_asked_for_before_its_room_can_be_reached`
/// for the two orderings the building fixes rather than taste.
///
/// Nothing once every one has been given, and then the stone is the stone
/// it always was.
pub fn still_held(trail: &Trail) -> Option<crate::trail::Asked> {
    crate::trail::Asked::every()
        .iter()
        .find(|one| !trail.given().contains(one))
        .copied()
}

/// And the line it cuts into the stone's face for that.
pub fn held_line(trail: &Trail) -> Option<&'static str> {
    still_held(trail).map(crate::says::still_held)
}

/// Whether a spot in the field is one the bridge keeps clear of trees.
///
/// **Because the field's scatter has never known the bridge is there.**
/// `field::open` keeps trees out of the building and out of nothing else,
/// so one grew through the runestone, and the low stretch of the deck runs
/// through the canopy of anything sown under it.
///
/// **Always, and not only once the bridge is up.** A clearing that appears
/// the moment somebody finishes the building is trees vanishing, which is
/// worse to watch than trees in a rainbow.
///
/// **And only the low stretch.** Past the point where the deck clears a
/// tree's head there is nothing to grow through, and a six metre avenue
/// running the whole hundred is a lane pointing at the one thing spec 0036
/// says nobody should be able to tell is there. This reaches as far as the
/// deck is lower than a tree and no further, which is about a fifth of it.
pub fn shades(reaches: f32, at: Vec3) -> bool {
    let foot = foot(reaches);
    let (stone, size) = runestone_at(reaches);

    // round the stone. **Its own test and not an else.** Written as the
    // far branch of how far off the deck's line a spot is, the stone fell
    // on the near side of that and so was never asked about: it sits four
    // and a half metres off a line whose own clearance is five.
    // measured off the heap and not the stone, which is the smaller
    // of the two now that the stone is set in a bank
    let footing = rubble_reaches();

    if (at.x - stone.x).abs() < (size.x * 0.5).max(footing) + CLEARS
        && (at.z - stone.z).abs() < (size.z * 0.5).max(footing) + CLEARS
    {
        return true;
    }

    // or under the low stretch of the deck
    if (at.z - foot.z).abs() > WIDE * 0.5 + CLEARS {
        return false;
    }

    let along = (foot.x - at.x) / RUN;

    (0.0..=1.0).contains(&along) && high(along) < crate::field::TREE_TALL
}

/// How much faster you get about while you are on the deck.
///
/// **Because the bridge is a road and not a hill.** Jake: it takes way
/// too long to run up the rainbow bridge. A hundred metres of span and
/// twenty six of climb is eleven or twelve seconds of holding one key
/// with nothing to do, every time, and the span is the thing that makes
/// it read from the far side of the field. Shortening it fixes the
/// climb by spoiling what the climb is for.
///
/// So the deck carries you. Bifröst is the road the gods ride to the
/// well every day, and a road that carries its traffic is the most
/// ordinary thing a mythic road could do.
///
/// **And the budget was the bigger half of it.** `walk::CLIMBS`
/// rations height, so twenty six metres of rise was floored at eleven
/// seconds whatever the pace, and the carry alone only made you walk
/// into risers faster. `walk::RAMPS` is the other half and the one
/// that mattered: the climb is three seconds at a walk now, measured
/// through the game's own loop by
/// `tests::the_bridge_is_walked_up_rather_than_climbed`.
///
/// **Only going up.** Coming down you have gravity and a view, and a
/// bridge that flings you back towards the building is a bridge
/// fighting you.
/// **Jake, twice: still laborious.** 1.8 was about five seconds at a
/// walk, which was my judgement of what a ride should feel like
/// against his of what it actually felt like. His wins.
pub const CARRIES: f32 = 2.8;

/// Whether a spot is on the deck, for the purpose of being carried.
///
/// Measured off the same `treads` the walking is done on, rather than
/// off the deck's drawn shape: two answers to where the bridge is would
/// come apart the first time either moved.
pub fn carries(reaches: f32, at: Vec3, going: Vec3, far: f32) -> f32 {
    let foot = foot(reaches);

    if (at.z - foot.z).abs() > WIDE * 0.5 || at.y < 0.4 {
        return 1.0;
    }

    let along = (foot.x - at.x) / RUN;

    // **Only over deck that is there.** Spec 0117: past the cut end there is
    // nothing to be carried up, and a carry that reached further would hold
    // you walking on air.
    if !(0.0..=far).contains(&along) {
        return 1.0;
    }

    // and only while you are going up it, faded rather than switched.
    //
    // **A hard test here was its own jitter.** `going` is where the
    // camera looks, not where the feet go, so walking up the deck while
    // glancing to one side crossed `going.x == 0` and the carry
    // snapped on and off with the mouse. Faded, the worst a glance does
    // is slow you.
    let up = (-going.x).clamp(0.0, 1.0);

    1.0 + (CARRIES - 1.0) * up
}

/// How far from either a tree has to be sown.
///
/// **A canopy and not a trunk.** The field's trees are `TREE_TALL` high
/// and their crowns are about half that across, so a metre and a half of
/// clearance is a tree whose trunk misses the stone and whose branches sit
/// over it, which is what the first go at this drew.
pub const CLEARS: f32 = 3.0;

/// Where the stone stands, whether or not the bridge is up.
///
/// The clearing round it has to hold either way, so the place it stands is
/// its own question and [`runestone`] is that place plus the bridge.
fn runestone_at(reaches: f32) -> (Vec3, Vec3) {
    let foot = foot(reaches);

    (
        vec3(
            // behind the deck's foot by the same clearance it stands
            // aside by, because in both directions what has to miss the
            // bridge is the rubble's rim and not the stone. This was
            // `RUNE_WIDE`, which only ever agreed with the right answer
            // by accident.
            foot.x + RUNE_ASIDE,
            RUNE_TALL * 0.5 - RUNE_SUNK,
            foot.z + WIDE * 0.5 + RUNE_ASIDE,
        ),
        vec3(RUNE_THICK, RUNE_TALL, RUNE_WIDE),
    )
}

/// Where the foot of it stands, in the field.
pub fn foot(reaches: f32) -> Vec3 {
    let out = crate::field::starts(reaches) - OUT;
    let (from, to) = crate::field::way(reaches);

    vec3(out, 0.0, (from + to) * 0.5)
}

/// How high the deck is a share `along` of the way out.
///
/// **It eases out of the ground and eases into the top**, so the steepest
/// part is the middle and both ends are nearly flat. The first shape tried
/// here was a quarter circle, which rises fastest where it leaves the
/// ground: at the numbers above it went up 1.29 in its first 1.46, which is
/// a wall to walk at. `it_leaves_the_ground_at_a_walkable_angle` is that
/// mistake, kept.
pub fn high(along: f32) -> f32 {
    let along = along.clamp(0.0, 1.0);
    let climbing = (along / LEVELS_AT).min(1.0);

    RISE * (1.0 - (climbing * std::f32::consts::PI).cos()) * 0.5
}

/// How steeply it climbs there, as a rise over the run.
///
/// The slope of [`high`], which is what a drawn piece is tilted by.
pub fn pitch(along: f32) -> f32 {
    let along = along.clamp(0.0, 1.0);

    if along >= LEVELS_AT {
        return 0.0;
    }

    let climbing = along / LEVELS_AT;

    RISE * std::f32::consts::PI * (climbing * std::f32::consts::PI).sin() / (2.0 * RUN * LEVELS_AT)
}

/// The kerb down either side of the ramp: how high it stands, how
/// thick it is, and how long each piece is.
///
/// **Because the deck had no edge at all.** Six metres across, a
/// hundred long, and nothing to either side of it. At four metres a
/// second that was survivable; carried at eleven it is not, because
/// three degrees off the straight is half a metre a second sideways
/// and the whole climb is five seconds of it. Jake went over the side
/// near the top and his recording catches him down in the field
/// looking back up at the bridge.
///
/// So the road gets a verge, which is the other thing that makes a
/// road a road. Higher than `walk::STEP`, so it stops you rather than
/// being walked up, and the two of them leave the deck's middle five
/// metres clear, which is wider than anything else you walk along in
/// this building.
///
/// Its pieces are ten treads long. Nothing stands on a kerb, so it
/// does not need the fine tread the deck needs to be smooth, and at
/// `TREAD` it would have been seventeen hundred boxes for a thing
/// whose whole job is to be in the way sideways.
pub const KERB_UP: f32 = 0.52;
pub const KERB_THICK: f32 = 0.5;
pub const KERB_LONG: f32 = TREAD * 10.0;
/// How far the drawn kerb stands outside the deck's edge, which is
/// only so that no face of it lies in a plane the deck already has.
pub const KERB_PROUD: f32 = 0.04;

/// The kerb as one swept ribbon per side, for looking at.
///
/// **Drawn as a sweep and collided as boxes.** `kerbs` is a row of
/// axis aligned blocks because that is the only thing the walk
/// understands, and drawing those blocks is a flight of stairs running
/// up either side of a rainbow: each piece a foot and a half longer
/// than the last is level on top, so the kerb went up in steps of half
/// a metre. Swept along the same curve the deck is swept along, it is
/// a line.
pub fn kerb_mesh(reaches: f32, side: f32, reach: f32) -> blitzkit::mesh::MeshData {
    let foot = foot(reaches);
    let middle = foot.z + side * (WIDE * 0.5 - KERB_THICK * 0.5);
    let half = KERB_THICK * 0.5;
    // **The verge stops where the deck does.** Spec 0117. It did not, and
    // the two kerbs ran the whole hundred metres out over a deck a sixth of
    // that: two thin arcs climbing out of the field and over, with nothing
    // under them, which is what a picture of an empty trail showed.
    let reach = reach.clamp(0.0, 1.0);

    let mut mesh = blitzkit::mesh::MeshData::surface(PIECES as u32, 3, |u, v| {
        let along = u * reach;
        let on = high(along);
        let x = foot.x - along * RUN;
        // round the kerb's cross section: down the outer face, over the
        // top, down the inner face
        // **Clear of the deck's own faces.** Standing exactly on the
        // deck's edge, the kerb's outer face shared a plane with it and
        // its underside shared one with the deck's underside, and both
        // came out speckled: the fault this building has found eight
        // times. Nudged out and cut short, neither touches.
        let out = middle + half + KERB_PROUD;
        let under = on - DEEP * 0.55;
        let (z, y) = match v {
            v if v < 0.34 => (out, under),
            v if v < 0.67 => (out, on + KERB_UP),
            v if v < 0.99 => (middle - half, on + KERB_UP),
            _ => (middle - half, under),
        };
        vec3(x, y, z)
    });
    mesh.compute_normals();

    // **And its `u` says which span it is on, not where it is along.**
    // Spec 0118: the kerb over each span wears the colour of the room that
    // paid for that span, and `spans` is the strip of those colours. A
    // surface's own `u` runs nought to one down the whole ribbon, so it is
    // replaced with the middle of the span's own slot: whole slots, never
    // a gradient, because a kerb that fades from one room into the next is
    // a progress bar and the colours are meant to be countable.
    for vertex in &mut mesh.vertices {
        let along = vertex.uv[0] * reach;
        let span = (along * SPANS as f32).floor().min(SPANS as f32 - 1.0);

        vertex.uv[0] = (span + 0.5) / SPANS as f32;
        vertex.uv[1] = 0.5;
    }

    mesh
}

/// How many spans the deck grows in, which is one more than the building
/// asks.
///
/// **One more**, because the foot is there before anything is answered:
/// spec 0117's promise at the beginning is a span nobody paid for.
///
/// **A number and not `Asked::every().len() + 1`**, because a texture's
/// width and a mesh's `u` both want it and neither can call a function at
/// compile time. What keeps it honest is
/// `tests::there_is_a_slot_for_every_span`, the same device
/// `trail::tests::every_one_of_them_is_in_every` uses on the list itself.
pub const SPANS: usize = 6;

/// The strip of colours the kerbs are drawn through: one slot a span, in
/// the order the rooms answered. Spec 0118.
///
/// **The kerbs and not the deck.** Bifröst is a rainbow and a span of it
/// in mahogany is not a rainbow bridge, so the deck is the one surface
/// here that cannot carry this. The kerb is already a ribbon down either
/// side in one flat colour and is what you look along for the whole climb.
///
/// **One picture and one draw**, rather than a mesh a span. The kerb is
/// two meshes and stays two meshes; what changed is that its `u` points at
/// a slot in this strip instead of running down its length.
///
/// The first slot is nobody's, per `spans`, and keeps `aim::KERB`. A slot
/// past what has been answered keeps it too, so the unbuilt end of the
/// ribbon is the colour the whole thing used to be.
pub fn span_colours(trail: &Trail) -> blitzkit::texture::TextureData {
    let mut pixels = Vec::with_capacity(SPANS * 4);

    for slot in 0..SPANS {
        // slot nought is the span that was there before anything was
        // asked, and after that each slot is the room that answered
        let paid = slot
            .checked_sub(1)
            .and_then(|nth| trail.given().get(nth))
            .map(|one| room_colour(*one))
            .unwrap_or(crate::aim::KERB);

        for c in 0..3 {
            pixels.push((paid[c].clamp(0.0, 1.0) * 255.0).round() as u8);
        }

        pixels.push(255);
    }

    blitzkit::texture::TextureData::from_pixels(SPANS as u32, 1, pixels)
}

/// What colour a room wears on the bridge.
///
/// **Taken from the room's own material wherever it has one**, so nothing
/// here is a second opinion about what a room looks like: the cellar wears
/// its bookcase, the nook its benches, the gallery its plinths, the baths
/// their water, the field its leaves. Two have no one material to take:
/// the space behind the wall is a room spec 0011 gives no voice and keeps
/// dark, and the sun is a light rather than a surface, so those two take
/// the nearest thing the building already names.
///
/// **A match and not a table**, so a station added to `trail::Asked` stops
/// this compiling rather than quietly wearing the kerb's own grey.
///
/// **Lifted to one brightness, so what tells them apart is hue.** Taken
/// straight, the kerb ran from (10, 10, 25) to (26, 31, 42) out of 255
/// down its whole length: the hues were right and nothing was legible,
/// because these colours were chosen for walls and benches seen across a
/// room and this is a hand's width of edge seen from a hundred metres.
/// Lifting them keeps each room's own hue and takes away the part of the
/// colour that was about being indoors.
pub fn room_colour(what: crate::trail::Asked) -> glam::Vec4 {
    let own = room_colour_own(what);
    let most = own.x.max(own.y).max(own.z).max(1e-4);
    let lift = KERB_LIT / most;

    glam::vec4(own.x * lift, own.y * lift, own.z * lift, 1.0)
}

/// How bright a room's colour is painted on the kerb, whatever it is
/// indoors.
pub const KERB_LIT: f32 = 0.78;

/// And the room's colour as the room itself wears it.
fn room_colour_own(what: crate::trail::Asked) -> glam::Vec4 {
    use crate::trail::Asked;

    match what {
        Asked::OpenedTheCase => crate::aim::SHELF,
        Asked::WadedInThePool => {
            let water = crate::aim::WATER;

            glam::vec4(water.x, water.y, water.z, 1.0)
        }
        Asked::SolvedTheCube => crate::aim::BENCH,
        Asked::GotBehindTheWall => crate::aim::EDGE_STONE,
        Asked::RodeToTheSun => {
            let sun = crate::aim::SUN;

            glam::vec4(sun.x, sun.y, sun.z, 1.0)
        }
    }
}

/// Where each piece of kerb sits: its box, in the same terms as
/// [`treads`].
/// The same, stopping where the deck does. Spec 0117.
pub fn kerbs(reaches: f32, reach: f32) -> Vec<Aabb> {
    let foot = foot(reaches);
    let reach = reach.clamp(0.0, 1.0);
    let pieces = (RUN / KERB_LONG).ceil() as usize;
    let mut out = Vec::with_capacity(pieces * 2);

    for piece in 0..pieces {
        let from = piece as f32 * KERB_LONG / RUN;

        if from >= reach {
            break;
        }

        let to = ((piece + 1) as f32 * KERB_LONG / RUN).min(reach);
        let top = high(to).max(high(from)) + KERB_UP;

        for side in [-1.0f32, 1.0] {
            let middle = foot.z + side * (WIDE * 0.5 - KERB_THICK * 0.5);

            out.push(Aabb::new(
                vec3(
                    foot.x - to * RUN,
                    top - KERB_UP - DEEP,
                    middle - KERB_THICK * 0.5,
                ),
                vec3(foot.x - from * RUN, top, middle + KERB_THICK * 0.5),
            ));
        }
    }

    out
}

/// What holds you up: one box the full width, every [`TREAD`] along.
///
/// **A different list from the drawn deck, and the reason is the engine.**
/// A collider is an axis-aligned box and cannot tilt, so a slope has to be
/// a staircase to walk on, and a staircase fine enough to walk is far too
/// fine to draw as boxes. Both come out of `high`, which is the one place
/// the shape is written.
/// The same, stopping where the deck does. Spec 0117.
///
/// **What holds you up ends where what is drawn ends**, or an unfinished
/// bridge has invisible deck past its own cut end and you walk out into the
/// air on nothing. The two come off the same reach for that reason, the way
/// they already come off the same `high`.
pub fn treads(reaches: f32, reach: f32) -> Vec<Aabb> {
    let foot = foot(reaches);
    let reach = reach.clamp(0.0, 1.0);
    let steps = (RUN / TREAD).ceil() as usize;
    let mut out = Vec::with_capacity(steps);

    for step in 0..steps {
        let from = step as f32 * TREAD / RUN;

        if from >= reach {
            break;
        }

        let to = ((step + 1) as f32 * TREAD / RUN).min(reach);
        let top = high(to).max(high(from));

        out.push(Aabb::new(
            vec3(foot.x - to * RUN, top - DEEP, foot.z - WIDE * 0.5),
            vec3(foot.x - from * RUN, top, foot.z + WIDE * 0.5),
        ));
    }

    out
}

/// The middle of the deck's top surface, every piece of the way along.
///
/// **The one account of where the deck is**, which the mesh is built from
/// and the tests read. It is the same curve [`treads`] steps, so what you
/// see and what holds you up are two readings of [`high`] and not two
/// shapes.
/// The same, stopping where the bridge has got to. Spec 0117.
///
/// **The cut end is put exactly at the reach**, rather than at the last
/// whole piece it passed. `PIECES` is 48 over a hundred metres, so rounding
/// to one would jump the end two metres at a time while a span winds in,
/// which is a thing lurching rather than growing.
pub fn stations(reaches: f32, reach: f32) -> Vec<Vec3> {
    let foot = foot(reaches);
    let reach = reach.clamp(0.0, 1.0);
    let at = |along: f32| vec3(foot.x - along * RUN, high(along), foot.z);
    let last = (PIECES as f32 * reach).floor() as usize;
    let mut out: Vec<Vec3> = (0..=last.min(PIECES))
        .map(|n| at(n as f32 / PIECES as f32))
        .collect();
    let end = at(reach);

    // and the end itself, unless the last whole piece already landed on it
    if !out.last().is_some_and(|one| (one.x - end.x).abs() <= 1e-4) {
        out.push(end);
    }

    out
}

/// What every bit of the deck is multiplied by.
///
/// The rainbow's own colours live in [`rainbow`], which is a picture and so
/// cannot hold anything past one. This carries the glow that the picture
/// cannot.
pub fn glow() -> Vec4 {
    Vec4::new(GLOW, GLOW, GLOW, 1.0)
}

/// The whole deck as one ribbon, swept along the arc.
///
/// **One mesh and not forty-eight boxes.** It was boxes, each tilted to the
/// pitch where it lay and lapping the next, and that worked for exactly as
/// long as every face of one was a single flat colour: a join you cannot
/// see is a join nobody minds. The moment the rainbow became a picture laid
/// across the width, every one of those joins turned into a bright chevron
/// and every end cap into a stripe, because an end cap wears the picture
/// too. Shrinking the lap only swapped chevrons for seams.
///
/// A ribbon has no joins to show. It is also one instance rather than
/// forty-eight, and its top surface passes through [`high`] at every
/// station rather than near it.
///
/// **Built for a given building rather than scaled to one.** The pitch
/// changes along the arc, so a unit mesh stretched to length is a straight
/// plank. `garden::trunk_mesh` is built per tree for the same reason.
///
/// **Its own faces, each with its own normal.** Four strips run the length
/// of it and the corners between them are sharp, so the vertices are not
/// shared across a corner: averaged round one, the top of a rainbow catches
/// the same light as its edge and the whole thing reads as a hose.
pub fn deck_mesh(reaches: f32, reach: f32) -> blitzkit::mesh::MeshData {
    ribbon(reaches, false, reach)
}

/// How high the deck has to be before the earth beside it stops.
///
/// **2.2 is over your head**, which is the whole point: the bank runs
/// exactly as far as the stretch you cannot pass, and stops where you can
/// walk under the deck instead.
pub const BANK_CLEAR: f32 = 2.2;

/// How far the bank spreads to either side for every metre it stands.
///
/// **0.7, and it was 1.5.** Earth left to itself lies at about thirty-five
/// degrees, which is a spread of one and a half, and that is what this was
/// built at. The bank stands 1.22 at its tallest, so one and a half put
/// 1.83 metres of gravel down either side of a six metre deck: nine and a
/// half metres across in all, better than half of it gravel, which Jake
/// read off the picture as a gravel road. Earth packed against a structure
/// and held by it stands steeper than earth tipped out of a barrow, so
/// this is steeper: 0.86 a side, and the bridge is the wider thing again.
pub const BANK_SLOPE: f32 = 0.7;

/// How far in from that spread the bank's edge wanders, as a share of it.
///
/// **A straight taper is a kerb.** Narrowing it left the outline a pair of
/// dead straight lines converging on the foot, which is a civil engineering
/// drawing and not a heap of spoil. The edge wanders instead, the way
/// `mound_rim` wanders round the runestone's footing.
///
/// Inward only, so the drawn toe is never outside the boxes that stop you:
/// the bank's whole claim is that what stops you is something you can see.
pub const BANK_RAGGED: f32 = 0.34;

/// How far under the field it is sunk, so no edge of it stands proud of
/// ground that is not quite level.
pub const BANK_UNDER: f32 = 0.6;

/// And how far clear of the deck's underside its top stops.
///
/// **Two faces in one plane is this building's oldest fault.** It has been
/// stripes round the pool, a chequerboard down a wall, a border sunk into a
/// floor, a sign on panelling, the sky in patches, a floor over a garden,
/// the underside of a neon can, and now a bank whose top was the soffit's
/// own plane to the millimetre, drawing as grey bars across the rainbow
/// that slid about as you walked.
///
/// A hand's breadth. The gap is under the deck and in its shadow, so
/// nothing is seen through it, and it is far more than any depth buffer
/// needs to tell the two apart.
pub const BANK_SHY: f32 = 0.08;

/// How deep the gravel lies where it is only a bed and not a bank.
///
/// A hand's depth, and under `crate::walk::RIDES`, so the edge of the
/// apron is walked over rather than stepped up: a threshold you cross
/// without noticing you crossed it, which is what a threshold is.
pub const BANK_LEAST: f32 = 0.07;

/// How far past the deck's edge the gravel lies wherever it lies at all.
///
/// **So the bed does not pinch to nothing at the foot.** Spread by height
/// alone the apron would be five centimetres wide where the bank is a
/// hand deep, which is a kerb and not a bed of gravel. This is the width
/// it has before any of the slope's own spread is added.
pub const BANK_APRON: f32 = 0.6;

/// And how far in front of the foot it runs, so the ramp comes out of
/// gravel rather than beginning in the middle of a field.
pub const BANK_AHEAD: f32 = 3.0;

/// And how much of that it spends coming up out of the field, so its
/// front edge is a bed thinning away rather than a cut across the grass.
pub const BANK_EDGE: f32 = 1.4;

/// How far the deck's underside has to stand over the field before the
/// bed of gravel goes under it rather than lying on the ground.
pub const BANK_ONTO: f32 = 0.25;

/// What share of the bank's height is spent easing it back into the field.
pub const BANK_DIES: f32 = 0.34;

/// How many steps across the bank's section, which wants enough to break
/// its slope into facets rather than one flat cheek a side.
pub const BANK_ACROSS: u32 = 16;

/// And how many along, over the bank's own stretch and not the whole run.
///
/// **Because the first build was stretched over the hundred metres.** The
/// surface ran `PIECES` steps from the foot to the far end of the bridge
/// while everything it drew was in the first ten, so the bank itself got
/// five of those steps: a ten metre heap in five facets, which is why it
/// read as poured rather than piled. Its own stretch, its own steps.
pub const BANK_ALONG: u32 = 40;

/// How long a collider box is along the climb, and how many bands wide the
/// wedge is to either side.
///
/// **A metre of terrace is a terrace.** `walk` takes a rise under
/// [`crate::walk::RIDES`] as ground rather than a step, which is a tenth of
/// a metre, and the bank's steepest stretch climbs a quarter of a metre for
/// every metre along. Three tenths of a metre a box keeps every riser under
/// that, so the wedge is walked and not climbed.
pub const BANK_LONG: f32 = 0.3;
pub const BANK_BANDS: usize = 3;

/// Where the bank begins and ends, as shares along the run.
///
/// It starts where the deck's underside leaves the ground and stops where
/// the deck clears your head, which are the two heights the bank is for.
/// **Said once**, because the drawing and the boxes that stop you both want
/// it and two lists of one thing drift.
pub fn bank_runs() -> (f32, f32) {
    (-BANK_AHEAD / RUN, along_at(BANK_CLEAR))
}

/// How far along the run the deck stands `y` high: the inverse of [`high`].
pub fn along_at(y: f32) -> f32 {
    let cos = (1.0 - 2.0 * y / RISE).clamp(-1.0, 1.0);

    (LEVELS_AT * cos.acos() / std::f32::consts::PI).clamp(0.0, 1.0)
}

/// The earth the bridge comes out of. Spec 0117.
///
/// **Because a ramp rising out of a field has a stretch that is a wall.**
/// Jake walked at the deck from the side on 2026-11-18 and stopped dead:
/// where the deck is between knee and head height its edge is a floating
/// slab, too high to step over and too low to duck under, and there is
/// nothing to say so. The curve cannot fix it, since `high` eases out of
/// the ground on purpose and the shape that leaves it sooner is the one
/// `it_leaves_the_ground_at_a_walkable_angle` exists to keep out: steep
/// enough to pass under quickly is steep enough to be a wall to walk up.
///
/// So the stretch stays and is given a reason. The earth is banked against
/// both sides and fills the gap under the deck, so the bridge grows out of
/// the ground rather than hovering over it, and the thing stopping you is
/// a bank you can see.
///
/// **It ends where the deck clears your head.** Past that you walk under
/// the bridge, which is worth being able to do, so the bank has no reason
/// to go further.
///
/// **A wedge and not a block.** The first build pushed one box a step,
/// full height out to the toe, which is a two metre wall standing where
/// the drawing is an inch of gravel: the invisible face this building has
/// been caught by three times. Each step is bands across instead, and
/// every band is as high as the drawn surface at its outer edge and so
/// never higher than what you can see.
pub fn abutment(reaches: f32, reach: f32) -> Vec<Aabb> {
    let foot = foot(reaches);
    let reach = reach.clamp(0.0, 1.0);
    let (begins, ends) = bank_runs();
    let ends = ends.min(reach);

    if ends <= begins {
        return Vec::new();
    }

    let half = WIDE * 0.5;
    let steps = ((ends - begins) * RUN / BANK_LONG).ceil() as usize;
    let mut out = Vec::new();

    for step in 0..steps {
        let from = begins + step as f32 * BANK_LONG / RUN;
        let to = (begins + (step + 1) as f32 * BANK_LONG / RUN).min(ends);
        // **The lower end of the two, and that way round on purpose.** The
        // deck's underside rises along the box, so a box as high as the
        // bank's taller end stands through the deck at its shorter one,
        // which `the_bank_stops_under_the_deck` catches. The other way
        // round leaves the box a riser under the drawn surface, and at
        // `BANK_LONG` along a slope of a quarter that riser is 0.08:
        // inside `walk::RIDES`, so it is walked rather than stepped.
        let top = bank_high(to).min(bank_high(from));

        // **The apron is walked over and not stopped at.** A bed of
        // gravel a hand deep is under `crate::walk::RIDES`, so boxes for
        // it would be boxes nothing can touch, and there are a hundred
        // and fifty of them. Only the stretch that actually stands in
        // your way gets a collider.
        if top <= BANK_LEAST + 1e-3 {
            continue;
        }

        // **Out to the narrowest the edge ever wanders, and no further.**
        // The drawing's own spread is `bank_spread`, which the rim pulls in
        // by up to `BANK_RAGGED`: tile the boxes out to the widest of those
        // and the bands past the wander stand in grass. So they stop at the
        // narrowest, and what nothing stops you walking into is the last
        // hand's breadth of the toe, which is ankle height and gravel.
        let spread = top * BANK_SLOPE * (1.0 - BANK_RAGGED);
        let (near, far) = (foot.x - to * RUN, foot.x - from * RUN);

        for band in 0..=BANK_BANDS {
            let step = spread / BANK_BANDS as f32;
            let (inner, outer) = if band == 0 {
                (0.0, half)
            } else {
                let from = half + step * (band - 1) as f32;

                (from, from + step)
            };
            // as high as the drawn surface at the band's outer edge, so a
            // band is never higher than what you can see of it
            let stands = if band == 0 {
                top
            } else {
                bank_profile(top, top, spread, outer)
            };

            if stands <= 0.0 {
                continue;
            }

            // **Band nought is one box across the whole deck, not a half.**
            // Written as `inner = 0.0` and mirrored only for the bands
            // outside it, the box under the deck ran from the middle to
            // the near edge and the other three metres had no collider at
            // all: a lane down the whole length of the bank that you drop
            // into. Jake, 2026-11-18, stuck coming up from under.
            let (south, north) = if band == 0 {
                (-outer, outer)
            } else {
                (inner, outer)
            };

            // up to the deck's own underside, never into it: two faces in
            // one plane is the fault this building has found seven times
            out.push(Aabb::new(
                vec3(near, foot.y - BANK_UNDER, foot.z + south),
                vec3(far, foot.y + stands, foot.z + north),
            ));

            if band > 0 {
                out.push(Aabb::new(
                    vec3(near, foot.y - BANK_UNDER, foot.z - outer),
                    vec3(far, foot.y + stands, foot.z - inner),
                ));
            }
        }
    }

    out
}

/// How high the earth stands a share `along` out, which is under the deck
/// and falling away as the deck lifts clear of your head.
///
/// **It dies back rather than ending in a cliff.** The bank is there for
/// the stretch you cannot pass, and at `BANK_CLEAR` you can walk under
/// instead, so it has no business standing two metres tall and stopping
/// dead. Over the last of its run it comes back down to the field.
///
/// **And it stops short of the soffit by [`BANK_SHY`].** Written as
/// `high(along) - DEEP` the bank's flat top is the deck's underside, the
/// same plane exactly, which is the fault this building has found seven
/// times and found again here: Jake, 2026-11-18, standing at the foot with
/// grey bars lying across the rainbow. The bars move when you move, which
/// is the tell. The bank has no reason to touch the deck; it has to fill
/// the gap, and a hand's breadth of air does that just as well.
pub fn bank_high(along: f32) -> f32 {
    let (begins, ends) = bank_runs();

    if !(begins..=ends).contains(&along) {
        return 0.0;
    }

    let under = bank_under(along);
    let stands = BANK_CLEAR - DEEP - BANK_SHY;

    if under >= stands {
        return 0.0;
    }

    // **Never less than a bed of it.** Before the deck's underside leaves
    // the ground there is nothing to fill, and the bank used to start
    // there: six and a half metres of ramp lying on bare grass with
    // nothing to say where the bridge began. Jake, 2026-11-18, asked at
    // the foot: the ramp doesn't make sense. So the gravel runs the whole
    // way and a little past the foot, and the shallowest it ever lies is
    // a hand's depth, which is a threshold you walk onto rather than a
    // step you climb: `crate::walk::RIDES` is deeper than this.
    let top = under.max(BANK_LEAST);
    // full under the low stretch, then eased down over the last third of it
    let dying = ((stands - under) / (stands * BANK_DIES)).min(1.0);
    // **And come up out of nothing at the front, not start in a cut.**
    // The far end dies back; the near end stopped dead, so the bed ended
    // in a straight line of gravel across the grass a few paces in front
    // of the foot.
    let coming = ((along - begins) * RUN / BANK_EDGE).clamp(0.0, 1.0);

    top * dying * coming
}

/// How high the bank lies under the deck, which is the deck's own
/// underside less [`BANK_SHY`] and is below the field over the first
/// stretch, where the deck has not left the ground yet.
///
/// **Its own function, because the apron made the two different.** Out
/// past the deck's edge the gravel never lies thinner than
/// [`BANK_LEAST`]; under the deck it follows the soffit down and is
/// buried, which is right: raising it to the apron's depth there would
/// put gravel over the first three metres of the rainbow.
pub fn bank_under(along: f32) -> f32 {
    high(along) - DEEP - BANK_SHY
}

/// How high the bank stands `out` metres from the middle of the deck, where
/// it is `top` high under the deck and spreads `spread` past the deck's own
/// edge. Zero at its toe, flat under the deck, and eased between.
pub fn bank_profile(under: f32, top: f32, spread: f32, out: f32) -> f32 {
    let half = WIDE * 0.5;

    if out <= half {
        return under;
    }

    if spread <= 0.0 {
        return 0.0;
    }

    let down = ((out - half) / spread).clamp(0.0, 1.0);

    top * (1.0 - down * down)
}

/// How far out the bank's edge wanders a share `along` into the run, as a
/// share of its spread.
///
/// **No one wave allowed to carry it**, which is `mound_rim`'s lesson one
/// object along: three big lobes down a bank is a shape, and what is wanted
/// is an edge with no shape at all. Four small waves, none of them large,
/// and their shares add to one so the narrowest it ever runs is a number
/// rather than a guess.
pub fn bank_rim(along: f32) -> f32 {
    let metres = along * RUN;
    let wobble = (metres * 0.9).sin() * 0.27
        + (metres * 2.3 + 1.7).sin() * 0.31
        + (metres * 4.7 + 0.5).sin() * 0.25
        + (metres * 9.1 + 2.4).sin() * 0.17;

    1.0 - BANK_RAGGED * (0.5 + wobble * 0.5)
}

/// How far past the deck's edge the bank reaches a share `along` into the
/// run, and how high its surface stands `across` metres from the middle.
///
/// **The one account of its shape.** The surface is drawn from these and
/// the boxes that stop you are checked against them, which is the only way
/// `nothing_stops_you_higher_than_the_bank_is_drawn` tests anything: a test
/// against a second copy of the arithmetic passes whatever the copy says.
pub fn bank_spread(along: f32) -> f32 {
    let (begins, ends) = bank_runs();

    if !(begins..=ends).contains(&along) {
        return 0.0;
    }

    (BANK_APRON + bank_high(along) * BANK_SLOPE) * bank_rim(along)
}

pub fn bank_at(along: f32, across: f32) -> f32 {
    let top = bank_high(along);
    // **In front of the foot the bed runs right across.** Past that it
    // keeps to the soffit and is buried, which is what stops gravel lying
    // over the first stretch of the rainbow; but there is no deck in front
    // of the foot to lie under, and keeping it down there left a six metre
    // lane of bare grass with the tufts already taken out of it, which
    // `the_field_is_drawn` caught because that picture is taken standing
    // on this exact spot.
    // **Onto the soffit only once there is a deck overhead to hide it.**
    // The bed's middle follows the deck's underside, and the underside is
    // below the field for the first six metres: taken there from the foot
    // it put the middle of the bed under the ground, and what you saw was
    // the field's own grass in a six metre lane with a strip of gravel
    // down either side of it. Jake, 2026-11-18, standing in it: it's not
    // consistent.
    //
    // So the blend is keyed on how far the soffit stands over the field
    // rather than on how far along you are. Until it stands clear the bed
    // lies on the ground across its whole width, which buries the ramp's
    // first three metres, and that is the thing wanted: the ramp comes
    // out of the gravel rather than starting on top of it.
    let sunk = (high(along) - DEEP).max(0.0);
    let onto = (sunk / BANK_ONTO).clamp(0.0, 1.0);
    let inner = top * (1.0 - onto) + bank_under(along).min(top) * onto;

    bank_profile(inner, top, bank_spread(along), across)
}

/// The bank as one surface, the way `mound_mesh` is one surface.
///
/// **Faceted, and thinning to nothing at its rim.** The first build of this
/// was a smooth surface with a flat top and a slope each side, and
/// `MeshData::surface` works its normals out by sampling around each point:
/// at the kink where the slope meets the top the sampling straddles the
/// kink and the normal comes out wrong, which drew as dark bars lying
/// across the deck. `mound_mesh` had already solved this one object along,
/// with `garden::faceted` for the normals and a rim that thins rather than
/// steps. This is that, run along a line instead of around a stone.
///
/// **Stone and gravel, not turf.** Jake, 2026-11-18: sand and gravel and
/// rocks are what make sense as an abutment. The first build was the
/// field's own grass raised, which read as neither ground nor structure.
pub fn bank_mesh(reaches: f32, reach: f32) -> blitzkit::mesh::MeshData {
    let foot = foot(reaches);
    let reach = reach.clamp(0.0, 1.0);
    let half = WIDE * 0.5;
    let (begins, ends) = bank_runs();
    let ends = ends.min(reach);
    let stretch = (ends - begins).max(0.0);

    let mut mesh = crate::garden::faceted(blitzkit::mesh::MeshData::surface(
        BANK_ALONG,
        BANK_ACROSS,
        |u, v| {
            let along = begins + u * stretch;
            let spread = bank_spread(along);
            let out = half + spread;
            let z = (v * 2.0 - 1.0) * out;
            let y = bank_at(along, z.abs());
            // and broken up on the way down, the way the mound is
            let down = if spread > 0.0 {
                ((z.abs() - half) / spread).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let rough = ((along * 90.0).sin() * (v * 17.0).sin()) * 0.05 * down;

            vec3(foot.x - along * RUN, foot.y + y + rough, foot.z + z)
        },
    ));

    // **And the stone laid at a size, not stretched over the whole bank.**
    // `MeshData::surface` runs its uv nought to one whichever way, so one
    // granite picture covered ten metres by seven and drew as smears a
    // metre long. This is the cube's fault with a different shape: a
    // surface's texture is its own size however big the surface is.
    let along = stretch * RUN / BANK_STONE;
    let across = (WIDE + bank_reaches() * 2.0) / BANK_STONE;

    for vertex in &mut mesh.vertices {
        vertex.uv[0] *= along;
        vertex.uv[1] *= across;
    }

    mesh
}

/// How many metres of bank one granite picture covers.
///
/// Two and a half, which is near enough what the runestone's footing does
/// with the same picture: that mound is four metres across and takes one.
pub const BANK_STONE: f32 = 2.5;

/// The furthest the bank's edge can stand from the deck's own.
pub fn bank_reaches() -> f32 {
    (BANK_CLEAR - DEEP) * BANK_SLOPE
}

/// Whether a spot in the field is under the bank, and so is ground the
/// field has no business sowing anything on.
///
/// **Because the field does not know the bank is there.** `field::cover`
/// scatters its tufts, flowers and litter wherever the ground is open and
/// asks the village but never the bridge, so grass came up through the
/// gravel: the same fault as a flower through a cart's bed, which is the
/// case that note was written about. Jake saw it on 2026-11-18.
///
/// Read off `bank_spread`, which is also what the surface is drawn to, so
/// the keep-out is the bank's own footprint rather than a second guess at
/// where the bank is.
/// Whether a spot is on spec 0118's ledger, which is dressed stone and so
/// is not ground anything grows out of.
///
/// **The same fault a third time in one day.** Grass came up through the
/// bank's gravel and a piece of rubble came to rest on this slab, both
/// because the thing scattering did not know the thing was there.
pub fn on_the_ledger(reaches: f32, at: Vec3) -> bool {
    let (stone, _) = runestone_at(reaches);
    let (slab, _) = slab_at();

    (at.x - stone.x - slab.x).abs() < SLAB_LONG * 0.5 && (at.z - stone.z).abs() < SLAB_WIDE * 0.5
}

pub fn banked(reaches: f32, at: Vec3) -> bool {
    let foot = foot(reaches);
    let (begins, ends) = bank_runs();
    let along = (foot.x - at.x) / RUN;

    if !(begins..=ends).contains(&along) {
        return false;
    }

    (at.z - foot.z).abs() <= WIDE * 0.5 + bank_spread(along)
}

/// The loose stone lying on the bank: where each piece sits, how big it is
/// and which way round.
///
/// **Because a smooth grey surface is a smooth grey surface.** The bank
/// came back from Jake on 2026-11-18 as stone that did not quite land, and
/// this is the lesson `rubble` already wrote one object along: what makes
/// ground read as gravel is pieces of it, and a raised surface with a
/// stone picture on it is a raised surface with a stone picture on it.
///
/// Coarsest at the toe and finest up against the deck, which is the way
/// round `rubble` is not, and for the opposite reason: there is no collar
/// here to pack. This is spoil tipped against an abutment, so what rolls
/// is what reaches the bottom.
pub fn bank_stones(reaches: f32, reach: f32) -> Vec<(Vec3, Vec3, f32)> {
    let foot = foot(reaches);
    let reach = reach.clamp(0.0, 1.0);
    let (begins, ends) = bank_runs();
    let ends = ends.min(reach);

    if ends <= begins {
        return Vec::new();
    }

    let half = WIDE * 0.5;
    let mut state = 0xB0FF_u32;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;

        state
    };
    let roll = |v: u32| (v % 10_000) as f32 / 10_000.0;
    let mut out = Vec::with_capacity(BANK_STONES);

    for _ in 0..BANK_STONES {
        let along = begins + roll(next()) * (ends - begins);
        let spread = bank_spread(along);
        // **Out past the deck's edge, which is all of it you can see.**
        // Under the deck the bank is a flat top in shadow and anything
        // lying on it is hidden by the deck itself.
        let share = roll(next());
        // **From the deck's own edge outward.** Spread from the middle
        // instead, nine pieces in ten lay under the deck, where the deck
        // itself hides them: the gravel reads as gravel only where it is
        // out in the open beside the kerb.
        let out_by = half * 0.94 + share * (half * 0.06 + spread);
        let side = if roll(next()) < 0.5 { -1.0 } else { 1.0 };
        let across = (0.09 + share * 0.17) * (0.7 + roll(next()) * 0.8);
        let z = foot.z + side * out_by;
        // sitting into the bank by about a third of itself, which is what
        // stops them reading as dropped
        let up = foot.y + bank_at(along, out_by) - across * 0.34;

        out.push((
            vec3(foot.x - along * RUN, up, z),
            vec3(
                across,
                across * (0.45 + roll(next()) * 0.4),
                across * (0.8 + roll(next()) * 0.6),
            ),
            roll(next()) * std::f32::consts::TAU,
        ));
    }

    out
}

/// How many pieces lie on it.
///
/// **Two hundred and forty over ten metres of bank each side.** The
/// runestone's footing is a hundred and sixty over four metres across, and
/// this has to read as gravel from the same distance.
pub const BANK_STONES: usize = 240;

/// How bright the underside is against the top, and how much of what is
/// behind it comes through.
///
/// **You climb this looking at its belly.** Your eye is `EYE` over a deck
/// rising at better than twenty degrees, so three and a half metres ahead
/// the deck is above you and stays above you the rest of the way: from the
/// middle of the climb almost the whole view is the underside, with a band
/// of the top at the crest. Drawn at the same `glow` as the top, which is
/// what it was until 2026-11-11, the two read as two ribbons lying on each
/// other and the band at the crest reads as a ledge. Jake, with a picture:
/// the bridge has a slight ledge, looks like a glitch.
///
/// **Dimmer and seen through, rather than either alone.** Dark alone makes
/// the climb a black hump with a rainbow sliver on it, which throws away
/// the thing worth walking up. Clear alone is an invisible bridge. Together
/// the belly stays a rainbow, the top is plainly brighter, and the landing
/// and its lit outbuildings come up through the deck while you are still
/// climbing, which turns the arrival into something you watch rather than
/// something that appears over a crest.
///
/// Jake picked these off four pairs rendered side by side.
pub const SOFFIT_LIT: f32 = 0.34;
pub const SOFFIT_SEE: f32 = 0.52;

/// What the underside is multiplied by, which is [`glow`] dimmed and let
/// through.
///
/// **Not `glow() * SOFFIT_LIT`.** That scales the alpha with the colour, so
/// one number would be setting both and they are not the same question. The
/// first build of this did exactly that and came out at 0.18 clear by
/// accident, which is how the idea turned up at all.
pub fn soffit_glow() -> Vec4 {
    let lit = glow() * SOFFIT_LIT;

    Vec4::new(lit.x, lit.y, lit.z, SOFFIT_SEE)
}

/// The underside of that ribbon, which is drawn by itself so it can be
/// dimmer than the top. See [`SOFFIT_LIT`].
pub fn soffit_mesh(reaches: f32, reach: f32) -> blitzkit::mesh::MeshData {
    ribbon(reaches, true, reach)
}

/// The deck's strips: the underside when `soffit`, and the top and the two
/// edges when not.
fn ribbon(reaches: f32, soffit: bool, reach: f32) -> blitzkit::mesh::MeshData {
    use blitzkit::mesh::Vertex;

    let along = stations(reaches, reach);
    let half = WIDE * 0.5;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // the four strips, as which corner of the cross-section each runs
    // between and which way it looks. The corners are numbered round it:
    // 0 top south, 1 top north, 2 under north, 3 under south.
    let strips: [(usize, usize, i32); 4] = [(0, 1, 0), (1, 2, 1), (2, 3, 2), (3, 0, 3)];
    // **Split by which way the strip looks and not by its place in the
    // list.** `2` is the one whose normal is `-over`, which is the whole of
    // what makes it the underside, and a strip reordered here would carry
    // its own answer with it.
    let want: Vec<(usize, usize, i32)> = strips
        .iter()
        .copied()
        .filter(|(_, _, looks)| (*looks == 2) == soffit)
        .collect();

    for (near, far, looks) in want {
        let first = vertices.len() as u32;

        for middle in along.iter() {
            // how the deck lies here, which is the slope of the curve: the
            // cross-section stands square to it rather than square to the
            // world, or the deck is thicker where it is steeper
            // **Where this station actually is along the run**, and not its
            // place in the list over `PIECES`. Spec 0117 cuts the deck at
            // the reach, so the last station is wherever that falls and the
            // two stopped agreeing: the cross-section at the cut end stood
            // at the wrong pitch and the rainbow's mapping ran short.
            let along_run = (foot(reaches).x - middle.x) / RUN;
            let slope = pitch(along_run);
            let over = vec3(slope, 1.0, 0.0).normalize();
            let under = *middle - over * DEEP;

            let corner = |which: usize| match which {
                0 => *middle - Vec3::Z * half,
                1 => *middle + Vec3::Z * half,
                2 => under + Vec3::Z * half,
                _ => under - Vec3::Z * half,
            };
            // across the width, so the picture lies the same way on the
            // top, the underside and both edges
            let across = |which: usize| if which == 1 || which == 2 { 1.0 } else { 0.0 };
            let normal = match looks {
                0 => over,
                1 => Vec3::Z,
                2 => -over,
                _ => -Vec3::Z,
            };
            let down = along_run;

            for which in [near, far] {
                let put = corner(which);

                vertices.push(Vertex::new(
                    [put.x, put.y, put.z],
                    [normal.x, normal.y, normal.z],
                    [across(which), down],
                ));
            }
        }

        // **One quad between each pair of stations, and not `PIECES` of
        // them.** This counted pieces while the loop above walked the
        // stations, and a truncated deck has fewer: it indexed two vertices
        // past the end of its own list and the engine panicked building the
        // tree over it.
        for at in 0..along.len().saturating_sub(1) as u32 {
            let one = first + at * 2;

            indices.extend([one, one + 1, one + 3, one, one + 3, one + 2]);
        }
    }

    blitzkit::mesh::MeshData::new(vertices, indices)
}

/// The landing at the top: how far it runs on past the deck, how wide it is
/// across, and how thick its slab is.
///
/// **Ten across against the deck's six.** Arriving wants to be stepping out
/// onto something rather than reaching the last plank, and two metres either
/// side is the difference you feel underfoot without it reading as a
/// courtyard. Seven deep is three strides, which is room to turn round and
/// look back and nothing more.
pub const LANDING_OUT: f32 = 7.0;
pub const LANDING_WIDE: f32 = 10.0;
pub const LANDING_DEEP: f32 = 0.5;

/// The parapet round it: how high it stands over the landing and how thick.
///
/// **Low, because it is a kerb and not a wall.** What it is for is that a
/// slab twenty-six metres up with nothing round it is a diving board, and
/// that a platform with an edge reads as somewhere to stand. Head height
/// would take away the view, which is the prize.
pub const RAIL_UP: f32 = 0.62;
pub const RAIL_THICK: f32 = 0.34;

/// How wide the way through the far parapet is.
///
/// **Because spec 0056 is through there.** The parapet was built on three
/// sides to stop somebody walking off a twenty-six metre drop, which it
/// does, and that included the side Valhalla is on: the walk test for
/// getting across fetched up eight tenths of a metre short of the edge,
/// against the rail. A gate with no gap in it is a wall.
///
/// On the line you walk in on, so the deck, the gap and the hall's door
/// are one straight way through.
pub const WAY_WIDE: f32 = 3.0;

/// Where the top of the deck is: the far end of the run, at full height.
pub fn top(reaches: f32) -> Vec3 {
    let foot = foot(reaches);

    vec3(foot.x - RUN, high(1.0), foot.z)
}

/// The landing, as its slab and the three sides of parapet round it.
///
/// Empty until the building has been done, like everything else here.
///
/// **It laps the deck rather than meeting it.** Two faces at the walking
/// surface stop a walk dead and `walk` reports nothing whatever, which this
/// building has been caught by three times in one day before now. The slab
/// runs back under the last stretch of deck by a tread's length.
pub fn landing(reaches: f32, far: f32) -> Vec<Aabb> {
    if far < 1.0 {
        return Vec::new();
    }

    let top = top(reaches);
    // east is where you arrive from, so the slab starts back under the deck
    let east = top.x + TREAD;
    let west = top.x - LANDING_OUT;
    let (south, north) = (top.z - LANDING_WIDE * 0.5, top.z + LANDING_WIDE * 0.5);

    vec![
        // the slab, its top flush with the deck's
        Aabb::new(
            vec3(west, top.y - LANDING_DEEP, south),
            vec3(east, top.y, north),
        ),
        // and the parapet on the three sides you do not arrive over. Each
        // one runs the full corner so that two of them meet rather than
        // leaving a notch you could step through.
        //
        // The far one is in two pieces with the way through between them,
        // the way the garden's west wall is split round the way out to the
        // field.
        Aabb::new(
            vec3(west, top.y, south),
            vec3(west + RAIL_THICK, top.y + RAIL_UP, top.z - WAY_WIDE * 0.5),
        ),
        Aabb::new(
            vec3(west, top.y, top.z + WAY_WIDE * 0.5),
            vec3(west + RAIL_THICK, top.y + RAIL_UP, north),
        ),
        Aabb::new(
            vec3(west, top.y, south),
            vec3(east, top.y + RAIL_UP, south + RAIL_THICK),
        ),
        Aabb::new(
            vec3(west, top.y, north - RAIL_THICK),
            vec3(east, top.y + RAIL_UP, north),
        ),
    ]
}

/// Where the watchman stands, and which way he faces.
///
/// **Off the line you walk in on, and not off the end of it.** Dead centre
/// he is a man standing in a doorway and you would walk into him: a body is
/// 0.9 across. Out at a third of the landing's width he is exactly on the
/// edge of the deck's own six, which is past the lane you come up and so is
/// somebody you never pass. A fifth of it leaves a metre of daylight either
/// side of two bodies and still puts him in front of you.
///
/// **Facing the climb**, which is back east down the deck. The yaw is the
/// one `draw_the_people` reads, where ahead is `(sin, 0, -cos)`, so east is
/// a quarter turn.
pub fn watchman(reaches: f32) -> (Vec3, f32) {
    let top = top(reaches);

    (
        vec3(
            top.x - LANDING_OUT * 0.48,
            top.y,
            top.z + LANDING_WIDE * 0.20,
        ),
        std::f32::consts::FRAC_PI_2,
    )
}

/// What you can stand on, which is the treads and nothing else.
pub fn solid(reaches: f32, far: f32) -> Vec<Aabb> {
    // **As far as it has got, and no further.** Spec 0117: the deck is
    // drawn to `reach` and this is what you stand on, so the two read the
    // same number or there is invisible deck past the cut end.
    let mut out = treads(reaches, far);

    out.extend(kerbs(reaches, far));
    // and the earth it comes out of, which is what stops you where the deck
    // is too low to duck under and too high to step over. Spec 0117.
    out.extend(abutment(reaches, far));

    // and the landing and what stands on it, which are the finished
    // bridge's rather than the reaching one's: there is no top to arrive at
    // until the last span is in.
    //
    // **Asked of the reach and not of the trail.** They say the same thing
    // at rest, `reach` being exactly 1.0 when every question is given, and
    // they come apart for the six seconds the last span is winding in:
    // asked of the trail, a landing and the piers on it would stand in the
    // air at the end of a deck that has not arrived. Spec 0119.
    if far < 1.0 {
        return out;
    }

    out.extend(landing(reaches, far));
    // and the piers, which oversail the kerb they stand on by enough to
    // put your head in one if they were only drawn
    out.extend(piers(reaches, far).into_iter().map(|at| {
        Aabb::from_center_size(
            at + Vec3::Y * (PIER_UP + PIER_CAP) * 0.5,
            vec3(PIER_WIDE, PIER_UP + PIER_CAP, PIER_WIDE),
        )
    }));
    // **`runestone_at` and not `runestone`.** That one takes a trail it
    // has not read since spec 0118 and hands back an `Option` that is
    // always `Some`; this is the place that is left and the stone's own
    // account of where it stands.
    let (stone, size) = runestone_at(reaches);

    out.push(Aabb::from_center_size(stone, size));

    out
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const REACHES: f32 = 20.0;

    fn done() -> Trail {
        let mut trail = Trail::new();

        for one in Asked::every() {
            trail.gave(*one);
        }

        trail
    }

    /// Spec 0054: the deck's underside is its own strip, dimmer than the
    /// top and seen through, and still a rainbow.
    ///
    /// **Asked as the gap between the two and not as either number.** The
    /// fault was that the belly and the top were one value, so the two
    /// could only ever agree; what matters is that they differ, which is
    /// what this holds while leaving Jake free to move both. The bounds at
    /// each end are his words: obviously a rainbow, and not an invisible
    /// bridge.
    #[test]
    fn the_belly_is_dimmer_than_the_top_and_still_a_rainbow() {
        let top = glow();
        let belly = soffit_glow();

        assert!(
            belly.x < top.x * 0.75,
            "the belly is {} against a top of {}",
            belly.x,
            top.x
        );
        assert!(
            belly.x > top.x * 0.1,
            "the belly is {}, which is a black hump",
            belly.x
        );
        assert!(
            belly.w < 0.8 && belly.w > 0.25,
            "the belly is {} through, which is a pane or a hole",
            belly.w
        );
        assert_eq!(top.w, 1.0, "the top is seen through");
    }

    /// And splitting it off lost no strip: the deck keeps three of the
    /// four and the underside is the fourth.
    ///
    /// **A control the picture tests cannot be.** A strip dropped from both
    /// meshes is a hole in the bridge that no reference of this building
    /// looks at, and one left in both is the fight this change was made to
    /// end, drawn twice.
    #[test]
    fn the_four_strips_are_split_three_and_one() {
        let deck = deck_mesh(REACHES, 1.0);
        let belly = soffit_mesh(REACHES, 1.0);

        assert_eq!(
            deck.indices.len(),
            belly.indices.len() * 3,
            "the deck has {} indices against the underside's {}",
            deck.indices.len(),
            belly.indices.len()
        );
        assert_eq!(
            deck.vertices.len(),
            belly.vertices.len() * 3,
            "the deck has {} vertices against the underside's {}",
            deck.vertices.len(),
            belly.vertices.len()
        );
    }

    /// One question short of done, which is what the bridge must not show
    /// for.
    fn nearly() -> Trail {
        let mut trail = Trail::new();

        for one in Asked::every().iter().skip(1) {
            trail.gave(*one);
        }

        trail
    }

    #[test]
    fn the_foot_is_there_before_anything_is_asked() {
        let empty = Trail::new();

        assert!(!whole(&empty), "nothing has been given");

        // **Not empty, which is the whole of spec 0117.** It used to be, and
        // a bridge that is nothing until the first question is answered is
        // the same surprise moved one step earlier rather than a promise.
        assert!(
            !solid(REACHES, reach(&empty)).is_empty(),
            "the foot is not there to stand on"
        );

        let far = reach(&empty);
        let every = Asked::every().len();

        assert!(
            (far - 1.0 / (every + 1) as f32).abs() < 1e-6,
            "the foot is {} of the run and should be one of {}",
            far,
            every + 1
        );
    }

    #[test]
    fn one_short_has_deck_but_no_landing() {
        let nearly = nearly();

        assert!(!whole(&nearly), "one answer short and it is still there");
        assert!(
            !solid(REACHES, reach(&nearly)).is_empty(),
            "five sixths of a bridge is still a bridge"
        );

        // **And nothing of the landing**, which is what the bridge arrives
        // at: a stone platform standing in the air ahead of the deck would
        // be the ending given away. Spec 0117.
        assert!(
            landing(REACHES, reach(&nearly)).is_empty(),
            "there is a landing before there is a top"
        );
        assert!(
            piers(REACHES, reach(&nearly)).is_empty(),
            "piers on no landing"
        );
    }

    /// Spec 0117: the bank runs exactly the stretch you cannot pass.
    ///
    /// **Both ends matter.** Short of the clearance it has to be there, or
    /// the wall is back. Past it the bank has to stop, or you can no longer
    /// walk under a bridge that is plainly high enough to walk under.
    #[test]
    fn the_bank_runs_out_where_the_deck_clears_your_head() {
        let banked = abutment(REACHES, 1.0);

        assert!(!banked.is_empty(), "nothing is banked at all");

        let foot = foot(REACHES);
        let ends = banked
            .iter()
            .map(|box_| box_.min.x)
            .fold(f32::MAX, f32::min);
        let along = (foot.x - ends) / RUN;

        assert!(
            high(along) <= BANK_CLEAR + 1e-3,
            "the bank runs to {} up, past the {} it is for",
            high(along),
            BANK_CLEAR
        );
        assert!(
            high(along) > BANK_CLEAR - 0.5,
            "the bank gives up at {} up and leaves a wall to {}",
            high(along),
            BANK_CLEAR
        );
    }

    /// Spec 0117, and Jake on 2026-11-18: the ramp doesn't make sense.
    ///
    /// **Because it began in the middle of a field.** The bank started
    /// where the deck's underside leaves the ground, six and a half
    /// metres in, so the first stretch of the ramp was a thin slab lying
    /// on grass with nothing to say a bridge began there. The gravel runs
    /// the whole way now and a little past the foot.
    ///
    /// The depths here are written out and not read off `BANK_LEAST` and
    /// `BANK_APRON`: a test that reads the constant agrees with the code
    /// whatever the constant says, which is how the first go at
    /// `the_bank_is_narrower_...` came to prove nothing.
    #[test]
    fn the_gravel_runs_to_the_foot_and_past_it() {
        let (begins, _) = bank_runs();

        assert!(
            begins < 0.0,
            "the gravel starts {} along, which is not in front of the foot",
            begins
        );

        assert!(
            bank_high(-1.0 / RUN) > 0.03,
            "the bed is {} deep a metre in front of the foot",
            bank_high(-1.0 / RUN)
        );

        let half = WIDE * 0.5;

        for step in 0..=200 {
            // only the stretch the bank used to skip: from the foot to
            // where the deck's underside leaves the ground. In front of
            // the foot it is thinning away, per `BANK_EDGE`, so the depth
            // there is asked for once above rather than all along.
            let along = along_at(DEEP) * step as f32 / 200.0;

            assert!(
                bank_high(along) >= 0.04,
                "the gravel is {} deep a share {} along",
                bank_high(along),
                along
            );
            assert!(
                bank_spread(along) >= 0.25,
                "the gravel reaches {} past the deck a share {} along",
                bank_spread(along),
                along
            );
            // and it is a bed on the ground, not a lip standing off it
            assert!(
                bank_at(along, half + bank_spread(along)) <= 1e-3,
                "the gravel's toe stands {} proud of the field",
                bank_at(along, half + bank_spread(along))
            );
            // **and nowhere under the field.** Where the bed's middle
            // sank below the ground there was no deck over it yet, so
            // what showed through was the field's own grass in a lane
            // between two strips of gravel.
            for across in 0..=12 {
                let out = half * across as f32 / 12.0;

                assert!(
                    bank_at(along, out) > 1e-3,
                    "the bed is {} at {} out a share {} along, under the field",
                    bank_at(along, out),
                    out,
                    along
                );
            }
        }
    }

    /// And none of it stands in the deck, nor in the deck's own plane.
    ///
    /// **A clearance written out rather than read off `BANK_SHY`.** The
    /// bank's top was `high(along) - DEEP` exactly, which is the soffit's
    /// plane to the millimetre, and it drew as grey bars sliding across
    /// the rainbow as you walked. A test that reads the same constant the
    /// code does agrees with the code whatever the constant says, so the
    /// number here is the margin the fault needs, not the margin the
    /// building happens to use.
    const CLEARS_THE_SOFFIT: f32 = 0.02;

    ///
    /// Asked from where the deck's underside stands clear of the field by
    /// `BANK_ONTO`. Below that the bed is lying on the ground rather than
    /// hanging under the deck, and it crosses the soffit's line once on
    /// its way down. A crossing is a line and draws clean; what speckles
    /// is two faces sharing a plane over an area, which is what this
    /// holds everywhere the two actually run together.
    #[test]
    fn the_bank_is_not_in_the_decks_own_plane() {
        let (_, ends) = bank_runs();
        let begins = along_at(DEEP + BANK_ONTO + 0.05);

        for step in 0..=400 {
            let along = begins + (ends - begins) * step as f32 / 400.0;
            let soffit = high(along) - DEEP;
            let bank = bank_at(along, 0.0);

            assert!(
                bank <= soffit - CLEARS_THE_SOFFIT,
                "the bank's top is {} a share {} along and the soffit is {}",
                bank,
                along,
                soffit
            );
        }
    }

    /// And none of the boxes that stop you stand in it either.
    #[test]
    fn the_bank_stops_under_the_deck() {
        let foot = foot(REACHES);

        for box_ in abutment(REACHES, 1.0) {
            let along = (foot.x - box_.max.x) / RUN;
            let deck = foot.y + high(along);

            assert!(
                box_.max.y <= deck - DEEP - CLEARS_THE_SOFFIT,
                "the bank reaches {} and the deck's underside is {}",
                box_.max.y,
                deck - DEEP
            );
        }
    }

    /// And nothing stops you where there is nothing to see stopping you.
    ///
    /// **The fault this building has been caught by three times**, under
    /// three names: a face at the walking surface, an unseen ledge, a
    /// slab you cannot step over or duck under. The bank's whole claim in
    /// spec 0117 is that what stops you is a bank you can see, and the
    /// first build broke it by pushing one full height box a step: two
    /// metres of wall standing where the drawing is an inch of gravel.
    ///
    /// Read against `bank_at`, which the surface itself is drawn from, so
    /// this is the drawing and not a second copy of its arithmetic. The
    /// surface also carries up to `0.05` of roughness either way, which is
    /// below the allowance here and is not a wall at any height.
    #[test]
    fn nothing_stops_you_higher_than_the_bank_is_drawn() {
        let foot = foot(REACHES);
        let banked = abutment(REACHES, 1.0);

        assert!(!banked.is_empty(), "nothing is banked at all");

        for box_ in &banked {
            let outer = (box_.max.z - foot.z).max(foot.z - box_.min.z);
            let stands = box_.max.y - foot.y;

            // at both ends of the box and across it, since the surface
            // falls as well as rises over the stretch one box covers
            for end in 0..=4 {
                let span = (box_.max.x - box_.min.x) * end as f32 / 4.0;
                let along = (foot.x - box_.max.x + span) / RUN;
                let drawn = bank_at(along, outer);

                assert!(
                    stands <= drawn + 0.06,
                    "a box stands {} at {} out, where the bank is drawn {} \
                     high a share {} along",
                    stands,
                    outer,
                    drawn,
                    along
                );
            }
        }
    }

    /// Spec 0117, and Jake on 2026-11-18: narrow it and break its line.
    ///
    /// **Narrow** is the gravel against the bridge: the two cheeks together
    /// come to less than half the deck's own width. "Narrower than the
    /// deck" was the first assertion here and it was worth nothing, since a
    /// cheek was 1.83 at the old spread and the deck's half is 3: the
    /// control put `BANK_SLOPE` back to 1.5 and the test still passed.
    ///
    /// **Broken** is the edge not running straight. Tested as bounds and a
    /// spread of values rather than as a shape, since the thing wanted is
    /// an edge with no shape at all.
    #[test]
    fn the_bank_is_narrower_than_the_deck_and_its_edge_wanders() {
        let (begins, ends) = bank_runs();
        let mut least = f32::MAX;
        let mut most = f32::MIN;

        for step in 0..=200 {
            let along = begins + (ends - begins) * step as f32 / 200.0;
            // **Before the wander takes anything off it**, since the rim
            // only ever pulls in. Read as `bank_spread` this passed at the
            // old 1.5 as well: the rim happens to run narrow just where
            // the bank is tallest, so the sampled widest came in under the
            // bar and the test said nothing about the constant at all.
            let spread = BANK_APRON + bank_high(along) * BANK_SLOPE;

            assert!(
                spread * 2.0 <= WIDE * 0.5,
                "the bank puts {} of gravel beside {} of bridge",
                spread * 2.0,
                WIDE
            );

            assert!(
                bank_spread(along) <= spread + 1e-4,
                "the wander puts the edge out to {} past a spread of {}",
                bank_spread(along),
                spread
            );

            let rim = bank_rim(along);

            assert!(
                (1.0 - BANK_RAGGED - 1e-4..=1.0 + 1e-4).contains(&rim),
                "the edge wanders to {} of its spread",
                rim
            );

            least = least.min(rim);
            most = most.max(rim);
        }

        assert!(
            most - least > BANK_RAGGED * 0.75,
            "the edge only wanders over {} of the {} it is allowed",
            most - least,
            BANK_RAGGED
        );
    }

    /// Spec 0117: each question given puts the end one span further out.
    #[test]
    fn a_question_adds_a_span() {
        let every = Asked::every();
        let mut trail = Trail::new();
        let mut was = reach(&trail);
        let span = 1.0 / (every.len() + 1) as f32;

        for one in every {
            trail.gave(*one);

            let now = reach(&trail);

            assert!(
                (now - was - span).abs() < 1e-6,
                "a question moved the end by {} and a span is {}",
                now - was,
                span
            );

            was = now;
        }

        assert!(
            (was - 1.0).abs() < 1e-6,
            "five questions and it ends at {}",
            was
        );
    }

    /// Spec 0119: the ledger says what was done and then what is left.
    ///
    /// **And every station has both lines**, which is checked here rather
    /// than in `says` because a line is nothing without the station it
    /// belongs to. Both matches are exhaustive, so a sixth question stops
    /// this compiling rather than leaving a row blank.
    #[test]
    fn the_stone_says_what_was_done_and_what_is_left() {
        let mut trail = Trail::new();

        assert_eq!(
            ledger(&trail),
            vec![crate::says::still_held(Asked::every()[0])],
            "a stone with nothing answered says something other than the \
             first question"
        );

        for (n, one) in Asked::every().iter().enumerate() {
            trail.gave(*one);

            let said = ledger(&trail);
            let done: Vec<&'static str> = Asked::every()[..=n]
                .iter()
                .map(|one| crate::says::given(*one))
                .collect();

            assert_eq!(
                said[..=n],
                done[..],
                "after {} answers the entries are not the {} rooms that \
                 answered, in the order they did",
                n + 1,
                n + 1
            );

            for (what, line) in Asked::every().iter().zip(&said) {
                assert!(!line.is_empty(), "{:?} has a blank line", what);
            }
        }
    }

    /// The rows of ink in a cut ledger, each as its top and bottom row of
    /// the picture.
    ///
    /// **Read off the picture and not off the arithmetic.** What the
    /// layout is supposed to do is one thing and what `cut_in` draws is
    /// another, and the gap between them is where every fault on this
    /// stone has lived: a line sized by its own width, a block placed by
    /// its top when it is measured from its bottom, a picture squeezed by
    /// the shape of what it is laid on.
    fn inked(cut: &blitzkit::texture::TextureData) -> Vec<(u32, u32)> {
        let (w, h) = (SAID_ACROSS, SAID_DOWN);
        let px = &cut.levels[0].pixels;
        let mut out = Vec::new();
        let mut run: Option<(u32, u32)> = None;

        for y in 0..=h {
            let any = y < h && (0..w).any(|x| px[((y * w + x) * 4 + 3) as usize] > 0);

            match (any, run) {
                (true, None) => run = Some((y, y)),
                (true, Some((a, _))) => run = Some((a, y)),
                (false, Some(was)) => {
                    out.push(was);
                    run = None;
                }
                (false, None) => {}
            }
        }

        out
    }

    /// Spec 0119: every row of the ledger is cut at one size.
    ///
    /// **Because `cut_in` takes a line down on its own to fit the face.**
    /// Left to itself it cuts a short entry larger than a long one, and
    /// five entries at five sizes is not a ledger. `ledger_tall` is what
    /// stops it, by sizing every row to the longest line the stone could
    /// ever say, and this reads the picture to check that it did.
    ///
    /// **And the gaps are even**, which is the other half of the same
    /// claim: one size with a pitch worked off something else would come
    /// out regular on one stone and not on the next.
    #[test]
    fn every_row_of_the_ledger_is_cut_at_one_size() {
        let rows = inked(&said_cut(&ledger(&done())));

        assert_eq!(
            rows.len(),
            super::rows(),
            "a finished trail cut {} rows of ink and the stone has {}",
            rows.len(),
            super::rows()
        );

        let tall: Vec<u32> = rows.iter().map(|(a, b)| b - a + 1).collect();
        let (least, most) = (
            tall.iter().copied().min().expect("rows"),
            tall.iter().copied().max().expect("rows"),
        );

        assert!(
            most - least <= 1,
            "the rows are cut from {} pixels tall to {}, which is five \
             sizes on one stone",
            least,
            most
        );

        let gaps: Vec<u32> = rows.windows(2).map(|two| two[1].0 - two[0].0).collect();
        let (near, far) = (
            gaps.iter().copied().min().expect("gaps"),
            gaps.iter().copied().max().expect("gaps"),
        );

        assert!(
            far - near <= 1,
            "the rows sit {} pixels apart at the closest and {} at the \
             widest",
            near,
            far
        );

        // and the rows are clear of each other and of the stone's edges,
        // which is what a pitch is for
        assert!(
            near > most,
            "the rows sit {} pixels apart and stand {} tall, so they touch",
            near,
            most
        );
        assert!(
            rows.first().expect("rows").0 > 0 && rows.last().expect("rows").1 < SAID_DOWN - 1,
            "the block runs off the stone, from {} to {} of {}",
            rows.first().expect("rows").0,
            rows.last().expect("rows").1,
            SAID_DOWN
        );
    }

    /// How tall a capital stands on the ledger, in metres.
    ///
    /// **Not asserted against a number, because how big is big enough is
    /// Jake's.** What is written down instead is where it comes from: the
    /// rows are cut at the smallest line the stone could ever say and
    /// width is what binds a line here, so the longest line is what sets
    /// the size of all five. Measured 2026-11-20 at 2.25 centimetres,
    /// against the 3.2 the one short line of spec 0118 had to itself and
    /// the same 2.25 its longest has always had. It read 1.9 while the
    /// copy markers were still in, and "about 2.8 once they are struck"
    /// was a guess, and wrong by half a centimetre: which is why the
    /// number here is one this test took rather than one anybody worked
    /// out.
    ///
    /// Widening the slab was tried and put back: see `SLAB_WIDE`.
    #[test]
    fn a_capital_on_the_ledger_is_worth_measuring() {
        let rows = inked(&said_cut(&ledger(&done())));
        let most = rows
            .iter()
            .map(|(a, b)| b - a + 1)
            .max()
            .expect("rows of ink");
        let on_the_stone = most as f32 / SAID_DOWN as f32 * SLAB_LONG;

        assert!(
            on_the_stone > 0.01,
            "a capital stands {:.3} metres on the stone, which is not \
             writing, it is grain",
            on_the_stone
        );
    }

    /// Spec 0119: an entry already written does not move.
    ///
    /// **Which is why the rows are laid out for five however many are on
    /// it.** A block centred on what it holds would walk every entry down
    /// the stone each time one was added.
    #[test]
    fn an_entry_already_written_does_not_move() {
        let tall = ledger_tall();
        let first = ledger_foot(0, tall);

        for row in 0..rows() {
            assert!(
                ledger_foot(row, tall) <= first,
                "row {} sits above the first one",
                row
            );
        }

        assert!(
            ledger_foot(0, tall) > ledger_foot(1, tall),
            "the rows do not run down the stone"
        );
        assert!(
            ledger_foot(rows() - 1, tall) > 0.0,
            "the last row's foot is off the bottom of the slab"
        );
        assert!(
            ledger_foot(0, tall) + tall < 1.0,
            "the first row runs off the top of the slab"
        );
    }

    /// Spec 0118 and 0119: nothing is held once every one has been given,
    /// and the entries stay.
    #[test]
    fn a_finished_trail_leaves_nothing_held() {
        let said = ledger(&done());

        assert_eq!(
            said.len(),
            Asked::every().len(),
            "a finished trail leaves {} lines on the stone",
            said.len()
        );
        assert!(
            held_line(&done()).is_none(),
            "a finished trail still holds something"
        );
        assert!(
            !said.contains(&crate::says::still_held(Asked::every()[0])),
            "a finished trail still says what it is waiting for"
        );
    }

    /// Spec 0119: the ledger never wants more rows than the slab has.
    ///
    /// A row is filled by a question answered and the last by the one
    /// still held, so four answered is four and one, and five answered is
    /// five and none. It cannot be six.
    #[test]
    fn the_ledger_never_wants_more_rows_than_it_has() {
        let mut trail = Trail::new();

        assert!(ledger(&trail).len() <= rows(), "an empty trail overflows");

        for one in Asked::every() {
            trail.gave(*one);

            assert!(
                ledger(&trail).len() <= rows(),
                "{} answered wants {} rows and there are {}",
                trail.given().len(),
                ledger(&trail).len(),
                rows()
            );
        }
    }

    /// Spec 0119: the ledger's picture is the slab's own shape.
    ///
    /// **Because `said_mesh` lays it on corner to corner.** A picture of
    /// another aspect stretches every letter, which is the thing
    /// `cut_in`'s `stretch` exists to undo on the panel above and which
    /// this avoids by matching.
    #[test]
    fn the_ledgers_picture_is_the_slabs_own_shape() {
        let picture = SAID_ACROSS as f32 / SAID_DOWN as f32;
        let slab = SLAB_WIDE / SLAB_LONG;

        assert!(
            (picture / slab - 1.0).abs() < 0.01,
            "the picture is {:.3} wide to tall and the slab {:.3}",
            picture,
            slab
        );
    }

    /// Spec 0119: a span is laid rather than switched on.
    ///
    /// **One frame is the control.** What this replaced was a step
    /// function, so a test that only checked the end arrives would have
    /// passed against the code that had the fault.
    #[test]
    fn a_span_takes_time_to_arrive() {
        let every = Asked::every();
        let span = 1.0 / (every.len() + 1) as f32;
        let mut trail = Trail::new();
        let was = reach(&trail);

        trail.gave(every[0]);

        let owed = reach(&trail);
        let dt = 1.0 / 60.0;

        assert!(
            winding(was, owed, dt) < owed - span * 0.5,
            "one frame of sixty got more than half the span in"
        );

        let mut drawn = was;
        let mut frames = 0;

        while drawn < owed && frames < 60 * 60 {
            drawn = winding(drawn, owed, dt);
            frames += 1;
        }

        let took = frames as f32 * dt;

        assert!(
            (took - WINDS).abs() < dt * 2.0,
            "a span took {}s and `WINDS` is {}",
            took,
            WINDS
        );
        assert_eq!(
            drawn, owed,
            "it stopped near what was owed rather than on it, which is              what keeps a finished bridge's landing from being kept out              by a millionth"
        );
    }

    /// Spec 0119: there is a note for every span the deck grows in.
    ///
    /// **The one number this building writes down twice.** The notes are a
    /// list of ratios and nothing can derive them, so a sixth question
    /// would leave the sixth span silent and nothing else would say so.
    #[test]
    fn there_is_a_note_for_every_span() {
        assert_eq!(
            crate::noise::STEPS.len(),
            Asked::every().len(),
            "there are {} notes and {} questions",
            crate::noise::STEPS.len(),
            Asked::every().len()
        );
    }

    /// Spec 0117: what holds you up stops where what is drawn stops.
    ///
    /// **Both ways round.** Deck past the treads is a step into the air on
    /// something you can see; treads past the deck is standing on nothing.
    #[test]
    fn you_cannot_walk_past_the_end() {
        for trail in [Trail::new(), nearly(), done()] {
            let far = reach(&trail);
            let foot = foot(REACHES);
            let ends = foot.x - far * RUN;
            let drawn = stations(REACHES, far);
            let stood = treads(REACHES, far);

            let past = drawn.iter().map(|at| at.x).fold(f32::MAX, f32::min);
            let held = stood.iter().map(|box_| box_.min.x).fold(f32::MAX, f32::min);

            assert!(
                (past - ends).abs() < 1e-3,
                "the deck ends at {} and the reach is {}",
                past,
                ends
            );
            assert!(
                (held - ends).abs() < TREAD,
                "what holds you up ends at {} and the deck at {}",
                held,
                ends
            );
        }
    }

    /// And the carry stops there too.
    #[test]
    fn nothing_carries_you_past_the_end() {
        let foot = foot(REACHES);
        let up = vec3(-1.0, 0.0, 0.0);

        for trail in [Trail::new(), nearly()] {
            let far = reach(&trail);
            let beyond = far + (1.0 - far) * 0.5;
            let at = vec3(foot.x - beyond * RUN, high(beyond), foot.z);

            assert_eq!(
                carries(REACHES, at, up, reach(&trail)),
                1.0,
                "it carried at {} of the run with {} built",
                beyond,
                far
            );
        }
    }

    /// Spec 0117: only the far end moves, so the foot never shifts.
    #[test]
    fn it_grows_from_the_far_end() {
        let foot = foot(REACHES);
        let mut trail = Trail::new();
        let mut was: Option<f32> = None;

        for one in Asked::every()
            .iter()
            .chain(std::iter::once(&Asked::every()[0]))
        {
            let near = treads(REACHES, reach(&trail))
                .iter()
                .map(|box_| box_.max.x)
                .fold(f32::MIN, f32::max);

            assert!(
                (near - foot.x).abs() < TREAD,
                "the foot is at {} and the bridge starts at {}",
                foot.x,
                near
            );

            if let Some(was) = was {
                assert!((was - near).abs() < 1e-4, "the foot moved");
            }

            was = Some(near);
            trail.gave(*one);
        }
    }

    /// **A sum, and it cannot tell you a term is empty.** This went green
    /// through the whole of the day `abutment` returned nothing at all,
    /// because the abutment was not one of the terms: a count that adds up
    /// the same calls `solid` makes says only that nothing is lost or
    /// counted twice on the way. What catches an empty one is a test that
    /// asks for a box and looks at it, which is what
    /// `the_bank_runs_out_where_the_deck_clears_your_head` and
    /// `nothing_stops_you_higher_than_the_bank_is_drawn` do.
    #[test]
    fn it_is_there_once_every_room_has_answered() {
        assert!(whole(&done()));
        assert_eq!(
            solid(REACHES, reach(&done())).len(),
            (RUN / TREAD).ceil() as usize
                + kerbs(REACHES, 1.0).len()
                + abutment(REACHES, 1.0).len()
                + landing(REACHES, reach(&done())).len()
                + piers(REACHES, reach(&done())).len()
                + runestone(REACHES, &done()).iter().count()
        );
    }

    /// Jake, twice: it reads as a slab with no reason to be standing, and
    /// then that it is still a thin slab. The answer both times was mass
    /// low down, so this holds the profile's shape rather than a number.
    #[test]
    fn the_stone_is_heaviest_where_it_leaves_the_ground() {
        let grass = RUNE_SUNK / RUNE_TALL;
        let (foot, _) = half_at(grass);
        let (shoulder, _) = half_at(0.78);

        assert!(
            foot > shoulder * 1.4,
            "{:.3} through at the grass line against {:.3} at the shoulder",
            foot,
            shoulder
        );

        // and it only ever loses width going up from there: a bulge
        // anywhere above the grass is a stone balanced on a waist
        let mut last = f32::INFINITY;

        for step in 0..=60 {
            let v = grass + (0.92 - grass) * step as f32 / 60.0;
            let (now, _) = half_at(v);

            assert!(now <= last + 1e-4, "it widens again at {:.3}", v);
            last = now;
        }

        // and it is a slab and not a post: wider across the face than it
        // is through, all the way up
        for step in 0..=20 {
            let v = grass + (0.92 - grass) * step as f32 / 20.0;
            let (through, across) = half_at(v);

            assert!(across > through * 1.15, "it is square at {:.3}", v);
        }
    }

    /// The carved panel keeps one width down a stone that does not, which
    /// is what stops the name reading as if seen from below.
    #[test]
    fn the_carved_panel_is_one_width_all_the_way_down() {
        let (mut least, mut most) = (f32::INFINITY, 0.0f32);

        for step in 0..=40 {
            let v = FACE_FROM + (FACE_TO - FACE_FROM) * step as f32 / 40.0;
            let arc = face_wide(v) * girth(v);

            least = least.min(arc);
            most = most.max(arc);
        }

        assert!(
            most - least < least * 0.01,
            "the band runs {:.4} to {:.4} across the face",
            least,
            most
        );
    }

    /// Jake: the stones around it do not make visual sense unless it was a
    /// gravel or rubble pile it sat on. A pile banks against what it was
    /// heaped against and thins to nothing at its edge; a ring of stones
    /// at one height is the thing he was looking at.
    #[test]
    fn the_rubble_banks_against_the_stone_and_thins_outward() {
        let (at, _) = runestone_at(30.0);
        let heap = rubble(30.0, &done());

        assert_eq!(heap.len(), RUBBLE);

        let reach = |where_: Vec3| {
            ((where_.x - at.x).powi(2) + (where_.z - at.z).powi(2)).sqrt() / RUBBLE_OUT
        };
        let mut near = (0.0f32, 0);
        let mut far = (0.0f32, 0);

        for (where_, size, _) in &heap {
            let top = where_.y + size.y * 0.5;

            if reach(*where_) < 0.5 {
                near = (near.0 + top, near.1 + 1);
            } else {
                far = (far.0 + top, far.1 + 1);
            }
        }

        assert!(
            near.1 > 10 && far.1 > 10,
            "{} in close, {} out",
            near.1,
            far.1
        );

        let (near, far) = (near.0 / near.1 as f32, far.0 / far.1 as f32);

        assert!(
            near > far * 1.8,
            "it stands {:.3} in close and {:.3} at the rim",
            near,
            far
        );
    }

    /// The bank and the pieces lying on it have to agree about where the
    /// edge is. They did not: the pieces read the rim at the opposite
    /// direction, so round half the heap they lay out past it.
    #[test]
    fn nothing_lies_off_the_edge_of_the_bank() {
        let (at, _) = runestone_at(30.0);

        for (where_, size, _) in rubble(30.0, &done()) {
            let off = ((where_.x - at.x).powi(2) + (where_.z - at.z).powi(2)).sqrt();
            let round = (where_.z - at.z).atan2(where_.x - at.x);
            let rim = RUBBLE_OUT * mound_rim(round);

            assert!(
                off <= rim + size.x * 0.5,
                "a piece sits {:.3} out where the bank ends at {:.3}",
                off,
                rim
            );
        }
    }

    /// And the bank itself keeps off the ramp, which the pieces alone
    /// cannot say: its edge wanders a fifth further than its nominal
    /// reach, and the clearance was worked out without that.
    #[test]
    fn the_bank_keeps_off_the_bridge() {
        let foot = foot(30.0);
        let (at, _) = runestone_at(30.0);

        for step in 0..360 {
            let round = step as f32 * std::f32::consts::TAU / 360.0;
            let out = RUBBLE_OUT * mound_rim(round);

            assert!(out <= rubble_reaches() + 1e-4, "the rim reaches {:.3}", out);

            let edge = vec3(at.x + round.cos() * out, 0.0, at.z + round.sin() * out);
            let aside = (edge.z - foot.z) - WIDE * 0.5;
            let behind = edge.x - foot.x;

            // how far that is from the deck's own footprint, which is
            // the half strip running out in -x within `WIDE` of the
            // line. **Not `aside > 0 || behind > 0`**, which is only
            // whether it is off the deck: at the offset this had, the
            // bank came within five centimetres of the ramp and that
            // test said nothing.
            let clear = match (aside > 0.0, behind > 0.0) {
                (true, true) => aside.hypot(behind),
                (true, false) => aside,
                (false, true) => behind,
                (false, false) => 0.0,
            };

            assert!(
                clear >= RUBBLE_CLEARS,
                "the bank comes within {:.3} of the ramp at {:.2} radians",
                clear,
                round
            );
        }
    }

    /// And it stops under the runes. A footing lapping the first row is a
    /// name that has to be read round.
    #[test]
    fn no_rubble_laps_the_carved_band() {
        let (at, _) = runestone_at(30.0);
        let band = at.y + (FACE_FROM - 0.5) * RUNE_TALL;

        for (where_, size, _) in rubble(30.0, &done()) {
            let top = where_.y + size.y * 0.5;

            assert!(
                top < band - 0.05,
                "a piece tops out at {:.3}, band at {:.3}",
                top,
                band
            );
        }
    }

    /// None of the heap is solid, so any of it over the ramp is rubble you
    /// walk through. It ran a metre onto the deck when the stone's offset
    /// was its own number rather than the heap's reach.
    #[test]
    fn the_rubble_keeps_off_the_bridge() {
        let foot = foot(30.0);

        for (where_, size, _) in rubble(30.0, &done()) {
            let (wide, deep) = (size.x * 0.5, size.z * 0.5);
            let aside = (where_.z - deep - foot.z) - WIDE * 0.5;
            let behind = (where_.x - wide) - foot.x;

            assert!(
                aside > 0.1 || behind > 0.1,
                "a piece sits {:.3} aside and {:.3} behind the foot",
                aside,
                behind
            );
        }
    }

    #[test]
    fn it_rises_out_of_the_field_and_levels_off() {
        assert_eq!(high(0.0), 0.0, "it starts on the ground");
        assert!((high(1.0) - RISE).abs() < 0.001, "it reaches its full rise");

        // level over the last stretch, which is what makes the top a place
        // to stand rather than an edge
        assert!((high(LEVELS_AT) - high(1.0)).abs() < 0.001);

        // and climbing all the way up to there
        let mut was = -1.0;
        for step in 0..=40 {
            let now = high(step as f32 / 40.0 * LEVELS_AT);
            assert!(now > was, "it climbs the whole way to the level part");
            was = now;
        }
    }

    /// The first shape here was a quarter circle, which rises fastest where
    /// it leaves the ground: 1.29 over its first 1.46, a wall to walk at.
    #[test]
    fn it_leaves_the_ground_at_a_walkable_angle() {
        let step = RUN / PIECES as f32;
        let up = high(1.0 / PIECES as f32);

        assert!(
            up < step * 0.34,
            "the foot rises {} over {}, which is a wall to walk at",
            up,
            step
        );
    }

    /// The one that decides whether any of this can be used. A collider is
    /// a box that cannot tilt, so the arc is a staircase to walk on, and a
    /// riser over `walk::STEP` is a wall with a view.
    #[test]
    fn no_tread_rises_more_than_a_step() {
        let treads = treads(REACHES, 1.0);
        let mut worst: f32 = 0.0;

        for pair in treads.windows(2) {
            let up = pair[1].max.y - pair[0].max.y;
            worst = worst.max(up);
        }

        assert!(
            worst < crate::walk::STEP,
            "the steepest riser is {} and a walk steps {}",
            worst,
            crate::walk::STEP
        );
    }

    /// And the control for it, so the test above is known to be able to
    /// fail: four times the tread is a staircase nobody climbs.
    #[test]
    fn a_coarse_tread_would_not_be_walkable() {
        let foot = foot(REACHES);
        // **A length of its own, not a multiple of `TREAD`.** Written
        // as four times the real one, this control tracked whatever
        // `TREAD` became: when the tread went to 0.12 to take the
        // jitter out, the control went to 0.48, which is still
        // walkable, and the test that was meant to prove the fine
        // tread matters stopped proving anything. A metre is a coarse
        // tread whatever the real one is doing.
        let coarse = 1.0;
        let steps = (RUN / coarse).ceil() as usize;
        let mut worst: f32 = 0.0;
        let mut was = 0.0;

        for step in 0..steps {
            let to = ((step + 1) as f32 * coarse / RUN).min(1.0);
            let top = high(to);
            worst = worst.max(top - was);
            was = top;
        }

        let _ = foot;
        assert!(
            worst > crate::walk::STEP,
            "the control did not go past a step"
        );
    }

    #[test]
    fn the_treads_lie_under_the_drawn_deck() {
        let treads = treads(REACHES, 1.0);
        let foot = foot(REACHES);

        // one curve, two lists off it: a tread's top and the deck's height
        // at the same place have to be the same thing
        for box_ in &treads {
            let along = (foot.x - box_.center().x) / RUN;

            assert!(
                (box_.max.y - high(along)).abs() < 0.2,
                "a tread at {} along tops out at {} and the deck is at {}",
                along,
                box_.max.y,
                high(along)
            );
        }
    }

    #[test]
    fn every_band_is_the_same_width_and_they_fill_it() {
        let made = deck_mesh(REACHES, 1.0);
        let foot = foot(REACHES);
        let (mut south, mut north) = (f32::MAX, f32::MIN);

        for one in made.vertices.iter() {
            south = south.min(one.position[2]);
            north = north.max(one.position[2]);
        }

        assert!(
            (north - south - WIDE).abs() < 0.001,
            "the deck is {:.3} across and should be {:.3}",
            north - south,
            WIDE
        );
        assert!(
            ((north + south) * 0.5 - foot.z).abs() < 0.001,
            "and it is centred on {:.3} rather than {:.3}",
            (north + south) * 0.5,
            foot.z
        );
    }

    /// Spec 0117: and no gap the other way, every drawn piece having tread
    /// under it.
    ///
    /// **The criterion that spec shipped without a test for.** Its pair,
    /// `you_cannot_walk_past_the_end`, holds that the colliders stop where
    /// the drawing does. This is the same claim the other way about: the
    /// drawing stops where the colliders do, so there is no stretch of
    /// deck you can see and fall through.
    #[test]
    fn every_piece_drawn_can_be_stood_on() {
        let mut trail = Trail::new();

        for one in Asked::every()
            .iter()
            .chain(std::iter::once(&Asked::every()[0]))
        {
            let far = reach(&trail);
            // the far end is the smallest x, the bridge running west
            let drawn = deck_mesh(REACHES, far)
                .vertices
                .iter()
                .map(|v| v.position[0])
                .fold(f32::MAX, f32::min);
            let stood = treads(REACHES, far)
                .iter()
                .map(|box_| box_.min.x)
                .fold(f32::MAX, f32::min);

            assert!(
                // **An allowance of `DEEP` and not of `TREAD`.**
                // `stations` ends the centre line exactly at `reach`, so
                // the walking surface does not overhang at all. What does
                // is the cut end's own face: a slab `DEEP` thick on a
                // slope reaches past its own top by `DEEP` times the sine
                // of the pitch, which at a reach of two sevenths measured
                // 0.133 and is underside rather than anything you could
                // stand on and fall through.
                drawn >= stood - DEEP,
                "with {} answered the deck is drawn to {} and the last \
                 tread ends at {}",
                trail.given().len(),
                drawn,
                stood
            );

            trail.gave(*one);
        }
    }

    /// Spec 0054: no two runes of the name are the same shape.
    ///
    /// **Because two of them were.** `fe` and `oss` are a stave with two
    /// branches each, and both were drawn with the branches falling to
    /// the right: the same glyph to within a few hundredths of a square,
    /// so BIFROST read b i f r f s t for as long as the stone has stood.
    /// Jake found it by looking at it, which is the only way anything
    /// here could have, since nothing reads these strokes but the chisel.
    ///
    /// Compared as sets of strokes, because the order they are cut in is
    /// not part of the letter.
    #[test]
    fn no_two_runes_of_the_name_are_the_same_shape() {
        let rounded = |one: &[(f32, f32, f32, f32)]| {
            let mut cut: Vec<[i32; 4]> = one
                .iter()
                .map(|(a, b, c, d)| {
                    [
                        (a * 20.0).round() as i32,
                        (b * 20.0).round() as i32,
                        (c * 20.0).round() as i32,
                        (d * 20.0).round() as i32,
                    ]
                })
                .collect();

            cut.sort();
            cut
        };

        for (one, first) in RUNES.iter().enumerate() {
            for (other, second) in RUNES.iter().enumerate().skip(one + 1) {
                assert_ne!(
                    rounded(first),
                    rounded(second),
                    "runes {} and {} of the name are cut the same",
                    one,
                    other
                );
            }
        }
    }

    /// Spec 0118: the stone is there before anything has been answered.
    ///
    /// **Because it is the one thing out there that says what any of this
    /// is**, and it used to arrive once the bridge was whole, which is
    /// after you have stopped needing telling. `shades` made this argument
    /// one object along for the clearing in the trees: a thing that
    /// appears the moment somebody finishes the building is worse to watch
    /// than a thing that was always there.
    #[test]
    fn the_stone_stands_before_the_first_answer() {
        let nothing = Trail::new();

        assert!(
            runestone(REACHES, &nothing).is_some(),
            "the stone is not there until the bridge is"
        );
        assert!(
            !rubble(REACHES, &nothing).is_empty(),
            "its footing is not there until the bridge is"
        );
        assert_eq!(
            runestone(REACHES, &nothing),
            runestone(REACHES, &done()),
            "the stone moves when the trail fills up"
        );
    }

    /// And its face names the first thing not given, and then nothing.
    #[test]
    fn the_stone_names_what_is_still_held() {
        let mut trail = Trail::new();

        for one in Asked::every() {
            assert_eq!(
                still_held(&trail),
                Some(*one),
                "with {:?} given it does not name {:?}",
                trail.given(),
                one
            );
            assert!(held_line(&trail).is_some(), "{:?} has no line to cut", one);

            trail.gave(*one);
        }

        assert_eq!(
            still_held(&trail),
            None,
            "it still names something once every room has answered"
        );
        assert_eq!(held_line(&trail), None, "and still has a line to cut");
    }

    /// And it names what is left rather than what has been done, whatever
    /// order the rooms were answered in.
    ///
    /// **The one that would catch a `given().last()`.** Naming the first
    /// not-given and naming the one after the last given are the same
    /// answer for anybody who walks the list in order, which is what the
    /// test above does, so it cannot tell them apart.
    #[test]
    fn it_names_what_is_left_and_not_what_was_last_done() {
        let mut trail = Trail::new();

        // the fifth one first, which nothing in the order forbids
        trail.gave(Asked::every()[4]);

        assert_eq!(
            still_held(&trail),
            Some(Asked::every()[0]),
            "answering the fifth made it name something other than the first"
        );
    }

    /// Spec 0118: a span wears the room that paid for it.
    #[test]
    fn a_span_wears_the_room_that_paid_for_it() {
        let mut trail = Trail::new();

        for (nth, one) in Asked::every().iter().enumerate() {
            trail.gave(*one);

            let strip = span_colours(&trail);
            let want = room_colour(*one);
            let at = (nth + 1) * 4;
            let got = [
                strip.levels[0].pixels[at] as f32 / 255.0,
                strip.levels[0].pixels[at + 1] as f32 / 255.0,
                strip.levels[0].pixels[at + 2] as f32 / 255.0,
            ];

            for c in 0..3 {
                assert!(
                    (got[c] - want[c]).abs() < 1.0 / 255.0 + 1e-4,
                    "span {} wears {:?} and {:?} paid for it, which is {:?}",
                    nth + 1,
                    got,
                    one,
                    want
                );
            }
        }
    }

    /// And the first span is nobody's: it was there before anything was
    /// asked, per spec 0117, so it keeps the kerb's own colour.
    #[test]
    fn the_first_span_is_nobodys() {
        for trail in [Trail::new(), done()] {
            let strip = span_colours(&trail);

            for c in 0..3 {
                assert!(
                    (strip.levels[0].pixels[c] as f32 / 255.0 - crate::aim::KERB[c]).abs()
                        < 1.0 / 255.0 + 1e-4,
                    "the first span is painted by somebody"
                );
            }
        }
    }

    /// And the strip is a record of your own walk rather than of how much
    /// of it is done.
    ///
    /// **The one that would catch a progress bar.** Painting span `k` by
    /// `Asked::every()[k - 1]` instead of by what was actually answered
    /// `k`th gives the same strip to anybody who walks the list in order,
    /// which is what `a_span_wears_the_room_that_paid_for_it` does.
    #[test]
    fn the_kerbs_are_a_record_of_your_own_walk() {
        let every = Asked::every();
        let mut one = Trail::new();
        let mut other = Trail::new();

        // the same two rooms, the other way about. Nothing in the order
        // forbids either, these two having no door between them.
        one.gave(every[1]);
        one.gave(every[4]);
        other.gave(every[4]);
        other.gave(every[1]);

        assert_ne!(
            span_colours(&one).levels[0].pixels,
            span_colours(&other).levels[0].pixels,
            "two walks that answered the same rooms in different orders \
             paint the kerbs the same"
        );
    }

    /// And every span has a slot to be painted in.
    #[test]
    fn there_is_a_slot_for_every_span() {
        let grows_in = Asked::every().len() + 1;

        assert_eq!(
            SPANS, grows_in,
            "the strip holds {} colours and the deck grows in {} spans",
            SPANS, grows_in
        );
        assert_eq!(span_colours(&done()).width() as usize, SPANS);
    }

    /// Spec 0117: and there is no lane under the bank to drop into.
    ///
    /// **Because half of it was missing.** The box under the deck was
    /// written from the middle out and mirrored only for the bands
    /// outside it, so it ran from the deck's centre line to one edge and
    /// the other three metres had nothing. You could walk in under the
    /// deck on that side, follow it down toward the foot, and be wedged
    /// between the field and the soffit with the bank's outer bands
    /// standing beside you as a wall. Jake, 2026-11-18, stuck coming up
    /// from under the bridge, which is the fault spec 0117 exists to
    /// fix and had not fixed.
    ///
    /// Asked of the boxes rather than by walking, because a walk proves
    /// one path and a gap is a place.
    #[test]
    fn there_is_no_lane_under_the_bank() {
        let foot = foot(REACHES);
        let boxes = abutment(REACHES, 1.0);
        let (_, ends) = bank_runs();
        let half = WIDE * 0.5;

        for step in 0..=160 {
            let along = ends * step as f32 / 160.0;
            let top = bank_high(along);

            if top <= 0.25 {
                continue;
            }

            let x = foot.x - along * RUN;

            for across in -8..=8 {
                // inside the deck's own edge rather than on it: a body
                // has a radius, and the kerb stands on that line anyway
                let z = foot.z + half * 0.98 * across as f32 / 8.0;
                // **Asked at shin height and not just under the top.** The
                // boxes are as high as the lower end of the stretch they
                // cover, so near the bank's steep dying back a box's top
                // can sit a riser under `bank_high` and an answer taken
                // from the top alone fails on a bank that is there. What
                // the fault was is a lane you can walk into, so the
                // question is whether anything is in the way at all.
                let y = foot.y + 0.15;
                let held = boxes.iter().any(|box_| {
                    box_.min.x <= x
                        && x <= box_.max.x
                        && box_.min.z <= z
                        && z <= box_.max.z
                        && box_.min.y <= y
                        && y <= box_.max.y
                });

                assert!(
                    held,
                    "nothing fills the bank at {:.2} {:.2} {:.2}, a share \
                     {:.4} along and {:.2} off the middle",
                    x,
                    y,
                    z,
                    along,
                    z - foot.z
                );
            }
        }
    }

    /// Spec 0054: what you see is what you stand on.
    ///
    /// **Jake, 2026-11-04: the base of the bridge should be flush with the
    /// grass, not a step.** It was worse than the base. The drawn deck was
    /// hung `DEEP * 0.5` above its own middle rather than below it, so the
    /// whole of it floated a third of a metre over its own treads: a step
    /// up out of the field at the foot, a step down onto spec 0055's
    /// landing at the top, and shin deep the length of it.
    #[test]
    fn the_deck_you_see_is_the_deck_you_stand_on() {
        let foot = foot(REACHES);

        for middle in stations(REACHES, 1.0) {
            let along = (foot.x - middle.x) / RUN;

            assert!(
                (middle.y - high(along)).abs() < 1e-4,
                "at {:.3} along, what you see tops out at {:.3} and what \
                 holds you up is at {:.3}",
                along,
                middle.y,
                high(along)
            );
        }
    }

    /// Spec 0054: and it comes out of the grass rather than off a kerb.
    #[test]
    fn it_springs_flush_from_the_grass() {
        // the top of what is drawn, which is what you would step up onto
        let lowest = deck_mesh(REACHES, 1.0)
            .vertices
            .iter()
            .map(|one| one.position[1])
            .fold(f32::MIN, f32::max)
            .min(
                stations(REACHES, 1.0)
                    .into_iter()
                    .map(|middle| middle.y)
                    .fold(f32::MAX, f32::min),
            );

        assert!(
            lowest < 0.05,
            "the near end of it stands {:.3} proud of the field",
            lowest
        );
    }

    /// Spec 0054: the seven run into each other rather than meeting at an
    /// edge.
    ///
    /// **Jake's other half of the same note**, on 2026-11-04: the colours
    /// between the bands should blend together, like a real rainbow.
    ///
    /// Two halves, and the first alone passes on a picture of one colour.
    /// Neighbouring texels have to be close, which is the blend, *and* the
    /// picture has to go somewhere over its width, which is what says
    /// there is still a rainbow in it.
    #[test]
    fn the_rainbow_blends_rather_than_steps() {
        let made = rainbow();
        let pixels = &made.levels[0].pixels;
        let mut biggest = 0i32;
        let mut over_all = 0i32;

        for at in 1..RAINBOW_WIDE as usize {
            for channel in 0..3 {
                let one = pixels[at * 4 + channel] as i32;
                let last = pixels[(at - 1) * 4 + channel] as i32;

                biggest = biggest.max((one - last).abs());
            }
        }

        for channel in 0..3 {
            let (mut low, mut high) = (255i32, 0i32);

            for at in 0..RAINBOW_WIDE as usize {
                let one = pixels[at * 4 + channel] as i32;
                low = low.min(one);
                high = high.max(one);
            }

            over_all = over_all.max(high - low);
        }

        assert!(
            biggest < 12,
            "two texels next to each other differ by {}, which is a band \
             edge and not a blend",
            biggest
        );
        assert!(
            over_all > 120,
            "the whole picture only moves by {}, so it is one colour and \
             not a rainbow",
            over_all
        );
    }

    /// Spec 0054: and every one of the seven is still in it, where it was.
    ///
    /// The stops sit at the middles of the bands they used to fill, so the
    /// colour at those seven places is what it always was and only what is
    /// between them is new.
    #[test]
    fn every_colour_of_the_rainbow_is_still_in_the_picture() {
        let made = rainbow();
        let pixels = &made.levels[0].pixels;

        for (n, want) in COLOURS.iter().enumerate() {
            let at = ((n as f32 + 0.5) / BANDS as f32 * RAINBOW_WIDE as f32) as usize;
            let at = at.min(RAINBOW_WIDE as usize - 1) * 4;

            for (channel, one) in [want.x, want.y, want.z].iter().enumerate() {
                let got = pixels[at + channel] as f32 / 255.0 * GLOW;

                assert!(
                    (got - one).abs() < 0.03,
                    "band {} channel {} came out {:.3} and is {:.3}",
                    n,
                    channel,
                    got,
                    one
                );
            }
        }
    }

    /// Walked rather than measured, which is the lesson of the pool's
    /// steps: four legal risers in a row were one solid block you could
    /// not get out of, and every rise passed.
    /// Spec 0055: nothing of the threshold exists until the building has
    /// been done, on the same condition and from the same list as the deck.
    ///
    /// A watchman on a bridge that is not there is a man standing in a
    /// field.
    #[test]
    fn no_threshold_until_the_building_is_done() {
        assert!(
            landing(REACHES, reach(&Trail::new())).is_empty(),
            "the landing is up before anything has been given"
        );
        assert!(
            landing(REACHES, reach(&nearly())).is_empty(),
            "the landing is up one question short"
        );
        assert!(
            !landing(REACHES, reach(&done())).is_empty(),
            "the building is done and there is nothing at the top"
        );
    }

    /// Spec 0055: you step out onto it rather than up onto it.
    #[test]
    fn the_landing_is_flush_with_the_deck() {
        let slab = landing(REACHES, reach(&done()))[0];

        assert!(
            (slab.max.y - top(REACHES).y).abs() < 1e-4,
            "the landing's top is {:.3} and the deck's is {:.3}",
            slab.max.y,
            top(REACHES).y
        );
    }

    /// Spec 0055, and the fault this building keeps making: two faces at
    /// the walking surface stop a walk dead and `walk` reports nothing.
    #[test]
    fn the_landing_laps_the_deck() {
        let slab = landing(REACHES, reach(&done()))[0];
        let top = top(REACHES);

        assert!(
            slab.max.x > top.x + 1e-3,
            "the landing stops at {:.3} and the deck ends at {:.3}, which \
             is two faces in one plane",
            slab.max.x,
            top.x
        );
    }

    /// Spec 0055: wider than the deck, on both sides.
    #[test]
    fn the_landing_is_wider_than_the_deck() {
        let slab = landing(REACHES, reach(&done()))[0];
        let top = top(REACHES);

        for (side, got) in [
            ("north", slab.max.z - (top.z + WIDE * 0.5)),
            ("south", (top.z - WIDE * 0.5) - slab.min.z),
        ] {
            assert!(
                got > 1.0,
                "the landing stands {:.2} proud of the deck on the {}",
                got,
                side
            );
        }
    }

    /// Spec 0055: the parapet closes three sides and leaves the way you
    /// arrive open, and spec 0056's way through is cut in the far one.
    #[test]
    fn the_parapet_leaves_the_arriving_side_open() {
        let built = landing(REACHES, reach(&done()));
        let top = top(REACHES);
        let rails = &built[1..];

        // four pieces over three sides, because the far one is split
        // round the way through to spec 0056
        assert_eq!(rails.len(), 4, "a parapet of {} pieces", rails.len());

        // nothing of it stands across the lane you walk in on, which is the
        // deck's own width at the landing's east edge
        for rail in rails {
            let across = rail.min.z < top.z + WIDE * 0.5 && rail.max.z > top.z - WIDE * 0.5;
            let at_the_east = rail.max.x > built[0].max.x - 1e-3;

            assert!(
                !(across && at_the_east),
                "a rail closes the way you arrive over: {:?} to {:?}",
                rail.min,
                rail.max
            );
        }
    }

    /// Spec 0056: and there is a way through the far side.
    ///
    /// A gate with no gap in it is a wall. This was found by the walk that
    /// crosses into Valhalla fetching up eight tenths of a metre short of
    /// the edge, against a rail built to stop somebody falling.
    #[test]
    fn the_far_parapet_has_a_way_through_it() {
        let built = landing(REACHES, reach(&done()));
        let top = top(REACHES);
        // the far ones are the thin-in-x pieces. Taken by where they
        // start instead, the north and south rails come too: they run the
        // full corner, so they begin at the same west face.
        let far: Vec<_> = built[1..]
            .iter()
            .filter(|rail| rail.max.x - rail.min.x < RAIL_THICK * 1.5)
            .collect();

        assert_eq!(far.len(), 2, "the far side is in {} pieces", far.len());

        let gap = far
            .iter()
            .map(|rail| {
                if rail.max.z < top.z {
                    rail.max.z
                } else {
                    rail.min.z
                }
            })
            .collect::<Vec<_>>();
        let wide = (gap[0] - gap[1]).abs();

        assert!(
            wide > crate::RADIUS * 3.0,
            "the way through is {:.2} and a body is {:.2}",
            wide,
            crate::RADIUS * 2.0
        );
        // and it is on the line you walk in on
        assert!(
            ((gap[0] + gap[1]) * 0.5 - top.z).abs() < 1e-3,
            "the way through is off the middle by {:.2}",
            ((gap[0] + gap[1]) * 0.5 - top.z).abs()
        );
    }

    /// Spec 0054: nothing is sown where the bridge comes down, or where
    /// the stone stands.
    ///
    /// **The field's scatter has only ever known about the building.**
    /// `field::open` keeps trees west of the garden's face and asks
    /// nothing else, so one grew through the runestone and the low
    /// stretch of the deck ran through whatever was under it.
    ///
    /// Swept over the cells round the foot rather than checked at a
    /// point, because a scatter is the one thing you cannot reason about
    /// from one sample.
    #[test]
    fn nothing_is_sown_where_the_bridge_comes_down() {
        let foot = foot(REACHES);
        let (stone, size) = runestone_at(REACHES);
        let mut looked = 0;

        for cell in crate::field::cells_round(foot, 46.0) {
            for stood in crate::field::trees(REACHES, cell) {
                looked += 1;

                let off_stone = (stood.at.x - stone.x)
                    .abs()
                    .max((stood.at.z - stone.z).abs() - size.z * 0.5);

                assert!(
                    off_stone > 1.0,
                    "a tree is sown {:.2} from the runestone, at {:?}",
                    off_stone,
                    stood.at
                );

                let along = (foot.x - stood.at.x) / RUN;

                if (0.0..=1.0).contains(&along)
                    && (stood.at.z - foot.z).abs() < WIDE * 0.5
                    && high(along) < crate::field::TREE_TALL
                {
                    panic!(
                        "a tree is sown under the deck at {:?}, where it is \
                         {:.2} up and a tree is {:.2}",
                        stood.at,
                        high(along),
                        crate::field::TREE_TALL
                    );
                }
            }
        }

        assert!(
            looked > 20,
            "only {} trees anywhere near the bridge, so this test is not \
             looking at a field",
            looked
        );
    }

    /// Spec 0055: you can walk off the end of the deck onto it.
    #[test]
    fn walked_from_the_top_of_the_deck_onto_the_landing() {
        let solid = solid(REACHES, reach(&done()));
        let top = top(REACHES);
        // standing on the last of the level stretch, walking on west
        let start = vec3(top.x + 6.0, top.y + 0.6, top.z);
        let ended = walked(
            start,
            vec3(top.x - LANDING_OUT * 0.5, top.y, top.z),
            12.0,
            &solid,
        );

        assert!(
            ended.x < top.x - 1.0,
            "it got to {:.2} and the deck ends at {:.2}, so it never \
             stepped off onto the landing",
            ended.x,
            top.x
        );
        assert!(
            (ended.y - top.y).abs() < 0.4,
            "it ended {:.2} up and the landing is at {:.2}",
            ended.y,
            top.y
        );
    }

    /// Spec 0055: and the parapet stops you walking off the far edge.
    ///
    /// Twenty-six metres up, which is the whole reason for a kerb.
    #[test]
    fn the_parapet_stops_you_at_the_far_edge() {
        let solid = solid(REACHES, reach(&done()));
        let top = top(REACHES);
        let slab = landing(REACHES, reach(&done()))[0];
        // off the middle line, because the middle of the far side is the
        // way through to spec 0056 and is meant to be open
        let aside = top.z + WAY_WIDE * 0.5 + crate::RADIUS * 2.0;
        let start = vec3(top.x - 1.0, top.y + 0.4, aside);
        // aimed well past the far parapet
        let ended = walked(start, vec3(slab.min.x - 20.0, top.y, aside), 10.0, &solid);

        assert!(
            ended.x > slab.min.x,
            "it walked out to {:.2} and the landing stops at {:.2}",
            ended.x,
            slab.min.x
        );
        assert!(
            ended.y > top.y - 1.0,
            "it ended {:.2} up, off a landing at {:.2}",
            ended.y,
            top.y
        );
    }

    /// Spec 0055: he is on the landing, not in the air and not in the
    /// parapet.
    #[test]
    fn the_watchman_stands_on_the_landing() {
        let built = landing(REACHES, reach(&done()));
        let slab = built[0];
        let (at, _) = watchman(REACHES);

        assert!(
            (at.y - slab.max.y).abs() < 1e-4,
            "he stands at {:.3} and the landing's top is {:.3}",
            at.y,
            slab.max.y
        );
        assert!(
            at.x > slab.min.x && at.x < slab.max.x && at.z > slab.min.z && at.z < slab.max.z,
            "he stands at {:?}, off a landing from {:?} to {:?}",
            at,
            slab.min,
            slab.max
        );

        // and his own width is clear of every rail, since a body is 0.9
        for rail in &built[1..] {
            let near = (at.x - at.x.clamp(rail.min.x, rail.max.x)).abs()
                + (at.z - at.z.clamp(rail.min.z, rail.max.z)).abs();

            assert!(
                near > crate::RADIUS,
                "he stands {:.2} from a rail, and he is {:.2} across",
                near,
                crate::RADIUS * 2.0
            );
        }
    }

    /// Spec 0055: facing back down the climb.
    ///
    /// Taken the way `draw_the_people` takes it, where ahead of a figure is
    /// `(sin, 0, -cos)` of its yaw. Written as a number here it would say
    /// nothing: a quarter turn is only east because of that line.
    #[test]
    fn the_watchman_faces_the_climb() {
        let (at, facing) = watchman(REACHES);
        let ahead = vec3(facing.sin(), 0.0, -facing.cos());
        let climb = (foot(REACHES) - at).normalize_or_zero();

        assert!(
            ahead.dot(climb) > 0.9,
            "he faces {:?} and the climb comes up from {:?}",
            ahead,
            climb
        );
    }

    /// Spec 0055: and clear of the way you arrive, so that you are met
    /// rather than blocked.
    #[test]
    fn the_watchman_is_not_in_the_way() {
        let (at, _) = watchman(REACHES);
        let top = top(REACHES);
        let off = (at.z - top.z).abs();

        assert!(
            off > crate::RADIUS * 2.0,
            "he stands {:.2} off the middle of a lane {:.2} wide, and a \
             body is {:.2} across",
            off,
            WIDE,
            crate::RADIUS * 2.0
        );
        assert!(
            off < WIDE * 0.5,
            "he stands {:.2} off the middle, which is outside the lane \
             you walk up: you would never pass him",
            off
        );
    }

    pub fn walked(from: Vec3, to: Vec3, seconds: f32, solid: &[Aabb]) -> Vec3 {
        let step = 1.0 / 60.0;
        let mut at = from;
        let mut falling = 0.0;

        for _ in 0..(seconds / step) as u32 {
            let way = vec3(to.x - at.x, 0.0, to.z - at.z);
            let wish = if way.length_squared() > 1e-4 {
                let going = way.normalize();

                // **Carried where the game carries you**, or this walks
                // the bridge at a pace nobody playing ever walks it at
                // and the time it reports is a time nobody waits.
                going * crate::SPEED * super::carries(REACHES, at, going, reach(&done()))
            } else {
                Vec3::ZERO
            };

            let (next, fell) =
                crate::walk::walk(at, wish, falling, crate::RADIUS, step, solid, true);
            at = next;
            falling = fell;
        }

        at
    }

    /// The deck carries you up itself, and only up, and only on itself.
    #[test]
    fn the_deck_carries_you_up_and_nothing_else_does() {
        let foot = foot(REACHES);
        let up = vec3(-1.0, 0.0, 0.0);
        let down = vec3(1.0, 0.0, 0.0);
        let on = vec3(foot.x - RUN * 0.5, high(0.5), foot.z);

        assert_eq!(carries(REACHES, on, up, reach(&done())), CARRIES);
        assert_eq!(
            carries(REACHES, on, down, reach(&done())),
            1.0,
            "it pushes you back"
        );
        // halfway along is inside five sixths, so one question short it
        // carries you there now. Spec 0117, and
        // `nothing_carries_you_past_the_end` holds the other side.
        assert_eq!(
            carries(REACHES, on, up, reach(&nearly())),
            CARRIES,
            "five sixths of a bridge does not carry you at halfway"
        );

        let aside = vec3(on.x, on.y, foot.z + WIDE);
        assert_eq!(
            carries(REACHES, aside, up, reach(&done())),
            1.0,
            "off the side"
        );

        let under = vec3(on.x, 0.0, foot.z);
        assert_eq!(
            carries(REACHES, under, up, reach(&done())),
            1.0,
            "underneath it"
        );

        let past = vec3(foot.x - RUN * 1.4, on.y, foot.z);
        assert_eq!(
            carries(REACHES, past, up, reach(&done())),
            1.0,
            "past the top"
        );

        let behind = vec3(foot.x + 6.0, 1.4, foot.z);
        assert_eq!(
            carries(REACHES, behind, up, reach(&done())),
            1.0,
            "short of the foot"
        );
    }

    #[test]
    fn walked_from_the_foot_to_the_top() {
        let solid = solid(REACHES, reach(&done()));
        let foot = foot(REACHES);
        let top = vec3(foot.x - RUN + 2.0, RISE, foot.z);

        // dropped in from above the foot rather than placed, because a
        // point on a deck can be inside it
        // **Nine seconds and not a minute.** Jake: it takes way too long
        // to run up the rainbow bridge, and a test that gave it a
        // minute could never have told anybody that. This is a walk
        // rather than a run, so `CARRIES` is the whole of what gets it
        // up inside the window: unassisted, 4.2 a second over a hundred
        // metres is twenty four.
        //
        // It was six, which fitted while the climb was getting a reach
        // of free travel out of `walk::over_a_step` at every one of its
        // 0.12 treads. Taking that away is what `walk::RIDES` did, and
        // a walk up here is now the 8.5 seconds the pace says it is.
        let start = vec3(foot.x - 1.0, 1.4, foot.z);
        let ended = walked(start, top, 9.0, &solid);

        assert!(
            ended.y > RISE * 0.9,
            "walked for nine seconds and got to {} up, and the top is {}",
            ended.y,
            RISE
        );
        // **0.92 and not 0.4.** Unassisted this walk covers 37.8 in nine
        // seconds, so a bound under that passed whether the deck carried
        // you or not and said nothing about the thing it is there for.
        assert!(
            foot.x - ended.x > RUN * 0.92,
            "got {} of the way along a run of {}",
            foot.x - ended.x,
            RUN
        );
    }

    /// The control. Without it the walk above passes for a bridge that is
    /// one flat slab at the height of the foot.
    #[test]
    fn a_walk_off_the_side_does_not_climb() {
        let solid = solid(REACHES, reach(&done()));
        let foot = foot(REACHES);
        let beside = vec3(foot.x - 1.0, 1.4, foot.z + WIDE);
        let ended = walked(beside, vec3(foot.x - RUN, 0.0, foot.z + WIDE), 60.0, &solid);

        assert!(
            ended.y < 1.0,
            "walking beside it climbed to {}, so the deck is wider than it says",
            ended.y
        );
    }

    #[test]
    fn it_crosses_nothing() {
        // west of where the garden's ground stops, every piece of it, so
        // it never lands on the garden or on the building behind it
        let stops = crate::field::starts(REACHES);

        for one in deck_mesh(REACHES, 1.0).vertices.iter() {
            assert!(
                one.position[0] < stops,
                "a corner of it at {} is east of the field's start at {}",
                one.position[0],
                stops
            );
        }
    }

    #[test]
    fn it_stays_inside_the_ground_it_stands_on() {
        let stops = crate::field::starts(REACHES);
        let out = treads(REACHES, 1.0)
            .iter()
            .map(|box_| stops - box_.min.x)
            .fold(f32::MIN, f32::max);

        assert!(
            out < crate::field::GROUND_REACH,
            "it runs {} out and the ground reaches {}",
            out,
            crate::field::GROUND_REACH
        );
    }
}

#[cfg(test)]
mod look {
    #[test]
    #[ignore]
    fn write_the_carving() {
        let made = super::carving();
        let (wide, tall) = (made.width(), made.height());
        let mut out = format!("P6\n{} {}\n255\n", wide, tall).into_bytes();

        for texel in made.levels[0].pixels.chunks(4) {
            let a = texel[3] as f32 / 255.0;

            for under in [120u8, 60, 44] {
                out.push((255.0 * a + under as f32 * (1.0 - a)) as u8);
            }
        }

        let to = std::env::temp_dir().join("carving.ppm");
        std::fs::write(&to, out).unwrap();
        eprintln!("WROTE {}", to.display());
    }
}

/// The piers along the landing's parapet, and the railing between them.
///
/// **A kerb you can count along.** Spec 0055's note on the photograph:
/// the parapet was one bar of grey from corner to corner, 0.62 high, and
/// a surface with nothing marking its length reads as one thing however
/// long it is. Piers at intervals in a warm stone, iron between them,
/// and the kerb becomes the plinth it already was.
///
/// **And it gives the landing a height.** At 0.62 you are standing at
/// the edge of a twenty-six metre drop behind something knee high, which
/// the walk tests say you cannot fall over and the eye does not believe
/// for a moment. The railing reaches `RAIL_HIGH`, which is a hand's
/// height, and you can still see over it because that is what the
/// landing is for.
/// **A pier stands through the railing and not under it.** At 0.30 on a
/// kerb of 0.62 its top came to 0.92 and the rail runs at 1.05, so the
/// iron passed over every pier and what the eye read was a railing with
/// some red blocks behind it. In the photograph the stone is the tall
/// thing and the iron stops at it, which is also what holds a railing
/// up.
/// **And it is sunk into the kerb rather than stood on it.** A pier is
/// wider than the parapet it marks, so its underside oversails the kerb
/// by three centimetres each side and those two strips share a plane
/// with the kerb's own top. Two drawn faces in one plane is the fault
/// this building has found eight times. Sunk by `PIER_SINKS` the
/// underside is inside the kerb and there is nothing to fight.
pub const PIER_SINKS: f32 = 0.02;
pub const PIER_WIDE: f32 = 0.46;
pub const PIER_UP: f32 = 1.16;
pub const PIER_CAP: f32 = 0.10;
pub const RAIL_HIGH: f32 = 1.05;
pub const BALUSTER: f32 = 0.055;

/// And the stone is the tall thing, which is checked where the numbers
/// are rather than in a test.
///
/// **A test comparing two constants is a test of nothing at run time.**
/// Clippy says so and it is right: these are known when the file is
/// compiled, so the place to refuse them is there. Change either number
/// the wrong way and the build stops.
const _: () = assert!(
    PIER_UP + PIER_CAP > RAIL_HIGH + BALUSTER,
    "a pier has to stand through the railing it carries, not under it"
);

/// How far apart the balusters are, centre to centre.
///
/// **Close enough to read as a railing from the foot of the deck.** At a
/// hundred metres a bar every half metre is a grey smear and a bar every
/// 0.14 is a line somebody drew, which is what the photograph's is. It
/// costs a draw each and they are the cheapest thing in the building.
pub const BALUSTER_STEP: f32 = 0.14;

/// Each run of the landing's parapet, as the line it follows: where it
/// starts, where it ends, and how high the kerb under it is.
///
/// Taken off `landing` rather than written again, so a parapet that
/// moves takes its railing with it.
pub fn parapet_runs(reaches: f32, far: f32) -> Vec<(Vec3, Vec3)> {
    landing(reaches, far)
        .into_iter()
        .skip(1)
        .map(|box_| {
            let (low, high) = (box_.min, box_.max);
            let along_x = high.x - low.x > high.z - low.z;
            let middle = box_.center();

            if along_x {
                (
                    vec3(low.x, high.y, middle.z),
                    vec3(high.x, high.y, middle.z),
                )
            } else {
                (
                    vec3(middle.x, high.y, low.z),
                    vec3(middle.x, high.y, high.z),
                )
            }
        })
        .collect()
}

/// A pier standing on the parapet: the foot of it, which is a little
/// inside the kerb's top.
///
/// One at each end of every run, so two runs meeting at a corner put a
/// pier on the corner rather than leaving the two railings to meet in
/// mid air.
pub fn piers(reaches: f32, far: f32) -> Vec<Vec3> {
    let mut out: Vec<Vec3> = Vec::new();

    for (from, to) in parapet_runs(reaches, far) {
        for end in [from, to] {
            let foot = end - Vec3::Y * PIER_SINKS;

            if !out.iter().any(|at| at.distance(foot) < PIER_WIDE) {
                out.push(foot);
            }
        }
    }

    out
}

/// And the balusters, as the middle of each one.
///
/// **Laid out from the middle of a run outward**, so the gaps either end
/// are equal and a run that changes length does not slide every bar
/// along it.
///
/// **And cleared against the piers rather than against the run's own
/// ends.** Two runs meeting at a corner have ends 0.24 apart and share
/// one pier, so a bar kept clear of its own run's end still stood inside
/// the stone of the other run's. `no_baluster_stands_inside_a_pier`
/// found it. The piers are placed first and the iron fits between them,
/// which is both the order a mason works in and the only way the two
/// cannot disagree.
pub fn balusters(reaches: f32, far: f32) -> Vec<Vec3> {
    let piers = piers(reaches, far);
    let clear = (PIER_WIDE + BALUSTER) * 0.5;
    let mut out = Vec::new();

    for (from, to) in parapet_runs(reaches, far) {
        let along = to - from;
        let fits = (along.length() / BALUSTER_STEP) as u32;
        let spread = fits as f32 * BALUSTER_STEP;
        let way = along.normalize_or_zero();
        let middle = (from + to) * 0.5;

        for n in 0..=fits {
            let at = middle + way * (n as f32 * BALUSTER_STEP - spread * 0.5);
            let inside = piers
                .iter()
                .any(|pier| (at.x - pier.x).abs() < clear && (at.z - pier.z).abs() < clear);

            if !inside {
                out.push(at);
            }
        }
    }

    out
}

#[cfg(test)]
mod railing_tests {
    use super::*;
    use crate::trail::Asked;

    const REACHES: f32 = 20.0;

    fn done() -> Trail {
        let mut trail = Trail::new();

        for one in Asked::every() {
            trail.gave(*one);
        }

        trail
    }

    /// Spec 0055: a pier at every end of every run, and one where two
    /// runs meet rather than two in the same place.
    ///
    /// **Six and not eight.** The landing's parapet is four runs with
    /// eight ends, and two pairs of those ends are the same corner. A
    /// pier drawn twice is a pier with a face in its own plane.
    #[test]
    fn a_corner_has_one_pier_and_not_two() {
        let piers = piers(REACHES, reach(&done()));

        assert_eq!(piers.len(), 6, "four runs, eight ends, two corners shared");

        for (n, one) in piers.iter().enumerate() {
            for other in piers.iter().skip(n + 1) {
                assert!(
                    one.distance(*other) > PIER_WIDE,
                    "two piers at {:?} and {:?} overlap",
                    one,
                    other
                );
            }
        }
    }

    /// Spec 0014: and its underside is not in the plane of the kerb's top.
    #[test]
    fn a_pier_is_sunk_into_the_kerb_it_stands_on() {
        let top = top(REACHES).y + RAIL_UP;

        for at in piers(REACHES, reach(&done())) {
            assert!(
                at.y < top - 1e-3,
                "a pier foot at {:.3} sits in the plane of the kerb top at {:.3}",
                at.y,
                top
            );
        }
    }

    /// Spec 0055: every baluster stands between two piers and none in one.
    #[test]
    fn no_baluster_stands_inside_a_pier() {
        let piers = piers(REACHES, reach(&done()));

        for bar in balusters(REACHES, reach(&done())) {
            for pier in piers.iter() {
                let apart = glam::vec2(bar.x - pier.x, bar.z - pier.z).abs();

                assert!(
                    apart.x > (PIER_WIDE + BALUSTER) * 0.5 - 1e-3
                        || apart.y > (PIER_WIDE + BALUSTER) * 0.5 - 1e-3,
                    "a baluster at {:?} stands inside the pier at {:?}",
                    bar,
                    pier
                );
            }
        }
    }
}

#[cfg(test)]
mod chisel_tests {
    use super::*;

    /// What the carving comes to on screen at an hour, which is the
    /// colour times the fill the engine multiplies it by.
    fn on_screen(share: f32) -> glam::Vec3 {
        let since = crate::garden::when(share);

        alight(since).truncate() * crate::garden::filled(since)
    }

    /// Spec 0054: and it burns hardest in the dark.
    ///
    /// **Written because it was backwards and read as right.** Every
    /// number round `alight` says the carving burns at night, and the
    /// fill the engine multiplies the colour by falls faster than those
    /// numbers rise: at midnight the carving was a third of white and at
    /// noon it was past it. Nothing in the arithmetic looked wrong,
    /// because the half that undid it lived in the shader.
    #[test]
    fn the_carving_burns_brightest_at_night() {
        let noon = on_screen(0.25).max_element();
        let dusk = on_screen(0.5).max_element();
        let midnight = on_screen(0.75).max_element();

        assert!(
            midnight > dusk && dusk > noon,
            "the carving comes to {:.2} at noon, {:.2} at dusk and {:.2} at \
             midnight, which is not a thing that burns in the dark",
            noon,
            dusk,
            midnight
        );

        // and past one, which is the whole of what a glow is here: a
        // channel clipped at the top of the canvas with its own colour
        // round it. Under one is a painted mark.
        assert!(
            midnight > 2.0,
            "at midnight the carving comes to {:.2}, which is lit like any \
             other paint",
            midnight
        );
    }

    /// Spec 0054: the glow round a cut carries the ember's colour and
    /// the cut itself does not.
    ///
    /// **Because the canvas clips and cannot tell them apart.** Past one
    /// every channel comes out the same white, so a glow that is gold in
    /// the groove and orange at its edge has to say so in the picture.
    /// Writing the skirt white is one line and leaves a pale wash round
    /// the runes that reads as fog.
    #[test]
    fn the_glow_round_a_cut_is_warmer_than_the_cut() {
        let cut = carving();
        let pixels = &cut.levels[0].pixels;
        let mut whole = None;
        let mut skirt = None;

        // **Above the name, because a part covered texel is not always a
        // skirt.** This read the whole picture and called anything between
        // the two alphas skirt. The name is cut after the halo now, so its
        // holes survive `HALO_OUT`, and that leaves its own antialiased
        // edges part covered and white: a cut, not a glow, and the last
        // one the scan met. What carries a skirt is the ribbon, and the
        // ribbon is everything above the lettering.
        let lettering = (CARVED as f32 * (LATIN_FOOT + LATIN_TALL * FACE_SQUEEZE)) as u32;

        for i in 0..((CARVED - lettering) * CARVED) as usize {
            let (red, green, alpha) = (pixels[i * 4], pixels[i * 4 + 1], pixels[i * 4 + 3]);

            match alpha {
                255 => whole = Some((red, green)),
                a if a > 16 && a < 64 => skirt = Some((red, green)),
                _ => (),
            }
        }

        let whole = whole.expect("nothing was cut whole");
        let skirt = skirt.expect("nothing carries a faint skirt");

        assert_eq!(whole, (255, 255), "a texel cut whole is not white");
        assert!(
            skirt.1 < whole.1 / 2,
            "the skirt's green is {} against the cut's {}, so the glow is \
             the same colour as what it comes off",
            skirt.1,
            whole.1
        );
    }

    /// Spec 0054: the carving has edges and not steps.
    ///
    /// **Because the blockiness was the alpha and not the width.** Every
    /// texel the chisel touched was written at 255 and every one it
    /// missed stayed at nought, so a stroke at any angle but a right one
    /// came out a staircase. Jake read the stone and said so. A part
    /// covered texel says how much of it was cut now, which the blended
    /// pass `aim::RUNE_GLAZE` puts the face in can actually show.
    ///
    /// Asked of the picture, because that is where the fault was. The
    /// same change made to the two width constants and checked against
    /// each other passed while altering nothing.
    #[test]
    fn the_carving_is_cut_with_edges_and_not_steps() {
        let cut = carving();
        let pixels = &cut.levels[0].pixels;
        let mut soft = 0;
        let mut hard = 0;

        // **Over what the halo can reach, which is the trap this test
        // fell into first.** `halo` lays a soft skirt round everything
        // cut, so part covered texels exist whether or not a stroke has
        // an edge, and the first version of this passed its own control.
        // The skirt tops out at `255 * HALO`, so anything above that and
        // short of solid came off a chisel.
        let skirt = (255.0 * HALO) as u8;

        for i in 0..(CARVED * CARVED) as usize {
            match pixels[i * 4 + 3] {
                255 => hard += 1,
                a if a > skirt => soft += 1,
                _ => (),
            }
        }

        assert!(hard > 0, "nothing was cut at all");

        // **Against the length of an edge and not the area inside it.**
        // This asked for a tenth as many part cut as cut whole, which
        // compares a perimeter with an area: raising `CARVED` four times
        // multiplies the inside by sixteen and the edge by four, and the
        // test failed on a picture that had got better. A row of the
        // face is a perimeter-sized number and holds at any resolution.
        assert!(
            soft > CARVED as usize,
            "{} texels are part cut against {} cut whole, which is a \
             picture drawn in steps",
            soft,
            hard
        );
    }

    /// And the writing is cut finer than the ribbon round it.
    ///
    /// **A width is a distance now.** It used to be a whole number of
    /// texels, so 0.0085 against 0.0115 was 1.09 against 1.47, both
    /// rounded to 2, and the two chisels were one. Nothing quantises it
    /// any more, so the only thing to hold is the order, and the place
    /// to hold that is where the numbers are.
    const _: () = assert!(
        RUNE_STROKE < STROKE,
        "the writing inside the ribbon is the finer work"
    );
}
