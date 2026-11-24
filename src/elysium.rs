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
pub fn ground(at_: Vec3) -> Aabb {
    let half = BROAD;
    // snapped to the sand's own tile, per `field::ground`: a slab that
    // followed exactly would carry its texture with it, so walking would
    // slide the ground under your feet. Snapped, it jumps a tile at a time
    // and a jump of a tile is a jump of nothing.
    let middle = vec3(
        (at_.x / SAND_TILE).round() * SAND_TILE,
        -DEEP - 0.7,
        (at_.z / SAND_TILE).round() * SAND_TILE,
    );

    Aabb::from_center_size(middle, vec3(half * 2.0, 0.6, half * 2.0))
}

/// The near shore, where the stair comes down, and the far one the garden
/// stands on. Each as its own slab.
pub fn shores(reaches: f32) -> (Aabb, Aabb) {
    let middle = at(reaches);
    let half = BROAD * 0.5;
    let edge = ACROSS * 0.5;

    // **The near one is the far side of the water, in z.** The room is
    // turned end for end, per `glass::Placed`, which puts its stairwell at
    // the low-z end of it whatever you spin it about; you come down that
    // stair walking the way you already faced, which is -z. So the shore
    // you land on is beyond the water in +z and the garden is short of it,
    // and the whole place reads in the direction you are already going.
    (
        Aabb::new(
            vec3(middle.x - half, -DEEP, middle.z + edge),
            vec3(middle.x + half, 0.0, middle.z + edge + SHORE),
        ),
        Aabb::new(
            vec3(middle.x - half, -DEEP, middle.z - edge - half),
            vec3(middle.x + half, 0.0, middle.z - edge),
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
pub fn solid(reaches: f32, at_: Vec3) -> Vec<Aabb> {
    let (near, far) = shores(reaches);
    let place = placing(reaches);
    let mut out = vec![near, bed(reaches), far];

    // **The ground that follows you only follows you here.** It is a slab
    // three hundred metres across centred on wherever you are standing,
    // which is right out here and is a floor through the building at a
    // metre down everywhere else: `tests::the_clip_walks_the_whole_route`
    // found it in the cellar, two of the route's places walled off by a
    // sheet of Elysium. The rest of this list is fixed geometry six
    // hundred metres away and wants no such test.
    if holds(reaches, at_) {
        out.push(ground(at_));
    }

    out.extend(
        crate::glass::solid(reaches)
            .into_iter()
            .map(|box_| place.box_of(box_)),
    );

    // **And the colonnade, which you could walk through.** It was drawn and
    // not stood: fifteen columns you passed straight into, which is the one
    // fault that cannot be argued as a first build. The steps are here too,
    // because a platform you do not step up onto is a painting of one.
    //
    // The architrave is five metres up with nothing here that climbs, so it
    // is in this list for the same reason the walls are: what you see and
    // what stops you come off one list or they come apart.
    out.extend(stylobate(reaches));
    out.extend(architraves(reaches));

    let wide = COLUMN_WIDE + BASE_OUT * 2.0;

    for (n, (stood, upright)) in columns(reaches).into_iter().enumerate() {
        out.push(Aabb::from_center_size(
            stood + Vec3::Y * BASE_TALL * 0.5,
            vec3(wide, BASE_TALL, wide),
        ));

        match upright {
            true => out.push(Aabb::from_center_size(
                stood + Vec3::Y * (COLUMN_TALL * 0.5 + BASE_TALL * 0.5),
                vec3(COLUMN_WIDE, COLUMN_TALL - BASE_TALL, COLUMN_WIDE),
            )),
            false => out.extend(drums(stood, n)),
        }
    }

    out
}

/// Where you stand when you come through the glass, which is the copy of
/// spec 0064's room this place keeps.
///
/// **The same room, offset.** Every number in it is `glass`'s own, moved by
/// the distance between the two places, so a change to that room is a change
/// to this one and there is no second account of it to drift.
pub fn placing(reaches: f32) -> crate::glass::Placed {
    let foot = crate::glass::stair_foot(reaches);
    let (near, _) = shores(reaches);

    // Twice the axis rather than a distance, per `glass::Placed`: the two
    // numbers are what carry the room's stair foot onto the near shore's
    // own edge, and the turn does the rest.
    crate::glass::Placed {
        turned: true,
        shift: vec3(foot.x * 2.0, 0.0, near.max.z + foot.z),
    }
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
pub fn through(reaches: f32, from: Vec3, facing: Vec3) -> Option<Vec3> {
    let place = placing(reaches);
    let (here, radius) = crate::glass::ball(reaches);

    for middle in [here, place.of(here)] {
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
            return Some(middle);
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
///
/// **Half the crossing and not all of it.** Per [`fold`], the arc runs
/// nought to two: this is what each half takes, so walking into the glass
/// and standing in the far room with it square again is twice this.
pub const THROUGH: f32 = 0.7;

pub const SWALLOWS: f32 = 2.4;

/// What the ball's index is, a share `through` of the way into the
/// crossing.
///
/// Eased so the first part of the push does little and the last of it does
/// most: leaning on glass that gives way at once is a door, and this is
/// meant to read as something resisting and then not.
///
/// **Squared and not cubed.** A cube spends the first two thirds of the
/// crossing doing almost nothing, and with the fold now starting the
/// moment you walk at the glass rather than when the plinth stops you,
/// that dead stretch is the whole of what you see first.
pub fn bending(through: f32) -> f32 {
    let eased = through.clamp(0.0, 1.0).powi(2);

    crate::glass::INDEX + (SWALLOWS - crate::glass::INDEX) * eased
}

/// Where in the fold a crossing stands, given how far it has run.
///
/// **The crossing is an arc and not a ramp.** It ran nought to one and the
/// move happened at the end of it, so the room you left folded up over
/// most of a second and the room you arrived in was simply there, square
/// and unbent. Jake, on his own run: the room i move into snaps into
/// place, not a good transition, a sense of space warping.
///
/// So it runs nought to two. One is the top, where the move happens and
/// the glass has closed over you; the second half is the same fold read
/// backwards, with the copy's own ball in front of you at full size
/// shrinking back onto its plinth while the half turn swings the stair
/// into view. What you see is one gesture that carries you through, and
/// not a wind-up and a cut.
pub fn fold(through: f32) -> f32 {
    if through <= 1.0 {
        through.max(0.0)
    } else {
        (2.0 - through).max(0.0)
    }
}

/// How much wider the ball is drawn, a share `through` of the way across.
///
/// **The bend alone is not being swallowed.** Winding the index bows the
/// room and fringes it, which is a lens doing lens things: Jake, watching
/// his own run, no wormhole or blackhole like visual transformation. What
/// a hole does is take you in, so the ball grows until it is around you.
///
/// **Stopped just short of closing over you.** At 4.5 the sphere passed
/// the eye with a quarter of the crossing still to run, and inside it its
/// far side is wound away and culled: the last stills of that fold were a
/// flat brown frame with the room gone out of it. Three is the width at
/// which the bent room wraps the whole view and the glass is still in
/// front of you, which is the picture the fold should end on.
pub const SWELLS: f32 = 3.0;

/// How far the glass has stopped letting the room through, a share of the
/// way across the fold.
///
/// **The hole is the glass going out, and not a ball inside it.** The
/// first go at this pushed an opaque black sphere at the ball's own
/// middle. Concentric and smaller, the lens's own surface is nearer the
/// eye and hides it; bigger, it covers the bending it was meant to be the
/// middle of. Either way it is a sphere in front of a sphere.
///
/// So the glass itself darkens. The room goes on being bent and wrapped
/// round you by the index and the swell, and then the thing doing the
/// bending stops passing any of it: a bright bent rim closing on a middle
/// with nothing in it, which is what Jake asked for when he said rotation
/// is an interesting transition, but not space warping that evokes the
/// feeling of a blackhole or a wormhole.
///
/// **Cubed, so it goes out last.** The lens grows on a square. Straight,
/// the dark ran ahead of the glass and the first half of the crossing was
/// black before anything had bent.
pub fn darkens(fold: f32) -> f32 {
    fold.clamp(0.0, 1.0).powi(3)
}

/// How far the whole frame is covered, a share of the way across the fold.
///
/// **Because the hole is in front of you and the turn is not.** The glass
/// darkening is the hole, and it covers whatever the ball covers; the half
/// turn at the top of the arc needs the frame itself covered, or you watch
/// the room swing. Jake, having watched it: after the sphere animation
/// there's still a rotation towards the stairs on the other side, they
/// should already be there.
///
/// [`darkens`] squared, so this is nothing for most of the crossing and
/// the hole does the work, and seals only at the last of it. Read twice
/// off the one arc: it shuts on the way in and opens on the way out, with
/// the move and the turn in the dark between.
pub fn seals(fold: f32) -> f32 {
    darkens(fold).powi(2)
}

/// The multiplier itself, squared like [`bending`] so the two move
/// together and the growing is the same gesture as the bending.
pub fn swelling(through: f32) -> f32 {
    1.0 + (SWELLS - 1.0) * through.clamp(0.0, 1.0).powi(2)
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

    // **Still a box, and a box has an edge you can walk out of.** This and
    // `ground` were the same 150 metres, so the edge of what held you and
    // the edge of what held you up were one line, and a step over it put
    // you in the arcade's field at night. The ground follows you now,
    // which takes the drop away, and the line is still there.
    //
    // Making this everything past a line instead was tried and reverted.
    // The field is unbounded in z on purpose and its own walks go out to
    // z 1019: `tests::you_cannot_walk_behind_the_building` and
    // `tests::the_clip_walks_the_whole_route` both failed, naming the
    // spot, because a half plane here takes the field's wall away out
    // there. Two unbounded worlds cannot share a direction, so the
    // boundary has to be handled rather than removed, and which way is a
    // question about this place and not about arithmetic.
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
    // **And over the room, which this used to stop short of.** The far
    // edge was the shore's, and the copy stands back from the shore: the
    // strip of ground under it was open for the field to sow, and a trunk
    // there has a room on top of it. It was the same gap before the copy
    // was turned, on the other side of the shore.
    let room = placing(reaches).box_of(crate::glass::inside(reaches));
    let back = room.max.z.max(near.max.z);

    (at_.x - middle.x).abs() < BROAD * 0.5 + CLEARS
        && at_.z > far.min.z - CLEARS
        && at_.z < back + CLEARS
}

/// How near you have to be for any of it to be drawn.
///
/// **Six hundred metres away, so never from the building.** Nothing here is
/// ever in the same frame as anything there, which is the whole arrangement,
/// so drawing it while you are in the arcade is a bill for pixels nobody can
/// see. `glass::SEES` does the same for the ball.
pub const SEES: f32 = 320.0;

/// Whether this place is being drawn at all, from where the eye is.
///
/// **Asked once.** The drawing gates on [`SEES`], and the water's surface
/// has to be stepped and written on exactly the frames it is drawn on or
/// you arrive at a sheet that stopped moving somewhere else. Those were
/// two conditions, this one and `holds`, which agree over most of the
/// ground and not all of it: two lists of one thing drift.
pub fn drawn(reaches: f32, eye: Vec3) -> bool {
    eye.distance(at(reaches)) <= SEES
}

/// The sand and the planes it is laid on, where the renderer has them.
///
/// Its own struct for `glass::Meshes`'s reason: one more parameter on a
/// draw that is already at the count clippy will take.
#[derive(Clone, Copy)]
pub struct Ground {
    pub sand: blitzkit::renderer::scene::TextureId,
    /// One plane apiece, near shore then far, each with its own tile count.
    pub shores: [blitzkit::renderer::scene::MeshId; 2],
    /// The water's own surface, which the renderer rebuilds every frame.
    pub sea: Option<blitzkit::renderer::scene::MeshId>,
    /// `field::hill_mesh`, which this borrows and tints its own way.
    pub hill: Option<blitzkit::renderer::scene::MeshId>,
    /// And the ground beyond the shores, which follows you and so is the
    /// sand you are standing on most of the time you are out here.
    pub beyond: Option<blitzkit::renderer::scene::MeshId>,
}

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
    fold: crate::glass::Fold,
    ground_: Option<Ground>,
    meshes: crate::glass::Meshes,
) {
    use blitzkit::mesh::Transform;

    if !drawn(reaches, eye) {
        return;
    }

    let cube = meshes.cube;
    let column = meshes.column;
    let (near, far) = shores(reaches);

    // the horizon first, because it stands behind everything here and a
    // thing at infinity is drawn before the things in front of it
    if let Some(hill) = ground_.and_then(|g| g.hill) {
        for (stood, wide, high) in horizon(eye) {
            scene.push_material(
                hill,
                &Transform::at(stood).with_scale(vec3(wide, high, wide)),
                crate::aim::ELYSIUM_HILL,
                crate::aim::DULL,
            );
        }
    }

    // **The slabs sink a hair and the sand is laid on their true top.**
    // A textured plane at the same height as the box's own face is two
    // exposed faces in one plane, which is the fault this building has
    // found seven times. Dropping the box instead of lifting the plane
    // keeps the ground you walk on where `solid` says it is.
    let sunk = match ground_ {
        Some(_) => 0.03,
        None => 0.0,
    };

    for piece in [ground(eye), bed(reaches)] {
        scene.push_material(
            cube,
            &Transform::at(piece.center()).with_scale(piece.size()),
            crate::aim::SHORE,
            crate::aim::MATTE,
        );
    }

    // and the sand on the ground that follows you, which is what you walk
    // on everywhere but the two shores
    if let (Some(ground_), Some(plane)) = (ground_, ground_.and_then(|g| g.beyond)) {
        let piece = ground(eye);
        let size = piece.size();

        scene.push_textured(
            plane,
            ground_.sand,
            &Transform::at(vec3(
                piece.center().x,
                piece.max.y + 0.004,
                piece.center().z,
            ))
            .with_scale(vec3(size.x, 1.0, size.z)),
            crate::aim::SHORE,
            crate::aim::VOID,
        );
    }

    for piece in [near, far] {
        let size = piece.size();

        scene.push_material(
            cube,
            &Transform::at(piece.center() - Vec3::Y * sunk).with_scale(size),
            crate::aim::SHORE,
            crate::aim::MATTE,
        );
    }

    // the wet band each side of it, laid on the sand
    for strip in waterline(reaches) {
        scene.push_material(
            cube,
            &Transform::at(strip.center()).with_scale(strip.size()),
            crate::aim::WET_SAND,
            crate::aim::MATTE,
        );
    }

    if let Some(ground_) = ground_ {
        for (plane, piece) in ground_.shores.iter().copied().zip([near, far]) {
            let size = piece.size();

            scene.push_textured(
                plane,
                ground_.sand,
                &Transform::at(vec3(piece.center().x, piece.max.y, piece.center().z))
                    .with_scale(vec3(size.x, 1.0, size.z)),
                crate::aim::SHORE,
                // no highlight, for the field ground's reason: a broad one
                // on a hundred and fifty metres of sand with the sun low is
                // a lit path down the middle that turns with you.
                crate::aim::VOID,
            );
        }
    }

    crate::glass::draw(scene, reaches, since, fold, placing(reaches), meshes);

    // and the water over the bed, which is the only thing here that is not
    // a thing you stand on. Drawn from `sea`'s own surface where the
    // renderer has it, per spec 0103: a quad is a lid.
    match ground_.and_then(|g| g.sea) {
        Some(mesh) => scene.push_material(
            mesh,
            &Transform::at(Vec3::ZERO),
            crate::aim::SHALLOWS,
            crate::aim::WET,
        ),
        None => {
            let (surface, size) = water(reaches);

            scene.push_material(
                cube,
                &Transform::at(surface).with_scale(vec3(size.x, 0.02, size.y)),
                crate::aim::SHALLOWS,
                crate::aim::MATTE,
            );
        }
    }

    // and the colonnade on the far shore, which is the garden
    let Some(column) = column else {
        return;
    };

    let block = |scene: &mut blitzkit::renderer::scene::Scene, box_: Aabb, tint| {
        scene.push_material(
            cube,
            &Transform::at(box_.center()).with_scale(box_.size()),
            tint,
            crate::aim::MATTE,
        );
    };

    // the steps it is set on, then what stands on them
    for step in stylobate(reaches) {
        block(scene, step, crate::aim::RUIN_SHADE);
    }

    for (n, (stood, upright)) in columns(reaches).into_iter().enumerate() {
        // **The base is there either way.** A fallen column leaves its
        // footing: a bare patch in the run says nothing fell, it says
        // somebody forgot to draw one.
        let wide = COLUMN_WIDE + BASE_OUT * 2.0;

        block(
            scene,
            Aabb::from_center_size(
                stood + Vec3::Y * BASE_TALL * 0.5,
                vec3(wide, BASE_TALL, wide),
            ),
            crate::aim::RUIN_SHADE,
        );

        if !upright {
            for drum in drums(stood, n) {
                scene.push_material(
                    column,
                    &Transform::at(drum.center())
                        .with_rotation(glam::Quat::from_rotation_z(std::f32::consts::FRAC_PI_2))
                        .with_scale(vec3(COLUMN_WIDE, drum.size().x, COLUMN_WIDE)),
                    crate::aim::RUIN_SHADE,
                    crate::aim::MATTE,
                );
            }

            continue;
        }

        let shaft = COLUMN_TALL - BASE_TALL - CAPITAL_TALL;

        scene.push_material(
            column,
            &Transform::at(stood + Vec3::Y * (BASE_TALL + shaft * 0.5)).with_scale(vec3(
                COLUMN_WIDE,
                shaft,
                COLUMN_WIDE,
            )),
            crate::aim::RUIN,
            // no highlight on a turned shaft, for `glass::draw_the_room`'s
            // reason: a point lamp on a cylinder bands every one of them at
            // its own height.
            crate::aim::VOID,
        );

        let capital = COLUMN_WIDE + CAPITAL_OUT * 2.0;

        block(
            scene,
            Aabb::from_center_size(
                stood + Vec3::Y * (COLUMN_TALL - CAPITAL_TALL * 0.5),
                vec3(capital, CAPITAL_TALL, capital),
            ),
            crate::aim::RUIN,
        );
    }

    // and what it still carries across the top
    for span in architraves(reaches) {
        block(scene, span, crate::aim::RUIN);
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

/// What a column is made of, which is four things and not one.
///
/// **A shaft on its own is a post.** The first build stood fifteen bare
/// cylinders of the shore's own colour on the shore's own sand, and from
/// the water they read as a fence: nothing under them to say they were
/// set, nothing over them to say they carried anything. Jake, looking at
/// that from halfway across: the other side of the sphere needs filling
/// out.
///
/// These are the pieces the eye reads a column by. The shaft keeps the
/// lathe; the rest are blocks, which is what they are in stone.
pub const BASE_TALL: f32 = 0.42;
pub const BASE_OUT: f32 = 0.17;
pub const CAPITAL_TALL: f32 = 0.40;
pub const CAPITAL_OUT: f32 = 0.23;

/// The stepped platform the whole run is set on.
///
/// **Because a building begins below the floor you walk on.** A column
/// rising straight out of sand has been dropped there. Three steps say
/// somebody levelled this ground first, and they give the run a line along
/// its foot that reads from across the water, where a column at that
/// distance is four pixels wide and its footing is the only thing with any
/// length to it.
pub const STEPS: usize = 3;
pub const STEP_RISE: f32 = 0.26;
pub const STEP_TREAD: f32 = 0.52;

/// What survives across the top, where anything does.
///
/// **Some of it and not all.** A full entablature is a building in repair
/// and this one is not; none of it is a row of posts again. What carries
/// is a handful of runs with gaps torn in them, which is the shape that
/// says there was once a roof here without claiming there still is.
pub const ARCHITRAVE_TALL: f32 = 0.74;
pub const ARCHITRAVE_DEEP: f32 = 1.12;

/// How many pieces a fallen column lies in.
///
/// **It broke, it did not topple.** A whole cylinder on the ground is a
/// pipe somebody put down, which is the note already in this file; drums
/// with gaps between them is a column that came apart where its courses
/// were.
pub const DRUMS: usize = 5;

/// How wide the wet strip is where each shore meets the water.
pub const WETTED: f32 = 1.3;

/// Those two strips, one along each shore's water edge.
///
/// **Because a shore is not an edge.** Sand meeting water on a ruled line
/// is a floor plan; what says water has been here is a band of darker
/// ground beside it that is not quite the same shape as the water. This
/// is drawn flat and untextured on purpose: wet sand is smoother than dry
/// and takes the grain out of it, so the one place here that wants no
/// texture is this.
pub fn waterline(reaches: f32) -> [Aabb; 2] {
    let (near, far) = shores(reaches);
    // proud by a hair, per the note on the sand: a strip at the shore's own
    // height is two exposed faces in one plane
    let up = 0.006;

    [
        Aabb::new(
            vec3(near.min.x, near.max.y, near.min.z),
            vec3(near.max.x, near.max.y + up, near.min.z + WETTED),
        ),
        Aabb::new(
            vec3(far.min.x, far.max.y, far.max.z - WETTED),
            vec3(far.max.x, far.max.y + up, far.max.z),
        ),
    ]
}

/// The land all the way round, standing where the sky meets the ground.
///
/// **Because the world ended in a ruled line.** The two shores and the
/// ground under them are slabs, and past the last of them the sand simply
/// stopped and the sky began: from the colonnade you could see the edge of
/// the place. The field solved this long ago and this is its solution, not
/// a second one.
///
/// **Round the eye and not on the ground**, per `field::horizon`: a hill
/// you can walk to is a hill that wants a collider and stops being a
/// horizon. These keep their bearing however far you go and never arrive.
///
/// **Its own ring and not the field's.** That one is `aim::FAR_HILL`, a
/// night blue, and a ninth of it is towers, which is a city at the edge of
/// a field at night. This place is a morning with nobody in it: hills
/// only, lower, wider, and pale enough to read as distance rather than as
/// shapes.
///
/// Off `field::sees` for how far out, because that is the number the sky's
/// own dome is counted from and a range outside the sky is drawn nowhere
/// at all. `field::horizon`'s note has what that cost to find out.
pub const HORIZON_MANY: usize = 56;
pub const HORIZON_OUT: f32 = 0.35;
pub const HILL_HIGH: f32 = 0.058;
pub const HILL_WIDE: f32 = 0.54;

/// Where each stands, how wide it is and how high.
pub fn horizon(at: Vec3) -> Vec<(Vec3, f32, f32)> {
    let out = crate::field::sees(at) * HORIZON_OUT;
    let mut rng: u32 = 0x9E37_79B1;
    let mut next = move || {
        rng ^= rng << 13;
        rng ^= rng >> 17;
        rng ^= rng << 5;
        (rng % 10_000) as f32 / 10_000.0
    };

    (0..HORIZON_MANY)
        .map(|n| {
            let bearing = n as f32 / HORIZON_MANY as f32 * std::f32::consts::TAU;
            // jittered off its own bearing, or the ring reads as a cog
            let aside = (next() - 0.5) * std::f32::consts::TAU / HORIZON_MANY as f32 * 1.7;
            let turn = bearing + aside;
            let size = 0.5 + next() * 0.5;
            // and standing nearer or further, so the range has depth.
            //
            // **Under the field's own ceiling and not level with it.** Its
            // spread tops out at 1.12 and so did this, which is the same
            // ceiling reached by different rolls: the first ring drawn this
            // way put a hill 164.5 out against the field's furthest at
            // 164.3, and `tests::the_horizon_stands_no_further_out_than_the
            // _fields` caught it on its first run.
            let far = out * (0.80 + next() * 0.26);

            (
                vec3(at.x + turn.cos() * far, 0.0, at.z + turn.sin() * far),
                out * HILL_WIDE * size,
                out * HILL_HIGH * size,
            )
        })
        .collect()
}

/// The water, as the engine simulates it.
///
/// **One account of it.** [`water`] says where the still surface is and
/// how far it reaches, which is what `wading` and the tests ask; this is
/// that, handed to `blitzkit::water::Water` so the surface has a shape.
/// It was one flat quad of a single colour, which from the shore was a
/// sheet of painted glass with a ruled edge.
///
/// **Sixty-four cells along a hundred and fifty metres**, which is a step
/// of a little over two. That is coarse for a wave and right for this:
/// spec 0133 calls this water a distance rather than an object, you are
/// never looking into it, and the fine detail of a surface is the chop,
/// which shades rather than moves. The pond spends seventy-two cells on
/// thirty metres because you stand over it.
pub const SEA_CELLS: u32 = 64;
/// Slower and longer than the pond's, which is what a width of water does
/// against a basin: `pond::RUNS` is 1.9 over thirty metres.
pub const SEA_RUNS: f32 = 2.6;
pub const SEA_SETTLES: f32 = 0.62;
/// Rougher than the pond's `CALM` and short of the engine's own `CHOP`,
/// because this is open and shallow and the sun is on it.
pub const SEA_CHOP: f32 = 0.52;

pub fn sea(reaches: f32) -> blitzkit::water::Water {
    let (middle, size) = water(reaches);
    let mut sea = blitzkit::water::Water::new(middle, size, DEEP, SEA_CELLS);

    sea.damping = SEA_SETTLES;
    sea.speed = SEA_RUNS;
    sea.chop = SEA_CHOP;

    sea
}

/// How big one tile of the shore's sand is, and the texture's own size.
///
/// **Because a cube's face wears one tile however big the face is.** The
/// two shores are boxes 150 metres by 75 and by 6, drawn as boxes, so the
/// whole of each was a single untextured span: the flattest thing in the
/// frame, and the reason a hard straight line is all that separated the
/// sand from the water. Laid as a plane with its own tile count, per
/// `room::tiled_plane`, a metre and a half of sand is a metre and a half
/// of sand wherever it is.
///
/// **Four metres, which is `field::TILE`'s number for `field::TILE`'s
/// reason.** At 1.6 the shore wore ninety-odd copies of one square across
/// its width and read as a grid: a tiling texture shows its tile at the
/// distance where one tile is a few pixels, and the only cure that costs
/// nothing is fewer of them.
pub const SAND: u32 = 128;
pub const SAND_TILE: f32 = 4.0;

/// Sand, as a tiling texture.
///
/// **Grains and not strokes.** `field::turf` lays short strokes every way
/// at once because that is what grass does to the light; sand has no
/// direction, so this is specks over a ground with a little drift of
/// darker and lighter in patches, which is what keeps a tiled plane from
/// reading as a grid at a distance.
pub fn sand(seed: u32) -> blitzkit::texture::TextureData {
    let mut rng = seed | 1;
    let mut next = move || {
        rng ^= rng << 13;
        rng ^= rng >> 17;
        rng ^= rng << 5;
        rng
    };
    let roll = |n: u32| (n % 10_000) as f32 / 10_000.0;

    let under = [206u8, 196, 176];
    let mut pixels = Vec::with_capacity((SAND * SAND * 4) as usize);

    // the ground, with a slow drift over it so the tile has large shapes in
    // it as well as small ones. Two sines of different periods, which do not
    // repeat against each other inside one tile but do at its edge.
    for n in 0..SAND * SAND {
        let (x, z) = ((n % SAND) as f32, (n / SAND) as f32);
        let turn = std::f32::consts::TAU / SAND as f32;
        let drift = ((x * turn * 2.0).sin() * (z * turn).cos()
            + (x * turn).cos() * (z * turn * 3.0).sin())
            * 0.014;

        for one in under {
            pixels.push(((one as f32) * (1.0 + drift)).clamp(0.0, 255.0) as u8);
        }

        pixels.push(255);
    }

    // and the grains
    for _ in 0..SAND * SAND / 5 {
        let x = (roll(next()) * SAND as f32) as u32;
        let z = (roll(next()) * SAND as f32) as u32;
        let lit = 0.84 + roll(next()) * 0.3;
        let at = ((z.min(SAND - 1) * SAND + x.min(SAND - 1)) * 4) as usize;

        for (n, one) in under.iter().enumerate() {
            pixels[at + n] = ((*one as f32) * lit).clamp(0.0, 255.0) as u8;
        }
    }

    blitzkit::texture::TextureData::from_pixels(SAND, SAND, pixels)
}

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
/// How far behind the front row the second one stands, and how many of it
/// there are.
///
/// **Because one row is a fence however well it is built.** With the
/// columns given their bases and capitals the run read as architecture and
/// still had nothing behind it: between every pair you saw sand to the
/// horizon, so the whole far side was one line deep. A second row two bays
/// back puts a standing column in most of those gaps, and walking changes
/// which, which is the only thing that says depth without saying it.
///
/// Two bays and not one. At one the two rows read as a thick wall; at two
/// there is room to be between them, which is what a peristyle is for.
pub const ROWS_BACK: f32 = COLUMN_APART * 2.0;

/// Which of the back row are standing.
///
/// **Offset from the front's gaps on purpose.** The front is down at 3, 4,
/// 9 and 12; if the back were down in the same places you would see
/// through both at once and the second row would have bought nothing. What
/// it is down at instead is a different run of the same shape.
pub fn behind_standing(n: usize) -> bool {
    !matches!(n, 1 | 6 | 7 | 13)
}

/// The back row itself, set out like the front and two bays behind it.
pub fn behind(reaches: f32) -> Vec<(Vec3, bool)> {
    colonnade(reaches)
        .into_iter()
        .enumerate()
        .map(|(n, (stood, _))| {
            (
                vec3(stood.x, stood.y, stood.z - ROWS_BACK),
                behind_standing(n),
            )
        })
        .collect()
}

/// Both rows, which is what everything that draws or stands on them wants.
pub fn columns(reaches: f32) -> Vec<(Vec3, bool)> {
    let mut out = colonnade(reaches);

    out.extend(behind(reaches));
    out
}

/// How high the colonnade's own floor stands over the shore.
///
/// Named once because the steps, the columns, the fallen drums and the
/// boxes you walk into all need it, and five places saying `STEPS as f32 *
/// STEP_RISE` is five places to get it wrong one at a time.
pub fn stands_on() -> f32 {
    STEPS as f32 * STEP_RISE
}

/// The stepped platform, widest at the bottom.
///
/// Each step oversails the one above by [`STEP_TREAD`] on every side, so
/// the run has a moulded foot rather than an edge.
pub fn stylobate(reaches: f32) -> Vec<Aabb> {
    let stood = colonnade(reaches);
    let (first, last) = (stood[0].0, stood[COLUMNS - 1].0);
    let half = COLUMN_WIDE * 0.5 + BASE_OUT + STEP_TREAD;

    (0..STEPS)
        .map(|n| {
            // the bottom step is the widest, so the count runs the other way
            let out = (STEPS - n) as f32 * STEP_TREAD;
            let low = n as f32 * STEP_RISE;

            Aabb::new(
                // back far enough for the second row, which stands on this
                // too: a platform that stopped at the front row would leave
                // the row behind it rising out of sand
                vec3(
                    first.x - half - out,
                    low - 0.4,
                    first.z - half - out - ROWS_BACK,
                ),
                vec3(last.x + half + out, low + STEP_RISE, last.z + half + out),
            )
        })
        .collect()
}

/// Whether the run still carries its architrave over the bay that starts
/// at column `n`.
///
/// **Only where both ends are up**, which is arithmetic rather than taste,
/// and then not over all of those: the gaps are what keep it from reading
/// as a building in repair.
///
/// **One long stretch and two fragments, not three goalposts.** The first
/// pick took out bay 6, which is the middle of the only four-column run
/// `standing` leaves: that split the one surviving piece into two single
/// bays, and a lintel on two columns on its own is a gateway. Seen from
/// the water it was three little arches in a row. The run 5 to 8 carries
/// whole now, and what is dropped is at the ends, where a single bay reads
/// as the last of something rather than as a thing in itself.
pub fn carries(n: usize) -> bool {
    standing(n) && standing(n + 1) && !matches!(n, 1 | 10)
}

/// What survives across the top of the run, bay by bay.
pub fn architraves(reaches: f32) -> Vec<Aabb> {
    let stood = colonnade(reaches);
    let deep = ARCHITRAVE_DEEP * 0.5;
    let top = stands_on() + COLUMN_TALL;

    (0..COLUMNS - 1)
        .filter(|n| carries(*n))
        .map(|n| {
            let (a, b) = (stood[n].0, stood[n + 1].0);

            Aabb::new(
                vec3(a.x, top, a.z - deep),
                vec3(b.x, top + ARCHITRAVE_TALL, b.z + deep),
            )
        })
        .collect()
}

/// A fallen column, lying along the run in pieces.
///
/// **Deterministic and not random.** The same stones are in the same
/// places every run, which is what makes a ruin a place rather than a
/// scatter: `n` is which column it was, and the two of them together pick
/// the drift off a short table.
pub fn drums(foot: Vec3, n: usize) -> Vec<Aabb> {
    let shaft = COLUMN_TALL - BASE_TALL - CAPITAL_TALL;
    let long = shaft / DRUMS as f32;
    let radius = COLUMN_WIDE * 0.5;
    // how far each drum rolled from where it would have landed, and how far
    // it strayed across the run. A line of touching cylinders is a pipe cut
    // into lengths; these fell.
    let drift = [0.0, 0.14, -0.09, 0.26, 0.05, -0.17];
    let stray = [0.0, -0.11, 0.18, -0.06, 0.13, 0.08];

    (0..DRUMS)
        .map(|k| {
            let step = (k as f32 + 0.5) * long * 1.1;
            let along = foot.x + radius + step + drift[(n + k) % drift.len()] * step;
            let across = foot.z + stray[(n * 2 + k) % stray.len()];
            // sunk, per the note on a cylinder laid on open ground
            let middle = vec3(along, stands_on() + radius * 0.62, across);
            let half = vec3(long * 0.5, radius, radius);

            Aabb::new(middle - half, middle + half)
        })
        .collect()
}

pub fn colonnade(reaches: f32) -> Vec<(Vec3, bool)> {
    let middle = at(reaches);
    let (_, far) = shores(reaches);
    let run = (COLUMNS - 1) as f32 * COLUMN_APART;

    (0..COLUMNS)
        .map(|n| {
            let along = n as f32 * COLUMN_APART - run * 0.5;

            (
                vec3(middle.x + along, stands_on(), far.max.z - COLONNADE_BACK),
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

        assert!(far.max.z <= near.min.z, "the shores meet without water");
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
    /// the world at once and where you are says which you are in. The way
    /// home is the way out read again: `Placed::of` is its own inverse, so
    /// if that ever stopped being true the door would strand somebody.
    ///
    /// The copy is turned, so arriving leaves its ball behind you and the
    /// stair ahead. Going back is walking at the ball you came out of,
    /// which is why the second look here faces the other way.
    #[test]
    fn the_glass_takes_you_through_and_back() {
        let reaches = reaches();
        let (here, _) = crate::glass::ball(reaches);
        let place = placing(reaches);
        let at = vec3(here.x, here.y, here.z + 1.0);
        let into = (here - at).normalize();

        let there = through(reaches, at, into).expect("the building's glass is a door");

        assert!(
            (there - here).length() < 1e-3,
            "the building's own ball is not what it found"
        );

        let over = place.of(at);
        let back = through(reaches, over, -into).expect("the copy's glass is a door too");

        assert!(
            (back - place.of(here)).length() < 1e-3,
            "the copy's ball is not what it found from {:?}",
            over
        );
        assert!(
            place.of(over).distance(at) < 1e-3,
            "going and coming back lands at {:?} rather than {:?}",
            place.of(over),
            at
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
        let foot = placing(reaches).of(crate::glass::stair_foot(reaches));
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
                at_.z < middle.z - size.y * 0.5,
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

        // and the ground the room stands on, which this stopped short of
        let room = placing(reaches).box_of(crate::glass::inside(reaches));

        for (which, z) in [("the stair's end", room.min.z), ("the far end", room.max.z)] {
            let spot = vec3(middle.x, 0.0, z);

            assert!(
                shades(reaches, spot),
                "the field may sow a tree at {}, {:?}, which is under the room",
                which,
                spot
            );
        }

        // and it stops: a clearing the size of the field is a field with no
        // trees in it
        let away = vec3(middle.x, 0.0, far.min.z - CLEARS - 20.0);

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
        let foot = placing(reaches).of(crate::glass::stair_foot(reaches));
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
            foot.z > middle.z + size.y * 0.5,
            "the stair lands in the water"
        );
        assert!(
            (foot.z - crate::glass::stair_foot(reaches).z).abs() > crate::field::GROUND_REACH,
            "this place is inside the field's own reach"
        );
    }

    /// This place's horizon stands no further out than the field's.
    ///
    /// **Which is how it stays inside the sky.** A backdrop is hung at a
    /// share of the far plane and a range put outside it is drawn nowhere
    /// at all, which looks exactly like a range that was never pushed:
    /// `field::horizon`'s note says that took four pictures to work out.
    /// The field's ring is held inside the dome by a test of its own, so
    /// the cheapest way for this one to be safe is to be no further out
    /// than that, measured off both functions rather than off the two
    /// constants agreeing.
    #[test]
    fn the_horizon_stands_no_further_out_than_the_fields() {
        let eye = at(reaches()) + Vec3::Y * 1.55;
        let furthest = |ds: Vec<f32>| ds.into_iter().fold(0.0f32, f32::max);

        let mine = furthest(
            horizon(eye)
                .into_iter()
                .map(|(stood, _, _)| stood.distance(vec3(eye.x, 0.0, eye.z)))
                .collect(),
        );
        let theirs = furthest(
            crate::field::horizon(eye)
                .into_iter()
                .map(|far| far.at.distance(vec3(eye.x, 0.0, eye.z)))
                .collect(),
        );

        assert!(
            mine <= theirs + 1e-3,
            "this place's furthest hill is {:.1} out and the field's is \
             {:.1}, so this one may be behind the sky",
            mine,
            theirs
        );
    }

    /// The two rows are not down in the same places.
    ///
    /// **Which is the whole of what the second row buys.** It is there so
    /// that the gaps in the front one have something standing behind them:
    /// a back row down wherever the front is down would be a second fence
    /// behind the first, visible from nowhere, and the far side would read
    /// one line deep exactly as it did with one row.
    ///
    /// Asked of every bay rather than of the two lists, because the lists
    /// are picked by hand and the point is what the pair of them leaves.
    #[test]
    fn you_cannot_see_through_both_rows_at_once() {
        for n in 0..COLUMNS {
            assert!(
                standing(n) || behind_standing(n),
                "column {} is down in both rows, so that bay is open to the \
                 horizon and the back row bought nothing there",
                n
            );
        }

        // and the second row is a row, not a copy: some of it is down, or
        // it is a wall behind a ruin
        let down = (0..COLUMNS).filter(|n| !behind_standing(*n)).count();

        assert!(down > 0, "nothing in the back row has fallen");
        assert!(down < COLUMNS / 2, "more of the back row is down than up");
    }

    /// The ball is a mirror with a room in it until you walk into it.
    ///
    /// **What the darkening must not do is darken.** It is the lens's own
    /// tint falling away, and that tint is what the ball wears standing on
    /// its plinth with nobody near it: a share that is anything but nought
    /// at rest is spec 0064's room permanently dimmed to buy spec 0133 an
    /// effect.
    ///
    /// The second half is that it goes out late. Straight rather than
    /// cubed, the glass was half gone by the middle of the crossing and
    /// the bending it is the middle of was never seen.
    #[test]
    fn the_glass_is_itself_until_you_walk_into_it() {
        assert_eq!(darkens(0.0), 0.0, "the ball at rest is not its own tint");
        assert_eq!(darkens(1.0), 1.0, "the glass never goes out at all");

        let half = darkens(0.5);

        assert!(
            half < 0.2,
            "the glass is {:.2} gone halfway across, which is before the \
             bending it should be the middle of",
            half
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
