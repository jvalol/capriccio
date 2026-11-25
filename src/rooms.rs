//! Which rooms this building photographs, and from where. Spec 0135.
//!
//! **One pose, read twice.** A room's camera is named here and nothing
//! else names one: the picture test that holds the room against its
//! reference reads this, and so does the staged shot that goes on the
//! site. Two lists of one thing drift, and this building has written that
//! down under a dozen names; what it would mean here is a published
//! picture of a room nobody tested, taken from a spot nobody argued about.
//!
//! **Places you stand in**, which is what decides the list. What is behind
//! the wall is nearly black on purpose and the clock and the readout are
//! instrument panels: those are framed to make a test fail cheaply, which
//! is a different job. The village, the field, the horizon and the
//! steading are out of doors.
//!
//! The reasoning for each pose sits with it below, because most of them
//! cost a round or two to arrive at and the reason a camera stands where
//! it does is the part a later hand needs.

use glam::{vec3, Vec3};

use crate::{back, cellar, garden, glass, mandel, mirrors, pond, room, spa, tesseract, valhalla};

/// A room, the argument that stages it, and the file its picture lands in.
pub struct Shot {
    /// What `--screenshot` takes, and the name of the reference under
    /// `tests/rooms`.
    pub slug: &'static str,
    /// Where the picture is published, relative to this repo.
    ///
    /// The hall's is `media/screenshot.png` rather than `media/hall.png`,
    /// which is spec 0121's doing and is left alone: that file is what the
    /// site and both readmes already point at.
    pub file: &'static str,
    /// Where the camera stands and what it looks at, or `None` for a room
    /// that poses itself.
    ///
    /// The tesseract is the one of those. It is nowhere in the building's
    /// coordinates, so its slug sets a mode rather than a camera and
    /// `tesseract::posed` is read where that mode is set up.
    pub pose: Option<(Vec3, Vec3)>,
    /// The hour and the sky it is photographed under, for a room with a
    /// sky in it, or `None` for one that is lit from inside.
    ///
    /// **Because a day here is twenty seconds.** A run that stages a shot
    /// takes a few of them, so the sun moves while the photographer is
    /// standing there, and a run that names no hour opens at nought,
    /// which is dawn on day one and day one is a storm. The garden and
    /// the pond came out as two dark pictures of a wet dawn.
    pub when: Option<(f32, &'static str)>,
    /// Whether the room is only reachable with the trail answered.
    ///
    /// Valhalla is. A run staging it has to open as though `--done` were
    /// given, or the photographer stands in a hall that the bridge has not
    /// risen to yet.
    pub at_the_end: bool,
}

/// How high the eye stands over the feet, repeated from `main` so this
/// module can be read on its own.
///
/// It is `crate::EYE` and must stay it; the constant is private to the
/// binary's root and this is a module of that binary, so this is the same
/// number rather than a second one.
fn eye() -> f32 {
    crate::EYE
}

/// What hour a room with a sky in it is photographed at, and under which
/// sky.
///
/// **The village's own numbers, named once.** Spec 0071 picked them for
/// the one outdoor shot this building had, and the garden and the pond
/// are the same question asked again: the published pictures of this
/// building's outdoors should agree about what time of day it is, so
/// this is that number rather than a second one that happens to match.
pub const OUTSIDE_AT: f32 = crate::village::SHOT_AT;
pub const OUTSIDE_SKY: &str = crate::village::SHOT_SKY;

/// Where the tiles live, which is not where a published picture lives.
///
/// **A folder of its own, because they are not published one at a
/// time.** What goes on the site is the grid, and `./check-listings`
/// asks that every file in a repo's `media` be shown somewhere or it
/// will rot unseen. That rule is right and these are not an exception to
/// it: a tile is shown, inside the grid, and the grid is checked like
/// any other picture. Putting them a folder down says which kind of
/// thing they are, and the check walks past a directory.
///
/// The hall is the one that is both. Its picture is the building's own
/// `media/screenshot.png`, published on the site and in two readmes, and
/// it is the grid's first tile as well.
/// Every room photographed, in the order the grid lays them out.
pub fn shots(reaches: f32) -> Vec<Shot> {
    let plain = |slug, file, at, looking_at| Shot {
        slug,
        file,
        pose: Some((at, looking_at)),
        when: None,
        at_the_end: false,
    };
    let outdoors = |slug, file, at, looking_at| Shot {
        when: Some((OUTSIDE_AT, OUTSIDE_SKY)),
        ..plain(slug, file, at, looking_at)
    };
    let eye = eye();
    let mut out = Vec::new();

    // **The hall, which poses itself and keeps the picture it has.** Its
    // staged pose is worked out from a cabinet, a step back from one with
    // it lit and offering its key, because spec 0003 darkens a screen
    // until you are within reach of it and the view from the doorway is a
    // dark room with nothing answering. That is the picture on the site
    // and in both readmes, and this list is not the place to change it.
    //
    // Its picture test stands in the doorway instead and that is not a
    // second account of one pose: it is a second view, held for a
    // different job, and `tests/rooms/hall.png` is not published.
    out.push(Shot {
        slug: "hall",
        file: "media/screenshot.png",
        pose: None,
        when: None,
        at_the_end: false,
    });

    // **The nook, along it rather than across it.** Across it is the view
    // from the middle of the floor at the books, and the middle of the
    // floor is where the way up stands: with the parapet up that camera
    // is a hand from a grey slab. Along the room you get the benches, the
    // books, the rug and the stair between them.
    let (nook, _) = room::nook_floor(reaches);
    out.push(plain(
        "nook",
        "media/rooms/nook.png",
        vec3(nook.x, eye, -reaches + 1.4),
        vec3(nook.x, eye, nook.z),
    ));

    // **The cellar, from the foot of its stair**, which is what
    // everything down here is measured from.
    let cellar = vec3(
        cellar::stair_foot() - 2.775,
        -cellar::DOWN,
        cellar::opening(reaches),
    );
    out.push(plain(
        "cellar",
        "media/rooms/cellar.png",
        cellar,
        vec3(cellar.x - 5.0, cellar.y + eye, cellar.z),
    ));

    // **The baths, across the water at the pool.**
    let middle = spa::at(reaches);
    let pool = spa::pool(reaches).0;
    let baths = vec3(middle.x + 7.5, middle.y, middle.z + 4.5);
    out.push(plain(
        "baths",
        "media/rooms/baths.png",
        baths,
        vec3(pool.x, baths.y + eye, pool.z),
    ));

    // **The garden from its way in**, looking at the pond.
    let garden = garden::at(reaches);
    let water = garden::pond(reaches).0;
    out.push(outdoors(
        "garden",
        "media/rooms/garden.png",
        garden,
        vec3(water.x, eye, water.z),
    ));

    // **And the pond itself**, from the rim at the second island, which is
    // the thing people stand and look at rather than the room it is in.
    let rim = pond::rim_at(reaches, 0.86, pond::VERGE + 1.2);
    let isle = pond::islands(reaches)[1].0;
    out.push(outdoors(
        "pond",
        "media/rooms/pond.png",
        rim,
        vec3(isle.x, eye - 0.35, isle.z),
    ));

    // **The gallery, from the way in**, with the whole room in one frame,
    // which is the point of it.
    let (panel, _) = mandel::panel(reaches);
    out.push(plain(
        "gallery",
        "media/rooms/gallery.png",
        vec3(panel.x, eye, panel.z + 5.0),
        vec3(panel.x, panel.y - 0.1, panel.z),
    ));

    // **The back room at the rig**, which spec 0025 made the point of it.
    // Just inside the door shows the whole room and was the right picture
    // while the room was the point.
    out.push(plain(
        "back-room",
        "media/rooms/back-room.png",
        crate::posed_in_the_back_room(reaches).0,
        back::block(reaches),
    ));

    // **The hall of mirrors, with its feet on the floor.** The rooms above
    // pass `EYE` as the height and `looking_from` adds `EYE` again, which
    // stands the camera at twice eye height: a corridor whose panels stop
    // at 2.55 photographs its own ceiling that way.
    let along = (mirrors::SOUTH + mirrors::NORTH) * 0.5;
    let hall_of_mirrors = vec3(mirrors::middle(), 0.0, along);
    out.push(plain(
        "mirrors",
        "media/rooms/mirrors.png",
        hall_of_mirrors,
        vec3(mirrors::WEST, hall_of_mirrors.y + eye, along),
    ));

    // **The glass room, from the end its stair comes up at**, looking
    // down the room at the sphere and the stairwell beyond it.
    let upstairs = glass::inside(reaches);
    let (ball, _) = glass::ball(reaches);
    out.push(plain(
        "glass",
        "media/rooms/glass.png",
        vec3(ball.x, glass::UP, upstairs.min.z + 1.2),
        vec3(ball.x, ball.y, glass::well(reaches).min.z),
    ));

    // **Valhalla's hall, from inside its door.** Only standing at all once
    // the trail is answered, which is what `at_the_end` is for.
    let hall = valhalla::hall(reaches);
    let (door, _, _) = valhalla::doorway(reaches);
    out.push(Shot {
        slug: "valhalla-hall",
        file: "media/rooms/valhalla-hall.png",
        pose: Some((
            vec3(door.x - 2.0, door.y + valhalla::FLOOR_UP, door.z),
            vec3(hall.at.x - hall.wide * 0.4, door.y + eye + 0.2, door.z),
        )),
        // **Indoors, though the way to it is not.** The hall at the top
        // of the bridge is lit by its own fire and its sky is a painted
        // ceiling, per spec 0060.
        when: None,
        at_the_end: true,
    });

    // **The tesseract, whose slug sets a mode as well as a camera.** The
    // eight cells are nowhere in the building's coordinates, so getting
    // there is through the plinth rather than by standing somewhere; what
    // is here is only where you stand once you are in, which is
    // `tesseract::posed` turned into a place and a thing to look at so
    // that it reads like every other room in this list.
    let (at, yaw, pitch) = tesseract::posed();
    out.push(plain(
        tesseract::SLUG,
        "media/rooms/tesseract.png",
        at,
        at + vec3(0.0, eye, 0.0)
            + vec3(
                yaw.sin() * pitch.cos(),
                pitch.sin(),
                -yaw.cos() * pitch.cos(),
            ),
    ));

    out
}

/// Which way a camera at `at` has to be turned and tilted to be looking
/// at `looking_at`.
///
/// **One account of it, because two would be the same fault one level
/// down.** The pose says where the camera is and what it is pointed at,
/// and turning that into a yaw and a pitch is arithmetic that the picture
/// test and the photographer both have to do. Done twice it is two
/// pictures of one room that differ by a degree, which is a diff nobody
/// can read.
///
/// The eye is `EYE` above the feet, which is what `looking_from` adds, so
/// a target at the height of a pond is looked down at rather than
/// levelled with.
pub fn aimed(at: Vec3, looking_at: Vec3) -> (f32, f32) {
    let look = looking_at - vec3(at.x, at.y + eye(), at.z);

    (
        look.x.atan2(-look.z),
        look.y.atan2(glam::vec2(look.x, look.z).length()),
    )
}

/// The one a slug names, if it names one.
pub fn of(reaches: f32, slug: &str) -> Option<Shot> {
    shots(reaches).into_iter().find(|shot| shot.slug == slug)
}

/// Where a room's camera stands and what it looks at, for the picture
/// tests, which know their room by name and want nothing else.
#[cfg(test)]
pub fn pose(reaches: f32, slug: &str) -> (Vec3, Vec3) {
    of(reaches, slug)
        .unwrap_or_else(|| panic!("no room is photographed as {:?}", slug))
        .pose
        .unwrap_or_else(|| panic!("{:?} poses itself", slug))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reaches() -> f32 {
        room::Room::of(crate::games()).reaches
    }

    /// Spec 0135: every shot names a slug and a file of its own.
    ///
    /// **Both halves, because they fail differently.** Two shots with one
    /// slug means `--screenshot` can only ever take the first of them, and
    /// the second is a room nobody can photograph. Two shots with one file
    /// means the photographer takes both and the second lands on the
    /// first, which passes every check there is and publishes one room
    /// twice.
    #[test]
    fn no_two_shots_want_the_same_name_or_file() {
        let shots = shots(reaches());

        assert!(shots.len() >= 12, "{} rooms is not twelve", shots.len());

        for (n, one) in shots.iter().enumerate() {
            for two in shots.iter().skip(n + 1) {
                assert_ne!(one.slug, two.slug, "two rooms are staged as {:?}", one.slug);
                assert_ne!(
                    one.file, two.file,
                    "{:?} and {:?} both land in {}",
                    one.slug, two.slug, one.file
                );
            }

            assert!(
                one.file.starts_with("media/") && one.file.ends_with(".png"),
                "{:?} goes to {}, which is not a picture in media",
                one.slug,
                one.file
            );
        }
    }

    /// Spec 0135: a published room is one the tests hold.
    ///
    /// **The whole point of the list, as a check.** Every slug here names
    /// a reference under `tests/rooms` that a picture test compares
    /// against, so a room cannot go on the site from a pose nobody
    /// photographed in a test. It is the file and not the test that is
    /// asked for, because a reference exists only if a test wrote it and
    /// `BLESS` is the only thing that writes one.
    #[test]
    fn the_tests_and_the_photographer_share_a_pose() {
        for shot in shots(reaches()) {
            let held = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/rooms")
                .join(format!("{}.png", shot.slug));

            assert!(
                held.exists(),
                "{:?} is published to {} and no test holds a picture of it \
                 at {}",
                shot.slug,
                shot.file,
                held.display()
            );
        }
    }

    /// Spec 0135: a room's camera stands in the room it is photographing.
    ///
    /// **Asked of the floor under it rather than of the room's own box**,
    /// because a room here is a set of boxes and the one you stand in is
    /// the gap between them. What this catches is a pose left behind by a
    /// room that moved: the camera ends up in the open or inside a wall,
    /// and both of those are a picture of nothing in particular.
    ///
    /// The tesseract is not asked, since its pose is in the eight cells
    /// rather than in the building and a box of the hall standing at the
    /// same three numbers means nothing. Valhalla is not asked either,
    /// since its floor is only there once the trail is answered.
    #[test]
    fn every_shot_stands_in_the_room_it_takes() {
        let reaches = reaches();
        let floors = room::Room::of(crate::games()).solid();

        for shot in shots(reaches) {
            let Some((at, looking_at)) = shot.pose else {
                continue;
            };

            if shot.at_the_end || shot.slug == tesseract::SLUG {
                continue;
            }

            assert!(
                at.distance(looking_at) > 0.5,
                "{:?} looks at a spot {:.2} from its own eye, which is no \
                 direction at all",
                shot.slug,
                at.distance(looking_at)
            );

            // and not inside anything the building is made of, which is
            // the fault a moved room leaves behind
            let standing = at + Vec3::Y * eye();

            for box_ in &floors {
                assert!(
                    !box_.contains_point(standing),
                    "{:?} stands inside the building at {:?}",
                    shot.slug,
                    at
                );
            }
        }
    }
}
