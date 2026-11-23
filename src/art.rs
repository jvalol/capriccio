//! The pictures on the nook's north wall. Spec 0131.
//!
//! **Eight paintings, hung in a line.** The nook is a library and the
//! building as a whole is a museum of curiosities, and until now there was
//! not a picture anywhere in it. Spec 0115's plan had this wall and Jake
//! took it back: ten metres is a mural's wall and a plan is a diagram.
//!
//! **They are the works themselves and not pastiche.** Every one is out of
//! copyright and every scan is open access off Wikimedia Commons, fetched
//! at five hundred pixels and re-encoded: three hundred kilobytes for the
//! eight of them, against three megabytes of footstep sounds already in
//! `res`. A painted-looking abstraction of a famous painting is a thing
//! that reads as placeholder, which is what this building calls a box.
//!
//! **The hang is asked of the paintings.** How tall each one is on the wall
//! comes from how tall it really is, compressed, and how wide it is comes
//! from the scan's own pixels rather than a number written here. So the row
//! has a Bosch two metres across and a Hokusai print a third of that, which
//! is what those two are.

use blitzkit::collision::Aabb;
use blitzkit::texture::TextureData;
use glam::{vec3, Vec3};

/// One picture: the scan, and how tall the painting really is.
///
/// **The real height and not the drawn one.** A row of pictures all the
/// same size is a catalogue page. What makes a wall of paintings read as a
/// wall of paintings is that they are not alike, and the sizes that are
/// already true are better than sizes somebody chose.
pub struct Picture {
    pub bytes: &'static [u8],
    pub tall_cm: f32,
}

/// The eight, west to east, as you meet them coming in.
///
/// Heights in centimetres as the holding museums give them. The Hokusai is
/// a woodblock print and is the reason these are compressed rather than
/// used straight: a quarter of a metre next to Bosch's three and a nine
/// tenths is a stamp beside a door.
pub const PICTURES: [Picture; 8] = [
    // Botticelli, The Birth of Venus, c. 1485, Uffizi
    Picture {
        bytes: include_bytes!("../res/art/venus.jpg"),
        tall_cm: 172.5,
    },
    // Leonardo, Mona Lisa, c. 1503, Louvre
    Picture {
        bytes: include_bytes!("../res/art/gioconda.jpg"),
        tall_cm: 77.0,
    },
    // Bosch, The Garden of Earthly Delights, c. 1500, Prado
    Picture {
        bytes: include_bytes!("../res/art/delights.jpg"),
        tall_cm: 220.0,
    },
    // Vermeer, Girl with a Pearl Earring, c. 1665, Mauritshuis
    Picture {
        bytes: include_bytes!("../res/art/pearl.jpg"),
        tall_cm: 44.5,
    },
    // Goya, Saturn Devouring His Son, c. 1823, Prado
    Picture {
        bytes: include_bytes!("../res/art/saturn.jpg"),
        tall_cm: 143.5,
    },
    // Hokusai, The Great Wave off Kanagawa, c. 1831, printed many times
    Picture {
        bytes: include_bytes!("../res/art/wave.jpg"),
        tall_cm: 25.7,
    },
    // van Gogh, The Starry Night, 1889, MoMA
    Picture {
        bytes: include_bytes!("../res/art/stars.jpg"),
        tall_cm: 73.7,
    },
    // Munch, The Scream, 1893, National Museum of Norway
    Picture {
        bytes: include_bytes!("../res/art/scream.jpg"),
        tall_cm: 91.0,
    },
];

/// How much the real heights are compressed, and how tall the biggest ends
/// up on the wall.
///
/// **Compressed and not scaled.** Bosch is nine times Hokusai, and a row
/// that honoured that would be one picture and seven postage stamps on a
/// wall ten metres long. Raised to two fifths the spread comes down to
/// about two and a half, which is the spread a real hang has: you can see
/// at a glance which are the big ones and none of them is unreadable.
pub const SQUASH: f32 = 0.4;
pub const TALLEST: f32 = 1.12;

/// How high the line their middles sit on is, how far apart they hang, and
/// how much wall is left clear at either end.
///
/// **One line through the middles**, which is how a room hangs pictures and
/// is the only thing that holds a row of eight different shapes together.
/// A little over eye height, because the dado rail is at waist and nothing
/// may cross it.
pub const HANGS: f32 = 1.82;
pub const BETWEEN: f32 = 0.24;
pub const MARGIN: f32 = 0.40;

/// How wide the frame's moulding is, how far it stands off the wall, and
/// how far the canvas inside it does.
///
/// The canvas is the shallower of the two, so the frame holds it rather
/// than sitting flush with it, and neither is in a plane with the
/// panelling behind.
pub const FRAME: f32 = 0.055;
pub const FRAME_DEEP: f32 = 0.05;
pub const CANVAS_DEEP: f32 = 0.022;

/// The band of the nook's north wall the pictures are allowed: the inner
/// face, between the dado rail and the cornice.
pub fn wall(reaches: f32) -> (f32, f32, f32) {
    let (at, size) = crate::room::nook_end(reaches);

    (
        at.x - size.x * 0.5 + MARGIN,
        at.x + size.x * 0.5 - MARGIN,
        at.z - size.z * 0.5,
    )
}

/// Which way a thing hung on that wall stands out of it.
///
/// The room is on the wall's smaller z, which is the same fact spec 0115
/// wrote down for the plan when it hung here.
pub const OUT: f32 = -1.0;

/// How tall and how wide each picture is drawn, before the row is fitted to
/// the wall.
///
/// **The shape comes out of the scan.** Decoding eight small pictures at
/// startup is nothing, and a width written down beside a picture is a
/// second account of its shape: crop the scan and the frame no longer fits
/// what is in it.
fn shapes() -> Vec<(f32, f32)> {
    let tallest = PICTURES
        .iter()
        .map(|one| one.tall_cm.powf(SQUASH))
        .fold(0.0f32, f32::max);

    PICTURES
        .iter()
        .map(|one| {
            let tall = TALLEST * one.tall_cm.powf(SQUASH) / tallest;
            let shape = match TextureData::from_bytes(one.bytes) {
                Ok(scan) => scan.width() as f32 / scan.height().max(1) as f32,
                // a scan that will not decode is drawn square rather than
                // taking the row down with it
                Err(_) => 1.0,
            };

            (tall * shape, tall)
        })
        .collect()
}

/// One hung picture: the frame's four bars and the canvas inside them.
pub struct Hung {
    pub bytes: &'static [u8],
    pub canvas: Aabb,
    pub frame: [Aabb; 4],
}

/// The row: every picture sized, fitted to the wall, and laid along it.
///
/// **Fitted rather than trusted to fit.** The natural widths come out of
/// eight scans and a compression, and what they add up to is not a number
/// anybody chose. So the row is worked out at its natural size and then
/// scaled until it fits the wall it is on, which means a ninth picture or a
/// different scan moves everything and breaks nothing.
pub fn hung(reaches: f32) -> Vec<Hung> {
    let (west, east, face) = wall(reaches);
    let shapes = shapes();
    let natural: f32 = shapes.iter().map(|(wide, _)| *wide).sum();
    // the frames and the gaps are not scaled with the pictures: a moulding
    // is a moulding whatever it is round, and scaling them with the rest
    // was what put the last frame a centimetre past the end of the wall
    let fixed = BETWEEN * (PICTURES.len() - 1) as f32 + FRAME * 2.0 * PICTURES.len() as f32;
    let fits = ((east - west) - fixed) / natural;
    let mut along = west;
    let mut out = Vec::with_capacity(PICTURES.len());

    for (one, (wide, tall)) in PICTURES.iter().zip(shapes) {
        let (wide, tall) = (wide * fits, tall * fits);
        let low = vec3(along + FRAME, HANGS - tall * 0.5, face);
        let high = vec3(along + FRAME + wide, HANGS + tall * 0.5, face);
        let deep = |box_: Aabb, by: f32| {
            Aabb::new(
                vec3(box_.min.x, box_.min.y, face.min(face + OUT * by)),
                vec3(box_.max.x, box_.max.y, face.max(face + OUT * by)),
            )
        };
        let flat = |a: Vec3, b: Vec3| Aabb::new(vec3(a.x, a.y, face), vec3(b.x, b.y, face));

        out.push(Hung {
            bytes: one.bytes,
            canvas: deep(flat(low, high), CANVAS_DEEP),
            frame: [
                deep(
                    flat(
                        vec3(low.x - FRAME, low.y - FRAME, face),
                        vec3(high.x + FRAME, low.y, face),
                    ),
                    FRAME_DEEP,
                ),
                deep(
                    flat(
                        vec3(low.x - FRAME, high.y, face),
                        vec3(high.x + FRAME, high.y + FRAME, face),
                    ),
                    FRAME_DEEP,
                ),
                deep(
                    flat(
                        vec3(low.x - FRAME, low.y - FRAME, face),
                        vec3(low.x, high.y + FRAME, face),
                    ),
                    FRAME_DEEP,
                ),
                deep(
                    flat(
                        vec3(high.x, low.y - FRAME, face),
                        vec3(high.x + FRAME, high.y + FRAME, face),
                    ),
                    FRAME_DEEP,
                ),
            ],
        });

        along += wide + FRAME * 2.0 + BETWEEN;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reaches() -> f32 {
        crate::room::Room::of(crate::games()).reaches
    }

    /// Spec 0131: every scan decodes, and the row's shapes come out of them.
    ///
    /// **The one thing a picture on a wall can get silently wrong.** A scan
    /// that will not decode draws as nothing at all, and nothing at all on
    /// a wall looks exactly like a wall. Every other test here would pass.
    #[test]
    fn every_scan_decodes_and_is_a_picture() {
        for (n, one) in PICTURES.iter().enumerate() {
            let scan = TextureData::from_bytes(one.bytes)
                .unwrap_or_else(|_| panic!("picture {} does not decode", n));

            assert!(
                scan.width() >= 128 && scan.height() >= 128,
                "picture {} is {} by {}, which is a thumbnail",
                n,
                scan.width(),
                scan.height(),
            );
            assert!(
                one.tall_cm > 10.0 && one.tall_cm < 600.0,
                "picture {} is said to be {} centimetres tall",
                n,
                one.tall_cm
            );
        }
    }

    /// Spec 0131: the row fits the wall, in a line, without touching.
    #[test]
    fn the_row_fits_the_wall_and_nothing_overlaps() {
        let reaches = reaches();
        let (west, east, face) = wall(reaches);
        let hung = hung(reaches);

        assert_eq!(hung.len(), PICTURES.len());

        let mut last: Option<f32> = None;

        for (n, one) in hung.iter().enumerate() {
            let whole = one.frame.iter().fold(one.canvas, |held, bar| {
                Aabb::new(held.min.min(bar.min), held.max.max(bar.max))
            });

            assert!(
                whole.min.x >= west - 1e-3 && whole.max.x <= east + 1e-3,
                "picture {} runs {:.2}..{:.2} and the wall gives {:.2}..{:.2}",
                n,
                whole.min.x,
                whole.max.x,
                west,
                east
            );
            // clear of the dado rail under it and the cornice over it
            assert!(
                whole.min.y > crate::study::DADO + crate::study::RAIL * 0.5
                    && whole.max.y < crate::room::TALL - crate::study::CORNICE,
                "picture {} runs {:.2}..{:.2} up, into the rail or the cornice",
                n,
                whole.min.y,
                whole.max.y
            );

            if let Some(last) = last {
                assert!(
                    whole.min.x >= last + BETWEEN - 1e-3,
                    "picture {} starts at {:.2} and the one before it ended at {:.2}",
                    n,
                    whole.min.x,
                    last
                );
            }

            last = Some(whole.max.x);

            // and the canvas sits inside its frame rather than over it
            assert!(
                OUT * (one.canvas.min.z + one.canvas.max.z - face * 2.0)
                    < OUT * (one.frame[0].min.z + one.frame[0].max.z - face * 2.0),
                "picture {}'s canvas stands as proud as its frame",
                n
            );
        }
    }

    /// Spec 0131: they are hung on one line, which is what holds a row of
    /// eight different shapes together.
    #[test]
    fn their_middles_are_all_on_one_line() {
        let reaches = reaches();
        let hung = hung(reaches);
        let mut tall: Vec<f32> = Vec::new();

        for (n, one) in hung.iter().enumerate() {
            let middle = (one.canvas.min.y + one.canvas.max.y) * 0.5;

            assert!(
                (middle - HANGS).abs() < 1e-3,
                "picture {} hangs with its middle at {:.3} and the line is {:.3}",
                n,
                middle,
                HANGS
            );

            tall.push(one.canvas.max.y - one.canvas.min.y);
        }

        // and they are not all one size, which is the other half of a hang
        let big = tall.iter().copied().fold(0.0f32, f32::max);
        let small = tall.iter().copied().fold(f32::MAX, f32::min);

        assert!(
            big / small > 1.8,
            "the biggest is {:.2} and the smallest {:.2}, so the row is a catalogue page",
            big,
            small
        );
    }

    /// Spec 0131: nothing of them stands in the nook's walls or furniture.
    #[test]
    fn nothing_hangs_inside_anything() {
        let reaches = reaches();
        let room = crate::room::Room::of(crate::games());

        for (n, one) in hung(reaches).iter().enumerate() {
            let whole = one.frame.iter().fold(one.canvas, |held, bar| {
                Aabb::new(held.min.min(bar.min), held.max.max(bar.max))
            });

            for box_ in room.solid() {
                // the wall it is hung on is the one thing it may touch, and
                // it touches it at a face rather than standing in it
                if box_.min.z >= whole.max.z - 1e-4 {
                    continue;
                }

                assert!(
                    box_.min.x >= whole.max.x - 1e-4
                        || box_.max.x <= whole.min.x + 1e-4
                        || box_.min.y >= whole.max.y - 1e-4
                        || box_.max.y <= whole.min.y + 1e-4
                        || box_.min.z >= whole.max.z - 1e-4
                        || box_.max.z <= whole.min.z + 1e-4,
                    "picture {} at {:?}..{:?} stands in {:?}..{:?}",
                    n,
                    whole.min,
                    whole.max,
                    box_.min,
                    box_.max
                );
            }
        }
    }
}
