//! A village out in the field, with people living in it. Spec 0068.
//!
//! **South of the way out, where the bridge is west.** Spec 0054's bridge runs
//! west along the gap's own middle line and the whole of the trail ends up
//! there. A village on that line is on the way to something; this one is off
//! it, so walking to it is a choice rather than a step.
//!
//! Nothing here is behind a door. The cabins are solid lumps with lit windows,
//! as spec 0056's outbuildings are: the hall at the top of the bridge is the
//! one building in this game you go into. What this place is for is being
//! walked through, and everything it has to show is on the ground.

use std::f32::consts::{FRAC_PI_2, PI, TAU};

use blitzkit::collision::Aabb;
use glam::{vec3, Vec3};

/// How far west of the garden's ground the village's middle stands, and how
/// far south of the way out.
///
/// **Measured off the things it must not touch.** The bridge's foot is
/// `bridge::OUT` west of the garden on the way's own line, and it climbs west
/// from there, so south is the free direction. Forty-four is far enough that
/// the garden's wall does not frame it and near enough that you can see it
/// from the gap and decide to go.
///
/// **That last clause was not true for a long time.** Jake, 2026-11-15: the
/// village seems sort of just randomly out there with no obvious path to it
/// from anywhere until it is already in view. Standing in the gap and aiming
/// straight at this place put a stand of birch across the whole of it, and
/// seventy-two metres of undifferentiated grass under that. The distance was
/// never the problem and moving it would not have helped. Spec 0086 keeps
/// the promise two ways instead: `path` wears a line from the gap to where
/// the street's ruts run out, and `along_the_path` is what stops the field
/// sowing anything across the view down it.
pub const WEST: f32 = 56.0;
pub const SOUTH: f32 = 46.0;

/// The middle of the street, on the ground.
pub fn at(reaches: f32) -> Vec3 {
    vec3(
        crate::field::starts(reaches) - WEST,
        0.0,
        crate::garden::at(reaches).z - SOUTH,
    )
}

/// How long the street runs and how wide it is between the fronts.
///
/// **Ten across, which is two wagons and a walk.** A wagon here is 2.4 wide
/// and the rule this building furnishes by wants 1.05 clear of everything for
/// a body: a street of six would be a corridor with carts in it.
pub const STREET_LONG: f32 = 58.0;
pub const STREET_WIDE: f32 = 10.0;

/// What the staged shot of the village is asked for by, and the name of the
/// file it lands in.
///
/// The one account of it, like `beyond::slugged`: `--screenshot=village` and
/// `media/village.png` and nowhere else says so.
pub const SHOT: &str = "village";

/// What hour the staged shot is taken at, and under which sky.
///
/// **A shot of this place is a shot of a sky**, which no other staged shot
/// in this building is: the rest are in the hall or behind its wall. The
/// first day the building ever has is a storm, so the hour is not enough on
/// its own and `garden::under` is what finds a day that is both.
///
/// Mid-morning, with the sun off the street's own line, so the fronts down
/// one side are lit and the other side is in shade. Noon down a street that
/// runs east and west is a street with no shadow in it.
pub const SHOT_AT: f32 = 0.28;
pub const SHOT_SKY: &str = "fair";

/// How long a day lasts while the shot is being taken.
///
/// **Its own number rather than `tour::FILM_DAY`.** The two agree today and
/// are not the same question: that one is how long a fifty-eight second film
/// may take to cross one evening, this one is how little five seconds of
/// staging may move the sky. The photographer runs a game for five seconds
/// before it draws, which at `garden::DAY` is a quarter of a day: the first
/// take of this shot asked for mid-morning and got the middle of the night.
pub const SHOT_DAY: f32 = 20.0 * 60.0;

/// Where a staged shot of the village stands, which way it looks and how far
/// down, per spec 0071.
///
/// **Said as shares of the street rather than as metres.** A picture of this
/// place framed by three numbers in metres is a picture that comes apart the
/// day a cabin moves, which is the fault `around` was written to answer one
/// paragraph below. A third of the way out from the middle is a third of the
/// way out however long the street gets.
///
/// Down the street from inside its east end, which is the end you would
/// arrive at, and an eighth of the width off the middle so the ruts leave
/// the frame at a corner rather than running up the centre of it. Aimed a
/// little under the eye, because the street is what the picture is of and
/// the sky is not.
///
/// **A quarter of the width off was too far**, and in the way Jake has
/// already named once, on the arcade's own still: the near cabin on that
/// side stood across a third of the frame, and the picture's job is the
/// street rather than a cabin.
///
/// **And a third of the way out was too near the wagon.** Spec 0086. At
/// 0.34 the east cart stood 5.7 metres down the street and 5.1 across,
/// which is close enough to be seen end on and cropped by the frame: a
/// wheel, a man, and no wagon. Spec 0084 put a sack in his hands and the
/// picture still read as somebody wrestling a cartwheel, because what was
/// wrong with it by then was the camera. At 0.42 the whole cart is in and
/// side on, which is what makes the errand legible.
///
/// Half the width off instead of 0.12, for the same reason 0.12 replaced
/// 0.25 and in the other direction: standing back put the timber stacked
/// against the near cabin into the left of the frame as a dark mass. The
/// offset still has its original job, which is keeping the ruts out of
/// the middle, and 0.06 is the least that still does it. Nought was tried
/// and runs them straight up the centre.
pub fn posed(reaches: f32) -> (Vec3, f32, f32) {
    let middle = at(reaches);
    let stand = middle + vec3(STREET_LONG * 0.42, 0.0, STREET_WIDE * 0.06);
    let look = middle + vec3(-STREET_LONG * 0.24, crate::EYE - 0.3, STREET_WIDE * 0.1);
    let way = look - (stand + Vec3::Y * crate::EYE);
    let flat = vec3(way.x, 0.0, way.z).length();

    (stand, way.x.atan2(-way.z), way.y.atan2(flat))
}

/// And how far the whole place reaches, which is what keeps trees out of it
/// and what the tests measure against.
///
/// **Worked out from what is in it rather than written down.** A second
/// number for how big the village is drifts from the first the moment a
/// cabin moves, and the first build had exactly that: a box that said 38
/// metres while the barn stood at 43.
pub fn around(reaches: f32) -> Aabb {
    let middle = at(reaches);
    let mut box_ =
        Aabb::from_center_size(middle + Vec3::Y * 5.0, vec3(STREET_LONG, 10.0, STREET_WIDE));

    for one in built(reaches) {
        let walls = one.footprint();
        box_ = Aabb::new(box_.min.min(walls.min), box_.max.max(walls.max));
    }

    // and a margin, so the edge of it is not the wall of a cabin
    Aabb::new(
        box_.min - vec3(5.0, 0.0, 5.0),
        box_.max + vec3(5.0, 0.0, 5.0),
    )
}

/// Whether this is inside the village, for the field to keep its trees and
/// its flowers out.
///
/// **A share wider than `around`**, because a tree whose trunk is just
/// outside still has a crown over the roof.
pub fn shades(reaches: f32, at: Vec3) -> bool {
    let box_ = around(reaches);
    let out = 3.0;

    at.x > box_.min.x - out
        && at.x < box_.max.x + out
        && at.z > box_.min.z - out
        && at.z < box_.max.z + out
}

/// One building: the middle of its footprint, how far it runs each way, and
/// how high its eaves and its ridge stand.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Built {
    pub at: Vec3,
    /// East and west, and north and south.
    pub wide: f32,
    pub long: f32,
    pub eaves: f32,
    pub ridge: f32,
    /// Which way its gable looks, as a facing. The street is east and west,
    /// so the cabins face across it and the barn closes the end.
    pub faces: f32,
}

impl Built {
    /// The box of its walls, which is what you walk round.
    pub fn walls(&self) -> Aabb {
        Aabb::from_center_size(
            self.at + Vec3::Y * self.eaves * 0.5,
            vec3(self.wide, self.eaves, self.long),
        )
    }

    /// And the box its roof fills, which is what one building must not share
    /// with another.
    pub fn footprint(&self) -> Aabb {
        Aabb::from_center_size(
            self.at + Vec3::Y * self.ridge * 0.5,
            vec3(self.wide, self.ridge, self.long),
        )
    }
}

/// How far back from the street the fronts stand.
pub const SET_OFF: f32 = STREET_WIDE * 0.5;

/// The cabins and the barn.
///
/// **None of them the same size**, which is the whole of what tells a village
/// from a row of sheds. Spec 0056's steading has three outbuildings off a lane
/// and reads as a steading because they differ; four identical ones read as a
/// barracks, which is what this was on its first build.
pub fn built(reaches: f32) -> Vec<Built> {
    let middle = at(reaches);
    let one = |along: f32, side: f32, wide: f32, long: f32, eaves: f32, ridge: f32| Built {
        at: vec3(
            middle.x + along,
            middle.y,
            middle.z + side * (SET_OFF + long * 0.5),
        ),
        wide,
        long,
        eaves,
        ridge,
        // **Looking at the street, which is the other way round from the
        // first build.** A facing of nought in this building looks down
        // -z, so the row standing north of the street has to be PI to
        // look back at it. Written the obvious way, every door and every
        // window was on the back of its own cabin and the street was a
        // run of blank gables.
        faces: if side > 0.0 { 0.0 } else { PI },
    };

    vec![
        // the north side, nearest the way you come in
        one(-16.0, -1.0, 7.0, 6.0, 2.5, 4.1),
        one(-4.0, -1.0, 9.0, 7.0, 2.8, 4.6),
        one(9.0, -1.0, 6.0, 5.5, 2.4, 3.8),
        // and the south, set along differently so the street is not a
        // corridor of pairs, and with a wide gap left between the last two
        // for the common the fire stands on
        one(-22.0, 1.0, 6.5, 6.0, 2.5, 4.0),
        one(-11.0, 1.0, 8.0, 6.5, 2.6, 4.3),
        one(11.0, 1.0, 7.5, 6.0, 2.5, 4.2),
        // **And the barn, closing the west end from beyond it.** It was
        // put inside the street's own run on the first build, 15 deep
        // against a street 10 wide, so it lay across the thing it was
        // meant to close. `tests::the_street_is_clear` said so before
        // anything had been drawn.
        Built {
            at: vec3(middle.x - STREET_LONG * 0.5 - 8.0, middle.y, middle.z),
            wide: 12.0,
            long: 15.0,
            eaves: 4.2,
            ridge: 7.4,
            faces: FRAC_PI_2,
        },
    ]
}

/// How far clear of a thing somebody stands when their errand is at it.
pub const CLEAR_OF: f32 = 1.6;

/// The firepit in the middle of the street, and the ring round it.
pub const RING_WIDE: f32 = 2.4;

/// How far from the middle of the fire somebody cooking on it stands.
///
/// **Nearer than `CLEAR_OF`, and it has to be.** The general clearance keeps
/// a body out of a thing it is walking past, which put the cook 2.8 metres
/// from a pot and gave him an arm 0.7 long: spec 0067's stir came out as a
/// man waving at a fire from across the common. This is the ring's own edge
/// and half a stride, which is where you stand to reach what is over it.
pub const TENDS_FROM: f32 = RING_WIDE * 0.5 + 0.5;
pub const RING_HIGH: f32 = 0.3;
pub const RING_STONES: usize = 13;
pub const RING_FLAMES: usize = 3;

/// **On the common and not in the street.** It was in the middle of the way
/// through on the first build, which is where a village puts a fire in a
/// picture and nowhere a village would put one: a street is walked down
/// before it is looked at, and `tests::the_street_is_clear` said so.
///
/// The common is the wide gap left in the south side between the last two
/// cabins, which `fences` leaves open for the same reason.
pub fn ring(reaches: f32) -> Vec3 {
    let middle = at(reaches);

    vec3(middle.x + 0.5, middle.y, middle.z + SET_OFF + 3.6)
}

/// The stones round it, each a lump of its own.
pub fn ring_stones(reaches: f32) -> Vec<(Vec3, f32)> {
    let middle = ring(reaches);

    (0..RING_STONES)
        .map(|n| {
            let round = n as f32 / RING_STONES as f32 * TAU;
            // not evenly, because a ring of even stones is a wheel
            let out = RING_WIDE * 0.5 * (0.93 + 0.14 * (round * 2.0).sin());

            (
                vec3(
                    middle.x + round.cos() * out,
                    middle.y + RING_HIGH * 0.5,
                    middle.z + round.sin() * out,
                ),
                round,
            )
        })
        .collect()
}

/// How high the pot hangs over it and how big it is.
pub const POT_UP: f32 = 1.15;
pub const POT_WIDE: f32 = 0.78;

/// The tripod the pot hangs from: three legs meeting over the fire.
pub fn tripod(reaches: f32) -> Vec<(Vec3, Vec3)> {
    let middle = ring(reaches);
    let foot = RING_WIDE * 0.62;

    (0..3)
        .map(|n| {
            let round = n as f32 / 3.0 * TAU + 0.4;

            (
                vec3(
                    middle.x + round.cos() * foot,
                    middle.y,
                    middle.z + round.sin() * foot,
                ),
                vec3(middle.x, middle.y + POT_UP + 0.5, middle.z),
            )
        })
        .collect()
}

/// The cook pot that hangs over the fire.
///
/// **Turned, because a pot is.** `cellar::turned` is what everything round
/// in this building comes off and a box with a lid on it is a crate. Open
/// at the top and two sided, or you see through the far wall of it.
pub fn pot_mesh() -> blitzkit::mesh::MeshData {
    crate::cellar::turned(18, 12, |v| {
        // a base that flares, a belly, a shoulder and a lid that closes.
        // **Closed at the top**, because `turned` makes one surface and an
        // open pot is one you see the far wall through from above, which is
        // where a pot over a fire is looked at from.
        let waist = if v < 0.14 {
            0.6 + v / 0.14 * 0.34
        } else if v < 0.72 {
            0.94 + ((v - 0.14) / 0.58 * PI).sin() * 0.1
        } else {
            let t = (v - 0.72) / 0.28;

            0.94 * (1.0 - t * t)
        };

        (v, waist)
    })
}

/// Where the well stands, and the trough beside it.
pub const WELL_WIDE: f32 = 1.5;
pub const WELL_HIGH: f32 = 0.85;
pub const TROUGH_LONG: f32 = 2.6;
pub const TROUGH_WIDE: f32 = 0.8;
pub const TROUGH_HIGH: f32 = 0.55;

/// Against the fronts on the south side, in the gap between two cabins, so
/// it is off the lane a wagon takes and still in the street.
pub fn well(reaches: f32) -> Vec3 {
    let middle = at(reaches);

    vec3(
        middle.x - 16.5,
        middle.y,
        middle.z + SET_OFF - WELL_WIDE * 0.7,
    )
}

pub fn trough(reaches: f32) -> Aabb {
    let one = well(reaches);

    Aabb::from_center_size(
        vec3(one.x + 2.6, one.y + TROUGH_HIGH * 0.5, one.z),
        vec3(TROUGH_LONG, TROUGH_HIGH, TROUGH_WIDE),
    )
}

/// How big a wagon is: the bed, and the wheels at its corners.
pub const WAGON_LONG: f32 = 3.6;
pub const WAGON_WIDE: f32 = 2.0;
pub const WAGON_BED: f32 = 0.95;
pub const WAGON_SIDE: f32 = 0.55;
pub const WHEEL_WIDE: f32 = 1.1;
pub const WHEEL_THICK: f32 = 0.16;
pub const SPOKES: usize = 10;

/// Where the wagons stand and which way they are pointed.
///
/// **Off the street and not on it.** A wagon in the middle of the way
/// through is the thing `tests::the_street_is_clear` exists to stop: this
/// village is walked down before it is looked at.
pub fn wagons(reaches: f32) -> Vec<(Vec3, f32)> {
    let middle = at(reaches);

    vec![
        (
            vec3(middle.x + 14.0, middle.y, middle.z - SET_OFF + 1.1),
            0.12,
        ),
        (
            vec3(middle.x - 24.0, middle.y, middle.z + SET_OFF - 1.3),
            PI - 0.2,
        ),
    ]
}

/// The hitching rail: two posts and a bar between them.
pub const RAIL_LONG: f32 = 5.0;
pub const RAIL_HIGH: f32 = 1.15;
pub const RAIL_THICK: f32 = 0.14;

pub fn rail(reaches: f32) -> Vec3 {
    let middle = at(reaches);

    vec3(middle.x + 19.5, middle.y, middle.z + SET_OFF - 1.2)
}

/// How big a horse is, as four heights off the ground and two across.
///
/// **Measured against a body**, which is `people::TALL` at 1.78. A horse's
/// back is about 1.5 and it is a bit over two metres nose to tail, so the
/// one standing next to somebody is lower at the shoulder and a good deal
/// longer. Drawn smaller it reads as a dog.
///
/// **Each one is a height and not an offset**, which is the lesson of the
/// first build: the legs were given the back's height and hung from the
/// barrel's middle, so they ran twenty centimetres into the ground and the
/// whole animal read as a table somebody had left out.
pub const HORSE_LONG: f32 = 2.1;
pub const HORSE_WIDE: f32 = 0.6;
/// The top of the back, the bottom of the barrel, and so how deep it is.
pub const HORSE_BACK: f32 = 1.5;
pub const HORSE_BELLY: f32 = 0.82;
pub const HORSE_DEEP: f32 = HORSE_BACK - HORSE_BELLY;
/// And how high the head is carried.
pub const HORSE_HEAD: f32 = 1.82;

/// Where the horses stand, and which way each faces.
///
/// At the rail, nose to it, which is where a horse that has been left is.
pub fn horses(reaches: f32) -> Vec<(Vec3, f32)> {
    let one = rail(reaches);

    (0..2)
        .map(|n| {
            let along = (n as f32 - 0.5) * RAIL_LONG * 0.46;

            (vec3(one.x + along, one.y, one.z + 1.3), 0.0)
        })
        .collect()
}

/// The fences between the cabins, each as the two ends of a run.
///
/// **So the street is a street.** Without them the gaps between the cabins
/// are ways out of it, and a village you can leave sideways from anywhere is
/// a scattering of sheds.
pub const FENCE_HIGH: f32 = 1.05;
pub const FENCE_POSTS: f32 = 1.5;

pub fn fences(reaches: f32) -> Vec<(Vec3, Vec3)> {
    let houses = built(reaches);
    let mut out = Vec::new();

    for side in [-1.0f32, 1.0] {
        let mut along: Vec<&Built> = houses
            .iter()
            .filter(|one| one.faces != FRAC_PI_2 && (one.at.z - at(reaches).z).signum() == side)
            .collect();
        along.sort_by(|a, b| a.at.x.total_cmp(&b.at.x));

        for pair in along.windows(2) {
            let (left, right) = (pair[0], pair[1]);
            let z = at(reaches).z + side * (SET_OFF + 0.4);
            let from = left.at.x + left.wide * 0.5;
            let to = right.at.x - right.wide * 0.5;

            // **A gap wide enough to take a wagon is a way in and not a
            // hole to patch.** Fencing every gap closes the common the
            // fire stands on, and a village with one way into it is a
            // stockade.
            if to - from > WAGON_LONG * 2.0 {
                continue;
            }

            out.push((vec3(from, 0.0, z), vec3(to, 0.0, z)));
        }
    }

    out
}

/// The beaten earth of the street, which is what tells it from a field.
///
/// **A village has a track through it and a field does not.** Without this
/// the street was grass between two rows of cabins, which reads as a mown
/// gap rather than as the way everybody walks. Laid a hair over the field's
/// own ground, as every floor in this building is laid over what is under
/// it, because two surfaces in one plane is the fault this place keeps
/// making.
pub const TRACK_UP: f32 = 0.015;

pub fn track(reaches: f32) -> Aabb {
    let middle = at(reaches);

    Aabb::from_center_size(
        middle + Vec3::Y * (TRACK_UP - 0.05),
        vec3(STREET_LONG + 14.0, 0.1, STREET_WIDE),
    )
}

/// How wide the way there is worn, in how many pieces, and how far clear of
/// it the field keeps its trees.
///
/// **A footpath and not a cart track.** The street's own ruts are two lines
/// at a wagon's wheel spacing, because a cart goes up and down it. Nothing
/// drives out to the gap in the garden wall, so what wears this is feet: one
/// band, wide enough for two to pass, and `RUT_WIDE` deliberately not reused.
///
/// `PATH_CLEAR` is not the crown's reach but the sightline's. The village's
/// own keep-out is a share wider than `around` so a trunk just outside does
/// not hang a crown over a roof, which is a question about what is above
/// you. This one is a question about what you can see along: the whole
/// reason the way there exists is that the module header promises you can
/// see this place from the gap and decide to go, and the field had grown a
/// stand of birch across that line. Crowns meeting overhead are welcome and
/// do not close the view.
///
/// **Four and not two and a half**, which was tried and is in a picture.
/// Two and a half keeps the run itself clear of trunks, which is all a
/// footpath needs, and left a conifer standing square across the view of
/// the village from the gap. The run and the sightline are not the same
/// line: the run meets the street's east end and the village's middle is
/// most of thirty degrees west of it, so a tree beside the path can still
/// be in front of the place. Four is what took that one out. This clears
/// the view by happening to rather than by aiming, and a second keep-out
/// along gap to middle would be two bare stripes crossing in a field.
pub const PATH_WIDE: f32 = 1.2;
pub const PATH_PIECES: usize = 40;
pub const PATH_CLEAR: f32 = 4.0;

/// How far out of the gap the way there begins.
///
/// **Clear of the wall it comes through**, so it reads as leaving the gap
/// rather than as a floor laid in it. The garden's own ground is two
/// centimetres over the field's and covers whatever it overlaps, so a piece
/// started in the opening would be a piece nobody sees.
pub const PATH_OFF: f32 = 2.0;

/// The two ends of the way there: out of the gap, to where the street's own
/// ruts run out.
///
/// **It ends where the ruts do and not where the street does.** They run
/// seven metres past each end already, because a cart that stops at the
/// first cabin is a cart that was lowered in. Meeting them there is what
/// makes one way of getting here rather than two things that nearly touch.
pub fn path_runs(reaches: f32) -> (Vec3, Vec3) {
    let (from, to) = crate::field::way(reaches);
    let middle = at(reaches);

    (
        vec3(
            crate::field::starts(reaches) - PATH_OFF,
            0.0,
            (from + to) * 0.5,
        ),
        vec3(middle.x + (STREET_LONG + 14.0) * 0.5, 0.0, middle.z),
    )
}

/// The way there, as a run of short worn pieces: where each sits, how big it
/// is, and which way it is turned.
///
/// **Turned, because this one runs on the diagonal.** The street's ruts are
/// boxes along x and need no rotation; the gap is north east of the village
/// and nothing between them is square to anything. Each piece carries the
/// run's own heading, the way a wagon's wheels carry the cart's.
///
/// It wanders like the ruts do and for the same reason. A worn line that is
/// perfectly straight for fifty metres is a road somebody surveyed.
pub fn path(reaches: f32) -> Vec<(Vec3, Vec3, f32)> {
    let (from, to) = path_runs(reaches);
    let way = to - from;
    let long = way.length();
    let along = way / long;
    // across the run, which is the heading turned a quarter
    let across = vec3(-along.z, 0.0, along.x);
    let piece = long / PATH_PIECES as f32;

    (0..PATH_PIECES)
        .map(|n| {
            let down = (n as f32 + 0.5) * piece;
            // **Read off the distance and barely there.** The first build
            // seeded on the piece's number with a tenth of a radian step,
            // which is a fast wave, and threw each piece half a width
            // sideways of the last: a line of paving slabs dealt out by
            // hand. The ruts' own comment says what that looks like, since
            // they were a kerb down the street for the same reason. Long
            // waves and a tenth of a metre, so a join is a soft edge
            // rather than a notch.
            let wander = (down * 0.21).sin() * 0.14 + (down * 0.53).sin() * 0.07;
            let at = from + along * down + across * wander;

            (
                // laid on the field's own ground, which sits `field::DROP`
                // under the garden's, and only far enough proud of it to
                // keep the two out of one plane
                vec3(at.x, -crate::field::DROP + 0.002, at.z),
                // a third again as long as its step, so the wander never
                // opens a gap between one piece and the next
                vec3(piece * 1.33, 0.005, PATH_WIDE),
                // **Not `people::facing`'s angle**, which is the one every
                // other turn in this file is. That one answers which way a
                // body looks and its forward is `(sin, 0, -cos)`; this is
                // handed to `Quat::from_rotation_y`, which sends a box's
                // own x to `(cos, 0, -sin)`. The first build used the
                // facing and laid every piece across the run instead of
                // along it: a path of slabs dealt sideways, a metre out of
                // line at each join, which read as the wander being too
                // coarse and was nothing of the sort.
                (-along.z).atan2(along.x),
            )
        })
        .collect()
}

/// Whether this is on the way there, for the field to keep its trees off it.
///
/// **The nearest point on the run and not a box round it.** A box round a
/// diagonal fifty metres long is a box with most of a field in it, and the
/// trees it took out would be a rectangle of bare ground with a path
/// somewhere inside it.
pub fn along_the_path(reaches: f32, at: Vec3) -> bool {
    let (from, to) = path_runs(reaches);
    let way = to - from;
    let how_far = way.length_squared();

    if how_far < 1e-6 {
        return false;
    }

    let part = ((at - from).dot(way) / how_far).clamp(0.0, 1.0);
    let near = from + way * part;

    vec3(at.x - near.x, 0.0, at.z - near.z).length() < PATH_CLEAR
}

/// How wide a wheel rut is worn.
///
/// **A track and not two painted lines.** The street was one flat slab of
/// earth, which is a surface and not a place anybody has driven over. What
/// says a cart goes up and down here is a pair of ruts at a wagon's own
/// wheel spacing, and what stops them reading as road markings is that they
/// wander, the way a wheel follows the last wheel without following it
/// exactly.
///
/// They were 58 slabs a side until spec 0096 painted them into the track's
/// own face instead. `RUT_PIECES` went with them, and so did the comment
/// about keeping each piece barely proud of the track: nothing stands proud
/// of anything now.
pub const RUT_WIDE: f32 = 0.34;

/// One channel of a colour as this engine's textures hold it.
///
/// **sRGB and not the number.** `texture::FORMAT` is `Rgba8UnormSrgb`, so
/// the sampler decodes every texel on the way in, where an instance's own
/// colour is handed to the shader as it stands. Written straight, the
/// track's earth of 0.235 came back as 0.045 and the street was nearly
/// black: the same colour, read twice.
fn encoded(channel: f32) -> u8 {
    let channel = channel.clamp(0.0, 1.0);
    let srgb = if channel <= 0.003_130_8 {
        channel * 12.92
    } else {
        1.055 * channel.powf(1.0 / 2.4) - 0.055
    };

    (srgb * 255.0).round().clamp(0.0, 255.0) as u8
}

/// How far the rut on this side has wandered at this point along the street.
///
/// **One account of the curve**, because the track's own face is painted
/// from it and the pieces used to be laid from it, and two would have come
/// apart the first time either moved. Spec 0096.
pub fn rut_wander(along: f32, side: f32) -> f32 {
    let seed = along * 0.37 + side * 11.0;

    (seed * 1.3).sin() * 0.22 + (seed * 0.47).sin() * 0.16
}

/// Where the middle of the rut on this side runs, across the street.
pub fn rut_runs(side: f32, along: f32) -> f32 {
    (WAGON_WIDE * 0.5 + WHEEL_THICK) * side + rut_wander(along, side)
}

/// How wide and tall the track's own face is drawn, in texels.
///
/// The track is 72 metres by 10, so this is a texel every 35 millimetres
/// along it and every 39 across, which puts nine of them across a rut. A
/// power of two each way because the mips halve to a pixel and a run of
/// odd sizes rounds differently every level.
pub const FACE_WIDE: u32 = 2048;
pub const FACE_TALL: u32 = 256;

/// The track's face: earth, with the two ruts worn into it. Spec 0096.
///
/// **Painted and not built.** The ruts were 116 slabs standing two
/// millimetres proud of the track, 58 a side, which is a texture's job done
/// with geometry: each one cast its own shadow, so the lines swung across
/// the street as the sun went round and aliased into a crawl at distance.
/// Jake, who found it: textured surfaces should have small shadows, mipmaps
/// are a thing for a reason. The engine's `build_mips` says the same in its
/// own words, that without it a textured floor seen at an angle shimmers.
pub fn face(reaches: f32) -> blitzkit::texture::TextureData {
    let ground = track(reaches);
    let (from, size) = (ground.min, ground.size());
    let earth = crate::aim::VILLAGE_EARTH;
    let rut = crate::aim::VILLAGE_RUT;
    let mut pixels = Vec::with_capacity((FACE_WIDE * FACE_TALL * 4) as usize);

    for y in 0..FACE_TALL {
        for x in 0..FACE_WIDE {
            // the texel's own place on the track, in the world's own numbers,
            // so the curve below is the one the wheels wore
            let along = from.x + (x as f32 + 0.5) / FACE_WIDE as f32 * size.x;
            let across = from.z + (y as f32 + 0.5) / FACE_TALL as f32 * size.z;

            // how far off the nearer rut it is, soft over the last fifth of
            // the width so the edge is worn rather than cut
            let off = [-1.0f32, 1.0]
                .iter()
                .map(|side| (across - (ground.center().z + rut_runs(*side, along))).abs())
                .fold(f32::MAX, f32::min);
            let edge = RUT_WIDE * 0.1;
            let worn = ((RUT_WIDE * 0.5 + edge - off) / (edge * 2.0)).clamp(0.0, 1.0);

            let colour = earth.lerp(rut, worn);
            pixels.extend_from_slice(&[
                encoded(colour.x),
                encoded(colour.y),
                encoded(colour.z),
                255,
            ]);
        }
    }

    blitzkit::texture::TextureData::from_pixels(FACE_WIDE, FACE_TALL, pixels)
}

/// And the common off it, where the fire stands.
pub fn common(reaches: f32) -> Aabb {
    let fire = ring(reaches);

    Aabb::from_center_size(
        vec3(fire.x, at(reaches).y + TRACK_UP - 0.05, fire.z),
        vec3(13.0, 0.1, 11.0),
    )
}

/// A cabin's door, which is a board a shade darker than its wall.
pub const DOOR_WIDE: f32 = 0.95;
pub const DOOR_HIGH: f32 = 1.95;

/// Where each one is, and which way it looks.
pub fn doors(reaches: f32) -> Vec<(Vec3, Vec3)> {
    built(reaches)
        .into_iter()
        .map(|one| {
            let looks = looks(one.faces);
            // a stride off the middle, so the door and the window are not
            // stacked on one line
            let aside = vec3(-looks.z, 0.0, looks.x) * one.wide * 0.22;

            (
                one.at + looks * (one.long * 0.5) + aside + Vec3::Y * DOOR_HIGH * 0.5,
                looks,
            )
        })
        .collect()
}

/// How far a roof oversails its gable, as a share of the building's length.
///
/// Spec 0056's roofs use 0.52 of the length either way. A pioneer cabin
/// wants more of an overhang than a longhouse, because the eaves are what
/// read as folksy at any distance.
pub const OVERSAILS: f32 = 0.58;

/// The run of one pitch, from eaves to ridge.
pub fn slope_run(one: &Built) -> f32 {
    (one.wide * 0.5).hypot(one.ridge - one.eaves)
}

/// How wide a board is, the gap between two, and how far they stand proud.
///
/// **Boards rather than a picture of boards.** The cellar's panelling says
/// why, and it is the same reason here: a cube's corners run nought to one
/// however big it is, so a timber texture on a cabin's wall would be a
/// single plank seven metres long. What makes sawn boards read is the
/// shadow line between one and the next, and a shadow line wants an edge.
pub const BOARD: f32 = 0.3;
pub const BOARD_GAP: f32 = 0.025;
pub const BOARD_OUT: f32 = 0.035;

/// Every board on one building: where it is, how big, and how its own tone
/// stands against its neighbours.
///
/// **No two the same shade.** A wall of identical boards is a wall with
/// lines ruled on it. Weathered sawn timber goes patchy, and the patchiness
/// is most of what says this was built by hand out of what was to hand.
///
/// Off the board's own place rather than off a roll, so one building is one
/// building from one frame to the next and nothing has to be remembered.
pub fn boarding(one: &Built) -> Vec<(Vec3, Vec3, f32, usize)> {
    let courses = ((one.eaves / (BOARD + BOARD_GAP)).floor() as usize).max(1);
    let mut out = Vec::new();

    for course in 0..courses {
        let up = (course as f32 + 0.5) * (BOARD + BOARD_GAP);

        for (face, size) in [
            (
                vec3(one.wide * 0.5 + BOARD_OUT * 0.5, up, 0.0),
                vec3(BOARD_OUT, BOARD, one.long),
            ),
            (
                vec3(-one.wide * 0.5 - BOARD_OUT * 0.5, up, 0.0),
                vec3(BOARD_OUT, BOARD, one.long),
            ),
            (
                vec3(0.0, up, one.long * 0.5 + BOARD_OUT * 0.5),
                vec3(one.wide, BOARD, BOARD_OUT),
            ),
            (
                vec3(0.0, up, -one.long * 0.5 - BOARD_OUT * 0.5),
                vec3(one.wide, BOARD, BOARD_OUT),
            ),
        ] {
            let seed = face.x * 3.1 + face.z * 7.7 + up * 13.3;
            // two sines that do not divide, so the pattern up a wall never
            // repeats and no two walls of one building agree
            let tone = 1.0 + (seed * 1.7).sin() * 0.07 + (seed * 0.41).sin() * 0.05;

            out.push((one.at + face, size, tone, crate::carpet::pick(seed)));
        }
    }

    out
}

/// How thick a roof's pitch is, which is the slab the courses lie on.
///
/// **Here rather than only where it is drawn.** `shingling` has to stand
/// its courses clear of this and the two were written in two places: the
/// courses came off the pitch's middle line by five centimetres and the
/// pitch is twenty thick, so every shingle on every roof in the village
/// was inside the slab it was laid on. Nothing showed, and nothing could.
/// `tests::every_course_stands_proud_of_its_pitch` holds it now.
pub const PITCH_DEEP: f32 = 0.2;

/// How deep one course of shingles is up the slope, the gap between one
/// course and the next, and how far each stands proud of the pitch.
///
/// Laid the way the boards are laid on the walls: proud, with a gap, so
/// the shadow line under a course is what the eye reads. Lapping them was
/// the first try and it puts two courses' outer faces in one plane over
/// the lap, which is this building's oldest fault.
pub const SHINGLE: f32 = 0.42;
pub const SHINGLE_GAP: f32 = 0.05;
pub const SHINGLE_OUT: f32 = 0.05;

/// And how wide one shingle is across the roof.
///
/// **Because a course in one piece is a plank.** Jake, 2026-11-13: make
/// the village roofs read as shingle, not planks. A course that ran the
/// whole length of the building gable to gable had no vertical joint
/// anywhere in it, and an unbroken band is what a sawn board looks like
/// however many of them you stack. What says shingle is the short joints
/// between one and its neighbour, staggered course to course so they
/// never line up into a seam.
pub const SHINGLE_WIDE: f32 = 0.48;

/// The courses on one roof: where each sits, how big, which way it leans,
/// and its own tone.
///
/// **Laid from the eaves up, each standing off the pitch.** A roof drawn as
/// two flat slabs is a roof nobody has been on. What reads as shingle at
/// thirty metres is the line of shadow under every course, which is why
/// each one stands `SHINGLE_OUT` off the face of the pitch it lies on.
pub fn shingling(one: &Built) -> Vec<(Vec3, Vec3, f32, f32)> {
    let run = slope_run(one);
    let rise = one.ridge - one.eaves;
    let slope = (rise / (one.wide * 0.5)).atan();
    let over = one.long * OVERSAILS;
    let courses = ((run / SHINGLE).floor() as usize).max(2);
    let mut out = Vec::new();

    for side in [-1.0f32, 1.0] {
        let eaves = vec3(one.at.x + side * one.wide * 0.5, one.at.y + one.eaves, 0.0);
        let ridge = vec3(one.at.x, one.at.y + one.ridge, 0.0);

        for course in 0..courses {
            let along = (course as f32 + 0.5) / courses as f32;
            let on = eaves + (ridge - eaves) * along;
            // **Out of the pitch along its own normal, far enough to clear
            // it.** Half the pitch and half the course, so the course sits
            // on the pitch's face the way a board sits on a wall.
            let deep = (PITCH_DEEP + SHINGLE_OUT) * 0.5;
            let up = vec3(side * slope.sin() * deep, slope.cos() * deep, 0.0);
            // **Across the roof in shingles, not in one band.** Spec
            // 0068, and `roofing::coursed` does the cutting for the
            // steading's turf as well.
            let long = over * 2.0 - 0.04;

            for (at_z, wide) in crate::roofing::coursed(long, SHINGLE_WIDE, SHINGLE_GAP, course) {
                let at_z = one.at.z + at_z;
                let seed = along * 23.0 + side * 5.0 + one.at.x * 0.7 + at_z * 1.9;
                let tone = 1.0 + (seed * 2.3).sin() * 0.09 + (seed * 0.57).sin() * 0.06;

                out.push((
                    vec3(on.x + up.x, on.y + up.y, at_z),
                    vec3(run / courses as f32 - SHINGLE_GAP, SHINGLE_OUT, wide),
                    -side * slope,
                    tone,
                ));
            }
        }
    }

    out
}

/// Where a cabin's lit window is, and which way it looks.
///
/// One a cabin, in the gable that faces the street. Spec 0056 lights its
/// outbuildings from inside and the opening is a hole in the same room;
/// this does the same thing with one opening instead of a row.
pub const WINDOW_WIDE: f32 = 0.85;
pub const WINDOW_HIGH: f32 = 0.95;
pub const WINDOW_UP: f32 = 1.25;
pub const SET_BACK: f32 = 0.3;

pub fn windows(reaches: f32) -> Vec<(Vec3, Vec3)> {
    built(reaches)
        .into_iter()
        .map(|one| {
            let looks = looks(one.faces);
            let out = if one.faces == FRAC_PI_2 {
                vec3(one.wide * 0.5 - SET_BACK, 0.0, 0.0)
            } else {
                vec3(
                    0.0,
                    0.0,
                    -(one.long * 0.5 - SET_BACK) * one.faces.cos().signum(),
                )
            };

            (one.at + out + Vec3::Y * WINDOW_UP, looks)
        })
        .collect()
}

/// Where the lamp inside each one sits, which is what the window shows.
///
/// Inside and back from the opening, so what you see through it is a room
/// and not a bulb. Spec 0056 learnt this at five metres; a cabin is four.
pub fn lamps(reaches: f32) -> Vec<Vec3> {
    built(reaches)
        .into_iter()
        .map(|one| one.at + Vec3::Y * (one.eaves * 0.45))
        .collect()
}

/// How far to one side of a stop something set down at somebody's feet
/// stands. Spec 0080.
///
/// **Clear of the body and inside a stoop's reach.** A body is 0.9 across,
/// so anything nearer than about 0.45 is under a shoulder and never seen,
/// and an arm is 0.74 long, so anything much past this cannot be reached
/// from a stoop without a stride.
pub const SETS_DOWN: f32 = 0.52;

/// Which way somebody facing `facing` is looking.
///
/// **Written out three times before this.** `doors` and `fronts` each said
/// it and so did the sack in the wagon, which is three accounts of one
/// convention and the fault this file already names for the stair's foot.
pub fn looks(facing: f32) -> Vec3 {
    vec3(facing.sin(), 0.0, -facing.cos())
}

/// A point `how_far` to one side of somebody standing at `at` and facing
/// `facing`.
///
/// Forward is [`looks`], so across it is `(cos, 0, sin)`.
pub fn aside(at: Vec3, facing: f32, how_far: f32) -> Vec3 {
    vec3(
        at.x + facing.cos() * how_far,
        at.y,
        at.z + facing.sin() * how_far,
    )
}

/// Where somebody stands to use a thing: clear of it, facing it.
fn at_the(thing: Vec3, from: Vec3, clear: f32) -> (Vec3, f32) {
    let way = (from - thing).normalize_or_zero();
    let at = vec3(thing.x + way.x * clear, 0.0, thing.z + way.z * clear);

    // facing the thing, which is back down the way we came out
    (at, crate::people::facing(at, thing))
}

/// What the people here are doing, as a round apiece. Spec 0068, over spec
/// 0067's machinery.
///
/// **One errand each, and each one Jake's.** From a cabin to the cook fire,
/// from the wagons to the horses, from the well to a cabin, and one that
/// walks the length of the street, which is somebody going somewhere you are
/// not. Four rounds in a village of seven buildings is not a crowd, and a
/// crowd is not what this is: what makes a place read as lived in is that
/// something is always happening somewhere in it.
pub fn rounds(reaches: f32) -> Vec<Vec<crate::people::Stop>> {
    use crate::gesture::Errand;
    use crate::people::Stop;

    let middle = at(reaches);
    let fire = ring(reaches);
    let houses = built(reaches);
    let doors = doors(reaches);
    let carts = wagons(reaches);
    let rail_at = rail(reaches);
    let water = well(reaches);
    let stop = |at: Vec3, facing: f32, stays: f32| Stop {
        at,
        facing,
        stays,
        doing: Errand::Wait,
        carries: None,
        sits: None,
        puts_at: None,
        takes_at: None,
    };
    let doing = |at: Vec3, facing: f32, stays: f32, doing: Errand| Stop {
        at,
        facing,
        stays,
        doing,
        carries: None,
        sits: None,
        puts_at: None,
        takes_at: None,
    };
    // the same, leaving with something in hand. Spec 0073.
    let carrying =
        |at: Vec3, facing: f32, stays: f32, errand: Errand, carries: crate::people::Load| Stop {
            carries: Some(carries),
            ..doing(at, facing, stays, errand)
        };
    let before = |thing: Vec3, looks: Vec3, stays: f32| {
        let (at, facing) = at_the(thing, thing + looks * 4.0, CLEAR_OF);

        doing(at, facing, stays, Errand::Knock)
    };
    // the lane a round walks along, which is the middle of the street and
    // where nothing stands
    let lane = |x: f32| stop(vec3(x, 0.0, middle.z), 0.0, 0.0);

    vec![
        // **Cooking**: out of the south cabin nearest the common, to the
        // fire, and back. This is the errand Jake named first.
        {
            let (by_fire, at_fire) = at_the(fire, vec3(fire.x + 6.0, 0.0, fire.z), TENDS_FROM);
            // **Out past the cabin's own corner before turning for the
            // fire.** Walked straight, the leg from the doorstep cut the
            // south-west corner of the cabin it had just come out of, which
            // is what `tests::nobody_walks_through_anything` said twice:
            // once for cutting it and once for not cutting it far enough.
            // The corner is due south of where they end up, so the last
            // run is straight up a line nothing stands on.
            let round = stop(
                vec3(by_fire.x, 0.0, doors[5].0.z + doors[5].1.z * 4.0),
                0.0,
                0.0,
            );

            vec![
                before(doors[5].0, doors[5].1, 5.0),
                round,
                doing(by_fire, at_fire, 7.0, Errand::Stir),
                round,
            ]
        },
        // **Loading**: between the wagon at the east end and the horses at
        // the rail, which is the other errand he named.
        vec![
            {
                let (at, facing) = at_the(
                    carts[0].0,
                    carts[0].0 + vec3(0.0, 0.0, 4.0),
                    WAGON_WIDE * 0.5 + CLEAR_OF,
                );
                // **And what is in the wagon is feed for the horses.**
                // Jake, 2026-11-14. Spec 0073 left this stop empty-handed
                // because what a wagon holds is not a thing arithmetic can
                // answer, and this round has gone from a wagon to a rail
                // of horses carrying nothing since spec 0068.
                //
                // **The sack is in the bed up to the moment he has it.**
                // Spec 0084. Spec 0078 said a load that vanishes when a
                // foot leaves a stop is not a transfer and spec 0080 said
                // the same of the other end, and this was the one stop
                // neither converted: five and a half seconds of a
                // two-handed heave with nothing in it, and the feed
                // appearing as he walked off. It was in the picture before
                // it was in a test.
                //
                // Taken from the cart and not from the man, though he is
                // stood at it: a sack on a bed is in the bed's place, and
                // the day the wagons move this moves with them. He is
                // `CLEAR_OF` clear of the near side, so the bed starts
                // that far ahead of him and the sack sits a little into it.
                let bed = at + looks(facing) * (CLEAR_OF + WAGON_WIDE * 0.25) + Vec3::Y * WAGON_BED;

                Stop {
                    takes_at: Some(bed),
                    ..carrying(at, facing, 5.5, Errand::Lift, crate::people::Load::Feed)
                }
            },
            lane(rail_at.x - 2.0),
            // **Down at their feet before anybody is patted.** Spec 0078.
            // Two stops in one place, because a stop has one errand and
            // this is two things: you set the sack down, and then you see
            // to the horse. The sack goes a little short of the rail,
            // which is where a horse can reach it and nobody trips on it.
            {
                let (at, facing) =
                    at_the(rail_at, rail_at + vec3(0.0, 0.0, -4.0), RAIL_LONG * 0.36);

                Stop {
                    // most of the way to the rail and not half, because
                    // half is at their own feet: a body is 0.9 across and
                    // they stand `RAIL_LONG * 0.36` off it, so the sack
                    // went down under them and was never once visible
                    puts_at: Some(vec3(at.x, 0.0, at.z + (rail_at.z - at.z) * 0.82)),
                    ..doing(at, facing, 3.5, Errand::Stoop)
                }
            },
            {
                let (at, facing) =
                    at_the(rail_at, rail_at + vec3(0.0, 0.0, -4.0), RAIL_LONG * 0.36);
                doing(at, facing, 6.0, Errand::Tend)
            },
            lane(rail_at.x - 2.0),
        ],
        // **Water**: the well to the cabin across the street and back.
        //
        // **And the pail goes with them.** Spec 0073. `Draw` is a bucket
        // up a well hand over hand, so the thing at the end of it is not a
        // decision: they leave here holding it, carry it across the street
        // and stand at the door with it, and come back empty.
        //
        // **And it comes up off the ground.** Spec 0080. Hauling is both
        // hands on a rope and the pail lands at the wellhead, so what the
        // hand does next is bend and lift it: two stops at the wellhead,
        // because a stop has one errand and hauling and lifting are two.
        {
            let (at, facing) = at_the(
                water,
                water + vec3(0.0, 0.0, -4.0),
                WELL_WIDE * 0.5 + CLEAR_OF,
            );
            // **Beside them and not in front.** Spec 0079 put a sack
            // halfway to the thing the stop was at and found it under the
            // body; the other answer is to one side, where nothing of
            // theirs is over it and a stooping hand still reaches.
            let beside = aside(at, facing, SETS_DOWN);

            vec![
                doing(at, facing, 6.0, Errand::Draw),
                Stop {
                    takes_at: Some(beside),
                    ..carrying(at, facing, 2.5, Errand::Stoop, crate::people::Load::Pail)
                },
                lane(water.x + 1.0),
                before(doors[1].0, doors[1].1, 4.5),
                lane(water.x + 1.0),
            ]
        },
        // **And somebody walking the length of it**, barn end to the east
        // end and back, which is what makes the street a street rather than
        // three errands that happen to be near each other.
        vec![
            stop(
                vec3(
                    houses[6].at.x + houses[6].wide * 0.5 + CLEAR_OF * 2.0,
                    0.0,
                    middle.z,
                ),
                -std::f32::consts::FRAC_PI_2,
                4.0,
            ),
            lane(middle.x),
            stop(
                vec3(middle.x + STREET_LONG * 0.5 - 3.0, 0.0, middle.z),
                std::f32::consts::FRAC_PI_2,
                4.0,
            ),
            lane(middle.x),
        ],
    ]
}

/// What you cannot walk through out here.
pub fn solid(reaches: f32) -> Vec<Aabb> {
    let mut out: Vec<Aabb> = built(reaches).iter().map(Built::walls).collect();

    out.push(Aabb::from_center_size(
        ring(reaches) + Vec3::Y * crate::walk::RIDES * 1.5,
        vec3(RING_WIDE, crate::walk::RIDES * 3.0, RING_WIDE),
    ));
    out.push(Aabb::from_center_size(
        well(reaches) + Vec3::Y * WELL_HIGH * 0.5,
        vec3(WELL_WIDE, WELL_HIGH, WELL_WIDE),
    ));
    out.push(trough(reaches));

    for (at, _) in wagons(reaches) {
        out.push(Aabb::from_center_size(
            at + Vec3::Y * (WAGON_BED + WAGON_SIDE) * 0.5,
            vec3(WAGON_LONG, WAGON_BED + WAGON_SIDE, WAGON_WIDE),
        ));
    }

    let one = rail(reaches);
    out.push(Aabb::from_center_size(
        one + Vec3::Y * RAIL_HIGH * 0.5,
        vec3(RAIL_LONG, RAIL_HIGH, RAIL_THICK * 2.0),
    ));

    for (from, to) in fences(reaches) {
        let middle = (from + to) * 0.5;
        out.push(Aabb::from_center_size(
            middle + Vec3::Y * FENCE_HIGH * 0.5,
            vec3((to.x - from.x).abs().max(0.1), FENCE_HIGH, 0.12),
        ));
    }

    out
}

#[cfg(test)]
mod tests {

    /// Spec 0096: the track's face is worn where the wheels run, and the
    /// earth between comes back the colour it went in.
    ///
    /// **Both halves were faults, in that order.** The ruts were built as
    /// 116 slabs standing proud of the track, so each cast a shadow and the
    /// lines swung across the street as the sun went round; painted, they
    /// can instead land in the wrong place, because the texture and the
    /// wheels are two readings of one curve. And the first painting came
    /// out nearly black: `texture::FORMAT` is sRGB and the colour was
    /// written straight, so 0.235 was read back as 0.045.
    #[test]
    fn the_track_is_worn_where_the_wheels_run() {
        let reaches = 9.0;
        let ground = track(reaches);
        let face = face(reaches);
        let (from, size) = (ground.min, ground.size());

        let texel = |along: f32, across: f32| {
            let x = (((along - from.x) / size.x) * FACE_WIDE as f32) as u32;
            let y = (((across - from.z) / size.z) * FACE_TALL as f32) as u32;
            let n = ((y.min(FACE_TALL - 1) * FACE_WIDE + x.min(FACE_WIDE - 1)) * 4) as usize;
            let level = face.levels.first().expect("a base level");

            level.pixels[n]
        };

        // a quarter, a half and three quarters of the way down it
        for share in [0.25f32, 0.5, 0.75] {
            let along = from.x + size.x * share;

            for side in [-1.0f32, 1.0] {
                let on = texel(along, ground.center().z + rut_runs(side, along));
                let off = texel(along, ground.center().z);

                assert!(
                    on < off,
                    "at {:.1} along, the {} rut reads {} against earth at {}",
                    along,
                    if side < 0.0 { "near" } else { "far" },
                    on,
                    off
                );
            }
        }

        // and the earth between them is the earth, read back through the
        // sRGB the sampler will undo
        let middle = texel(from.x + size.x * 0.5, ground.center().z);
        let back = {
            let one = middle as f32 / 255.0;
            if one <= 0.040_45 {
                one / 12.92
            } else {
                ((one + 0.055) / 1.055).powf(2.4)
            }
        };

        assert!(
            (back - crate::aim::VILLAGE_EARTH.x).abs() < 0.01,
            "the track's earth went in at {:.3} and comes back at {:.3}",
            crate::aim::VILLAGE_EARTH.x,
            back
        );
    }
    use super::*;

    const REACHES: f32 = 20.0;

    /// Spec 0071. The one thing a staged shot can get wrong that nobody
    /// notices until it is published: a camera pointing somewhere else.
    #[test]
    fn the_staged_shot_looks_down_the_street() {
        let (stand, yaw, pitch) = posed(REACHES);
        let around = around(REACHES);

        assert!(
            stand.x > around.min.x
                && stand.x < around.max.x
                && stand.z > around.min.z
                && stand.z < around.max.z,
            "the shot stands at {:?}, which is outside the village",
            stand
        );

        // the street runs east and west, and this looks west along it, so
        // the way ahead is nearly all -x. Yaw nought faces -z and grows
        // toward +x, so west is -pi/2.
        let ahead = vec3(yaw.sin(), 0.0, -yaw.cos());

        assert!(
            ahead.x < -0.9,
            "the shot looks {:?}, which is not down the street",
            ahead
        );

        // and a little under the eye rather than at the sky or at its own
        // feet
        assert!(
            (-0.1..0.0).contains(&pitch),
            "the shot is pitched {:.3}, which is not a look along a street",
            pitch
        );
    }

    /// Spec 0071. The first three takes of this were the middle of the
    /// night, so the hour is worth a test of its own.
    ///
    /// **Asked of the garden rather than of the number.** A range written
    /// round a constant is checked by the compiler and proves nothing; what
    /// this wants to know is whether the sun is up at that hour, which only
    /// the thing that moves the sun can say.
    #[test]
    fn the_shot_is_taken_in_daylight() {
        let shot = crate::garden::when(SHOT_AT);

        assert!(
            crate::garden::sun_up(shot) > 0.5,
            "the sun stands at {:.2} at the hour of the shot, which is not \
             a street anybody can see",
            crate::garden::sun_up(shot)
        );

        // and the five seconds the photographer runs for have to leave it
        // there, which is the whole of why `SHOT_DAY` exists: at
        // `garden::DAY` those five seconds are a quarter of a day.
        let drawn = crate::garden::when(SHOT_AT + 5.0 / SHOT_DAY);

        assert!(
            crate::garden::sun_up(drawn) > 0.5,
            "the sun has fallen to {:.2} by the time the picture is drawn",
            crate::garden::sun_up(drawn)
        );
    }

    /// Spec 0068: a roof is pieces, and two courses do not line up.
    ///
    /// **Which is a look, so this holds the two things under it.** A course
    /// in one piece has no vertical joint anywhere in it, and that is a
    /// sawn board however many you stack; courses that line up are a seam
    /// running up the pitch. Jake, 2026-11-13: make the village roofs read
    /// as shingle, not planks.
    ///
    /// Where the pieces fall is `roofing::coursed`'s own business and its
    /// own tests. What this holds is that `shingling` asks for them and hands
    /// it the course it is on.
    #[test]
    fn a_roof_is_pieces_and_two_courses_do_not_line_up() {
        for one in built(REACHES) {
            // one pitch of it, since both carry the same courses at the
            // same heights and merging them says nothing
            let mut courses: Vec<(f32, Vec<f32>)> = Vec::new();

            for (at, _, _, _) in shingling(&one).into_iter().filter(|one| one.2 < 0.0) {
                match courses.iter_mut().find(|(up, _)| (up - at.y).abs() < 1e-3) {
                    Some((_, along)) => along.push(at.z),
                    None => courses.push((at.y, vec![at.z])),
                }
            }

            courses.sort_by(|a, b| a.0.total_cmp(&b.0));

            for (_, along) in courses.iter() {
                assert!(
                    along.len() > 1,
                    "a course of a {:.1} by {:.1} roof is one piece across the whole of it",
                    one.wide,
                    one.long
                );
            }

            for pair in courses.windows(2) {
                assert!(
                    pair[0].1.len() != pair[1].1.len()
                        || pair[0]
                            .1
                            .iter()
                            .zip(pair[1].1.iter())
                            .any(|(under, over)| (under - over).abs() > 1e-3),
                    "two courses next to each other have their pieces in the same places"
                );
            }
        }
    }

    /// Spec 0068: a course lies on the face of its pitch, not inside it.
    ///
    /// **Because the course and the pitch were written in two places.**
    /// The pitch's thickness lived where it was drawn and the courses came
    /// off the pitch's middle line, so every shingle on every roof was
    /// buried in the slab it was laid on and the roofs read as two smooth
    /// planes. Nothing in either file was wrong on its own.
    #[test]
    fn every_course_stands_proud_of_its_pitch() {
        for one in built(REACHES) {
            let slope = ((one.ridge - one.eaves) / (one.wide * 0.5)).atan();

            for (at, size, lean, _) in shingling(&one) {
                // which pitch it is on, which its own lean says
                let side = if lean < 0.0 { 1.0f32 } else { -1.0 };
                let eaves = vec3(one.at.x + side * one.wide * 0.5, one.at.y + one.eaves, 0.0);
                let normal = vec3(side * slope.sin(), slope.cos(), 0.0);
                let off = (vec3(at.x, at.y, 0.0) - eaves).dot(normal);

                assert!(
                    off - size.y * 0.5 >= PITCH_DEEP * 0.5 - 1e-3,
                    "a course's underside is {:.3} off the pitch's middle and the pitch is {:.3} thick",
                    off - size.y * 0.5,
                    PITCH_DEEP
                );
                assert!(
                    off + size.y * 0.5 >= PITCH_DEEP * 0.5 + SHINGLE_OUT - 1e-3,
                    "a course stands {:.3} proud of the pitch's face",
                    off + size.y * 0.5 - PITCH_DEEP * 0.5
                );
            }
        }
    }

    /// Spec 0068: there is a clear way down the street.
    ///
    /// **A lane through it and not an empty rectangle.** The street is the
    /// one thing here everybody uses and a village you have to pick your
    /// way through is a yard, but a well and a trough standing against the
    /// fronts are what a street has in it. What has to hold is that a
    /// wagon can go from one end to the other, which is `WAGON_WIDE` and
    /// a body either side.
    #[test]
    fn the_street_is_clear() {
        let middle = at(REACHES);
        let body = crate::RADIUS * 2.0;
        let lane = Aabb::from_center_size(
            middle + Vec3::Y * 1.0,
            vec3(STREET_LONG - 2.0, 2.0, WAGON_WIDE + body * 2.0),
        );

        for one in solid(REACHES) {
            assert!(
                !lane.intersects(&one),
                "something stands in the street: {:?} to {:?}",
                one.min,
                one.max
            );
        }
    }

    /// Spec 0068: no two buildings share ground.
    #[test]
    fn no_two_buildings_overlap() {
        let houses = built(REACHES);

        for (n, one) in houses.iter().enumerate() {
            for two in houses.iter().skip(n + 1) {
                assert!(
                    !one.footprint().intersects(&two.footprint()),
                    "two buildings share ground at {:?} and {:?}",
                    one.at,
                    two.at
                );
            }
        }
    }

    /// Spec 0068: it stands clear of everything else out there.
    ///
    /// **The garden, the bridge and the field's own stone.** Each of those
    /// was put where it is by its own spec and none of them knows this
    /// place exists, so the clearance is this file's to keep.
    #[test]
    fn it_stands_clear_of_everything_else() {
        let box_ = around(REACHES);
        let trail = {
            let mut trail = crate::trail::Trail::new();
            for one in crate::trail::Asked::every() {
                trail.gave(*one);
            }
            trail
        };

        for one in crate::bridge::solid(REACHES, crate::bridge::reach(&trail)) {
            assert!(
                !box_.intersects(&one),
                "the bridge runs through the village at {:?}",
                one.min
            );
        }

        let garden = crate::garden::at(REACHES);
        let half = crate::garden::half();

        assert!(
            box_.max.x < garden.x - half.x,
            "the village reaches the garden: {} against {}",
            box_.max.x,
            garden.x - half.x
        );
    }

    /// Spec 0068: nobody on a round walks through anything.
    ///
    /// **Walked rather than measured**, as spec 0067's is at the steading
    /// and for the reason this repo keeps relearning: a stop that is clear
    /// and the stop after it that is clear say nothing about the cabin
    /// between them.
    #[test]
    fn nobody_walks_through_anything() {
        let solid = solid(REACHES);
        let body = vec3(
            crate::RADIUS * 2.0,
            crate::people::TALL,
            crate::RADIUS * 2.0,
        );

        for (n, round) in rounds(REACHES).into_iter().enumerate() {
            let whole: f32 = round
                .iter()
                .enumerate()
                .map(|(i, stop)| {
                    stop.stays
                        + stop.at.distance(round[(i + 1) % round.len()].at) / crate::people::PACE
                })
                .sum();
            let steps = (whole / 0.15).ceil() as usize;

            for step in 0..steps {
                let doing = crate::people::doing(&round, whole * step as f32 / steps as f32);
                let stands =
                    Aabb::from_center_size(doing.at + Vec3::Y * crate::people::TALL * 0.5, body);

                for one in solid.iter() {
                    assert!(
                        !stands.intersects(one),
                        "round {} is inside {:?} to {:?} at {:?}",
                        n,
                        one.min,
                        one.max,
                        doing.at
                    );
                }
            }
        }
    }

    /// Spec 0079: the feed gets from the wagon to the horses and is put
    /// down where one of them could reach it.
    ///
    /// **Where it lands is the half that can go wrong.** Half way from the
    /// stop to the rail is under the body that carried it: somebody stands
    /// `RAIL_LONG * 0.36` off the rail and is 0.9 across, so the sack was
    /// at their own feet and never once showed.
    #[test]
    fn the_feed_gets_from_the_wagon_to_the_horses() {
        let rail_at = rail(REACHES);
        let carts = wagons(REACHES);
        let round = &rounds(REACHES)[1];
        let mut at_the_wagon = false;
        let mut laid = None;
        let mut held = false;

        for step in 0..6_000 {
            let doing = crate::people::doing(round, step as f32 * 0.05);
            let holding = doing.holding == Some(crate::people::Load::Feed);

            if holding {
                at_the_wagon |= doing.at.distance(carts[0].0) < 5.0;
            }

            // **The step it leaves his hands, and not every step it is on
            // the ground.** Spec 0084 gave the wagon stop a `takes_at`, so
            // `left` now says "lying here" at both ends of the errand
            // rather than only after one: read as it was, this took the
            // sack waiting in the wagon bed for the sack laid at the rail
            // and measured nine metres. What this is after is the moment
            // the carrier lets go, which is the step after one he was
            // holding it on.
            if let Some(put) = doing.left {
                if held {
                    laid = Some((put, doing.at));
                }
            }

            held = holding;
        }

        assert!(at_the_wagon, "the feed is never picked up at the wagon");

        let Some(((load, where_), stood)) = laid else {
            panic!("the feed is never put down");
        };

        assert_eq!(
            load,
            crate::people::Load::Feed,
            "something else is put down"
        );
        // **Within a head's reach of the rail**, which is the whole of
        // where it should be: half way from the stop to the rail is a
        // sack at the carrier's own feet, and that measures a metre from
        // the rail and a metre from them, so a looser bound passes it.
        let to_rail = where_.distance(vec3(rail_at.x, where_.y, rail_at.z));
        let to_them = where_.distance(vec3(stood.x, where_.y, stood.z));

        assert!(
            to_rail < 0.5,
            "the feed is laid {:.2} from the rail and {:.2} from the person, \
             which is nearer them than the horses",
            to_rail,
            to_them
        );
    }

    /// Spec 0073: the pail leaves the well and comes back without it.
    ///
    /// **Asked of the round rather than of the stop that sets it**, because
    /// what this wants to know is whether anybody in this village is ever
    /// seen carrying anything, and a field set on a struct is not that.
    #[test]
    fn the_pail_leaves_the_well_and_comes_back_empty() {
        let water = well(REACHES);
        let round = &rounds(REACHES)[2];
        let mut carried = 0;
        let mut empty = 0;
        let mut furthest: f32 = 0.0;

        for step in 0..4_000 {
            let doing = crate::people::doing(round, step as f32 * 0.05);

            if doing.holding == Some(crate::people::Load::Pail) {
                carried += 1;
                furthest = furthest.max(doing.at.distance(water));
            } else {
                empty += 1;
            }
        }

        assert!(carried > 0, "nobody ever carries the pail");
        assert!(empty > 0, "the pail is never put down");
        assert!(
            furthest > 6.0,
            "the pail never gets more than {:.1} metres from the well, so it \
             is not being taken anywhere",
            furthest
        );
    }

    /// Spec 0080: and it comes up off the wellhead rather than out of a
    /// hand.
    ///
    /// **Where it lies is the half of this a test can see.** That the hand
    /// closes on it is `people::tests::a_load_comes_up_off_the_ground`'s;
    /// what this one is for is that the place it lies is beside the body
    /// and not under it, which is the mistake spec 0079 made and which no
    /// amount of correct timing fixes.
    #[test]
    fn the_pail_comes_up_off_the_wellhead() {
        let water = well(REACHES);
        let round = &rounds(REACHES)[2];
        let mut lay: Option<Vec3> = None;
        let mut stood = Vec3::ZERO;

        for step in 0..4_000 {
            let doing = crate::people::doing(round, step as f32 * 0.05);

            if let Some((load, where_)) = doing.left {
                assert_eq!(
                    load,
                    crate::people::Load::Pail,
                    "something other than the pail is on the ground"
                );
                lay = Some(where_);
                stood = doing.at;
            }
        }

        let lay = lay.expect("the pail is never on the ground at all");
        let aside = lay.distance(stood);

        assert!(
            aside > crate::RADIUS,
            "the pail stands {:.2} metres from the feet of the person who \
             lifts it, and a body is {:.2} across, so it is under them",
            aside,
            crate::RADIUS * 2.0
        );
        assert!(
            aside < crate::people::ARM_TALL,
            "the pail stands {:.2} metres away, which is past the reach of \
             an arm {:.2} long",
            aside,
            crate::people::ARM_TALL
        );
        assert!(
            lay.distance(water) < WELL_WIDE * 0.5 + CLEAR_OF + SETS_DOWN,
            "the pail is lifted {:.2} metres from the well, which is not a \
             wellhead",
            lay.distance(water)
        );
        assert_eq!(lay.y, 0.0, "the pail is lying off the ground");
    }

    /// Spec 0084: and the feed is in the wagon up to the moment he has it.
    ///
    /// **The wellhead's test one errand along, asking a different thing.**
    /// That one asks whether the pail is beside the body rather than under
    /// it, because a pail on the ground is somewhere a stoop can reach and
    /// the mistake worth guarding is putting it where nobody can see it.
    /// A sack in a wagon has a height as well as a place, and a sack on the
    /// ground beside a wagon is not what is in a wagon.
    ///
    /// **It does not ask whether he can reach it, and that is not an
    /// oversight.** `CLEAR_OF` stands everybody 1.6 metres clear of the
    /// thing their errand is at, and an arm is `people::ARM_TALL`, 0.74. So
    /// nobody in this street touches what they work at: the drawer at the
    /// well hauls a rope that is not there either. Asserting a reach here
    /// would be asserting against the convention rather than against this
    /// change.
    ///
    /// **Nor whether it is in front of him**, which was written and taken
    /// out: the wagon is what he faces, so anything inside its footprint is
    /// ahead of him by construction and the assertion could not fail. Its
    /// negative control failed on the footprint instead, which is how it
    /// was caught.
    ///
    /// Found by walking the rounds rather than by naming one. The wellhead's
    /// test says `rounds(REACHES)[2]`, and the day an errand is added before
    /// it that index is a different person.
    #[test]
    fn the_feed_is_in_the_wagon_until_it_is_lifted() {
        let cart = wagons(REACHES)[0].0;
        let mut lay: Option<Vec3> = None;

        for round in rounds(REACHES).iter() {
            for step in 0..4_000 {
                let doing = crate::people::doing(round, step as f32 * 0.05);

                if let Some((crate::people::Load::Feed, where_)) = doing.left {
                    lay = Some(where_);
                }
            }
        }

        let lay = lay.expect("the feed is never lying anywhere at all");

        assert!(
            (lay.y - WAGON_BED).abs() < 1e-4,
            "the feed lies at {:.2} and a wagon's bed is at {:.2}, so it is \
             not in the wagon",
            lay.y,
            WAGON_BED
        );

        let across = vec3(lay.x - cart.x, 0.0, lay.z - cart.z).length();

        assert!(
            across < WAGON_LONG * 0.5,
            "the feed lies {:.2} metres from the middle of a wagon {:.2} \
             long, so it is beside one rather than in one",
            across,
            WAGON_LONG
        );
    }

    /// Spec 0067: and being moved out of each other does not move anybody
    /// into a wall.
    ///
    /// **The push has no idea what is around it.** `people::apart` moves two
    /// people along the line between them and knows nothing else, so the
    /// thing to ask is not whether it is careful but whether it ever lands
    /// anybody somewhere solid. Ten minutes of every round, nudged, against
    /// the same boxes the walk above is held to.
    #[test]
    fn being_moved_apart_never_puts_anybody_in_a_wall() {
        let solid = solid(REACHES);
        let rounds = rounds(REACHES);
        let body = vec3(
            crate::RADIUS * 2.0,
            crate::people::TALL,
            crate::RADIUS * 2.0,
        );

        for step in 0..12_000 {
            let since = step as f32 * 0.05;
            let mut here: Vec<Vec3> = rounds
                .iter()
                .enumerate()
                .map(|(n, round)| crate::people::doing(round, since + n as f32 * 5.7).at)
                .collect();

            crate::people::apart(&mut here);

            for (n, at) in here.iter().enumerate() {
                let stands =
                    Aabb::from_center_size(*at + Vec3::Y * crate::people::TALL * 0.5, body);

                for one in solid.iter() {
                    assert!(
                        !stands.intersects(one),
                        "round {} is pushed inside {:?} to {:?} at {:.1} seconds",
                        n,
                        one.min,
                        one.max,
                        since
                    );
                }
            }
        }
    }

    /// Spec 0068: you can walk in from the garden and out again.
    ///
    /// **With the game's own `walk`**, because the baths' steps had four
    /// legal rises and were one solid block. A village you can see and
    /// cannot get into is scenery.
    #[test]
    fn you_can_walk_into_it() {
        let mut solid = solid(REACHES);
        solid.push(crate::field::ground(at(REACHES)));

        let middle = at(REACHES);
        let from = vec3(middle.x + STREET_LONG * 0.5 + 6.0, 1.0, middle.z);
        let to = vec3(middle.x - 10.0, 0.0, middle.z);
        let (mut body, mut falling) = (from, 0.0);

        // walked at the game's own step, dropped in from above rather than
        // placed, because a point on the ground can be inside the furniture
        for _ in 0..1_200 {
            let way = (to - body) * vec3(1.0, 0.0, 1.0);
            let wish = if way.length() < 1.0 {
                Vec3::ZERO
            } else {
                way.normalize() * crate::SPEED
            };

            (body, falling) =
                crate::walk::walk(body, wish, falling, crate::RADIUS, 1.0 / 60.0, &solid, true);
        }

        assert!(
            (body - to).length() < 2.5,
            "walking in from the east stopped at {:?}, short of {:?}",
            body,
            to
        );
    }

    /// Spec 0068: no tree grows inside it.
    /// Spec 0086: and there is a way there, with nothing growing across it.
    ///
    /// **The field's own trees and not just the predicate.** The sibling
    /// below asks `shades` at a grid of points, which says what the rule
    /// answers and not what the field does with it. This one walks the
    /// cells the run crosses and asks `field::trees` and `field::cover`
    /// what they actually put there, because the fault being fixed was a
    /// stand of birch standing exactly where the rule now says it may not:
    /// a rule nothing consults reads the same as no rule.
    #[test]
    fn nothing_grows_across_the_way_to_the_village() {
        let (from, to) = path_runs(REACHES);

        // it leaves the gap and meets the ruts where they run out, which
        // is what makes it one way of getting here
        let (a, b) = crate::field::way(REACHES);
        let gap = vec3(crate::field::starts(REACHES), 0.0, (a + b) * 0.5);
        let middle = at(REACHES);

        assert!(
            from.distance(gap) < PATH_OFF + 0.01,
            "the way there starts {:.1} metres from the gap",
            from.distance(gap)
        );
        assert!(
            (to.x - (middle.x + (STREET_LONG + 14.0) * 0.5)).abs() < 0.01
                && (to.z - middle.z).abs() < 0.01,
            "the way there ends at {:?}, which is not where the ruts run out",
            to
        );

        // every cell the run passes through, which is what the field is
        // asked for a cell at a time
        let cell = crate::field::CELL;
        let mut stood = 0;

        for n in 0..=120 {
            let on = from + (to - from) * (n as f32 / 120.0);
            let (i, j) = ((on.x / cell).round() as i32, (on.z / cell).round() as i32);

            for about in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
                let at = (i + about.0, j + about.1);

                for one in crate::field::trees(REACHES, at)
                    .into_iter()
                    .chain(crate::field::cover(REACHES, at))
                {
                    stood += 1;

                    assert!(
                        !along_the_path(REACHES, one.at),
                        "something grows at {:?}, which is on the way to the village",
                        one.at
                    );
                }
            }
        }

        // and the sweep found things to check, or it says nothing at all
        assert!(
            stood > 100,
            "only {} things stand anywhere near the run, so this sweep is \
             not looking at the field it thinks it is",
            stood
        );

        // and the far side of the field is not on it
        assert!(!along_the_path(REACHES, to + vec3(0.0, 0.0, -60.0)));
    }

    #[test]
    fn nothing_is_sown_in_the_village() {
        let box_ = around(REACHES);

        for n in 0..40 {
            for m in 0..40 {
                let at = vec3(
                    box_.min.x + (box_.max.x - box_.min.x) * n as f32 / 39.0,
                    0.0,
                    box_.min.z + (box_.max.z - box_.min.z) * m as f32 / 39.0,
                );

                assert!(shades(REACHES, at), "a tree may grow at {:?}", at);
            }
        }

        // and the far side of the field is not shaded, or this says nothing
        assert!(!shades(REACHES, vec3(box_.min.x - 60.0, 0.0, box_.min.z)));
    }
}
