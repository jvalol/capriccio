//! The clock in the back room, and the train that runs it. Spec 0130.
//!
//! **In the load room, because it is about the building rather than in it.**
//! Spec 0024's back room is where the arcade keeps its instruments: a bench,
//! a block of cubes, and gauges saying what a frame costs. Jake, on where
//! this belongs: the load room, since it also has to do with representing
//! the overall mechanics of the building. A clock whose wheels are in plain
//! sight is the same kind of thing as a gauge that says what a frame costs,
//! and it was a utility room of its own for an hour before that was said.

use blitzkit::collision::Aabb;
use glam::{vec2, vec3, Quat, Vec2, Vec3};

/// How big a tooth is: the pitch diameter of a wheel is this times its
/// count.
///
/// **One module across the whole train**, so the distance between two
/// arbors is half of this times the sum of their tooth counts and nothing
/// is placed by hand. Nine millimetres, which makes the centre wheel of
/// sixty four a little over half a metre: big enough to count the teeth on
/// from across the room.
pub const MODULE: f32 = 0.009;

/// How far a tooth stands over the pitch circle and how far the root falls
/// under it, and how thick a wheel is.
pub const TOOTH_OUT: f32 = MODULE * 0.9;
pub const TOOTH_IN: f32 = MODULE * 1.1;
pub const WHEEL_THICK: f32 = 0.016;

/// The chapter ring: how far across it is and how wide its band.
/// **As large as the room will take.** The ceiling is the baths' own at
/// 2.4 and the pendulum wants a metre under the movement, so a ring any
/// bigger than this is a ring with its bob in the floor. A deeper room
/// would carry a grander one, and that is a step down through the doorway
/// and its own spec.
pub const RING: f32 = 1.84;
pub const RING_BAND: f32 = 0.21;

/// How high the middle of the dial stands off the floor.
/// **Higher and larger than the first one**, because the back room is 2.8
/// where the utility room was the baths' own 2.4. The pendulum wants a metre
/// under the escape arbor and the ring wants to clear the floor by more than
/// the bob swings.
pub const DIAL_UP: f32 = 1.76;

/// One wheel of the train: what it has on its arbor, where that arbor
/// stands in the face of the clock, and how many turns an hour it makes.
#[derive(Clone)]
pub struct Wheel {
    /// The wheel's own teeth, which are what drives the next pinion.
    pub teeth: u32,
    /// The pinion on the same arbor, driven by the wheel before it. Nought
    /// for the first one, which is driven by nothing.
    pub pinion: u32,
    /// Where the arbor stands, from the middle of the dial.
    pub at: Vec2,
    /// Turns an hour, signed: a wheel drives its pinion the other way
    /// round.
    pub turns: f32,
    /// Whether this arbor moves in steps of a second rather than turning.
    ///
    /// **Everything from the escape wheel onward does.** That is what an
    /// escapement is: the pendulum lets one tooth by every two beats, and
    /// a pendulum that beats seconds therefore moves the escape wheel one
    /// tooth every two seconds and nothing at all in between. The train in
    /// front of it turns smoothly, being geared down from a wheel that
    /// does not.
    pub beats: bool,
}

/// How many turns an hour each hand makes, which is what the arbor it is on
/// makes.
pub const MINUTE_HAND: f32 = 1.0;
pub const HOUR_HAND: f32 = 1.0 / 12.0;
pub const SECOND_HAND: f32 = 60.0;

/// Where the going train's arbors stand, as the angle from the arbor before
/// them.
///
/// Down and to one side rather than straight down, so the escape wheel and
/// its pendulum stay inside the ring: straight down the train is longer
/// than the dial is wide.
pub const THIRD_WAY: f32 = 4.0;
pub const ESCAPE_WAY: f32 = 5.3;

/// The going train, from the arbor that carries the minute hand down to the
/// escape wheel that carries the seconds.
///
/// **The rates are worked out and not written down.** A wheel of `w` teeth
/// driving a pinion of `p` turns it `w / p` times as fast and the other way
/// round, so the only numbers here are tooth counts and the places they put
/// the arbors.
pub fn going() -> Vec<Wheel> {
    let mut out = vec![Wheel {
        teeth: 64,
        pinion: 0,
        at: Vec2::ZERO,
        turns: MINUTE_HAND,
        beats: false,
    }];

    for (teeth, pinion, way) in [(60u32, 8u32, THIRD_WAY), (30, 8, ESCAPE_WAY)] {
        let before = out.last().expect("a wheel to be driven by");
        let apart = MODULE * (before.teeth + pinion) as f32 * 0.5;
        let (sin, cos) = way.sin_cos();

        out.push(Wheel {
            teeth,
            pinion,
            at: before.at + vec2(cos, sin) * apart,
            turns: -before.turns * before.teeth as f32 / pinion as f32,
            beats: teeth == 30,
        });
    }

    out
}

/// How many teeth the seconds train's wheels have.
///
/// **All four the same, because the hand has to turn at the escape wheel's
/// own rate and the right way round.** Two meshes of equal counts is one
/// to one twice over and two reversals, which is where it started. What
/// the count is for is reach: each mesh spans half the module times the
/// sum of its pair, so a pair of `n` spans `MODULE * n`, and two of those
/// have to bridge the gap from the escape arbor to the middle of the dial.
pub const SECONDS_TEETH: u32 = 32;

/// The seconds train: the escape arbor's turn carried to the middle of the
/// dial.
///
/// **Because a seconds hand on the escape arbor is a wristwatch's.** Jake,
/// on the first one: it looks like it sweeps, but because it's so
/// diminutive, almost like what you'd see on a wristwatch not a wall clock,
/// it's close to meaningless. It was a quarter of the dial's radius long,
/// stuck out at an arbor half a metre off centre, in a thicket of brass.
/// A wall clock's seconds hand is the longest thing on the dial and turns
/// about the same point the others do.
///
/// **Which is a real thing and not a cheat.** Indirect centre seconds is
/// how a clock whose escape wheel is not in the middle puts seconds in the
/// middle anyway: a wheel on the escape arbor, an idler, and a pinion on a
/// tube through the centre. Nothing here is told how fast to turn; the
/// hand goes round once a minute because the teeth say so.
///
/// The idler stands where both meshes close, which is one of the two points
/// at the right distance from each end. It takes the one away from the
/// third wheel, so the triangle opens into the clear half of the dial
/// rather than folding back over the going train.
pub fn seconds() -> Vec<Wheel> {
    let escape = going().last().expect("an escape wheel").at;
    let apart = MODULE * SECONDS_TEETH as f32;
    let half = escape.length() * 0.5;
    let out = (apart * apart - half * half).max(0.0).sqrt();
    let along = escape.normalize_or_zero();
    let idler = escape * 0.5 + vec2(-along.y, along.x) * out;

    vec![
        Wheel {
            teeth: SECONDS_TEETH,
            pinion: 0,
            at: escape,
            turns: SECOND_HAND,
            beats: true,
        },
        Wheel {
            teeth: SECONDS_TEETH,
            pinion: SECONDS_TEETH,
            at: idler,
            turns: -SECOND_HAND,
            beats: true,
        },
        Wheel {
            teeth: 0,
            pinion: SECONDS_TEETH,
            at: Vec2::ZERO,
            turns: SECOND_HAND,
            beats: true,
        },
    ]
}

/// The motion work, which is the twelve to one that the hour hand wants.
///
/// **Two meshes of the same span.** The cannon pinion rides the minute
/// hand's own arbor and the hour wheel is concentric with it, so the minute
/// wheel between them is the same distance from the centre in both meshes:
/// the two pairs have to sum to the same number of teeth. Ten into thirty
/// and eight into thirty two is three times four, and both sums are forty.
pub fn motion() -> Vec<Wheel> {
    let apart = MODULE * (CANNON + MINUTE_WHEEL) as f32 * 0.5;

    vec![
        Wheel {
            teeth: CANNON,
            pinion: 0,
            at: Vec2::ZERO,
            turns: MINUTE_HAND,
            beats: false,
        },
        Wheel {
            teeth: MINUTE_PINION,
            pinion: MINUTE_WHEEL,
            at: vec2(0.0, -apart),
            turns: -MINUTE_HAND * CANNON as f32 / MINUTE_WHEEL as f32,
            beats: false,
        },
        Wheel {
            teeth: 0,
            pinion: HOUR_WHEEL,
            at: Vec2::ZERO,
            turns: MINUTE_HAND * CANNON as f32 / MINUTE_WHEEL as f32 * MINUTE_PINION as f32
                / HOUR_WHEEL as f32,
            beats: false,
        },
    ]
}

pub const CANNON: u32 = 10;
pub const MINUTE_WHEEL: u32 = 30;
pub const MINUTE_PINION: u32 = 8;
pub const HOUR_WHEEL: u32 = 32;

/// How long a pendulum that beats seconds is.
///
/// **A fact about this planet and not a number that fits the room.** The
/// half period of a pendulum is pi root l over g, so a beat of one second
/// wants g over pi squared, which is very nearly a metre. Everything else
/// about this clock follows from the beat: one tooth of the escape wheel
/// every two of them, thirty teeth, a minute.
pub const GRAVITY: f32 = 9.80665;

pub fn pendulum_long() -> f32 {
    GRAVITY / (std::f32::consts::PI * std::f32::consts::PI)
}

/// How far the pendulum swings each way, in radians.
pub const SWING: f32 = 0.08;

/// Where the pendulum hangs from, which is the escape wheel's own arbor.
pub fn pendulum_at() -> Vec2 {
    going().last().map(|wheel| wheel.at).unwrap_or(Vec2::ZERO)
}

/// What time it is on the machine this is running on, as hours past
/// midnight.
///
/// **The player's own clock and not the building's.** The arcade's sky runs
/// a day in `garden::DAY`, which is twenty seconds, and this room ignores
/// it: the one thing a clock on a wall is for is agreeing with the watch on
/// your wrist. There is no readable alternative at that length anyway. A
/// minute hand on the building's own time would go round three times a
/// second and the escapement, whose whole point is one tick a second, would
/// beat a hundred and eighty times in one.
///
/// Read every frame rather than once, so a machine whose clock is set while
/// you are standing there follows it, and so does one that crosses a time
/// zone.
pub fn now() -> f32 {
    use chrono::Timelike;

    let now = chrono::Local::now();

    now.hour() as f32
        + now.minute() as f32 / 60.0
        + (now.second() as f32 + now.nanosecond() as f32 * 1e-9) / 3600.0
}

/// The hour a picture of this room is taken at.
///
/// **Ten past ten, which is where the hands sit in every photograph of a
/// clock ever taken**, and where the nook's own mantel clock is stuck. A
/// picture of a room whose whole point is the time has to be taken at one,
/// or the reference is a different picture every minute.
#[cfg(test)]
pub const SHOWN: f32 = 10.0 + 10.0 / 60.0;

/// How far round a wheel has turned at `hours` past midnight.
///
/// Clockwise seen from the room, which is the way a clock turns and the way
/// the hands on these arbors have to go.
pub fn turned(turns_an_hour: f32, hours: f32) -> f32 {
    -turns_an_hour * hours * std::f32::consts::TAU
}

/// The time an arbor sees: the hour itself, or the last whole second of
/// it for anything the escapement lets go of a tooth at a time.
///
/// **A seconds hand that creeps is a quartz clock's.** This one is on a
/// pendulum beating seconds through a thirty tooth escape wheel, which
/// physically cannot move except in steps: one tooth every two beats. The
/// step is also most of what makes the hand read from across the room,
/// a sixth of a degree a second being nothing to see and six degrees at
/// once being a jump.
pub fn ticked(hours: f32) -> f32 {
    (hours * 3600.0).floor() / 3600.0
}

pub fn when(beats: bool, hours: f32) -> f32 {
    if beats {
        ticked(hours)
    } else {
        hours
    }
}

/// Where the pendulum stands at `hours` past midnight, in radians off
/// upright.
pub fn swung(hours: f32) -> f32 {
    let seconds = hours * 3600.0;

    SWING * (seconds * std::f32::consts::PI).sin()
}

/// Which way a thing at `turn` lies in the face of the clock: across the
/// dial and up it, as a unit vector.
///
/// **One direction, read twice.** A hand is a bar as long as its own reach,
/// slid half that along the way it points, so where it sits and which way
/// it lies are the same number. Written out twice they came apart: the
/// offset ran one way round the dial and the rotation the other, which left
/// every hand a chord with neither end on the arbor it is meant to turn on.
/// Everything that turns on the face asks this, and `turned_to` is the same
/// answer as a rotation.
pub fn along(turn: f32) -> Vec2 {
    let (sin, cos) = turn.sin_cos();

    vec2(sin, cos)
}

/// And the turn that lies a given way, which is the same answer read
/// backwards: `along(laid_along(d))` is `d` with its length taken off.
///
/// Wanted because the ornament is written as lines rather than as angles. A
/// stroke of a numeral is two corners, and what it needs to be drawn is the
/// turn that lays a bar between them.
pub fn laid_along(d: Vec2) -> f32 {
    d.x.atan2(d.y)
}

/// Where a point in a numeral's own frame falls in the face of the clock,
/// and which way a line in that frame runs.
///
/// **Upright means out from the middle, and left to right means the room's
/// left to right.** A chapter ring's numerals each stand on their own
/// radius, so a numeral at four o'clock is lying on its side as far as the
/// dial is concerned. Its own x is the way somebody in the room reads,
/// which is the dial's own x backwards, east being on their left.
pub fn aimed(turn: f32, d: Vec2) -> Vec2 {
    let up = along(turn);

    vec2(-up.y, up.x) * d.x + up * d.y
}

pub fn placed(turn: f32, at: Vec2, p: Vec2) -> Vec2 {
    at + aimed(turn, p)
}

/// The same turn as a rotation, to be applied after `facing`.
pub fn turned_to(turn: f32) -> Quat {
    Quat::from_rotation_x(turn)
}

/// The quarter turn that takes the clock's own face to the wall it hangs
/// on, which is the only thing in it that knows which way the room points.
pub fn facing() -> Quat {
    Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)
}

/// Where a thing in the face of the clock stands in the room: across the
/// dial, up it, and out of the wall toward the floor you stand on.
pub fn stood(face: Vec3, at: Vec2, out: f32) -> Vec3 {
    face + facing() * vec3(out, at.y, at.x)
}

/// The board the clock hangs on, and the one the readout does: where each
/// stands and how big it is.
///
/// **Black, because the white only reads against it.** Jake: the clock
/// might look better with a black background on the wall. It is the same
/// argument the bay at the other end of this room already settled, per spec
/// 0025: the block of cubes is a plain 0.88 white and what makes it burn is
/// a near-black lining behind it. The ring borrows the cubes' colour and
/// this is the other half of why they look the way they do.
///
/// **As tall as the room takes, because what it backs is not just a ring.**
/// A board with an even margin round the dial would be a board the pendulum
/// hangs off the bottom of. This one carries the bob as well, and its top
/// margin is thinner than its sides because the clock is already as large
/// as the ceiling allows.
pub const BOARD_EDGE: f32 = 0.26;
pub const BOARD_OFF: f32 = 0.03;

pub fn board(reaches: f32) -> (Vec3, Vec3) {
    let face = dial_at(reaches);
    let floor = crate::back::floor(reaches).max.y;
    let under = floor + BOARD_EDGE * 0.46;
    let over = floor + crate::back::TALL - BOARD_EDGE * 0.23;

    (
        vec3(
            face.x,
            (under + over) * 0.5,
            face.z + OFF_THE_WALL - BOARD_OFF * 0.5,
        ),
        vec3(RING + BOARD_EDGE * 2.0, over - under, BOARD_OFF),
    )
}

/// The room the pendulum needs: everything the rod and the bob pass
/// through at the ends of their swing, with a hand's breadth round it.
///
/// **Because the floor under a clock is not the clock's.** The back room's
/// clutter was placed by hand along this wall long before anything hung on
/// it, and the bob came down into a bucket. A rule that scatters things
/// puts one in the doorway; a list placed by hand puts one wherever the
/// next thing to arrive wants to be. So the clock says what floor it needs
/// and a test asks the room.
///
/// Nothing drawn asks this: the pendulum is drawn from where it hangs and
/// the clutter from where it stands, and what this is for is the test that
/// those two never meet.
#[allow(dead_code)]
pub const SWINGS_CLEAR: f32 = 0.1;

#[allow(dead_code)]
pub fn swings_over(reaches: f32) -> Aabb {
    let face = dial_at(reaches);
    let hangs = stood(face, pendulum_at(), -PLATES * 0.5);
    let long = rod_long();
    let out = SWING.sin() * long + BOB * 0.5 + SWINGS_CLEAR;

    Aabb::new(
        vec3(
            hangs.x - out,
            hangs.y - long - BOB * 0.5 - SWINGS_CLEAR,
            hangs.z - BOB_THICK * 0.5 - SWINGS_CLEAR,
        ),
        vec3(
            hangs.x + out,
            hangs.y,
            hangs.z + BOB_THICK * 0.5 + SWINGS_CLEAR,
        ),
    )
}

/// How many spokes a wheel is crossed out with, how far the hub runs and
/// where the rim starts, as shares of the wheel's own radius.
///
/// **Because a wheel that is not crossed out is a disc.** A clock wheel has
/// five or six crossings cut in it, and what you see through them is the
/// wheel behind: solid, the train came out as a stack of brass plates with
/// a fringe, which is not a movement.
pub const CROSSINGS: u32 = 5;
pub const HUB: f32 = 0.17;
pub const RIM_IN: f32 = 0.78;
pub const SPOKE: f32 = 0.09;

/// How far in the face of a wheel of `teeth` reaches at a share of the way
/// round it: the hub and the spokes, or the inside of the rim between them.
pub fn face_in(teeth: u32, on: f32) -> f32 {
    let out = MODULE * teeth as f32 * 0.5;
    let along = (on * CROSSINGS as f32).fract();
    let spoke = along < SPOKE * 0.5 || along > 1.0 - SPOKE * 0.5;

    if spoke || teeth <= 12 {
        out * HUB
    } else {
        out * RIM_IN
    }
}

/// How far out a wheel of `teeth` stands at a share of the way round it.
///
/// A trapezoid: flat across the tip, flat across the root, and a flank
/// between. Nothing here is an involute, which is a curve that matters to a
/// gear that carries a load and not to one you are looking at.
pub fn tooth_out(teeth: u32, on: f32) -> f32 {
    let pitch = MODULE * teeth as f32 * 0.5;
    let along = (on * teeth as f32).fract();
    // a tooth and a gap are half the pitch each, with a flank of a fifth
    // either side of the tooth
    let out = if along < 0.1 {
        along / 0.1
    } else if along < 0.4 {
        1.0
    } else if along < 0.5 {
        (0.5 - along) / 0.1
    } else {
        0.0
    };

    pitch - TOOTH_IN + (TOOTH_IN + TOOTH_OUT) * out
}

/// A wheel, as a mesh: a disc with its teeth cut, lying with its arbor
/// along x so that turning it is a turn about x.
pub fn wheel_mesh(teeth: u32) -> blitzkit::mesh::MeshData {
    use std::f32::consts::TAU;

    let half = WHEEL_THICK * 0.5;
    let mut mesh = blitzkit::mesh::MeshData::surface(teeth * 8, 8, |round, across| {
        let turn = -round * TAU;
        let out = tooth_out(teeth, round);
        let inner = face_in(teeth, round);
        // the face out from the hub or the inside of the rim, then the rim
        // itself, then the face again
        let (along, far) = if across < 0.25 {
            (half, inner + (out - inner) * (across / 0.25))
        } else if across < 0.75 {
            (half - WHEEL_THICK * (across - 0.25) / 0.5, out)
        } else {
            (-half, inner + (out - inner) * ((1.0 - across) / 0.25))
        };

        vec3(along, turn.cos() * far, turn.sin() * far)
    });
    mesh.compute_normals();

    mesh
}

/// Every tooth count the clock has a wheel of, each once.
///
/// **One mesh a count and not one a wheel.** Two wheels of sixty four teeth
/// are the same wheel, and a pinion of eight is a pinion of eight wherever
/// it rides.
pub fn counts() -> Vec<u32> {
    let mut out: Vec<u32> = going()
        .iter()
        .chain(motion().iter())
        .chain(seconds().iter())
        .flat_map(|wheel| [wheel.teeth, wheel.pinion])
        .filter(|teeth| *teeth > 0)
        .collect();

    out.sort_unstable();
    out.dedup();

    out
}

/// The chapter ring, as a moulding rather than a flat band.
///
/// **Because a flat band is a washer.** The ring is the one part of this
/// anybody looks straight at, and a section with a step, a bead and a
/// hollow in it catches the light in three places instead of one. The
/// numbers are shares of the band's own width, so widening the ring widens
/// the moulding with it.
/// How thick the ring is, and how far forward of its back the flat part of
/// it stands, as a share of that.
///
/// **The marks are laid on this and not at a depth of their own.** Written
/// separately, the minute track and all twelve numerals sat nine
/// millimetres inside a moulding that rose over them, and what a chapter
/// ring carrying its own numbers came out as was a ring with a few extra
/// ticks on it.
pub const BEZEL_THICK: f32 = 0.045;
pub const BEZEL_FACE: f32 = 0.55;

pub fn chapter_face() -> f32 {
    BEZEL_THICK * BEZEL_FACE
}

const BEZEL: [(f32, f32); 9] = [
    // how far out along the band, and how far forward of the back face
    (0.00, 0.00),
    (0.00, 0.42),
    (0.05, 0.68),
    (0.10, BEZEL_FACE),
    (0.86, BEZEL_FACE),
    (0.90, 0.94),
    (0.96, 0.62),
    (1.00, 0.78),
    (1.00, 0.00),
];

/// Where the flat runs, as shares of the band: nothing may be laid on the
/// moulding at either end of it.
pub const CHAPTER_IN: f32 = 0.10;
pub const CHAPTER_OUT: f32 = 0.86;

/// Where a table of (out, up) pairs has got to a share of the way along
/// itself, which is how every moulding and outline in here is written.
fn stepped(table: &[(f32, f32)], on: f32) -> (f32, f32) {
    let step = on * (table.len() - 1) as f32;
    let n = (step.floor() as usize).min(table.len() - 2);
    let at = step - n as f32;
    let (out_a, up_a) = table[n];
    let (out_b, up_b) = table[n + 1];

    (out_a + (out_b - out_a) * at, up_a + (up_b - up_a) * at)
}

pub fn bezel_mesh(inner: f32, outer: f32, thick: f32) -> blitzkit::mesh::MeshData {
    use std::f32::consts::TAU;

    let band = outer - inner;
    let mut mesh =
        blitzkit::mesh::MeshData::surface(128, (BEZEL.len() * 3) as u32, |round, across| {
            let turn = -round * TAU;
            let (out, up) = stepped(&BEZEL, across);

            vec3(
                up * thick,
                turn.cos() * (inner + band * out),
                turn.sin() * (inner + band * out),
            )
        });
    mesh.compute_normals();

    mesh
}

/// A hand's outline: half its width at points along its own length, both
/// as shares of its reach.
///
/// **One shape, three hands.** What differs between an hour hand and a
/// minute hand on a clock like this is how far each reaches and how
/// heavily each is cut, which are two numbers, and not what the shape of a
/// hand is. It runs from behind the arbor, because a hand has a tail: the
/// counterweight is most of what stops a pair of hands reading as two
/// sticks laid on a plate.
const OUTLINE: [(f32, f32); 11] = [
    (-0.20, 0.000),
    (-0.17, 0.024),
    (-0.13, 0.032),
    (-0.08, 0.017),
    (0.00, 0.030),
    (0.05, 0.013),
    (0.56, 0.011),
    (0.64, 0.036),
    (0.72, 0.029),
    (0.90, 0.008),
    (1.00, 0.000),
];

/// A hand, as a mesh: that outline, a hair thick, lying along its own up
/// with its arbor at nothing and its point one away.
///
/// **It is placed at the arbor and not slid half its length along it**,
/// which is how the first pair of these were drawn and how each came apart
/// from its own rotation. A mesh that already knows where its arbor is has
/// nothing left to keep in step.
pub fn hand_mesh(cut: f32) -> blitzkit::mesh::MeshData {
    use std::f32::consts::TAU;

    let from = OUTLINE[0].0;
    let to = OUTLINE[OUTLINE.len() - 1].0;
    let mut mesh = blitzkit::mesh::MeshData::surface(96, 16, |along, round| {
        let up = from + (to - from) * along;
        let mut wide = 0.0;

        for pair in OUTLINE.windows(2) {
            let ((a, wa), (b, wb)) = (pair[0], pair[1]);

            if up >= a && up <= b {
                wide = wa + (wb - wa) * ((up - a) / (b - a));
            }
        }

        let turn = -round * TAU;

        vec3(turn.sin() * 0.5, up, turn.cos() * wide * cut)
    });
    mesh.compute_normals();

    mesh
}

/// The two pillars the movement hangs between, turned rather than sawn.
///
/// Foot, torus, shaft, swell, cap and a finial on top, which is what holds
/// a skeleton clock's plates apart and is the one part of it that is there
/// to be looked at rather than to do something.
const PILLAR: [(f32, f32); 15] = [
    (0.00, 0.00),
    (0.01, 0.88),
    (0.05, 0.88),
    (0.07, 0.60),
    (0.11, 0.78),
    (0.15, 0.48),
    (0.45, 0.40),
    (0.51, 0.68),
    (0.57, 0.42),
    (0.85, 0.38),
    (0.88, 0.70),
    (0.91, 0.46),
    (0.95, 0.78),
    (0.99, 0.34),
    (1.00, 0.00),
];

pub fn pillar_mesh() -> blitzkit::mesh::MeshData {
    crate::cellar::turned(20, (PILLAR.len() * 3) as u32, |v| {
        let (up, wide) = stepped(&PILLAR, v);

        (up - 0.5, wide)
    })
}

/// The bob, as a lens rather than a disc.
///
/// A pendulum bob is a lens because it has to be heavy and thin, and the
/// dome is the whole of why one reads as a weight hanging rather than as a
/// coin stuck to the wall.
pub fn lens_mesh(across: f32, thick: f32) -> blitzkit::mesh::MeshData {
    use std::f32::consts::TAU;

    let mut mesh = blitzkit::mesh::MeshData::surface(48, 24, |round, section| {
        let turn = -round * TAU;
        // out to the rim on the front half of it and back in on the other
        let (face, share) = if section < 0.5 {
            (1.0, section * 2.0)
        } else {
            (-1.0, (1.0 - section) * 2.0)
        };
        let far = across * 0.5 * share;
        let dome = thick * 0.5 * (1.0 - share * share).max(0.0).sqrt();

        vec3(dome * face, turn.cos() * far, turn.sin() * far)
    });
    mesh.compute_normals();

    mesh
}

/// Where the twelve hour marks stand on the ring: each one is where the
/// hour hand points at that hour, which is what an hour mark is.
///
/// **Written as `n` twelfths of a turn it was the ring backwards**, and
/// nothing said so for as long as the marks were twelve identical bars: a
/// dial mirrored about noon has a bar at every place a dial has a bar.
/// The numerals are what made it visible, I standing where XI belongs.
pub fn hours() -> Vec<f32> {
    (0..12).map(|n| turned(HOUR_HAND, n as f32)).collect()
}

/// The minute track: where every one of the sixty stands and how long its
/// mark is.
///
/// **One list, not two.** The hours were twelve bars of their own and the
/// minutes would have been sixty more, which is the same ring measured
/// twice: a mark at the hour is a mark on this track that is longer, and
/// nothing can put the two out of step because there is only one of them.
pub const TICK: f32 = 0.022;
pub const TICK_HOUR: f32 = 0.036;
pub const TICK_WIDE: f32 = 0.006;
pub const TICK_HOUR_WIDE: f32 = 0.013;
pub const TRACK_IN: f32 = 0.004;

pub fn track() -> Vec<(f32, f32, f32)> {
    (0..60)
        .map(|n| {
            let turn = turned(MINUTE_HAND, n as f32 / 60.0);

            if n % 5 == 0 {
                (turn, TICK_HOUR, TICK_HOUR_WIDE)
            } else {
                (turn, TICK, TICK_WIDE)
            }
        })
        .collect()
}

/// How far out the track's marks hang from, and where a numeral's middle
/// stands.
///
/// Both off the ring's own band, so widening the ring moves them together
/// and a numeral cannot end up on the track or off the inside edge.
pub fn track_at() -> f32 {
    RING * 0.5 - RING_BAND * (1.0 - CHAPTER_OUT) - TRACK_IN
}

pub fn numeral_at() -> f32 {
    (chapter_in() + track_at() - TICK_HOUR) * 0.5
}

/// Where the flat part of the ring starts, which is as far in as anything
/// written on it may go.
pub fn chapter_in() -> f32 {
    RING * 0.5 - RING_BAND * (1.0 - CHAPTER_IN)
}

/// How tall a numeral stands, which is the room left between the inside of
/// the band and the ends of the hour marks.
///
/// **Not a number of its own.** Written as one it was a third of the room
/// it had, and twelve numerals too small to read are twelve smudges: the
/// one thing a chapter ring is for is being read from across a room.
pub fn numeral_tall() -> f32 {
    (track_at() - TICK_HOUR - chapter_in()) * 0.84
}

/// How thick its strokes are cut, and the gap between two letters, both as
/// shares of that.
pub const NUMERAL_THICK: f32 = 0.13;
pub const NUMERAL_GAP: f32 = 0.1;

/// A letter of a Roman numeral, as the lines it is drawn with and how much
/// room it takes, in a box one tall about its own middle.
///
/// Three letters is the whole alphabet a clock needs, and every one of them
/// is straight lines. Four is `IIII` and not `IX`, which is what a clock
/// dial does and has done since clocks had dials: the four and the eight
/// stand opposite each other and `IIII` balances `VIII`.
fn letter(of: char) -> (f32, Vec<(Vec2, Vec2)>) {
    let half = 0.5;

    match of {
        'V' => (
            0.62,
            vec![
                (vec2(-0.31, half), vec2(0.0, -half)),
                (vec2(0.0, -half), vec2(0.31, half)),
            ],
        ),
        'X' => (
            0.62,
            vec![
                (vec2(-0.31, -half), vec2(0.31, half)),
                (vec2(-0.31, half), vec2(0.31, -half)),
            ],
        ),
        _ => (0.16, vec![(vec2(0.0, -half), vec2(0.0, half))]),
    }
}

/// An hour's numeral, as lines in its own frame: one tall, centred on
/// nothing, and as wide as its letters make it.
pub fn numeral(hour: usize) -> Vec<(Vec2, Vec2)> {
    const SAID: [&str; 12] = [
        "XII", "I", "II", "III", "IIII", "V", "VI", "VII", "VIII", "IX", "X", "XI",
    ];

    let said = SAID[hour % 12];
    let letters: Vec<(f32, Vec<(Vec2, Vec2)>)> = said.chars().map(letter).collect();
    let wide: f32 =
        letters.iter().map(|(w, _)| w).sum::<f32>() + NUMERAL_GAP * (letters.len() - 1) as f32;
    let mut at = -wide * 0.5;
    let mut out = Vec::new();

    for (w, lines) in letters {
        let middle = vec2(at + w * 0.5, 0.0);

        out.extend(lines.into_iter().map(|(a, b)| (a + middle, b + middle)));
        at += w + NUMERAL_GAP;
    }

    out
}

/// The digital readout: the same clock said the other way.
///
/// **Because a skeleton clock shows you how and not what.** Jake: with a
/// digital version somewhere. The movement in the ring is an argument about
/// how a machine divides a day, and reading it is work: you find the hands
/// among the wheels and you take the minute off a track. The readout takes
/// no finding at all, and the two of them standing on one wall saying the
/// same second is the whole point of putting them there together.
///
/// **And it is a twenty-four hour clock**, which is the one thing the dial
/// cannot say. A pair of hands at ten past ten does not tell you which ten
/// past ten it is, and this room is full of instruments that would rather
/// be exact than pretty.
///
/// Which segments each digit lights, as the usual seven: a across the top,
/// b and c down the right, d across the bottom, e and f down the left, and
/// g across the middle.
const LIT: [u8; 10] = [
    0b0111111, 0b0000110, 0b1011011, 0b1001111, 0b1100110, 0b1101101, 0b1111101, 0b0000111,
    0b1111111, 0b1101111,
];

/// Where each of those seven runs, in a box one across and two tall.
const SEGMENTS: [(Vec2, Vec2); 7] = [
    (Vec2::new(-0.5, 1.0), Vec2::new(0.5, 1.0)),
    (Vec2::new(0.5, 1.0), Vec2::new(0.5, 0.0)),
    (Vec2::new(0.5, 0.0), Vec2::new(0.5, -1.0)),
    (Vec2::new(-0.5, -1.0), Vec2::new(0.5, -1.0)),
    (Vec2::new(-0.5, 0.0), Vec2::new(-0.5, -1.0)),
    (Vec2::new(-0.5, 1.0), Vec2::new(-0.5, 0.0)),
    (Vec2::new(-0.5, 0.0), Vec2::new(0.5, 0.0)),
];

/// How wide a digit is, a colon, and the gap between two of them, in the
/// same units the box above is written in.
pub const DIGIT_WIDE: f32 = 1.0;
pub const COLON_WIDE: f32 = 0.25;
pub const READOUT_GAP: f32 = 0.3;
pub const SEGMENT_THICK: f32 = 0.15;

/// What the readout says at `hours` past midnight: hours, minutes and
/// seconds, each as its two digits.
pub fn said(hours: f32) -> [u32; 6] {
    let seconds = (hours * 3600.0).rem_euclid(24.0 * 3600.0) as u32;
    let (h, m, s) = (seconds / 3600, seconds / 60 % 60, seconds % 60);

    [h / 10, h % 10, m / 10, m % 10, s / 10, s % 10]
}

/// How wide the whole readout is in its own units, which is what the
/// drawing divides by to fit it to a wall.
pub fn readout_across() -> f32 {
    DIGIT_WIDE * 6.0 + COLON_WIDE * 2.0 + READOUT_GAP * 7.0
}

/// Every piece of it at `hours`: a line, and whether it is alight.
///
/// **The dark ones are drawn too.** Seven segments with four of them
/// missing is four bars floating on a board; it is a figure because the
/// other three are there and unlit, which is what a display looks like
/// switched on.
pub fn readout(hours: f32) -> Vec<(Vec2, Vec2, bool)> {
    let digits = said(hours);
    let mut at = -readout_across() * 0.5;
    let mut out = Vec::new();
    let mut shown = 0;

    for piece in 0..8 {
        at += READOUT_GAP;

        // two digits, a colon, two digits, a colon, two digits
        if piece == 2 || piece == 5 {
            let middle = at + COLON_WIDE * 0.5;

            for up in [0.45f32, -0.45] {
                out.push((vec2(middle, up - 0.05), vec2(middle, up + 0.05), true));
            }

            at += COLON_WIDE;
            continue;
        }

        let middle = at + DIGIT_WIDE * 0.5;
        let lit = LIT[digits[shown] as usize];

        for (n, (from, to)) in SEGMENTS.iter().enumerate() {
            out.push((
                *from + vec2(middle, 0.0),
                *to + vec2(middle, 0.0),
                lit & (1 << n) != 0,
            ));
        }

        at += DIGIT_WIDE;
        shown += 1;
    }

    out
}

/// Where the readout hangs and how wide it is.
///
/// **Beside the clock and not under it**, the pendulum having the floor
/// under the dial, and in the strip of wall east of the board, over the
/// end of the bench: the room's other readouts are on the bench wall and a
/// figure you check at a glance belongs where you are working.
///
/// **The west strip is where the leaning things are.** The stepladder and
/// the broom are both in that corner, and the ladder crosses it at exactly
/// the height a readout wants, which it did: the first one was written
/// there and the ladder lay across the digits. Same lesson as the bucket
/// under the pendulum, found the same way.
///
/// **Both taken off that strip rather than typed.** Written as numbers of
/// their own the plate was wider than the wall it had left and ran into
/// the clock's board, where two faces in one plane fight. It is centred in
/// what is there and as wide as a share of it, so it cannot be.
pub const READOUT_UP: f32 = 1.52;
pub const READOUT_EDGE: f32 = 0.09;
pub const READOUT_SHARE: f32 = 0.74;
#[allow(dead_code)]
pub const READOUT_CLEAR: f32 = 0.3;

pub fn beside_the_board(reaches: f32) -> (f32, f32) {
    let (_, east) = crate::back::span();
    let edge = dial_at(reaches).x + board(reaches).1.x * 0.5;

    ((east + edge) * 0.5, east - edge)
}

/// The wall the readout is on and the air in front of it, which nothing
/// may stand in.
#[allow(dead_code)]
pub fn readout_clear(reaches: f32) -> Aabb {
    let (middle, size) = readout_board(reaches);

    Aabb::new(
        vec3(
            middle.x - size.x * 0.5,
            middle.y - size.y * 0.5,
            middle.z - size.z * 0.5 - READOUT_CLEAR,
        ),
        middle + size * 0.5,
    )
}

pub fn readout_wide(reaches: f32) -> f32 {
    beside_the_board(reaches).1 * READOUT_SHARE
}

pub fn readout_at(reaches: f32) -> Vec3 {
    let face = dial_at(reaches);
    let floor = crate::back::floor(reaches).max.y;

    vec3(
        beside_the_board(reaches).0,
        floor + READOUT_UP,
        face.z + OFF_THE_WALL - BOARD_OFF,
    )
}

/// And the plate it is on, which stands against the wall with the segments
/// a few millimetres over its face.
pub fn readout_board(reaches: f32) -> (Vec3, Vec3) {
    let at = readout_at(reaches);
    let unit = readout_wide(reaches) / readout_across();

    (
        vec3(at.x, at.y, at.z + BOARD_OFF * 0.5),
        vec3(
            readout_wide(reaches) + READOUT_EDGE * 2.0,
            unit * 2.0 + READOUT_EDGE * 2.0,
            BOARD_OFF,
        ),
    )
}

/// How long each hand is, as a share of the ring's own radius, and how
/// thick a hand is cut.
pub const HOUR_LONG: f32 = 0.56;
pub const MINUTE_LONG: f32 = 0.84;
/// **The longest of the three**, which is what a seconds hand on a wall
/// clock is: it runs out past the minute hand to the track itself, where
/// the minute hand stops a little short of it.
pub const SECOND_LONG: f32 = 0.9;
pub const HAND_THICK: f32 = 0.014;

/// One hand: the arbor it is on, how fast that turns, how far it reaches
/// and how heavily it is cut.
pub struct Hand {
    pub turns: f32,
    pub at: Vec2,
    pub long: f32,
    pub cut: f32,
    /// Whether this is the one the escapement moves, which is the one that
    /// steps rather than turning, the one that has to be seen first, and
    /// therefore the one that is red.
    pub beats: bool,
}

/// The three of them, each on the arbor that turns at its own rate: the
/// hour on the motion work's last wheel, the minute on the centre, the
/// seconds out on the escape.
pub fn hands() -> Vec<Hand> {
    vec![
        Hand {
            turns: HOUR_HAND,
            at: Vec2::ZERO,
            long: HOUR_LONG,
            cut: 1.4,
            beats: false,
        },
        Hand {
            turns: MINUTE_HAND,
            at: Vec2::ZERO,
            long: MINUTE_LONG,
            cut: 1.0,
            beats: false,
        },
        Hand {
            turns: SECOND_HAND,
            at: Vec2::ZERO,
            long: SECOND_LONG,
            cut: 0.5,
            beats: true,
        },
    ]
}

/// How long the pendulum's rod is from the arbor it hangs on, and how big
/// its bob.
pub fn rod_long() -> f32 {
    pendulum_long()
}

pub const BOB: f32 = 0.17;
pub const BOB_THICK: f32 = 0.035;

/// How far the movement stands off the wall, and how far apart the bars
/// that hold it are.
/// **Deeper than it was**, because the seconds train runs behind the going
/// train rather than beside it: two layers of wheels want two plates' worth
/// of air between the dial and the board.
pub const OFF_THE_WALL: f32 = 0.28;
pub const PLATES: f32 = 0.1;

/// Where the clock's middle stands: on the back room's north wall, which is
/// the one clear wall in there.
///
/// **The wall you turn to, not the one you face.** The door is in the west
/// side, the bench is down the east and the bay with the rig in it takes the
/// whole south end: what is left is this, and it is five metres of nothing
/// at the one end of the room you can stand well back from.
pub fn dial_at(reaches: f32) -> Vec3 {
    let (_, east) = crate::back::span();
    let floor = crate::back::floor(reaches).max.y;

    vec3(
        east - crate::back::WIDE * 0.5,
        floor + DIAL_UP,
        crate::back::near_wall(reaches).min.z - OFF_THE_WALL,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spec 0130: every meshing pair's arbors stand the distance their own
    /// tooth counts put them at.
    ///
    /// **One module across the train**, so the distance is half the module
    /// times the sum of the counts and nothing is placed by hand. A wheel
    /// that does not reach its pinion is a clock with a gap in it, which is
    /// the one fault a drawing of a movement can have that a drawing of a
    /// room cannot.
    #[test]
    fn the_wheels_mesh_because_of_their_teeth() {
        let going: Vec<Wheel> = going();
        // the tip and the root of a wheel as it is actually cut, taken off
        // the same profile the mesh is built from rather than off the
        // module: asked of the module, this test and the placement are the
        // same arithmetic twice and a wheel could be cut any size at all
        let cut = |teeth: u32| {
            let mut tip: f32 = 0.0;
            let mut root = f32::MAX;

            for step in 0..teeth * 32 {
                let out = tooth_out(teeth, step as f32 / (teeth * 32) as f32);

                tip = tip.max(out);
                root = root.min(out);
            }

            (tip, root)
        };

        // the seconds train meshes the same way, and its reach is the whole
        // reason its counts are what they are: two spans of `MODULE` times
        // the count have to bridge the gap from the escape arbor to the
        // middle, and a count too small is a pair of wheels with daylight
        // between them
        let trains = [going.clone(), seconds()];

        for pair in trains.iter().flat_map(|train| train.windows(2)) {
            let (driver, driven) = (&pair[0], &pair[1]);
            let apart = driver.at.distance(driven.at);
            let (wheel_tip, wheel_root) = cut(driver.teeth);
            let (pinion_tip, pinion_root) = cut(driven.pinion);

            assert!(
                wheel_tip + pinion_tip > apart,
                "a wheel of {} and a pinion of {} stand {} apart and their teeth reach {}, \
                 so they do not touch",
                driver.teeth,
                driven.pinion,
                apart,
                wheel_tip + pinion_tip
            );
            assert!(
                wheel_root + pinion_root < apart,
                "a wheel of {} and a pinion of {} stand {} apart and their roots reach {}, \
                 so they are cut into each other",
                driver.teeth,
                driven.pinion,
                apart,
                wheel_root + pinion_root
            );
        }

        // and the motion work, whose two meshes have to span the same
        // distance because the hour wheel is concentric with the cannon
        // pinion that drives the wheel between them
        assert_eq!(
            CANNON + MINUTE_WHEEL,
            MINUTE_PINION + HOUR_WHEEL,
            "the motion work's two meshes do not span the same distance"
        );

        let motion = motion();
        let want = MODULE * (CANNON + MINUTE_WHEEL) as f32 * 0.5;

        assert!(
            (motion[0].at.distance(motion[1].at) - want).abs() < 1e-4,
            "the cannon pinion does not reach the minute wheel"
        );
        assert!(
            (motion[2].at.distance(motion[1].at) - want).abs() < 1e-4,
            "the minute wheel's pinion does not reach the hour wheel"
        );
    }

    /// Spec 0130: the train divides an hour into a minute.
    #[test]
    fn the_train_divides_the_hour() {
        let going = going();
        let escape = going.last().expect("an escape wheel");

        assert_eq!(going.len(), 3, "the going train has {} wheels", going.len());
        assert!(
            (escape.turns.abs() - 60.0).abs() < 1e-3,
            "the escape wheel turns {} times an hour",
            escape.turns
        );
        assert_eq!(
            escape.teeth, 30,
            "the escape wheel has {} teeth",
            escape.teeth
        );

        // and the motion work is the twelve to one
        let hour = motion().last().expect("an hour wheel").turns;

        assert!(
            (hour.abs() - 1.0 / 12.0).abs() < 1e-4,
            "the hour wheel turns {} times an hour",
            hour
        );

        // which is what the hands are put on
        assert!((hour.abs() - HOUR_HAND).abs() < 1e-4);
        assert!((escape.turns.abs() - SECOND_HAND).abs() < 1e-3);
    }

    /// Spec 0130: the hands say the time they are given.
    ///
    /// **And the hour hand creeps**, which is the difference between a
    /// clock and a readout: at half past six it stands between six and
    /// seven, not on six.
    #[test]
    fn the_hands_say_the_time() {
        let round = std::f32::consts::TAU;
        // where a hand points, as a share of a turn clockwise from twelve
        let pointing = |turns: f32, hours: f32| {
            let at = -turned(turns, hours) / round;

            at.rem_euclid(1.0)
        };

        for (hours, minute, hour) in [
            (0.0f32, 0.0f32, 0.0f32),
            (3.0, 0.0, 0.25),
            (6.5, 0.5, 6.5 / 12.0),
            (9.25, 0.25, 9.25 / 12.0),
            (23.75, 0.75, 11.75 / 12.0),
        ] {
            assert!(
                (pointing(MINUTE_HAND, hours) - minute).abs() < 1e-4,
                "at {} the minute hand points at {} and wants {}",
                hours,
                pointing(MINUTE_HAND, hours),
                minute
            );
            assert!(
                (pointing(HOUR_HAND, hours) - hour).abs() < 1e-4,
                "at {} the hour hand points at {} and wants {}",
                hours,
                pointing(HOUR_HAND, hours),
                hour
            );
        }

        // the hour hand is between the hours and not on one
        let at_half_six = pointing(HOUR_HAND, 6.5);

        assert!(
            at_half_six > 6.0 / 12.0 && at_half_six < 7.0 / 12.0,
            "at half past six the hour hand points at {}",
            at_half_six
        );
    }

    /// Spec 0130: the pendulum beats seconds, and the escape wheel lets one
    /// tooth by every two beats.
    #[test]
    fn the_pendulum_beats_seconds() {
        // a beat is the swing from one side to the other, so it is back
        // where it started after two of them
        let at = |seconds: f32| swung(seconds / 3600.0);

        assert!(at(0.0).abs() < 1e-5, "it does not start upright");
        assert!(at(0.5) > SWING * 0.98, "it is not over at half a beat");
        assert!(
            at(1.5) < -SWING * 0.98,
            "it is not back at a beat and a half"
        );
        assert!(at(2.0).abs() < 1e-3, "it is not upright after two beats");

        // thirty teeth, one every two beats, is a minute
        let going = going();
        let escape = going.last().expect("an escape wheel");
        let a_tooth = 3600.0 / (escape.turns.abs() * escape.teeth as f32);

        assert!(
            (a_tooth - 2.0).abs() < 1e-4,
            "a tooth goes by every {} seconds, which is not two beats of one",
            a_tooth
        );

        // and the rod is the length a seconds pendulum has
        assert!(
            (rod_long() - 0.994).abs() < 0.01,
            "the pendulum is {} long, against the 0.994 a seconds one wants",
            rod_long()
        );
    }

    /// Spec 0130: a thing laid at a turn lies the way the face says it
    /// lies.
    ///
    /// **This is the one the drawn clock got wrong.** Where a hand sat came
    /// off `along` and which way it lay came off a rotation written
    /// separately, with the sign the other way about. Every hand was then a
    /// chord of the dial running from one arbitrary point to another, and
    /// the picture test passed because a reference only says what it looked
    /// like when somebody was happy with it.
    #[test]
    fn a_turn_lays_a_thing_the_way_the_face_says() {
        for step in 0..24 {
            let turn = turned(MINUTE_HAND, step as f32 / 24.0);
            // the way a bar along the clock's own upright ends up pointing
            let laid = facing() * turned_to(turn) * Vec3::Y;
            let says = along(turn);

            assert!(
                (laid - stood(Vec3::ZERO, says, 0.0)).length() < 1e-5,
                "at {} of a turn the face says {} and a thing laid on it points {}",
                turn,
                says,
                laid
            );
        }
    }

    /// Spec 0130: and the face runs clockwise for somebody standing in the
    /// room.
    ///
    /// **Which is not the same claim.** The two above it agree with each
    /// other whichever way round the dial goes; this is the one that knows
    /// the clock is on the north wall and that a player at it is looking
    /// north, so east is on their left and a hand at quarter past has to
    /// run west.
    #[test]
    fn the_face_runs_the_way_the_room_sees_a_clock_run() {
        const REACHES: f32 = 9.5;

        let face = dial_at(REACHES);
        let points = |hours: f32| stood(face, along(turned(MINUTE_HAND, hours)), 0.0) - face;

        // the room is south of the wall, so out of the face is toward it
        assert!(
            stood(face, Vec2::ZERO, 1.0).z < face.z,
            "the face of the clock looks into the wall"
        );

        let quarter = points(0.25);

        assert!(
            quarter.x < -0.99 && quarter.y.abs() < 0.01,
            "at quarter past the minute hand runs {}, which is not to the west",
            quarter
        );

        let half = points(0.5);

        assert!(
            half.y < -0.99 && half.x.abs() < 0.01,
            "at half past the minute hand runs {}, which is not downward",
            half
        );
    }
    /// Spec 0130: nothing in the room stands where the pendulum swings.
    ///
    /// **Jake, on the first picture of it in the load room: the pendulum
    /// intersects an object.** A bucket against the near wall, put there
    /// when the room was built and the wall was bare. The bob comes down
    /// to a quarter of a metre off the floor, which is bucket height, and
    /// the clutter runs the length of the one wall a clock wants.
    #[test]
    fn nothing_stands_where_the_pendulum_swings() {
        const REACHES: f32 = 9.5;

        let swings = swings_over(REACHES);

        // off the room's own list of what it stops you walking through,
        // which is the ladder and the broom as well as the clutter: a
        // second list of one thing drifts
        for box_of in crate::back::clutter_solid(REACHES) {
            assert!(
                !swings.intersects(&box_of),
                "something standing from {} to {} is in the way of a pendulum that swings {} to {}",
                box_of.min,
                box_of.max,
                swings.min,
                swings.max
            );
        }
    }

    /// Spec 0130: and nothing stands in front of the readout.
    ///
    /// **The stepladder did.** It leans in the west corner of that wall and
    /// crosses head height a metre in, which is where the readout was
    /// written first, so the digits had a rail across them. The same fault
    /// as the bucket under the pendulum and found the same way, by looking
    /// at it: both are things placed along a bare wall years before
    /// anything was hung on it.
    #[test]
    fn nothing_stands_in_front_of_the_readout() {
        const REACHES: f32 = 9.5;

        let clear = readout_clear(REACHES);

        for box_of in crate::back::clutter_solid(REACHES) {
            assert!(
                !clear.intersects(&box_of),
                "something standing from {} to {} is in front of a readout from {} to {}",
                box_of.min,
                box_of.max,
                clear.min,
                clear.max
            );
        }

        // and it is on the wall rather than through the clock's own board
        let (middle, size) = readout_board(REACHES);
        let (face, board) = (dial_at(REACHES), board(REACHES).1);

        assert!(
            (middle.x - face.x).abs() - size.x * 0.5 > board.x * 0.5,
            "the readout's plate runs into the clock's board"
        );
    }
    /// Spec 0130: the numeral at an hour stands where the hour hand points
    /// at that hour.
    ///
    /// **Which is what an hour mark is, and the ring had it mirrored.**
    /// The marks were written as twelfths of a turn and the hands as a
    /// rate times an hour, with the signs opposite, so the dial ran one way
    /// and the hands the other. Twelve identical bars cannot show that: a
    /// dial mirrored about noon has a bar wherever a dial has a bar, and it
    /// took putting numbers on them for I to turn up where XI belongs.
    #[test]
    fn an_hour_mark_stands_where_the_hour_hand_points() {
        let marks = hours();

        assert_eq!(marks.len(), 12);

        for (hour, mark) in marks.iter().enumerate() {
            let hand = turned(HOUR_HAND, hour as f32);
            let apart = (along(*mark) - along(hand)).length();

            assert!(
                apart < 1e-5,
                "the mark for {} o'clock lies {} and the hour hand at {} o'clock points {}",
                hour,
                along(*mark),
                hour,
                along(hand)
            );
        }

        // and three o'clock is on the room's right, which is where the
        // minute hand stands at quarter past
        let three = along(marks[3]);

        assert!(
            three.x < -0.99,
            "three o'clock lies {}, which is not to the west",
            three
        );
    }
    /// Spec 0130: the readout says what the hands say.
    ///
    /// **Two readings of one clock, and the point of having both is that
    /// they agree.** The hands come off `turned` and a rate an hour; the
    /// digits come off the same hours counted into seconds. Nothing joins
    /// them but the number they are given, which is what this checks.
    #[test]
    fn the_readout_says_what_the_hands_say() {
        let round = std::f32::consts::TAU;

        for hours in [0.0f32, 3.0, 6.5, 9.25, 13.004, 23.75, 23.999] {
            let [h_ten, h_one, m_ten, m_one, s_ten, s_one] = said(hours);
            let (hour, minute) = (h_ten * 10 + h_one, m_ten * 10 + m_one);
            let seconds = s_ten * 10 + s_one;

            // **A hand creeps**, so a hand is read against the whole time
            // the readout says and not against its own figure: at half
            // past six the readout says 6 and the hour hand stands at six
            // and a half, which is the clock being right rather than the
            // two disagreeing.
            let says = (minute as f32 + seconds as f32 / 60.0) / 60.0;
            let apart = |hand: f32, want: f32, of: f32| {
                let at = (-turned(hand, hours) / round).rem_euclid(1.0) * of;

                ((at - want).rem_euclid(of)).min((want - at).rem_euclid(of))
            };

            assert!(
                apart(MINUTE_HAND, minute as f32 + seconds as f32 / 60.0, 60.0) < 0.02,
                "at {} the readout says {} minutes and {} seconds, which the minute hand does not",
                hours,
                minute,
                seconds
            );
            assert!(
                apart(HOUR_HAND, (hour % 12) as f32 + says, 12.0) < 0.02,
                "at {} the readout says {}:{:02}, which the hour hand does not",
                hours,
                hour,
                minute
            );

            assert!(hour < 24 && minute < 60 && seconds < 60);
        }
    }

    /// Spec 0130: and every digit lights its own set of segments.
    ///
    /// **A table of ten numbers written by hand.** Two of them the same is
    /// a clock that says six when it means five, and nothing else in here
    /// would notice: the time would still advance, the segments would
    /// still light, and the figure would simply be wrong.
    #[test]
    fn every_digit_lights_its_own_segments() {
        for digit in 0..10u32 {
            let shown = readout(digit as f32 / 3600.0);
            let alight: Vec<bool> = shown.iter().map(|(_, _, lit)| *lit).collect();

            assert_eq!(
                shown.len(),
                7 * 6 + 4,
                "the readout is not six digits and two colons"
            );

            for other in 0..10u32 {
                if other == digit {
                    continue;
                }

                let theirs = readout(other as f32 / 3600.0);
                let lit: Vec<bool> = theirs.iter().map(|(_, _, lit)| *lit).collect();

                assert_ne!(
                    alight, lit,
                    "{} and {} light the same segments",
                    digit, other
                );
            }
        }
    }
    /// Spec 0130: the board is against the wall, and the clock is on the
    /// board.
    ///
    /// **Because where that wall's face is was worked out by hand and got
    /// wrong.** `dial_at` took a wall's thickness off `deep`, which is a
    /// claim about a box nobody checked: the near wall runs from `near`
    /// outward and its face is at `near`. The whole clock stood thirty
    /// centimetres into the room, and the board behind it cut the buckets
    /// along that wall in half. Jake: the cans are clipped.
    #[test]
    fn the_board_is_against_the_wall() {
        const REACHES: f32 = 9.5;

        let face = crate::back::near_wall(REACHES).min.z;

        for (what, (middle, size)) in [
            ("the clock's board", board(REACHES)),
            ("the readout's plate", readout_board(REACHES)),
        ] {
            let back = middle.z + size.z * 0.5;

            assert!(
                (back - face).abs() < 1e-4,
                "{} has its back at {} and the wall is at {}",
                what,
                back,
                face
            );
        }

        // and nothing of the clock is behind the board's own face
        let front = board(REACHES).0.z - board(REACHES).1.z * 0.5;

        assert!(
            dial_at(REACHES).z < front,
            "the dial stands at {}, which is behind the board's face at {}",
            dial_at(REACHES).z,
            front
        );
    }
    /// Spec 0130: the seconds hand turns about the middle of the dial, once
    /// a minute, the way the escape wheel does.
    ///
    /// **Jake, on the first one: close to meaningless.** It was a quarter
    /// of the radius long, on an arbor half a metre off centre, among the
    /// brass. This checks the three things that fixing it depended on: the
    /// train comes back to the middle, it is one to one so the hand keeps
    /// the escape wheel's own rate, and it turns the same way round rather
    /// than backwards, which an odd number of meshes would give.
    #[test]
    fn the_seconds_come_back_to_the_middle() {
        let train = seconds();
        let escape = going().last().expect("an escape wheel").turns;
        let last = train.last().expect("a wheel at the middle");

        assert_eq!(
            train[0].at,
            going().last().expect("an escape wheel").at,
            "the seconds train does not start on the escape arbor"
        );
        assert!(
            last.at.length() < 1e-5,
            "the seconds train ends {} from the middle of the dial",
            last.at.length()
        );
        assert!(
            (last.turns - escape).abs() < 1e-3,
            "the escape wheel turns {} an hour and the seconds hand {}",
            escape,
            last.turns
        );

        // and the hand is on it, at the middle, and is the longest of the
        // three
        let hands = hands();
        let seconds_hand = hands
            .iter()
            .find(|hand| hand.beats)
            .expect("a seconds hand");

        assert!(
            seconds_hand.at.length() < 1e-5,
            "the seconds hand is off centre"
        );
        assert!(
            hands.iter().all(|hand| hand.long <= seconds_hand.long),
            "the seconds hand is not the longest"
        );
    }

    /// Spec 0130: and it steps rather than sweeping, because that is what
    /// an escapement does.
    #[test]
    fn what_the_escapement_holds_moves_a_second_at_a_time() {
        let hand = turned(SECOND_HAND, when(true, 10.0 + 30.4 / 3600.0));
        let same = turned(SECOND_HAND, when(true, 10.0 + 30.9 / 3600.0));
        let next = turned(SECOND_HAND, when(true, 10.0 + 31.1 / 3600.0));
        let step = std::f32::consts::TAU / 60.0;

        assert!(
            (hand - same).abs() < 1e-4,
            "the hand moved {} within one second",
            (hand - same).abs()
        );
        // a part in a thousand, which is what f32 has left after counting
        // ten hours in seconds: 36,030 needs sixteen bits of the
        // twenty-four, and the step is what is left over
        assert!(
            ((same - next).abs() - step).abs() < 1e-3,
            "the hand moved {} over a second's edge, against a step of {}",
            (same - next).abs(),
            step
        );

        // and the train in front of the escapement does not step
        let smooth = going().iter().filter(|wheel| !wheel.beats).count();

        assert_eq!(
            smooth, 2,
            "{} wheels of the going train turn smoothly",
            smooth
        );
        assert!(
            (turned(MINUTE_HAND, 1.0) - turned(MINUTE_HAND, when(false, 1.0))).abs() < 1e-6,
            "the minute hand is being stepped"
        );
    }
}
