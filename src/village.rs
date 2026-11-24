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
    /// Which render it wears, per spec 0123, or none at all if it is the
    /// barn. A barn is not a townhouse and painting it one would say the
    /// place has no working buildings in it.
    pub render: Option<usize>,
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
    let one =
        |along: f32, side: f32, wide: f32, long: f32, eaves: f32, ridge: f32, render: usize| {
            Built {
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
                render: Some(render),
            }
        };

    // **Three storeys and four, per spec 0123.** They stood 2.4 to 2.8 to
    // the eaves, which is one, and a ten metre street between one-storey
    // buildings is a frontier main street because that is what it is. The
    // street is not squeezed to answer that; the buildings are given the
    // height the street already had room for. Each still differs from its
    // neighbour, which is spec 0068's own argument for why this reads as a
    // village and not a row of sheds, and no two side by side take the same
    // render.
    vec![
        // the north side, nearest the way you come in
        one(-16.0, -1.0, 7.0, 6.0, 9.2, 13.0, 0),
        one(-4.0, -1.0, 9.0, 7.0, 11.6, 15.8, 2),
        one(9.0, -1.0, 6.0, 5.5, 8.4, 12.0, 4),
        // and the south, set along differently so the street is not a
        // corridor of pairs, and with a wide gap left between the last two
        // for the common the fire stands on
        one(-22.0, 1.0, 6.5, 6.0, 8.8, 12.4, 1),
        one(-11.0, 1.0, 8.0, 6.5, 10.4, 14.4, 3),
        one(11.0, 1.0, 7.5, 6.0, 9.6, 13.4, 5),
        // **And the barn, closing the west end from beyond it.** It was
        // put inside the street's own run on the first build, 15 deep
        // against a street 10 wide, so it lay across the thing it was
        // meant to close. `tests::the_street_is_clear` said so before
        // anything had been drawn.
        Built {
            at: vec3(middle.x - STREET_LONG * 0.5 - 8.0, middle.y, middle.z),
            wide: 12.0,
            long: 15.0,
            eaves: 6.0,
            ridge: 10.6,
            faces: FRAC_PI_2,
            render: None,
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

/// How wide the way there is worn, and how far clear of it the field keeps
/// its trees.
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
pub const PATH_CLEAR: f32 = 4.0;

/// How far the way there bows off the straight line between its ends, and in
/// how many steps the ribbon that carries it is built.
///
/// **Jake, 2026-11-22: it looks like a series of rectangles and not a true
/// curved path.** Both halves of that were true and they are separate
/// faults. The rectangles were forty boxes laid end to end, each the run's
/// one heading, each thrown a tenth of a metre sideways of the last: the
/// joins notched the edges and the end faces showed. The straightness was
/// the line itself. What it had was a wander of a seventh of a metre over
/// fifty, which is a surveyed road with a tremor in it.
///
/// A bow of three and a half is a path that visibly goes somewhere other
/// than straight, and it leans toward the village's own middle rather than
/// away from it: `PATH_CLEAR` clears the sightline from the gap by lying
/// near it, so a bow the other way would walk the cleared strip off the
/// view it exists to keep open.
///
/// Ninety-six steps over fifty-two metres is a vertex every half metre,
/// which carries a bow of this size with no corner you can find by walking
/// along it.
pub const PATH_BOW: f32 = 3.6;
pub const PATH_STEPS: usize = 96;

/// How much of the way there one tile of [`grit`] covers, in metres.
///
/// A little over a pace, so a chip in it is a chip and the tile's own
/// repeat is longer than the path is wide. The path's uv is laid in these
/// rather than nought to one over the whole run: a 52 metre ribbon with one
/// tile stretched down it would be a chip half a metre long.
pub const PATH_TILE: f32 = 1.1;

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

/// A point on the middle of the way there, from nought at the gap to one
/// where the street's own ruts take over.
///
/// **One account of the line**, which is the lesson the ruts already carry:
/// the ribbon is built from this, the trees are kept off it by this, and
/// the two would have come apart the first time either moved.
///
/// The bow leans toward the village and returns to the straight line at
/// both ends, because both ends are fixed: the gap is a hole in a wall and
/// the far end is where the ruts run out. The short wander on top of it is
/// the one the pieces always had, read off the distance down the run rather
/// than off a step number, so it is a wave in the line and not a tremor in
/// it.
pub fn path_at(reaches: f32, on: f32) -> Vec3 {
    let (from, to) = path_runs(reaches);
    let way = to - from;
    let long = way.length();
    let along = way / long;
    // across the run, which is the heading turned a quarter
    let across = vec3(-along.z, 0.0, along.x);
    // which side of the straight line the village's own middle is on, named
    // rather than written as a sign: the run ends at the street's east end
    // and the place itself is most of thirty degrees west of that, so
    // "toward the village" and "to the left" are not the same claim and only
    // one of them stays true if either end moves
    let toward = (at(reaches) - to).dot(across).signum();
    let down = on * long;

    // **One envelope over both of them.** The bow and the wander are each
    // how far off the straight line the walk goes, and both ends of the walk
    // are pinned: a wander that does not fade with the bow leaves the line
    // five centimetres clear of the gap it comes out of.
    from + along * down
        + across
            * (on * PI).sin()
            * (PATH_BOW * toward + (down * 0.21).sin() * 0.14 + (down * 0.53).sin() * 0.07)
}

/// Which way the way there is heading at a point on it, and which way is
/// across it there.
fn path_heads(reaches: f32, on: f32) -> (Vec3, Vec3) {
    let step = 0.5 / PATH_STEPS as f32;
    let ahead = path_at(reaches, (on + step).min(1.0)) - path_at(reaches, (on - step).max(0.0));
    let along = ahead.normalize_or_zero();

    (along, vec3(-along.z, 0.0, along.x))
}

/// How far the way there is worn out from its middle at a share of the run,
/// on the given side.
///
/// **A path has two edges and neither is a line.** Worn by feet that mostly
/// go down the middle, so the band is about as wide everywhere and about is
/// the whole of it: an edge off one number is a kerb. Each side has its own
/// phase, so the two do not swell and shrink together like a throat.
///
/// **Read off the share of the run and not off the metres walked**, which
/// are two numbers that nearly agree and are wanted in two places. The
/// ribbon's uv needs the metres actually walked, or the grit stretches
/// where the line bends. Its edge needs a number anything asking about the
/// edge can work out for itself: `on_the_way` is handed a spot in a field
/// and has no walk behind it, and an edge it reckoned a different way from
/// the one the mesh is built on is the grass line and the real line coming
/// apart.
fn path_edge(reaches: f32, on: f32, side: f32) -> f32 {
    let (from, to) = path_runs(reaches);
    let down = (to - from).length() * on;

    PATH_WIDE * 0.5
        + (down * 0.9 + side * 5.0).sin() * 0.09
        + (down * 2.3 + side * 11.0).sin() * 0.05
}

/// How far in from its edge the way there is trodden bare.
///
/// **Grass comes over the edge of a path, and the edge is where it starts.**
/// Spec 0126 left the grit stopping dead at a line with mown turf beside it,
/// because `field::cover` was kept off `along_the_path`: four metres either
/// side of a path 1.2 wide, so the one thing that could have broken the line
/// was the one thing not allowed within three metres of it. A hand's breadth
/// in from each side is what a tuft may stand on.
pub const PATH_OVER: f32 = 0.18;

/// Whether a spot is on the worn part of the way there, where nothing grows.
///
/// **The footprint and not the sightline.** [`along_the_path`] is the view
/// down the run, which is why it is four metres wide and why the trees
/// honour it. Ground cover honours this instead, which is the ribbon itself
/// less [`PATH_OVER`] at each side. The bridge's abutment is the same
/// lesson written down two specs earlier: its own footprint and not
/// `bridge::shades`, or there is a bald stripe down the field.
pub fn on_the_way(reaches: f32, at: Vec3) -> bool {
    beside_the_way(reaches, at).is_some_and(|off| off < 0.0)
}

/// How far a spot lies outside the worn part of the way there, in metres,
/// or nothing at all if it is not near it.
///
/// Negative on the path itself. [`on_the_way`] is this and a sign test; the
/// number is here because whether the grit's edge is broken is a question
/// about how near the nearest tuft gets to it, and a test that can only ask
/// yes or no cannot tell a verge from a path.
pub fn beside_the_way(reaches: f32, at: Vec3) -> Option<f32> {
    // the four metre corridor first, which is the cheap half of this and
    // takes nearly every spot the field asks about
    if !along_the_path(reaches, at) {
        return None;
    }

    let (from, to) = path_runs(reaches);
    let way = to - from;
    let how_far = way.length_squared();

    if how_far < 1e-6 {
        return None;
    }

    // and then the curve itself, finely, because a tuft judged off the path
    // when it is on it stands in the middle of the gravel
    let part = ((at - from).dot(way) / how_far).clamp(0.0, 1.0);
    let (on, near) = (-20..=20)
        .map(|n| (part + n as f32 * 0.005).clamp(0.0, 1.0))
        .map(|on| (on, path_at(reaches, on)))
        .min_by(|a, b| {
            a.1.distance_squared(at)
                .total_cmp(&b.1.distance_squared(at))
        })?;

    let (_, across) = path_heads(reaches, on);
    let off = (at - near).dot(across);

    Some(off.abs() - (path_edge(reaches, on, off.signum()) - PATH_OVER))
}

/// The way there, as one worn ribbon laid on the field.
///
/// **One mesh and not forty boxes.** Each box carried the run's one heading
/// and was thrown sideways of the last, so every join notched both edges
/// and showed an end face: Jake read it off a screenshot as a series of
/// rectangles. A strip built from [`path_at`] turns where the line turns,
/// has no joins to notch, and costs a tenth of the triangles the boxes did.
///
/// Its uv runs in [`PATH_TILE`]s along the walk and across the width, so
/// [`grit`] tiles down it at the size a chip of gravel actually is, and the
/// bump the engine bends by is read in the same units.
pub fn path_mesh(reaches: f32) -> blitzkit::mesh::MeshData {
    use blitzkit::mesh::Vertex;

    // laid on the field's own ground, which sits `field::DROP` under the
    // garden's, and only far enough proud of it to keep the two out of one
    // plane
    let high = -crate::field::DROP + 0.002;
    let up = [0.0, 1.0, 0.0];

    let mut vertices = Vec::with_capacity((PATH_STEPS + 1) * 2);
    let mut indices = Vec::with_capacity(PATH_STEPS * 6);
    // metres walked rather than the share of the run, because the bow makes
    // the line longer than the line between its ends and a texture laid in
    // shares would stretch where it bends. The edge is reckoned off the
    // share instead, and `path_edge` says why the two are not one number.
    let mut walked = 0.0f32;
    let mut last = path_at(reaches, 0.0);

    for n in 0..=PATH_STEPS {
        let on = n as f32 / PATH_STEPS as f32;
        let here = path_at(reaches, on);
        let (_, across) = path_heads(reaches, on);

        walked += here.distance(last);
        last = here;

        for side in [-1.0f32, 1.0].iter().copied() {
            let half = path_edge(reaches, on, side);
            let at = here + across * (half * side);

            // **u in tiles and v edge to edge.** Along the walk a tile has
            // to be a fixed size on the ground or the grit stretches where
            // the path bends. Across it, nought and one are the two edges
            // whatever the edge is doing there, which is what lets the tile
            // know where its own edges are: `grit` wears itself thin at
            // nought and one and the worn part lands on the edge of the
            // path rather than near it. The width wobbles by a sixth, so
            // the grit is stretched across by that much and no eye finds it
            // on a stone.
            vertices.push(Vertex::new(
                [at.x, high, at.z],
                up,
                [walked / PATH_TILE, 0.5 + side * 0.5],
            ));
        }

        if n < PATH_STEPS {
            // wound so the face looks up: the two on this step, then the far
            // side of the next, which is `across` crossed into `along` and
            // not the other way about
            let (near, far) = ((n * 2) as u32, (n * 2 + 2) as u32);

            indices.extend_from_slice(&[near, near + 1, far + 1, near, far + 1, far]);
        }
    }

    blitzkit::mesh::MeshData::new(vertices, indices)
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

    // **Measured to the line that is walked and not to the line between its
    // ends.** The bow stands three and a half metres off the straight one,
    // which is most of `PATH_CLEAR`: measured to the straight, the keep-out
    // would hang off the far edge of the path altogether and leave room for
    // a tree on it. The foot of the perpendicular on the straight line is
    // near enough the right place to look on a curve this gentle, and the
    // two steps either side of it cover where it is not.
    [-0.06f32, -0.03, 0.0, 0.03, 0.06]
        .iter()
        .map(|off| path_at(reaches, (part + off).clamp(0.0, 1.0)))
        .any(|near| vec3(at.x - near.x, 0.0, at.z - near.z).length() < PATH_CLEAR)
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
pub const FACE_WIDE: u32 = 4096;

/// **Seven hundred and sixty eight across, and four thousand along.** The
/// face is one texture over a street 58 by 10. At 256 across, a texel was
/// 3.9 centimetres and spec 0123's joint was 3.5: the joints fell under a
/// texel, aliased away, and the street read as a chequer of bare stones. 512
/// bought a joint two and a half texels wide. Spec 0124 wants the stones of
/// Jake's second photograph, which are a hand across and rounded, so the
/// setts came down to 0.20 and the face went up again: a texel is now 1.4
/// centimetres along and 1.3 across, which is fifteen to a stone, enough to
/// carry a dome and a joint. Twelve megabytes, built once.
pub const FACE_TALL: u32 = 768;

/// How big one sett is and how wide the joint between two of them.
pub const SETT: f32 = 0.20;
pub const JOINT: f32 = 0.035;

/// How far apart the fans are laid down the street.
///
/// **Segmentbogenpflaster, which is what Jake's first photograph is.** A
/// German street is not laid in courses across it. It is laid in fans: the
/// stones run on concentric arcs about a point on the centre line, and the
/// next fan starts where the last one ran out. A running bond is a pavement
/// and this is a street.
///
/// A little over half the street's width, which is what sets how far round
/// an arc bends before the next fan takes over.
pub const ARC: f32 = STREET_WIDE * 0.55;

/// Which stone a spot on the street belongs to, and how far it is from that
/// stone's own edge.
///
/// **Cutting these as geometry was tried and taken out.** Spec 0124 laid the
/// street as a `MeshData::surface` with every joint sunk into it, so the sun
/// would shade the grooves itself the way it shades the garden's water. It
/// came back as bright streaks down the street, and a groove of four
/// millimetres drew the same streaks as one of thirty five, which rules out
/// the angle the walls are lit at and points at the mesh's own normals or at
/// how its texture registers on it. The street is a flat quad, and what the
/// sun shades is [`bumps`], which answers the same question this does without
/// a vertex in it. Spec 0125.
///
/// **The fan, which is the whole of the pattern.** The nearest fan centre is
/// on the centre line, `ARC` apart down the street. From there a stone is a
/// ring out and a step round, so the courses are arcs and not rows, and
/// where one fan runs out the next begins, which is the seam every street
/// like this has.
///
/// The distance is in metres and is to the nearer of the two edges, the ring
/// boundary and the one round the arc, so one number answers both what the
/// joint is and how domed the stone is at that spot.
pub fn sett_at(along: f32, across: f32, middle: f32) -> ((f32, f32, f32), f32) {
    let fan = ((along / ARC) - 0.5).round();
    let (dx, dz) = (along - (fan + 0.5) * ARC, across - middle);
    let far = (dx * dx + dz * dz).sqrt();

    // **The keystone.** Every radial joint in a fan meets at its middle, so
    // reckoned the same way as the rest the middle is a point with three
    // joints running out of it and `atan2` swinging a whole turn across one
    // texel. In the colour that is a fleck and in `bumps` it was a
    // starburst leaning fifty three out of a hundred and twenty eight, in
    // the middle of what should be the flattest stone on the street. A fan
    // is laid from one stone and not from a point, which is what this is.
    let keystone = SETT * 0.5;

    if far < keystone {
        return ((fan, -1.0, 0.0), (keystone - far).min(to_seam(along, fan)));
    }

    // the rings are reckoned from the keystone's edge, so every one of them
    // is a whole sett deep
    let ring = ((far - keystone) / SETT).floor();
    let inner = keystone + ring * SETT;

    // how far round the arc one stone takes, which is a stone's width in
    // arc length rather than in angle: the outer rings hold more of them
    let step = SETT / far;
    let round = dz.atan2(dx);
    let nth = (round / step).floor();

    let to_ring = (far - inner).min(inner + SETT - far);

    // **And the ray the angle wraps on is a joint as well.** `atan2` answers
    // between minus pi and pi, so the one cell in every ring that straddles
    // the far side of a fan's centre is cut in two by the arithmetic rather
    // than by a joint, and its two halves sit at whatever height each of
    // them reckons. Found the same way the seam was: a line of leaning
    // texels in the middle of what `sett_at` called a stone, out along minus
    // x from every fan centre on the street.
    let to_side = (round - nth * step)
        .min((nth + 1.0) * step - round)
        .min(std::f32::consts::PI - round.abs())
        * far;

    // **The joint round the arc counts for more than the joint along it.**
    // The street is looked down, so a ring joint crosses the view and a
    // radial one runs away along it: read straight, the arcs are all you
    // see and the stones in them are not there. Three quarters is what puts
    // a stone back inside its own arc.
    (
        (fan, ring, nth),
        to_ring.min(to_side * 0.75).min(to_seam(along, fan)),
    )
}

/// How far a spot is from the line where one fan gives way to the next.
///
/// **Where one fan ends is a joint too.** `fan` steps at the half way line
/// between two centres, so on that line what a stone is reckoned from
/// changes: read without this, the last ring of one fan butts the first of
/// the next at whatever height each of them happened to be, which is a step
/// in the street rather than a joint in it. In the colour it passes for a
/// butt line and in [`bumps`] it is a ridge across the street every `ARC`,
/// which is how it was found.
fn to_seam(along: f32, fan: f32) -> f32 {
    ARC * (0.5 - ((along / ARC) - 0.5 - fan).abs())
}

/// How wide the kerb either side of the street is and how far it stands over
/// the paving.
///
/// **What gives the stone a thickness you can see.** A groove shows a sett
/// has a top; a kerb shows the whole street is a course of stone laid on the
/// ground rather than a pattern printed on it.
pub const KERB_WIDE: f32 = 0.3;
pub const KERB_UP: f32 = 0.11;

pub fn kerbs(reaches: f32) -> Vec<Aabb> {
    let ground = track(reaches);

    [-1.0f32, 1.0]
        .iter()
        .map(|side| {
            let at = ground.center().z + side * (ground.size().z * 0.5 + KERB_WIDE * 0.5);

            Aabb::new(
                vec3(ground.min.x, ground.max.y - KERB_UP, at - KERB_WIDE * 0.5),
                vec3(ground.max.x, ground.max.y + KERB_UP, at + KERB_WIDE * 0.5),
            )
        })
        .collect()
}

/// A number between nought and one from two, which is what gives one sett a
/// different grey from the one beside it.
///
/// Its own here rather than reached for out of `plasma`: that one is a dice
/// for an arm of a plasma globe and this one is a dice for a paving stone,
/// and a module borrowing another module's randomness is a coupling that
/// says the two have something to do with each other.
fn speckle(a: f32, b: f32) -> f32 {
    let x = (a * 12.9898 + b * 78.233).sin() * 43758.547;

    x - x.floor()
}

/// The street's face: setts, with the two polished lines the wheels run in.
/// Spec 0096, paved by spec 0123.
///
/// **Painted and not built.** The ruts were 116 slabs standing two
/// millimetres proud of the track, 58 a side, which is a texture's job done
/// with geometry: each one cast its own shadow, so the lines swung across
/// the street as the sun went round and aliased into a crawl at distance.
/// Jake, who found it: textured surfaces should have small shadows, mipmaps
/// are a thing for a reason. The engine's `build_mips` says the same in its
/// own words, that without it a textured floor seen at an angle shimmers.
/// Setts are the same argument at a smaller size: a street of little boxes
/// would be nine thousand of them.
///
/// **And the ruts stayed when the earth went.** A paved street does not rut,
/// but it polishes: the stone where the wheels run is darker and smoother
/// than the stone either side, which is the same two lines down the same
/// curve and keeps the picture's own account of itself true.
pub fn face(reaches: f32) -> blitzkit::texture::TextureData {
    let ground = track(reaches);
    let (from, size) = (ground.min, ground.size());
    let sett = crate::aim::VILLAGE_SETT;
    let joint = crate::aim::VILLAGE_JOINT;
    let rut = crate::aim::VILLAGE_RUT;
    let mut pixels = Vec::with_capacity((FACE_WIDE * FACE_TALL * 4) as usize);

    for y in 0..FACE_TALL {
        for x in 0..FACE_WIDE {
            // the texel's own place on the track, in the world's own numbers,
            // so the curve below is the one the wheels wore
            let along = from.x + (x as f32 + 0.5) / FACE_WIDE as f32 * size.x;
            let across = from.z + (y as f32 + 0.5) / FACE_TALL as f32 * size.z;

            // which stone this is, and how far from its own edge
            let ((fan, ring, nth), near) = sett_at(along, across, ground.center().z);
            // **Its own joint, wider or narrower than its neighbour's.** A
            // fan laid off one arithmetic is a perfect fan, and a perfect
            // fan reads as a moulding rather than as stones somebody set by
            // hand. A fifth of the joint either way is enough to break it.
            let wonky = speckle(ring * 2.7 + nth, fan * 5.9);
            let near = near - JOINT * 0.35 * wonky;
            // over the joint's whole width and not half of it: at half,
            // the dark band saturates a centimetre and a half from the edge
            // and the street washes out to one grey
            let laid = (near / JOINT).clamp(0.0, 1.0);
            let laid = laid * laid * (3.0 - 2.0 * laid);

            // **Its own grey, warm or cool.** Jake's second photograph is a
            // street of stones that are each a different stone: one pale,
            // one blue, one brown, none of them the same. One grey varied
            // in brightness alone reads as one stone cut up.
            let own = speckle(fan * 7.3 + ring, nth);
            let own = own * own * (3.0 - 2.0 * own);
            let warm = speckle(nth * 3.1 + ring, fan);
            let mine = sett
                * (0.78 + 0.46 * own)
                * vec3(0.95 + 0.1 * warm, 1.0, 1.06 - 0.12 * warm).extend(1.0);

            // **The dome stays, and it is the occlusion half of it.** It
            // was written when this engine had no normal maps and stood in
            // for the shape of a stone altogether. `bumps` carries the
            // shape now, so the obvious thing was to take this out, and
            // taking it out washed the street pale: it falls away in every
            // direction at once rather than toward the sun, which is what
            // occlusion looks like and what a slot between two stones
            // actually does to the light reaching the bottom of it. What
            // `bumps` adds is the part this one cannot have, which is a
            // stone lit on the side the sun is on.
            let dome = 0.78 + 0.26 * (near / (SETT * 0.5)).clamp(0.0, 1.0).sqrt();
            let stoned = joint.lerp(mine * dome, laid);

            // how far off the nearer polished line it is, soft over the last
            // fifth of its width so the edge is worn rather than cut
            let off = [-1.0f32, 1.0]
                .iter()
                .map(|side| (across - (ground.center().z + rut_runs(*side, along))).abs())
                .fold(f32::MAX, f32::min);
            let wear = RUT_WIDE * 0.1;
            let worn = ((RUT_WIDE * 0.5 + wear - off) / (wear * 2.0)).clamp(0.0, 1.0);

            // **The joints run through the polished line too.** Lerped to
            // the rut's own colour alone, the wheel lines came out as two
            // bands with no stones in them, which reads as a strip laid over
            // the paving rather than as the paving worn, and averaged
            // lighter than the street beside them because the street's
            // average has its joints in it.
            let wet = joint.lerp(rut * (0.92 + 0.16 * own) * dome, laid);
            let colour = stoned.lerp(wet, worn);
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

/// How deep the joint is cut and how far the stone domes over it, in metres.
///
/// **A hand deep is not what a street is.** Spec 0124 reached for thirty five
/// millimetres of groove when it was cutting them as geometry, which is a
/// gutter rather than a joint; a sett is set in sand with a centimetre or so
/// of it showing. The dome is the smaller of the two on purpose: a cobble is
/// rounded and a sett is cut, and what reads as stone laid by hand is the
/// slot between them rather than the curve on top.
pub const SUNK: f32 = 0.014;
pub const DOMED: f32 = 0.004;

/// How high the street stands at one spot, in metres over its own plane.
///
/// Off `sett_at`'s one number, so the shape the sun shades and the joint the
/// colour draws cannot come apart: this is the same stone falling away toward
/// the same edge.
fn proud_at(along: f32, across: f32, middle: f32) -> f32 {
    let (_, near) = sett_at(along, across, middle);
    // the joint's wall, over the joint's own width, and smoothed because a
    // step of one texel is a seam rather than an edge once the mips average
    // it
    let up = (near / JOINT).clamp(0.0, 1.0);

    SUNK * up * up * (3.0 - 2.0 * up) + DOMED * (near / (SETT * 0.5)).clamp(0.0, 1.0).sqrt()
}

/// The street's shape, as a normal map in register with [`face`]. Spec 0125,
/// on the engine's spec 0110.
///
/// **This is the thing spec 0124 put out of scope.** That spec baked the
/// depth into the colour and said in as many words what it cost: a stone
/// that falls away toward its joint in the albedo does not move when the sun
/// does, so at eight in the morning and at noon the street was lit exactly
/// alike. The engine can be told now, so it is told, and [`face`]'s dome came
/// down to the part of it that was never about the sun.
///
/// **Raw bytes and not [`encoded`].** The bump binding reads the upload
/// through `texture::DATA`, which is the same bytes as numbers rather than as
/// a colour. Put the sRGB curve on a direction and it is not that direction:
/// a half encodes to 188, so a surface meant to lean a little leans a long
/// way and the whole street tilts one way. Every other texture in this
/// building goes through `encoded` and this one must not.
///
/// The field is built first and differenced after, rather than asking
/// `sett_at` five times a texel: it is 3.1 million texels and four of the
/// five answers would be thrown away.
pub fn bumps(reaches: f32) -> blitzkit::texture::TextureData {
    let ground = track(reaches);
    let (from, size) = (ground.min, ground.size());
    let middle = ground.center().z;
    let (wide, tall) = (FACE_WIDE as usize, FACE_TALL as usize);

    let mut high = vec![0.0f32; wide * tall];
    for (y, row) in high.chunks_exact_mut(wide).enumerate() {
        let across = from.z + (y as f32 + 0.5) / FACE_TALL as f32 * size.z;

        for (x, at) in row.iter_mut().enumerate() {
            let along = from.x + (x as f32 + 0.5) / FACE_WIDE as f32 * size.x;
            *at = proud_at(along, across, middle);
        }
    }

    // how much world a texel is worth each way, which is what turns a
    // difference in height into a slope. The street is 72 along by 10
    // across, so they are not the same number.
    //
    // **The quad's own u and v, which `room::tiled_plane` lays along x and z
    // in that order.** The shader builds its frame off those, so red leans
    // the normal along the street and green across it. Swapped, the joints
    // would light as if the sun were somewhere else.
    let (per_u, per_v) = (size.x / FACE_WIDE as f32, size.z / FACE_TALL as f32);
    // laid once on the one street, so its edge is the kerb and not another
    // copy of itself
    let pixels = normals_of(wide, tall, &high, per_u, per_v, false);

    blitzkit::texture::TextureData::from_pixels(FACE_WIDE, FACE_TALL, pixels)
}

/// A field of heights turned into the bytes of a normal map.
///
/// **Raw bytes and not [`encoded`].** The bump binding reads the upload
/// through `texture::DATA`, which is the same bytes as numbers rather than
/// as a colour. Put the sRGB curve on a direction and it is not that
/// direction: a half encodes to 188, so a surface meant to lean a little
/// leans a long way.
///
/// `per_u` and `per_v` are how much world a texel is worth each way, which
/// is what turns a difference in height into a slope rather than into a
/// number of texels.
///
/// **`wrapped` is a question about what the texture is for.** The street's
/// face is laid on the street once and its edges are its kerbs, so a slope
/// read there is read from the edge inward. A tile of grit is repeated down
/// a path, so its left neighbour is its own right edge and that is where the
/// slope at its border comes from.
///
/// It is a correctness worth having and not a fault anybody would see: a
/// clamped border reads its own height twice over half the span, which on a
/// field as smooth as this one comes out within a byte of the right answer.
/// What actually keeps the tile seamless is that `grit_field` sows its chips
/// across the border, and that is what the test guards.
fn normals_of(
    wide: usize,
    tall: usize,
    high: &[f32],
    per_u: f32,
    per_v: f32,
    wrapped: bool,
) -> Vec<u8> {
    let step = |n: usize, by: isize, many: usize| {
        if wrapped {
            (n as isize + by).rem_euclid(many as isize) as usize
        } else if by < 0 {
            n.saturating_sub(1)
        } else {
            (n + 1).min(many - 1)
        }
    };
    // how many texels apart the two samples are, which is two everywhere
    // but at a clamped edge, where one of them is the texel itself
    let apart = |a: usize, b: usize| if wrapped { 2.0 } else { (b - a) as f32 };
    let byte = |n: f32| ((n * 0.5 + 0.5) * 255.0).round().clamp(0.0, 255.0) as u8;
    let mut pixels = Vec::with_capacity(wide * tall * 4);

    for y in 0..tall {
        for x in 0..wide {
            let (left, right) = (step(x, -1, wide), step(x, 1, wide));
            let (near, far) = (step(y, -1, tall), step(y, 1, tall));
            let du =
                (high[y * wide + right] - high[y * wide + left]) / (apart(left, right) * per_u);
            let dv = (high[far * wide + x] - high[near * wide + x]) / (apart(near, far) * per_v);
            let out = vec3(-du, -dv, 1.0).normalize();

            pixels.extend_from_slice(&[byte(out.x), byte(out.y), byte(out.z), 255]);
        }
    }

    pixels
}

/// How many texels across one tile of grit, how many chips are scattered
/// into it, how big the smallest and biggest of them are across their own
/// middle, and how far the biggest stands proud. Metres, all three of the
/// last.
///
/// **Five hundred and twelve for a tile of 1.1 metres**, which is a texel
/// every 2.1 millimetres: a chip two centimetres across is nine texels and
/// carries a shape, where at 256 it was four and carried a smudge. One
/// megabyte, built once.
///
/// Twelve hundred and fifty chips of this size cover about three fifths of
/// the tile once they overlap. Two thousand covered four fifths and the
/// path came out the palest thing in the frame, a ribbon of salt laid on a
/// field: what makes it read as a track is the earth between the stones,
/// and at four fifths there was none of it left to see. The chips are
/// scattered into the field rather than the field asked of every chip, so
/// the work is the area of the chips and not the area of the tile times
/// their number.
pub const GRIT_TEXELS: u32 = 512;
pub const CHIPS: usize = 1250;
pub const CHIP_SMALL: f32 = 0.008;
pub const CHIP_BIG: f32 = 0.034;
pub const CHIP_PROUD: f32 = 0.009;

/// One tile of grit: how high it stands at each texel, and the dice of
/// whichever chip is the one standing there.
///
/// **Both answers from one scattering**, so the chip the colour gives its
/// own grey to is the chip the sun shades. The street learned this the
/// expensive way and `proud_at` says so.
///
/// It wraps. A chip sown near an edge is written out of the other side as
/// well, because the tile is repeated down the path and a chip cut off at a
/// border is a border you can see from forty metres.
fn grit_field() -> (Vec<f32>, Vec<f32>) {
    let many = GRIT_TEXELS as usize;
    let per_metre = GRIT_TEXELS as f32 / PATH_TILE;
    let mut high = vec![0.0f32; many * many];
    let mut whose = vec![0.0f32; many * many];

    for chip in 0..CHIPS {
        // kept small, because `speckle` is a sine of its arguments and two
        // thousand of them unscaled is a sine of twenty-six thousand, where
        // a float has no decimals left to tell one chip from the next
        let seed = chip as f32 * 0.113;
        let dice = speckle(seed, 4.1);
        // squared, so there are many small chips and few big ones. Evenly
        // spread, every chip is nearly the biggest and the tile reads as
        // peas
        let wide = CHIP_SMALL + (CHIP_BIG - CHIP_SMALL) * dice * dice;
        let proud = CHIP_PROUD * (0.45 + 0.55 * wide / CHIP_BIG);
        let (at_x, at_y) = (speckle(seed, 2.3), speckle(seed, 8.7));
        let own = speckle(seed, 6.9);

        let middle = |share: f32| (share * GRIT_TEXELS as f32) as isize;
        let (mid_x, mid_y) = (middle(at_x), middle(at_y));
        let reach = (wide * per_metre).ceil() as isize;

        for down in -reach..=reach {
            for along in -reach..=reach {
                let far = ((along * along + down * down) as f32).sqrt() / (wide * per_metre);

                if far >= 1.0 {
                    continue;
                }

                // **Flatter than a dome.** A chip of road metal is a pebble
                // trodden into the earth and not a ball sitting on it, and
                // a hemisphere's edge is a cliff the mips turn into a ring.
                let stands = proud * (1.0 - far * far);
                let x = (mid_x + along).rem_euclid(many as isize) as usize;
                let y = (mid_y + down).rem_euclid(many as isize) as usize;
                let n = y * many + x;

                if stands > high[n] {
                    high[n] = stands;
                    whose[n] = own;
                }
            }
        }
    }

    // **And worn thin at the two edges.** A path is walked down its middle,
    // so the stone at its sides is the stone that has been trodden furthest
    // into the earth. Left alone, the band of grit stopped dead at a line
    // and read as a carpet runner laid on a field. The tile's v is the path
    // edge to edge, so nought and one here are exactly its two sides.
    for (y, row) in high.chunks_exact_mut(many).enumerate() {
        let share = (y as f32 + 0.5) / many as f32;
        let worn = 0.30 + 0.70 * (share * PI).sin();

        for at in row.iter_mut() {
            *at *= worn;
        }
    }

    (high, whose)
}

/// The grit of the way there, as a tile to be repeated down it. Spec 0126.
///
/// **A tile and not a face.** The street's own surface is one texture laid
/// on it once, because the ruts follow a curve down the street in the
/// world's own numbers and nothing repeating can carry that. Nothing about
/// this path varies along it: it is the same grit at the gap as at the
/// ruts, so a tile of it repeated is the same picture for a thousandth of
/// the memory.
pub fn grit() -> blitzkit::texture::TextureData {
    let (high, whose) = grit_field();
    let many = GRIT_TEXELS as usize;
    let chip = crate::aim::VILLAGE_GRIT;
    let earth = crate::aim::VILLAGE_GRIT_EARTH;
    let mut pixels = Vec::with_capacity(many * many * 4);

    for y in 0..many {
        for x in 0..many {
            let n = y * many + x;
            let up = (high[n] / CHIP_PROUD).clamp(0.0, 1.0);

            // the earth between, speckled fine, because a flat colour
            // between the stones is a sheet of card with stones on it
            let grain = speckle(x as f32 * 0.37, y as f32 * 0.71);
            let under = earth * (0.82 + 0.36 * grain);

            // **Each chip its own grey, warm or cool.** Two chips that
            // touch share an outline and nothing else; given one colour
            // they are a puddle of stone with dents in it. This is the
            // street's own argument at a quarter of the size.
            let own = whose[n];
            let tint = vec3(1.0 + 0.07 * (own - 0.5), 1.0, 1.0 - 0.09 * (own - 0.5));
            let mine = chip * (0.70 + 0.52 * own) * tint.extend(1.0);

            // and the occlusion half of the shape, as the street's dome is:
            // the bump says which way a chip faces, and what it cannot say
            // is that the gap between two of them is a hole the light has
            // to get into
            let lit = 0.70 + 0.30 * up.sqrt();
            // over the chip's own rim rather than its whole height, so a
            // chip has an edge and not a halo
            let colour = under.lerp(mine * lit, (up * 3.5).clamp(0.0, 1.0));

            pixels.extend_from_slice(&[
                encoded(colour.x),
                encoded(colour.y),
                encoded(colour.z),
                255,
            ]);
        }
    }

    blitzkit::texture::TextureData::from_pixels(GRIT_TEXELS, GRIT_TEXELS, pixels)
}

/// And its shape, as a normal map in register with [`grit`]. Spec 0126, on
/// the engine's spec 0110.
///
/// Wrapped, which [`normals_of`] says the most about: this one is repeated
/// and the street's is not.
pub fn grit_bumps() -> blitzkit::texture::TextureData {
    let (high, _) = grit_field();
    let many = GRIT_TEXELS as usize;
    let per = PATH_TILE / GRIT_TEXELS as f32;

    blitzkit::texture::TextureData::from_pixels(
        GRIT_TEXELS,
        GRIT_TEXELS,
        normals_of(many, many, &high, per, per, true),
    )
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

/// How far the painted surround stands out past the pane, and how far off
/// the wall each of the three layers sits.
///
/// **A pane alone is invisible in daylight.** It is drawn through
/// `garden::gives_off`, which is never under one, so by day it is a pale
/// warm patch on a pale warm render and by night it is a lit window. The
/// surround is what gives it an edge at noon, which is what every Altstadt
/// facade has in white against its colour.
pub const SURROUND: f32 = 0.09;
pub const PROUD: f32 = 0.03;
pub const WINDOW_UP: f32 = 1.45;

/// How tall a storey is, and how wide a bay.
///
/// **What the rows are spaced by, per spec 0123.** There was one window to a
/// building, at 1.25, which says nothing about how tall the building is. A
/// row to each floor is the other half of what three storeys look like, and
/// the bays across are what stop a four-window row on a nine metre front
/// reading as portholes.
pub const STOREY: f32 = 2.9;

/// How wide a shutter leaf is.
pub const SHUTTER_WIDE: f32 = 0.34;
pub const BAY: f32 = 3.0;

/// How many floors of windows one of them carries.
///
/// Its eaves in whole storeys, so a taller building has more of them without
/// anything being written down twice.
pub fn storeys(one: &Built) -> usize {
    ((one.eaves / STOREY).floor() as usize).max(1)
}

/// And how many windows across its front.
pub fn bays(across: f32) -> usize {
    ((across / BAY).round() as usize).max(2)
}

/// Each of a building's four walls: which way it looks, where its face is,
/// how wide it is and which way along it the bays run.
///
/// **All four and not the front alone, per spec 0123.** A terrace shows one
/// face and these stand free, so three of their four walls were blank. It
/// matters more now than it did: a blank wall two and a half metres high is
/// a shed and the same wall at eleven is a slab, and the staged shot looks
/// along the street, where the fronts are side on and the flanks are what
/// you see.
pub fn walls_of(one: &Built) -> Vec<(Vec3, Vec3, f32, Vec3)> {
    [
        (Vec3::Z, one.long, one.wide, Vec3::X),
        (-Vec3::Z, one.long, one.wide, Vec3::X),
        (Vec3::X, one.wide, one.long, Vec3::Z),
        (-Vec3::X, one.wide, one.long, Vec3::Z),
    ]
    .iter()
    .copied()
    .map(|(looks, deep, across, sideways)| {
        // **On the wall's face.** It was three tenths inside it, which is
        // a pane inside a solid box, which the engine draws nowhere at all:
        // every window in this village had been buried since it was built,
        // and nobody saw it because a one-storey cabin at thirty metres is
        // a shape rather than a facade.
        (looks, looks * (deep * 0.5), across, sideways)
    })
    .collect()
}

pub fn windows(reaches: f32) -> Vec<(Vec3, Vec3)> {
    let mut out = Vec::new();

    for one in built(reaches) {
        for (looks, face, across, sideways) in walls_of(&one) {
            let bays = bays(across);
            let step = across / bays as f32;

            for floor in 0..storeys(&one) {
                for bay in 0..bays {
                    let off = (bay as f32 + 0.5) * step - across * 0.5;
                    let up = WINDOW_UP + floor as f32 * STOREY;

                    out.push((one.at + face + sideways * off + Vec3::Y * up, looks));
                }
            }
        }
    }

    out
}

/// What each one is rendered in, which is a colour from spec 0123's palette
/// or the barn's own brown.
pub fn rendered(one: &Built) -> glam::Vec4 {
    match one.render {
        Some(n) => crate::aim::VILLAGE_RENDERS[n % crate::aim::VILLAGE_RENDERS.len()],
        None => crate::aim::VILLAGE_WALL,
    }
}

/// How high the stone course at the foot of a rendered wall stands.
pub const STONE_UP: f32 = 1.0;

/// That course, as the box it is drawn with: the walls again, cut to its
/// height and standing a little proud so it reads as stone laid against the
/// render rather than as a stripe painted on it.
pub const STONE_OUT: f32 = 0.08;

pub fn stone_course(one: &Built) -> Aabb {
    Aabb::from_center_size(
        one.at + Vec3::Y * STONE_UP * 0.5,
        vec3(
            one.wide + STONE_OUT * 2.0,
            STONE_UP,
            one.long + STONE_OUT * 2.0,
        ),
    )
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
        let texel_blue = |along: f32, across: f32| {
            let x = (((along - from.x) / size.x) * FACE_WIDE as f32) as u32;
            let y = (((across - from.z) / size.z) * FACE_TALL as f32) as u32;
            let n = ((y.min(FACE_TALL - 1) * FACE_WIDE + x.min(FACE_WIDE - 1)) * 4 + 2) as usize;
            let level = face.levels.first().expect("a base level");

            level.pixels[n]
        };

        // a quarter, a half and three quarters of the way down it
        for share in [0.25f32, 0.5, 0.75] {
            let along = from.x + size.x * share;

            // **Averaged over a stride and not read off one texel.** The
            // street was one flat colour when this was written, so one
            // texel answered for it. Spec 0124 gave every stone its own
            // grey and a dark joint round it, so a single texel on the rut
            // can land on a stone while the one beside it lands in a joint,
            // and the rut reads lighter than the street it is worn into.
            // and the line is followed as it wanders, rather than held at
            // one z while the walk goes on without it
            let steps = 24;
            let mean = |off: &dyn Fn(f32) -> f32| {
                let total: u32 = (0..steps)
                    .map(|n| {
                        let a = along + n as f32 * SETT * 0.5;

                        texel(a, off(a)) as u32
                    })
                    .sum();

                total as f32 / steps as f32
            };

            for side in [-1.0f32, 1.0] {
                // beside the line rather than at the street's middle: the
                // claim is that the stone the wheels run on is darker than
                // the stone next to it, and the middle of the street is a
                // different question
                let on = mean(&|a| ground.center().z + rut_runs(side, a));
                let off = mean(&|a| ground.center().z + rut_runs(side, a) + RUT_WIDE * 2.0);

                assert!(
                    on < off,
                    "at {:.1} along, the {} rut reads {:.0} against the street at {:.0}",
                    along,
                    if side < 0.0 { "near" } else { "far" },
                    on,
                    off
                );
            }
        }

        // and what is between them is paving, read back through the sRGB
        // the sampler will undo.
        //
        // **By its hue and not by its brightness.** This asked for
        // `VILLAGE_EARTH` exactly, which was right while the street was one
        // flat colour, and spec 0123 paved it: a sett is its own tenth
        // either side of `VILLAGE_SETT`, the joints are darker and the band
        // between the two is every value in between, so no brightness is
        // the right answer any more. The earth and the joint are within a
        // thousandth of each other in red, which is what made the first
        // try of this fail: what tells them apart is that earth is warm and
        // stone is not.
        let linear = |texel: u8| {
            let one = texel as f32 / 255.0;

            if one <= 0.040_45 {
                one / 12.92
            } else {
                ((one + 0.055) / 1.055).powf(2.4)
            }
        };
        let warmth = |colour: glam::Vec4| colour.z / colour.x;

        for share in [0.2f32, 0.4, 0.6, 0.8] {
            let along = from.x + size.x * share;
            let red = linear(texel(along, ground.center().z));
            let blue = linear(texel_blue(along, ground.center().z));
            let mine = blue / red;

            assert!(
                mine > warmth(crate::aim::VILLAGE_EARTH) + 0.1,
                "the street reads {:.2} blue to red at {:.1} along, and the earth it was paved \
                 over reads {:.2}",
                mine,
                along,
                warmth(crate::aim::VILLAGE_EARTH)
            );
            assert!(
                (mine - warmth(crate::aim::VILLAGE_SETT)).abs() < 0.12,
                "the street reads {:.2} blue to red at {:.1} along, where a sett reads {:.2}",
                mine,
                along,
                warmth(crate::aim::VILLAGE_SETT)
            );
        }
    }

    /// Spec 0125: the street's shape is in a normal map, it leans the way
    /// the stone it is on falls, and it is written as numbers rather than
    /// as a colour.
    ///
    /// **The last of those three is the one that cannot be seen.** A bump
    /// map put through `encoded` still looks like a bump map and still
    /// bends every draw on the street, by the wrong amount and in a way no
    /// picture names: a flat texel's 128 encodes to 188, so the middle of
    /// every stone leans a quarter turn off the surface it is on. The check
    /// for it is the flat half of this test, which is why the bound there
    /// is tight.
    #[test]
    fn the_street_leans_at_its_joints_and_lies_flat_on_its_stones() {
        let reaches = 9.0;
        let ground = track(reaches);
        let map = bumps(reaches);
        let (from, size) = (ground.min, ground.size());
        let level = map.levels.first().expect("a base level");

        assert_eq!(
            (map.width(), map.height()),
            (FACE_WIDE, FACE_TALL),
            "the shape and the colour are read off one set of texture \
             coordinates, so they have to be the same size"
        );

        // nothing on the street leans into the ground it is laid on
        let under = level
            .pixels
            .as_chunks::<4>()
            .0
            .iter()
            .filter(|texel| texel[2] < 128)
            .count();
        assert_eq!(
            under, 0,
            "{} texels of the street lean into the earth",
            under
        );

        let world = |x: usize, y: usize| {
            (
                from.x + (x as f32 + 0.5) / FACE_WIDE as f32 * size.x,
                from.z + (y as f32 + 0.5) / FACE_TALL as f32 * size.z,
            )
        };
        let lean = |x: usize, y: usize| {
            let n = (y * FACE_WIDE as usize + x) * 4;

            (
                level.pixels[n] as f32 - 128.0,
                level.pixels[n + 1] as f32 - 128.0,
            )
        };

        let (mut flattest, mut keenest) = (0.0f32, 0.0f32);
        let (mut stones, mut joints) = (0, 0);

        for y in [
            FACE_TALL as usize / 4,
            FACE_TALL as usize / 2,
            FACE_TALL as usize * 3 / 4,
        ] {
            for x in 1..FACE_WIDE as usize - 1 {
                let (along, across) = world(x, y);
                let (_, near) = sett_at(along, across, ground.center().z);
                let (du, dv) = lean(x, y);
                let most = du.abs().max(dv.abs());

                if near >= SETT * 0.3 {
                    // The middle of a stone, where the dome is all there
                    // is. Three tenths and not a half: `sett_at` answers
                    // with the nearer of two edges and the one round the
                    // arc is counted at three quarters, so the furthest
                    // from an edge anywhere on this street can be is
                    // 0.075 and a half of a sett is unreachable.
                    stones += 1;
                    flattest = flattest.max(most);
                } else if near < JOINT * 0.3 {
                    // and the bottom of a joint, where the wall is
                    joints += 1;
                    keenest = keenest.max(most);
                }
            }
        }

        assert!(
            stones > 100 && joints > 100,
            "{} stone texels and {} joint ones is not enough of either to be reading the street",
            stones,
            joints
        );
        assert!(
            flattest < 12.0,
            "the middle of a stone leans {:.0} out of 128, where the dome alone is worth about \
             three and an sRGB encode would be worth sixty",
            flattest
        );
        assert!(
            keenest > 25.0,
            "the keenest joint in the street leans {:.0} out of 128, which is a street the sun \
             cannot find the joints in",
            keenest
        );

        // **And each joint wall leans up out of the joint.** The sign the
        // other way round lights every joint as a ridge, which is a street
        // of raised lines rather than of set stones and reads at a glance
        // as the same street. Read on a radius out of a fan's own centre,
        // at the angular middle of a cell where `to_side` is a whole half
        // stone and cannot be the nearer edge, so what the gradient has in
        // it is the ring wall alone and the inner and outer walls of one
        // stone must lean opposite ways.
        //
        // The angle is not along either axis, so a street whose red and
        // green were swapped fails this as well: the radius has both an x
        // and a z in it and they are not the same number.
        let middle = ground.center().z;
        let fan = ((ground.center().x / ARC) - 0.5).round();
        let centre = (fan + 0.5) * ARC;
        let mut walls = 0;

        for ring in [2.0f32, 3.0, 5.0] {
            for turn in [0.6f32, 2.3, -1.1] {
                let inner = SETT * 0.5 + ring * SETT;

                for (at, sense) in [
                    (inner + JOINT * 0.5, -1.0f32),
                    (inner + SETT - JOINT * 0.5, 1.0),
                ] {
                    // the angular middle of whichever cell that angle falls
                    // in, where `to_side` is a whole three quarters of half
                    // a sett and so cannot be the nearer edge: what is left
                    // in the gradient is the ring's own wall
                    let step = SETT / at;
                    let turn = ((turn / step).floor() + 0.5) * step;
                    let (out, back) = (turn.cos(), turn.sin());
                    let (along, across) = (centre + out * at, middle + back * at);
                    let x = ((along - from.x) / size.x * FACE_WIDE as f32) as usize;
                    let y = ((across - from.z) / size.z * FACE_TALL as f32) as usize;
                    let (du, dv) = lean(x, y);
                    // how far the lean points out along the radius it was
                    // read on, which is the whole claim in one number
                    let outward = (du * out + dv * back) * sense;

                    walls += 1;
                    assert!(
                        outward > 8.0,
                        "the {} wall of ring {} at {:.1} radians leans {:.0} {:.0}, which is \
                         {:.0} up out of its own joint",
                        if sense < 0.0 { "inner" } else { "outer" },
                        ring,
                        turn,
                        du,
                        dv,
                        outward
                    );
                }
            }
        }

        assert_eq!(
            walls, 18,
            "{} joint walls is not the two faces of each of nine stones",
            walls
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
            // the line that is walked and not the line between its ends,
            // which spec 0126 bowed three and a half metres apart
            let on = path_at(REACHES, n as f32 / 120.0);
            let (i, j) = ((on.x / cell).round() as i32, (on.z / cell).round() as i32);

            for about in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
                let at = (i + about.0, j + about.1);

                // **A tree and a tuft are kept off two different things.**
                // The trees honour the sightline, which is what spec 0086
                // wore this run for. Ground cover honours the footprint,
                // since spec 0127: kept off the sightline it left a mown
                // verge three metres wide either side, and the one thing
                // that could break the grit's edge was the one thing not
                // allowed near it.
                for one in crate::field::trees(REACHES, at) {
                    stood += 1;

                    assert!(
                        !along_the_path(REACHES, one.at),
                        "a tree grows at {:?}, which is in the view down the \
                         way to the village",
                        one.at
                    );
                }

                for one in crate::field::cover(REACHES, at) {
                    stood += 1;

                    assert!(
                        !on_the_way(REACHES, one.at),
                        "something grows at {:?}, which is on the way to the \
                         village and not beside it",
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

    /// Spec 0126: the way there is a curve, and it leans toward the place
    /// it goes to.
    #[test]
    fn the_way_there_bends_toward_the_village() {
        let (from, to) = path_runs(REACHES);

        // both ends are still the ends: the gap is a hole in a wall and the
        // far end is where the ruts run out, so the bow has to come back to
        // them
        for (on, end) in [(0.0f32, from), (1.0, to)] {
            let at = path_at(REACHES, on);

            assert!(
                at.distance(end) < 0.01,
                "at {} the way there is {:?} and not {:?}",
                on,
                at,
                end
            );
        }

        // and the middle of it stands well off the line between them, which
        // is the whole of what Jake asked for: a tenth of a metre of wander
        // over fifty is a surveyed road
        let straight = (from + to) * 0.5;
        let bowed = path_at(REACHES, 0.5);
        let off = bowed.distance(straight);

        assert!(
            off > 3.0,
            "the middle of the way there is {:.2} off the straight line, \
             which reads as straight",
            off
        );

        // toward the village and not away from it, because `PATH_CLEAR`
        // keeps the view open by lying near the sightline
        let middle = at(REACHES);

        assert!(
            bowed.distance(middle) < straight.distance(middle),
            "the bow leans away from the village: {:.1} against {:.1}",
            bowed.distance(middle),
            straight.distance(middle)
        );

        // and it goes there without a corner in it or a step back
        let walk: Vec<Vec3> = (0..=PATH_STEPS)
            .map(|n| path_at(REACHES, n as f32 / PATH_STEPS as f32))
            .collect();

        for (n, three) in walk.windows(3).enumerate() {
            let (one, two) = (three[1] - three[0], three[2] - three[1]);
            let turn = one.normalize().dot(two.normalize()).clamp(-1.0, 1.0).acos();

            assert!(
                turn < 0.09,
                "the way there turns {:.1} degrees at step {}, which is a \
                 corner and not a bend",
                turn.to_degrees(),
                n
            );
            assert!(
                two.dot(to - from) > 0.0,
                "the way there doubles back at step {}",
                n
            );
        }
    }

    /// Spec 0126: and it is one ribbon rather than a row of boxes.
    #[test]
    fn the_ribbon_lies_face_up_with_no_notch_in_it() {
        let mesh = path_mesh(REACHES);

        assert_eq!(mesh.vertices.len(), (PATH_STEPS + 1) * 2);
        assert_eq!(mesh.indices.len(), PATH_STEPS * 6);

        // **Every triangle faces up.** Wound the other way they are culled
        // and the path is not drawn at all, which is the one fault here
        // that a picture would show as nothing rather than as something
        // wrong.
        for (n, triangle) in mesh.indices.as_chunks::<3>().0.iter().enumerate() {
            let corner = |at: u32| {
                let one = mesh.vertices[at as usize].position;

                vec3(one[0], one[1], one[2])
            };
            let (a, b, c) = (
                corner(triangle[0]),
                corner(triangle[1]),
                corner(triangle[2]),
            );
            let up = (b - a).cross(c - a);

            assert!(
                up.y > 0.0,
                "triangle {} faces {:?}, which is away from the sky",
                n,
                up.normalize_or_zero()
            );
        }

        // the two sides are the two sides, each about a half width off the
        // middle of the line, and neither of them a ruled edge
        let mut widths = Vec::new();
        let mut walked = 0.0f32;
        let mut last = path_at(REACHES, 0.0);

        for n in 0..=PATH_STEPS {
            let on = n as f32 / PATH_STEPS as f32;
            let middle = path_at(REACHES, on);
            let (left, right) = (&mesh.vertices[n * 2], &mesh.vertices[n * 2 + 1]);

            walked += middle.distance(last);
            last = middle;

            // v is the path edge to edge, which is what lets the grit know
            // where the edges of it are
            assert_eq!((left.uv[1], right.uv[1]), (0.0, 1.0));
            // and u is the metres walked, in tiles of them
            assert!(
                (left.uv[0] - walked / PATH_TILE).abs() < 0.01,
                "at step {} the grit has run {:.2} tiles and the walk {:.2}",
                n,
                left.uv[0],
                walked / PATH_TILE
            );

            let across = vec3(
                right.position[0] - left.position[0],
                0.0,
                right.position[2] - left.position[2],
            );

            widths.push(across.length());
        }

        let (narrow, wide) = widths
            .iter()
            .fold((f32::MAX, 0.0f32), |(a, b), one| (a.min(*one), b.max(*one)));

        assert!(
            narrow > PATH_WIDE * 0.7 && wide < PATH_WIDE * 1.4,
            "the way there runs from {:.2} to {:.2} wide",
            narrow,
            wide
        );
        assert!(
            wide - narrow > 0.1,
            "both edges of the way there are ruled lines: {:.2} to {:.2}",
            narrow,
            wide
        );
    }

    /// Spec 0126: the grit is a direction and not a colour, and it is a tile
    /// rather than a picture with a border.
    #[test]
    fn the_grit_tiles_without_a_seam_and_leans_out_of_the_earth() {
        let map = grit_bumps();
        let many = GRIT_TEXELS as usize;

        assert_eq!((map.width(), map.height()), (GRIT_TEXELS, GRIT_TEXELS));

        let texel = &map.levels[0].pixels;
        let blue = |x: usize, y: usize| texel[(y * many + x) * 4 + 2];
        let lean = |x: usize, y: usize| {
            let n = (y * many + x) * 4;

            (texel[n] as f32 - 128.0).hypot(texel[n + 1] as f32 - 128.0)
        };

        // nothing leans into the ground it is laid on
        for y in 0..many {
            for x in 0..many {
                assert!(
                    blue(x, y) >= 128,
                    "the grit at {} {} leans into the earth at {}",
                    x,
                    y,
                    blue(x, y)
                );
            }
        }

        // **And the earth between the chips lies flat**, which is also the
        // check that this was not written as a colour: put through sRGB a
        // flat texel is 188 rather than 128 and the whole tile tips a
        // quarter turn off the path
        let flattest = (0..many)
            .flat_map(|y| (0..many).map(move |x| (x, y)))
            .map(|(x, y)| lean(x, y))
            .fold(f32::MAX, f32::min);

        assert!(
            flattest < 8.0,
            "the flattest texel in the grit leans {:.0} out of 128, so none \
             of it is lying down",
            flattest
        );

        // and the chips themselves have sides on them
        let keenest = (0..many)
            .flat_map(|y| (0..many).map(move |x| (x, y)))
            .map(|(x, y)| lean(x, y))
            .fold(0.0f32, f32::max);

        assert!(
            keenest > 25.0,
            "the keenest slope in the grit is {:.0} out of 128, which is not \
             a stone",
            keenest
        );

        // **And the chips cross the border.** The tile is repeated down the
        // path, so a chip sown near one edge has to come in at the other
        // one. Dropped instead, each border loses the chips sown on the far
        // side of it and the way there wears a bare line across it every
        // 1.1 metres.
        //
        // Read off the two outermost columns, which hold 0.76 of the tile's
        // own mean as this is written and 0.53 with the wrap taken out. The
        // first of those is not 1.0 because twelve hundred chips thrown at
        // a tile clump, and a single column of it runs from 0.72 to 1.36 of
        // the mean wherever you read it: this separates the two cases and
        // is not a measure of anything on its own.
        //
        // The two borders the wear above does not touch, since it thins the
        // other two on purpose and one mean cannot tell the two reasons
        // apart.
        let (high, _) = grit_field();
        let mean = |of: Vec<f32>| of.iter().sum::<f32>() / of.len() as f32;
        let all = mean(high.clone());
        let border = mean(
            (0..many)
                .flat_map(|y| [0usize, 1, many - 2, many - 1].map(|x| high[y * many + x]))
                .collect(),
        );

        assert!(
            border > all * 0.65,
            "the grit stands {:.4} high at the tile's two side borders and \
             {:.4} across the whole of it, so its chips stop at the edge",
            border,
            all
        );
    }

    /// Spec 0126: and what the field keeps clear is the line that is
    /// walked.
    #[test]
    fn the_keep_out_follows_the_curve() {
        let (from, to) = path_runs(REACHES);
        let straight = (from + to) * 0.5;
        let bowed = path_at(REACHES, 0.5);

        assert!(
            along_the_path(REACHES, bowed),
            "the middle of the way there is not on the way there"
        );

        // **The far side of the straight line.** It is as far from the line
        // between the ends as the path itself is, and nearly seven metres
        // from the path: measured to the straight one, as this was until
        // spec 0126 bowed the line, a tree could stand here.
        let other = straight + (straight - bowed);

        assert!(
            !along_the_path(REACHES, other),
            "a tree may not grow at {:?}, which is {:.1} metres off the way \
             there and {:.1} off the line between its ends",
            other,
            other.distance(bowed),
            other.distance(straight)
        );
    }

    /// Spec 0127: and the field grows up to the edge of it.
    #[test]
    fn the_grass_comes_over_the_edge_of_the_way_there() {
        // the middle of the path is the path, and a step off it is not
        let middle = path_at(REACHES, 0.5);
        let (_, across) = path_heads(REACHES, 0.5);

        assert!(
            on_the_way(REACHES, middle),
            "the middle of the way there is not on it"
        );
        assert!(
            !on_the_way(REACHES, middle + across * (PATH_WIDE * 0.5 + 0.2)),
            "a hand's breadth past the edge is still counted as the path"
        );

        // **And the thing that was actually wrong.** Kept off the
        // sightline, ground cover stopped three metres short of the grit on
        // both sides and the edge had nothing to break it. So this counts
        // what stands near enough to do so, which is the number the change
        // was made for: a yes or no about the path itself would pass just
        // as well with the verge back.
        let cell = crate::field::CELL;
        let mut near = 0;
        let mut nearest: Option<f32> = None;

        for n in 0..=120 {
            let on = path_at(REACHES, n as f32 / 120.0);
            let (i, j) = ((on.x / cell).round() as i32, (on.z / cell).round() as i32);

            for about in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
                for one in crate::field::cover(REACHES, (i + about.0, j + about.1)) {
                    let Some(off) = beside_the_way(REACHES, one.at) else {
                        continue;
                    };

                    nearest = Some(nearest.map_or(off, |was: f32| was.min(off)));

                    if off < 0.3 {
                        near += 1;
                    }
                }
            }
        }

        assert!(
            near > 20,
            "only {} things the field sows stand within a third of a metre \
             of the way there, and the nearest of anything it sows is {}: \
             the grit ends at a line with a verge beside it",
            near,
            nearest.map_or(
                "nothing within four metres at all".to_string(),
                |off| format!("{off:.2} off its edge")
            )
        );
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
