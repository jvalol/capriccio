//! The second paradise, through the glass. Spec 0133.
//!
//! **Somewhere else, and reached by walking.** Valhalla is the precedent: a
//! whole place built elsewhere in the world and arrived at on foot. The
//! glass room stands over the nook and its stair comes down into the nook,
//! so one piece of world cannot give that stair two bottoms. What is here is
//! a second copy of that room, a long way off, whose stair goes down to a
//! shore instead.
//!
//! **A long way off so that nothing of one place is in frame with the
//! other.** Not for the sky's sake: `Scene::backdrop` hangs the sky off the
//! eye rather than standing it in the world, so this place's own daylight is
//! a colour drawn while you are here. The distance is so that nothing of the
//! building is on this horizon and nothing of this is on the building's,
//! which is the whole of what spec 0055 made one sky to protect.

use blitzkit::collision::Aabb;
use glam::{vec2, vec3, Vec2, Vec3};

/// How far south of the building it stands.
///
/// **Past everything, with room to spare.** The field's ground reaches
/// `field::GROUND_REACH`, 160, and is thinned rather than ended past it. Six
/// hundred is nearly four times that, so nothing of the field is near this
/// and nothing of this is in the field.
pub const OUT: f32 = 600.0;

/// How wide the water is, and how deep you stand in it.
///
/// **Thirty-five, which is twenty seconds.** `SPEED` is 4.2 and
/// `spa::WADE` is 0.42 of it, so wading is 1.76 a second. Twice the hall's
/// 19 and a third of the bridge's 100: long enough to be a passage, short
/// enough that the walk back is not a toll, and you walk back every time you
/// leave. Spec 0133.
pub const ACROSS: f32 = 35.0;

/// How far the water and the ground run either side of the way over, so
/// that it reads as a crossing rather than a corridor.
pub const BROAD: f32 = 150.0;

/// How far under the shore the water's bed lies, and how far under its own
/// surface.
///
/// **Shin deep.** `spa::wading` asks only that your feet are under the
/// surface, so any depth wades. This one is shallow because a garden you can
/// see the whole way is the point, and water you would have to swim is water
/// you look at instead of cross.
pub const DEEP: f32 = 0.34;

/// How far the shore runs from the stair's foot to the water's edge, and the
/// same on the far side before the garden starts.
pub const SHORE: f32 = 6.0;

/// Where the middle of the whole place is: the water's middle, on the line
/// you walk in on.
pub fn at(reaches: f32) -> Vec3 {
    vec3(crate::glass::stair_foot(reaches).x, 0.0, OUT)
}

/// The ground, which is this place's own and reaches past everything on it.
///
/// **Because the field's does not come here.** That slab follows you and
/// knows about the building and nothing else, so it laid grass under the
/// shore and round the water until it was told to keep out. Something has
/// to be under you instead, and it has to reach further than you can see
/// across, or the edge of it is the edge of the world.
pub fn ground(reaches: f32) -> Aabb {
    let middle = at(reaches);
    let half = BROAD;

    Aabb::new(
        vec3(middle.x - half, -DEEP - 1.0, middle.z - half),
        vec3(middle.x + half, -DEEP - 0.4, middle.z + half),
    )
}

/// The near shore, where the stair comes down, and the far one the garden
/// stands on. Each as its own slab.
pub fn shores(reaches: f32) -> (Aabb, Aabb) {
    let middle = at(reaches);
    let half = BROAD * 0.5;
    let edge = ACROSS * 0.5;

    (
        Aabb::new(
            vec3(middle.x - half, -DEEP, middle.z - edge - SHORE),
            vec3(middle.x + half, 0.0, middle.z - edge),
        ),
        Aabb::new(
            vec3(middle.x - half, -DEEP, middle.z + edge),
            vec3(middle.x + half, 0.0, middle.z + edge + half),
        ),
    )
}

/// The bed under the water, which is what holds you up while you wade.
pub fn bed(reaches: f32) -> Aabb {
    let middle = at(reaches);
    let half = BROAD * 0.5;
    let edge = ACROSS * 0.5;

    Aabb::new(
        vec3(middle.x - half, -DEEP - 0.4, middle.z - edge),
        vec3(middle.x + half, -DEEP, middle.z + edge),
    )
}

/// The water's own surface: where it is and how big, as a middle and a size
/// across.
pub fn water(reaches: f32) -> (Vec3, Vec2) {
    let middle = at(reaches);

    (vec3(middle.x, 0.0, middle.z), vec2(BROAD, ACROSS))
}

/// Whether your feet are in it, which is what slows you to a wade.
///
/// **The baths' own question, asked of this water.** `spa::wading` reads the
/// pool's own middle and size; this reads these. One account of what wading
/// is and two places that are waded.
pub fn wading(reaches: f32, at_: Vec3) -> bool {
    let (middle, size) = water(reaches);

    (at_.x - middle.x).abs() <= size.x * 0.5
        && (at_.z - middle.z).abs() <= size.y * 0.5
        && at_.y < middle.y - 0.05
}

/// Everything solid here: the two shores, the bed between them, and the
/// copy of the glass room standing over the near one.
///
/// **The room's own boxes, moved.** Every one of them is `glass::solid`'s,
/// offset by [`room_off`], so the two rooms cannot come apart: a tread added
/// to that stair is a tread on this one. A second account of a room that is
/// meant to be the same room is the fault this building has written down
/// under a dozen names.
pub fn solid(reaches: f32) -> Vec<Aabb> {
    let (near, far) = shores(reaches);
    let off = room_off(reaches);
    let mut out = vec![ground(reaches), near, bed(reaches), far];

    out.extend(
        crate::glass::solid(reaches)
            .into_iter()
            .map(|box_| Aabb::new(box_.min + off, box_.max + off)),
    );

    out
}

/// Where you stand when you come through the glass, which is the copy of
/// spec 0064's room this place keeps.
///
/// **The same room, offset.** Every number in it is `glass`'s own, moved by
/// the distance between the two places, so a change to that room is a change
/// to this one and there is no second account of it to drift.
pub fn room_off(reaches: f32) -> Vec3 {
    let foot = crate::glass::stair_foot(reaches);
    let (near, _) = shores(reaches);

    vec3(0.0, 0.0, near.min.z - foot.z)
}

/// How near the sphere's middle you have to be for it to be the thing you
/// are at, and how far off your line it may be and still be what you are
/// looking into.
///
/// **A pace and a half, because the plinth decides it.** You are held 0.90
/// out from the axis, so anything tighter can never be reached and anything
/// much looser would catch you crossing the room.
pub const AT_IT: f32 = 1.35;

/// Which way the glass takes you from here, if you are at a sphere at all.
///
/// `Some((off, middle))` is the move that puts you in the other room and
/// the middle of the sphere you are looking into: the building's sphere
/// hands you [`room_off`] and this one hands you its negative, so the door
/// works both ways and neither end needs to know which it is. `None` is
/// everywhere else in both buildings.
///
/// The middle comes back with it because the caller has to ask whether you
/// are getting any closer to that glass, and working out which of the two
/// you matched a second time is two accounts of one answer.
///
/// **Which sphere you are at is the whole of the state.** There is no flag
/// saying where you are, because there does not need to be one: both rooms
/// stand in the world at once and your own position says which you are in. A
/// flag would be a second account of that, and this building has learned
/// what two accounts of one thing do.
///
/// **Looking into it, and not merely near it.** The ray has to pass through
/// the glass, which is what makes it the thing you are doing rather than the
/// thing you walked past. Spec 0133.
pub fn through(reaches: f32, from: Vec3, facing: Vec3) -> Option<(Vec3, Vec3)> {
    let off = room_off(reaches);
    let (here, radius) = crate::glass::ball(reaches);

    for (middle, way) in [(here, off), (here + off, -off)] {
        if from.distance(middle) > radius + AT_IT {
            continue;
        }

        // the ray's nearest approach to the middle, which is inside the
        // glass when you are looking into it
        let to = middle - from;
        let along = to.dot(facing);

        if along <= 0.0 {
            continue;
        }

        if (to - facing * along).length() <= radius {
            return Some((way, middle));
        }
    }

    None
}

/// How long pushing into the glass takes to carry you through, in seconds,
/// and what the ball's index winds to on the way.
///
/// **Nothing is held down for it.** Walking into the glass starts it and it
/// finishes on its own: no leaning, nothing to keep up and nothing to fail
/// at. Jake, having tried two versions that asked him to hold: i don't
/// think leaning is a mechanism that works, if i walk at it without
/// stopping the transition should happen without delay.
///
/// So this is how long the room takes to fold away once you have walked
/// into it, and not how long you have to keep at it. His log has the first
/// version climbing to 0.89 and falling away, which is what leaning on
/// something that does not give gets you.
///
/// `glass::INDEX` is -1, which engine spec 0098 draws as a mirror; a
/// positive index bends what is behind it instead. Winding from one to the
/// other is the room letting go of its own reflection and swallowing what
/// is in front of it, which is the nearest thing in this engine to a hole
/// opening, and it costs no pass, no target and no shader.
pub const THROUGH: f32 = 0.7;

pub const SWALLOWS: f32 = 2.4;

/// What the ball's index is, a share `through` of the way into the
/// crossing.
///
/// Eased so the first part of the push does little and the last of it does
/// most: leaning on glass that gives way at once is a door, and this is
/// meant to read as something resisting and then not.
pub fn bending(through: f32) -> f32 {
    let eased = through.clamp(0.0, 1.0).powi(3);

    crate::glass::INDEX + (SWALLOWS - crate::glass::INDEX) * eased
}

/// Whether you are in this place at all, which is what decides whose
/// ground is under you and whose sky is over you.
///
/// **Asked of where you are and of nothing else.** There is no flag saying
/// which world you are in, for the reason [`through`] gives: both stand in
/// the world at once and your own position is the answer. This is that
/// question written down once so that the ground, the sky and the field all
/// read the same one.
///
/// Generous past the edges, because the horizon is part of being somewhere:
/// a shore whose grass turns back into the field's ten metres out is a
/// shore in a field.
pub fn holds(reaches: f32, at_: Vec3) -> bool {
    let middle = at(reaches);

    (at_.x - middle.x).abs() < BROAD && (at_.z - middle.z).abs() < BROAD
}

/// The hour it is here, always.
///
/// **Morning, and it does not move.** The building runs a day in
/// `garden::DAY`, twenty seconds, and whatever hour you arrive at is
/// whatever hour it is. Here it is one hour for ever, which is half of what
/// makes this somewhere else: spec 0133 wants three differences visible in
/// the first frame and this is the one you cannot miss.
///
/// **An hour, not a `since`.** `garden::hour` is a `since` divided by the
/// day and offset by `garden::OPENS_AT`, so the two are not the same number
/// and `garden::when` is the one function that turns this one into the
/// other. Written as a `since` instead, this read 0.95 of a day, which is
/// the small hours: Elysium was lit by nothing at all and the first three
/// faults reported off it were a dark sky, flat grey water and an unlit
/// room, all of them this. Dividing by `DAY` was wrong a second way, since
/// a run under `--film` or `--shot` sets its own day length.
///
/// 0.17 is morning. `garden::sun_up` is a plain sine of the hour, so it
/// rises from nothing at 0.0 and stands highest at 0.25: this is well up,
/// with the shadows still long enough to have a direction, and short of the
/// hour with no shape to it.
pub const MORNING: f32 = 0.17;

/// How far past its own edge this keeps the field's scatter off.
///
/// A tree is `field::TREE_TALL` and leans, so a clearing that stops at the
/// shore's edge still has branches over it.
pub const CLEARS: f32 = 8.0;

/// Whether a spot in the field is one this place keeps clear of trees.
///
/// **Because the field's scatter has never known anything is there.**
/// `field::open` keeps trees out of the building and out of nothing else,
/// which is the hole the bridge found when one grew through its runestone,
/// and the village after it. This is the third, and the first picture of
/// this place had trees standing in the water.
///
/// **Always, and not only once you can get here.** The bridge's own note
/// says why: a clearing that appears the moment somebody finds the way in
/// is trees vanishing, which is worse to watch than trees in a lake. Here
/// it costs nothing to say always, because nobody is ever standing in the
/// field six hundred metres south to see either.
pub fn shades(reaches: f32, at_: Vec3) -> bool {
    let middle = at(reaches);
    let (near, far) = shores(reaches);

    (at_.x - middle.x).abs() < BROAD * 0.5 + CLEARS
        && at_.z > near.min.z - CLEARS
        && at_.z < far.max.z + CLEARS
}

/// How near you have to be for any of it to be drawn.
///
/// **Six hundred metres away, so never from the building.** Nothing here is
/// ever in the same frame as anything there, which is the whole arrangement,
/// so drawing it while you are in the arcade is a bill for pixels nobody can
/// see. `glass::SEES` does the same for the ball.
pub const SEES: f32 = 320.0;

/// Draws it: the shores, the water, and the copy of the glass room.
///
/// **The room is `glass::draw` again, moved.** It was a loop over
/// [`solid`]'s boxes in one flat colour, which is a blocking model: Jake
/// walked it and the copy was a dark slab where the room he had just left
/// has a floor, a colonnade, coffering and a stair. Spec 0064's room is
/// drawn by one function and that function now takes where to put it, so
/// this is the same joinery and not a second account of it.
///
/// What is left here is the ground: the two shores, the bed and the water,
/// which belong to this place and to nothing else.
pub fn draw(
    scene: &mut blitzkit::renderer::scene::Scene,
    reaches: f32,
    since: f32,
    eye: Vec3,
    index: f32,
    meshes: crate::glass::Meshes,
) {
    use blitzkit::mesh::Transform;

    let middle = at(reaches);

    if eye.distance(middle) > SEES {
        return;
    }

    let cube = meshes.cube;
    let column = meshes.column;
    let (near, far) = shores(reaches);

    for piece in [ground(reaches), near, bed(reaches), far] {
        scene.push_material(
            cube,
            &Transform::at(piece.center()).with_scale(piece.size()),
            crate::aim::SHORE,
            crate::aim::MATTE,
        );
    }

    crate::glass::draw(scene, reaches, since, index, room_off(reaches), meshes);

    // and the water over the bed, which is the only thing here that is not
    // a thing you stand on
    let (surface, size) = water(reaches);

    scene.push_material(
        cube,
        &Transform::at(surface).with_scale(vec3(size.x, 0.02, size.y)),
        crate::aim::SHALLOWS,
        crate::aim::MATTE,
    );

    // and the colonnade on the far shore, which is the garden
    let Some(column) = column else {
        return;
    };

    for (stood, upright) in colonnade(reaches) {
        let turned = match upright {
            true => (
                stood + Vec3::Y * COLUMN_TALL * 0.5,
                glam::Quat::IDENTITY,
                vec3(COLUMN_WIDE, COLUMN_TALL, COLUMN_WIDE),
            ),
            // **A fallen one lies along the row and half buried.** A
            // cylinder laid on grass with its whole face showing reads as a
            // pipe somebody put down; sunk a little it reads as something
            // that has been there a while.
            false => (
                stood + Vec3::Y * COLUMN_WIDE * 0.34,
                glam::Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                vec3(COLUMN_WIDE, COLUMN_TALL * 0.78, COLUMN_WIDE),
            ),
        };

        scene.push_material(
            column,
            &Transform::at(turned.0)
                .with_rotation(turned.1)
                .with_scale(turned.2),
            crate::aim::SHORE,
            crate::aim::MATTE,
        );
    }
}

/// The colonnade on the far shore: how tall a column stands, how thick, how
/// far apart they are set and how many there are.
///
/// **A row of columns and nothing holding them up.** Spec 0133 asks for a
/// garden with nobody in it, and the thing that makes a garden read as one
/// without a single figure is something built standing in it with no use
/// left. This is the one shape in that language this building already owns:
/// `glass::column_mesh`, the same lathe the room you came from is full of.
///
/// Set across the way you walk in, so you come out of the water looking
/// down the length of it rather than at one end.
pub const COLUMN_TALL: f32 = 5.4;
pub const COLUMN_WIDE: f32 = 0.78;
pub const COLUMN_APART: f32 = 4.6;
pub const COLUMNS: usize = 15;

/// How far back from the water's edge the colonnade stands.
///
/// Past the far shore's own width, so you walk out of the water, up the
/// shore, and then to it rather than straight into it.
pub const COLONNADE_BACK: f32 = 16.0;

/// Which of them are still standing.
///
/// **Not every other one.** A gap on a beat is a pattern and reads as
/// design; what a ruin looks like is a run of them standing and then two
/// missing and then one. These are picked by hand because fifteen is few
/// enough to pick by hand, and a rule that made them look accidental would
/// be a rule nobody could check.
pub fn standing(n: usize) -> bool {
    !matches!(n, 3 | 4 | 9 | 12)
}

/// Where each column of the colonnade stands, with whether it is upright.
///
/// A fallen one lies where it stood, along the row, which is where a column
/// goes when it goes: outward would be a column somebody pushed.
pub fn colonnade(reaches: f32) -> Vec<(Vec3, bool)> {
    let middle = at(reaches);
    let (_, far) = shores(reaches);
    let run = (COLUMNS - 1) as f32 * COLUMN_APART;

    (0..COLUMNS)
        .map(|n| {
            let along = n as f32 * COLUMN_APART - run * 0.5;

            (
                vec3(middle.x + along, 0.0, far.min.z + COLONNADE_BACK),
                standing(n),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reaches() -> f32 {
        crate::room::Room::of(crate::games()).reaches
    }

    /// Spec 0133: the crossing is about twenty seconds of wading.
    ///
    /// **Held in seconds and not in metres**, because the number that
    /// matters is how long you are in the water and the width is only how
    /// that was arrived at. A later hand widening the water has to move this
    /// line and see what it is spending.
    #[test]
    fn the_crossing_takes_about_twenty_seconds() {
        let wading = crate::SPEED * crate::spa::WADE;
        let seconds = ACROSS / wading;

        assert!(
            (19.0..=22.0).contains(&seconds),
            "the crossing is {:.1} seconds at {:.2} a second",
            seconds,
            wading
        );
    }

    /// Spec 0133: you cross the water and cannot walk round it.
    ///
    /// **The water is wider than the shores it lies between**, so there is
    /// no way along the bank that gets you to the far side.
    #[test]
    fn the_water_is_crossed_and_not_skirted() {
        let (near, far) = shores(reaches());
        let (_, size) = water(reaches());

        for (which, shore) in [("the near shore", near), ("the far shore", far)] {
            assert!(
                shore.max.x - shore.min.x <= size.x + 1e-3,
                "{} runs {:.1} wide past water {:.1} wide",
                which,
                shore.max.x - shore.min.x,
                size.x
            );
        }

        assert!(near.max.z <= far.min.z, "the shores meet without water");
    }

    /// Spec 0133: wading here is the baths' own question asked of this
    /// water, so standing on either shore is not wading and standing in it
    /// is.
    #[test]
    fn the_crossing_is_waded_at_the_baths_own_pace() {
        let reaches = reaches();
        let (near, far) = shores(reaches);
        let (middle, _) = water(reaches);

        let in_it = vec3(middle.x, -DEEP + 0.05, middle.z);
        assert!(wading(reaches, in_it), "the middle of the water is dry");

        for (which, shore) in [("near", near), ("far", far)] {
            let on_it = vec3(
                (shore.min.x + shore.max.x) * 0.5,
                0.01,
                (shore.min.z + shore.max.z) * 0.5,
            );

            assert!(
                !wading(reaches, on_it),
                "standing on the {} shore counts as wading",
                which
            );
        }
    }

    /// Spec 0133: the glass takes you through from either end, and the two
    /// moves are each other's opposite, so the door works both ways.
    ///
    /// **Which is the whole reason there is no flag.** Both rooms stand in
    /// the world at once and where you are says which you are in. If these
    /// two offsets were ever not opposites, the door would put you somewhere
    /// that is neither.
    #[test]
    fn the_glass_takes_you_through_and_back() {
        let reaches = reaches();
        let (here, _) = crate::glass::ball(reaches);
        let off = room_off(reaches);
        let at = vec3(here.x, here.y, here.z + 1.0);
        let into = (here - at).normalize();

        let (there, _) = through(reaches, at, into).expect("the building's glass is a door");
        let (back, _) = through(reaches, at + off, into).expect("the other one is too");

        assert!(
            (there + back).length() < 1e-3,
            "the two ways through are {:?} and {:?}, which are not opposites",
            there,
            back
        );
        assert!(
            (there - off).length() < 1e-3,
            "it does not take you to the copy"
        );
    }

    /// Spec 0133: looking at it is not going through it, and neither is
    /// being near it while looking elsewhere.
    ///
    /// **The pair that matters.** The walk is what triggers the crossing,
    /// and this holds the other half: the ray has to be in the glass. One of
    /// these without the other passes on a room you fall through by standing
    /// in it.
    #[test]
    fn standing_beside_it_is_not_going_through_it() {
        let reaches = reaches();
        let (here, radius) = crate::glass::ball(reaches);
        let at = vec3(here.x, here.y, here.z + 1.0);

        assert!(
            through(reaches, at, vec3(1.0, 0.0, 0.0)).is_none(),
            "looking across the room counts as looking into the glass"
        );
        assert!(
            through(reaches, at, vec3(0.0, 0.0, 1.0)).is_none(),
            "looking away from it counts as looking into it"
        );

        // **Ahead of you and still a miss**, which is the case the two
        // above do not reach: both of those are behind you, so the guard
        // that catches them is the one asking whether it is in front at
        // all. This one is in front and off to the side, and only the ray
        // itself can refuse it.
        let past = vec3(0.707, 0.0, -0.707);

        assert!(
            through(reaches, at, past).is_none(),
            "looking past it, {:.2} off its middle, counts as looking into it",
            (here - at - past * (here - at).dot(past)).length()
        );

        let far = vec3(here.x, here.y, here.z + radius + AT_IT + 1.0);

        assert!(
            through(reaches, far, (here - far).normalize()).is_none(),
            "looking into it from across the room counts as being at it"
        );
    }

    /// Spec 0133: the glass is a mirror until somebody leans on it, and a
    /// lens by the time they are through.
    ///
    /// **Both ends, because either alone is a crossing that looks like a
    /// cut.** Jake, on the first one: no wormhole or blackhole like visual
    /// transformation. Engine spec 0098 draws an index at or under nought
    /// as a mirror and a positive one as something that bends what is
    /// behind it, so the sign changing is the room letting go of its own
    /// reflection.
    #[test]
    fn the_glass_turns_from_a_mirror_into_a_lens() {
        assert!(
            (bending(0.0) - crate::glass::INDEX).abs() < 1e-6,
            "untouched it is {:.2} and the room draws {:.2}",
            bending(0.0),
            crate::glass::INDEX
        );
        assert!(
            bending(0.0) <= 0.0,
            "untouched it is already bending rather than reflecting"
        );
        assert!(
            bending(1.0) > 0.0,
            "all the way through it is still a mirror at {:.2}",
            bending(1.0)
        );

        // and it only ever goes one way on the trip in
        let mut was = bending(0.0);

        for n in 1..=10 {
            let now = bending(n as f32 / 10.0);

            assert!(
                now >= was,
                "it went back at {:.1}: {:.2} then {:.2}",
                n,
                was,
                now
            );
            was = now;
        }
    }

    /// Spec 0133: standing anywhere in this place, it is morning here and
    /// the building's own hour is somewhere else.
    ///
    /// **Including inside the copy of the glass room**, which is the spot
    /// you arrive at and the one place it would be easy to miss: the room
    /// stands over the near shore rather than on the water, so a `holds`
    /// drawn round the water alone would leave whoever just came through
    /// standing in the building's night. Jake's recording of exactly that,
    /// 2026-11-23: emptiness on the other side.
    #[test]
    fn it_is_morning_everywhere_in_it() {
        let reaches = reaches();
        let middle = at(reaches);
        let foot = crate::glass::stair_foot(reaches) + room_off(reaches);
        let (near, far) = shores(reaches);

        for (which, spot) in [
            ("the water", middle),
            ("the near shore", vec3(middle.x, 0.0, near.min.z)),
            ("the far shore", vec3(middle.x, 0.0, far.max.z)),
            ("the stair's foot", foot),
            ("the room over it", foot + vec3(0.0, crate::glass::UP, 0.0)),
        ] {
            assert!(
                holds(reaches, spot),
                "{} is not held by this place, so the building's hour follows you there",
                which
            );
        }

        // and the building itself is not
        assert!(
            !holds(reaches, crate::glass::stair_foot(reaches)),
            "the building is held by this place, so its own day would stop"
        );
    }

    /// Spec 0133: the colonnade stands on the far shore, across the way
    /// you walk in, and every column of it is on dry ground.
    ///
    /// **Across and not along.** You come out of the water looking down the
    /// length of it, which is the one arrangement where a row of columns
    /// reads as something that was a building. End on it is a column.
    #[test]
    fn the_colonnade_stands_across_the_way_in_on_dry_ground() {
        let reaches = reaches();
        let (_, far) = shores(reaches);
        let (middle, size) = water(reaches);
        let stood = colonnade(reaches);

        assert_eq!(stood.len(), COLUMNS);

        let (mut least, mut most) = (f32::MAX, f32::MIN);

        for (at_, _) in &stood {
            assert!(
                at_.z > middle.z + size.y * 0.5,
                "a column stands at {:.1}, which is in the water",
                at_.z
            );
            assert!(
                at_.z >= far.min.z && at_.z <= far.max.z,
                "a column stands at {:.1}, off the far shore which runs {:.1}..{:.1}",
                at_.z,
                far.min.z,
                far.max.z
            );

            least = least.min(at_.x);
            most = most.max(at_.x);
        }

        // and it runs across you rather than away from you
        let across = most - least;
        let deep = stood.iter().map(|(at_, _)| at_.z).fold(f32::MIN, f32::max)
            - stood.iter().map(|(at_, _)| at_.z).fold(f32::MAX, f32::min);

        assert!(
            across > deep,
            "the colonnade runs {:.1} across and {:.1} away, so you meet it end on",
            across,
            deep
        );
    }

    /// Spec 0133: some of it has fallen, and not on a beat.
    ///
    /// **A gap every other one is a pattern.** What a ruin looks like is a
    /// run standing, then two gone, then one. This holds that some are down,
    /// that most are not, and that the gaps are not evenly spaced.
    #[test]
    fn the_colonnade_is_a_ruin_and_not_a_pattern() {
        let down: Vec<usize> = (0..COLUMNS).filter(|n| !standing(*n)).collect();

        assert!(!down.is_empty(), "nothing has fallen, so it is a colonnade");
        assert!(
            down.len() * 3 < COLUMNS * 2,
            "{} of {} are down, which is a heap rather than a ruin",
            down.len(),
            COLUMNS
        );

        let gaps: Vec<usize> = down.windows(2).map(|two| two[1] - two[0]).collect();

        assert!(
            gaps.windows(2).any(|two| two[0] != two[1]),
            "the fallen ones are evenly spaced at {:?}, which reads as design",
            gaps
        );
    }

    /// Spec 0133: nothing the field sows stands in this place.
    ///
    /// **The third time this building has had to say it.** `field::open`
    /// keeps the scatter out of the building and out of nothing else, so a
    /// tree grew through the bridge's runestone, then through the village,
    /// and the first picture of this place had them standing in the water.
    ///
    /// Asked over the whole thing and a margin, because a tree is
    /// `field::TREE_TALL` and leans.
    #[test]
    fn the_field_sows_nothing_here() {
        let reaches = reaches();
        let (near, far) = shores(reaches);
        let middle = at(reaches);

        for z in [near.min.z, near.max.z, middle.z, far.min.z, far.max.z] {
            for x in [middle.x - BROAD * 0.5, middle.x, middle.x + BROAD * 0.5] {
                let spot = vec3(x, 0.0, z);

                assert!(
                    shades(reaches, spot),
                    "the field may sow a tree at {:?}, which is in the water or on a shore",
                    spot
                );
            }
        }

        // and it stops: a clearing the size of the field is a field with no
        // trees in it
        let away = vec3(middle.x, 0.0, far.max.z + CLEARS + 20.0);

        assert!(
            !shades(reaches, away),
            "the clearing reaches {:?}, which is well past the far shore",
            away
        );
    }

    /// Spec 0133: the copy of the glass room stands over the near shore, so
    /// its stair comes down on this side of the water.
    ///
    /// **And the whole place is far enough from the building that neither
    /// is on the other's horizon**, which is what lets this one have its own
    /// sky at all.
    #[test]
    fn the_stair_comes_down_on_the_near_shore() {
        let reaches = reaches();
        let foot = crate::glass::stair_foot(reaches) + room_off(reaches);
        let (near, _) = shores(reaches);
        let (middle, size) = water(reaches);

        assert!(
            foot.z >= near.min.z - 1e-3 && foot.z <= near.max.z + 1e-3,
            "the stair lands at {:.2} and the near shore runs {:.2}..{:.2}",
            foot.z,
            near.min.z,
            near.max.z
        );
        assert!(
            foot.z < middle.z - size.y * 0.5,
            "the stair lands in the water"
        );
        assert!(
            (foot.z - crate::glass::stair_foot(reaches).z).abs() > crate::field::GROUND_REACH,
            "this place is inside the field's own reach"
        );
    }

    /// The sun is up over this place, and it is morning rather than evening.
    ///
    /// This asks `garden::sun_up`, which is the thing that was wrong, rather
    /// than asking `MORNING` what it says. Written as a `since` the constant
    /// read 0.95 of a day and the sun stood at -0.31: three faults reported
    /// off that one line, all of them a dark world.
    #[test]
    fn the_sun_stands_well_up_over_elysium() {
        let hour = crate::garden::when(MORNING);
        let up = crate::garden::sun_up(hour);

        // Only that the sun is up, which is the thing that was wrong.
        // Morning against afternoon wants the hour rather than the
        // brightness, and the hour here is a constant, so that assertion
        // was one clippy rejects and one a wrong value passed anyway.
        assert!(up > 0.8, "the sun stands at {:+.3} over Elysium", up);
    }
}
