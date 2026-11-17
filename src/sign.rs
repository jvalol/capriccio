//! The sign hung over the aisle that sends you down it. Spec 0006.
//!
//! The nook is off the far end of the left wall and nothing in the hall says
//! so. You find it by walking to the end and looking left, which is finding it
//! by accident. Everything else in the room announces itself: a cabinet has the
//! game's name lit across the top of it and that is most of what makes anyone
//! walk up to one.
//!
//! Not neon. Neon is what the hall is and the nook is the one room in the
//! building that is not it, so a neon arrow pointing at a panelled study is a
//! sign for the wrong room. This one is a painted plank on two chains with a
//! lamp over it, and the end of it is cut to a point.
//!
//! Nothing here draws or reads a key, so where it hangs and how far it clears
//! your head are arithmetic and can be checked without a window.

use blitzkit::mesh::{MeshData, Vertex};
use glam::Vec3;

/// What it says.
pub const SAYS: &str = "NOOK";

/// How big the plank is: across the aisle, down, and through.
pub const WIDE: f32 = 1.8;
pub const TALL: f32 = 0.56;
pub const THICK: f32 = 0.07;

/// How far past the plank the pointed end reaches.
///
/// The point is the whole of the direction. An arrow drawn on the face would
/// have to be mirrored on the back to go on pointing the same way in the world,
/// and a mirrored face is mirrored lettering. A plank cut to a point is the
/// same shape from both sides and from underneath, which is the thing a drawn
/// arrow cannot do and is why every pointing sign ever nailed to a post is cut
/// this way.
pub const POINT: f32 = 0.44;

/// The gold rim round the plank, and how far the painted face stands proud of
/// it.
pub const BORDER: f32 = 0.045;
pub const PROUD: f32 = 0.006;

/// The middle of the plank off the floor.
///
/// You are 1.55 at the eye and the ceiling is 3.2, so there is not much room
/// for a hanging sign to hang in. This leaves the bottom of it just over two
/// units up, which is head height and a bit, and about half a unit of chain
/// above it.
pub const HANGS: f32 = 2.34;

/// How far in from each end a chain comes down, and what a link is made of.
pub const CHAIN_AT: f32 = 0.6;
pub const LINK: f32 = 0.06;
pub const RING: f32 = 0.016;

/// The lettering: how many texels tall it is drawn and how tall it is hung.
pub const TEXELS: f32 = 64.0;
pub const LETTERS: f32 = 0.3;

/// The lamp over it: how far above the plank the shade hangs, how big it is,
/// and what it throws.
///
/// A pendant and not a glow. The lettering could carry a multiplier over one
/// and read from the far end of the hall with nothing lighting it, which is
/// exactly the thing this room was told off for: a bright patch on a surface
/// and no reason for it. A sign with a lamp over it has a reason, and the lamp
/// is a thing you can look at.
pub const LAMP_UP: f32 = 0.42;
pub const SHADE: Vec3 = glam::vec3(0.22, 0.1, 0.22);
pub const LAMP_LIT: f32 = 0.62;
pub const LAMP_RANGE: f32 = 4.6;
pub const LAMP_COLOUR: Vec3 = glam::vec3(1.0, 0.86, 0.62);

/// Where it hangs: over the middle of the aisle, level with the end of the
/// cabinet rows.
///
/// That is the spot where the hall stops offering you anything and you have to
/// decide whether to keep going. Hung at the nook's own doorway it would only
/// be read by somebody already standing in it.
pub fn at(reaches: f32) -> Vec3 {
    glam::vec3(0.0, HANGS, -reaches + crate::room::END)
}

/// One plank: what it says, where it hangs, and which way its point looks.
///
/// `points` is minus one for a point cut to the left, plus one for the right
/// and nought for a plank with no point at all. A room you walk straight on
/// into does not want an arrow: the point is the whole of the direction, and
/// a direction of "onward" is the one an arrow cannot draw.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plank {
    pub says: &'static str,
    pub at: Vec3,
    pub points: f32,
}

/// What the other two say. Spec 0113.
pub const GARDEN: &str = "GARDEN";
pub const GALLERY: &str = "GALLERY";

/// How low the gallery's plank hangs, against the other two.
///
/// **Because two planks at one height on one line are one plank.** Spec
/// 0113: the nook's hangs over the middle of the aisle and so did this, and
/// from the whole length of the hall the nearer covered the farther exactly.
/// Jake, shown it: hang the gallery lower.
///
/// It cannot clear the nook's altogether. The nook's reaches down to 2.06
/// and anything whose top is under that hangs at 1.78, which is a plank you
/// walk into. This is as low as it goes with a head's room under it, and
/// what it buys is the lower edge showing below the nearer plank rather
/// than the whole of it.
pub const GALLERY_HANGS: f32 = 2.10;

/// How far short of the hall's far end the gallery's plank hangs.
///
/// Inside the hall rather than in the gallery, for the nook's own reason one
/// room along: a sign in the room it names is read by somebody already
/// standing in it. It is also what keeps one ceiling over all three, since
/// the chains are cut to `room::soffit` and the gallery's is not that.
pub const BEFORE_THE_GALLERY: f32 = 0.5;

/// Every plank hung in the hall, in the order you meet them walking in.
///
/// **Only the ways that announce themselves.** Spec 0113: the cellar is
/// behind the bookcase that swings, the mirrors behind the other one, and
/// the room behind the wall is called that for a reason. A sign over any of
/// those is the secret given away by the building itself, so the three that
/// are found stay found and the two that are open get a plank.
pub fn planks(reaches: f32) -> Vec<Plank> {
    vec![
        Plank {
            says: SAYS,
            at: at(reaches),
            points: -1.0,
        },
        Plank {
            says: GALLERY,
            at: glam::vec3(0.0, GALLERY_HANGS, -reaches + BEFORE_THE_GALLERY),
            points: 0.0,
        },
    ]
}

/// The garden's name, on the hall's wall beside its way in rather than on a
/// plank over the aisle.
///
/// **Because that opening has no head to hang a name over.** Spec 0113: the
/// garden's lintel was taken out on purpose and the opening stops at the
/// hall's own ceiling, so there is no wall above it and a plank hung at its
/// own z lands among the entrance's neon with its blank back to you.
///
/// Beside it, on the aisle side of the wall, which is the baths' own pattern
/// one room along. Jake, shown the plank: the wall pattern for the garden.
///
/// Comes back as the middle of the board and how big it is. The board's face
/// looks into the hall along +x, since the wall it is on is the left one.
pub fn garden_board(reaches: f32) -> (Vec3, Vec3) {
    let way = crate::garden::way(reaches);
    let wall = crate::garden::wall_at();

    (
        glam::vec3(
            wall + BOARD_OUT * 0.5 + crate::room::THICK * 0.5,
            BOARD_UP,
            way.0 - BOARD_OFF - BOARD_WIDE * 0.5,
        ),
        glam::vec3(BOARD_OUT, BOARD_TALL, BOARD_WIDE),
    )
}

/// How big the garden's board is, how far it stands off the wall, how far
/// along from the opening it sits and how high its middle is.
pub const BOARD_WIDE: f32 = 1.5;
pub const BOARD_TALL: f32 = 0.46;
pub const BOARD_OUT: f32 = 0.05;
pub const BOARD_OFF: f32 = 0.18;
pub const BOARD_UP: f32 = 2.05;

/// How many links it takes to reach the ceiling from the top of the plank.
pub fn links(ceiling: f32) -> usize {
    let gap = ceiling - (HANGS + TALL * 0.5);

    ((gap / LINK).floor() as usize).max(1)
}

/// The pointed end: a wedge, flat front and back, running from the plank at x
/// nought out to a point at x minus one.
///
/// Its own mesh because the engine has a cube and a plane and neither of them
/// comes to a point. Six faces would be a cube; this is five, and the sixth is
/// where the plank is.
pub fn point_mesh() -> MeshData {
    // the slanted faces look out along the cut and up or down across it. The
    // long part of that is the up, not the out: the cut runs one unit out for
    // half a unit down, so its normal leans the other way about. Written the
    // obvious way round first and the two slants came out facing into the
    // wedge, which draws nothing at all and says nothing about it.
    let slant = (1.0f32 + 0.25).sqrt();
    let (back, up) = (0.5 / slant, 1.0 / slant);

    let faces: [([f32; 3], [[f32; 3]; 3]); 4] = [
        // the front, wound counter clockwise seen from +z, which is the side
        // its normal points. A face wound the other way is culled and drawn
        // nowhere.
        (
            [0.0, 0.0, 1.0],
            [[0.0, 0.5, 0.5], [-1.0, 0.0, 0.5], [0.0, -0.5, 0.5]],
        ),
        (
            [0.0, 0.0, -1.0],
            [[0.0, -0.5, -0.5], [-1.0, 0.0, -0.5], [0.0, 0.5, -0.5]],
        ),
        // and the two slants, which are what you see of it from below
        (
            [-back, up, 0.0],
            [[0.0, 0.5, -0.5], [-1.0, 0.0, -0.5], [-1.0, 0.0, 0.5]],
        ),
        (
            [-back, -up, 0.0],
            [[0.0, -0.5, 0.5], [-1.0, 0.0, 0.5], [-1.0, 0.0, -0.5]],
        ),
    ];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for (normal, corners) in faces.iter() {
        let from = vertices.len() as u32;
        for at in corners.iter() {
            vertices.push(Vertex::new(*at, *normal, [0.0, 0.0]));
        }
        indices.extend([from, from + 1, from + 2]);
    }

    // the slants are quads, not triangles: each one needs its fourth corner
    // along the top or the bottom of the plank end
    for (normal, corners, far) in [
        (
            [-back, up, 0.0],
            [[0.0, 0.5, -0.5], [-1.0, 0.0, 0.5]],
            [0.0, 0.5, 0.5],
        ),
        (
            [-back, -up, 0.0],
            [[0.0, -0.5, 0.5], [-1.0, 0.0, -0.5]],
            [0.0, -0.5, -0.5],
        ),
    ] {
        let from = vertices.len() as u32;
        for at in [corners[0], corners[1], far] {
            vertices.push(Vertex::new(at, normal, [0.0, 0.0]));
        }
        indices.extend([from, from + 1, from + 2]);
    }

    MeshData::new(vertices, indices)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spec 0006: you walk under it rather than into it.
    #[test]
    fn it_hangs_over_your_head() {
        let under = HANGS - TALL * 0.5;

        assert!(
            under > crate::EYE + 0.2,
            "the sign hangs at {} and your eye is at {}",
            under,
            crate::EYE
        );
        // and under the ceiling, with chain between the two
        let over = HANGS + TALL * 0.5;
        assert!(
            over < crate::room::TALL - LINK,
            "the top of the sign is at {} and the ceiling is at {}",
            over,
            crate::room::TALL
        );
        assert!(
            links(crate::room::TALL) >= 4,
            "a sign on one link is nailed up"
        );
    }

    /// Spec 0006: and it hangs in the aisle, not through a cabinet.
    #[test]
    fn it_hangs_clear_of_the_cabinets() {
        let reach = WIDE * 0.5 + POINT;
        let aisle = crate::room::WALL - crate::room::CABINET.x * 0.5;

        assert!(
            reach < aisle,
            "the sign reaches {} across an aisle {} wide",
            reach,
            aisle
        );
    }

    /// Spec 0006: the pointed end is a solid with every face wound to be seen.
    ///
    /// A face wound the wrong way is culled and drawn nowhere, which is a hole
    /// in a thing rather than an error anybody is told about.
    #[test]
    fn the_point_is_wound_to_be_seen() {
        let mesh = point_mesh();

        assert_eq!(mesh.indices.len() % 3, 0);
        assert!(mesh.indices.len() >= 18, "a wedge is six triangles");

        for triangle in mesh.indices.chunks_exact(3) {
            let corner = |n: usize| Vec3::from(mesh.vertices[triangle[n] as usize].position);
            let (a, b, c) = (corner(0), corner(1), corner(2));
            let wound = (b - a).cross(c - b);
            let normal = Vec3::from(mesh.vertices[triangle[0] as usize].normal);

            assert!(
                wound.length() > 1e-6,
                "a triangle with no area is drawn nowhere"
            );
            assert!(
                wound.normalize().dot(normal) > 0.9,
                "a face winds {:?} and claims to look {:?}",
                wound.normalize(),
                normal
            );
        }
    }

    /// Spec 0006: and it points the way the nook is.
    #[test]
    fn the_point_points_at_the_nook() {
        let out = point_mesh()
            .vertices
            .iter()
            .map(|v| v.position[0])
            .fold(f32::INFINITY, f32::min);

        assert_eq!(out, -1.0, "the wedge does not come to a point");
        // the nook is behind the left wall, which is -x
        assert!(at(9.4).x - WIDE * 0.5 > -(crate::room::WALL + crate::room::CABINET.x));
    }

    /// Spec 0113: every plank clears a head and hangs under the ceiling, the
    /// gallery's hangs below the nook's, and the garden's board is beside
    /// its opening rather than over it.
    ///
    /// **The second of those is the whole of what Jake sent back.** Two
    /// planks over the middle of one aisle at one height are one plank from
    /// anywhere along it, and the farther was the one nobody could see.
    ///
    /// **And the third is why the garden has a board at all.** That opening
    /// stops at the hall's own ceiling, so there is no head to hang a name
    /// over: a board that overlapped it would be hanging in the doorway.
    #[test]
    fn every_sign_clears_a_head_and_none_hides_another() {
        let reaches = 9.5;
        let planks = planks(reaches);
        let soffit = crate::room::soffit();

        // 1.7 is a body, and a plank you walk into is worse than no plank
        for plank in planks.iter() {
            let under = plank.at.y - TALL * 0.5;

            assert!(
                under > 1.7,
                "{} hangs at {:.2}, which is a plank you walk into",
                plank.says,
                under
            );
            assert!(
                plank.at.y + TALL * 0.5 < soffit,
                "{} hangs through the ceiling",
                plank.says
            );
        }

        let nook = planks
            .iter()
            .find(|plank| plank.says == SAYS)
            .expect("the nook's plank");
        let gallery = planks
            .iter()
            .find(|plank| plank.says == GALLERY)
            .expect("the gallery's plank");

        assert!(
            gallery.at.y < nook.at.y,
            "the gallery's plank hangs at {} and the nook's at {}, so the \
             nearer hides the farther",
            gallery.at.y,
            nook.at.y
        );

        // and they are on the same line down the aisle, which is what makes
        // the height the only thing separating them
        assert!(
            (gallery.at.x - nook.at.x).abs() < 1e-4,
            "they are not over the same line, so this test is checking nothing"
        );

        // the garden's board: on the wall, and clear of the way through it
        let (at, size) = garden_board(reaches);
        let way = crate::garden::way(reaches);
        let (near, far) = (at.z - size.z * 0.5, at.z + size.z * 0.5);

        assert!(
            far < way.0 || near > way.1,
            "the board runs {:.2} to {:.2} and the opening {:.2} to {:.2}, so \
             it is hanging in the doorway",
            near,
            far,
            way.0,
            way.1
        );
        assert!(
            at.x < 0.0 && at.x > crate::garden::wall_at(),
            "the board is not on the hall's side of its own wall"
        );
    }
}
