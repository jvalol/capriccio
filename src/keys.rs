//! Every key this building binds, named once. Spec 0116.
//!
//! **Because the only account of them was the dispatch itself.** Twenty keys
//! were bound and the corner line named five. The rest were learned by
//! reading `process_keyboard`, and the comment on the newest of them says so
//! outright: "One more undocumented inspection key is the convention here
//! rather than an exception to it." A convention that a key is a secret is a
//! convention worth ending.
//!
//! **A table and a test, rather than a dispatch driven by the table.** Each
//! row would have to carry the thing it does, and what these do is reach into
//! `Arcade` in twenty different ways: a field toggled, a method called, a
//! value wound. Written as data that would be twenty closures over `&mut
//! self` and a worse `match` than the one there now. So the dispatch keeps
//! its arms, this holds the names, and
//! `tests::every_key_the_arcade_binds_is_in_the_list` is what stops the two
//! coming apart. That is the same shape as ./check-listings, which holds a
//! readme against the disk rather than generating it.

use blitzkit::keyboard::KeyboardKey;

/// Which half of the list a binding is in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Group {
    /// What the building is played with.
    About,
    /// What it is looked at with. Spec 0013's outlines, spec 0056's frame,
    /// spec 0048's planes, spec 0045's pose and spec 0087's flight: five
    /// keys that were each added for one afternoon's hunt and then stayed.
    Looking,
}

/// One binding: what the screen calls it, which keys it actually is, and
/// what it does.
///
/// **`keys` is a list because a row is not always one key.** WASD is four,
/// the arrows are four, and shift is two; a table with one key a row would
/// put eight rows on screen that a person reads as two.
pub struct Bound {
    pub shown: &'static str,
    /// **Read by the test and by nothing that runs.** The screen draws
    /// `shown`, which is the reading form: "WASD" rather than four rows. This
    /// is what those letters actually are, and it exists so
    /// `tests::every_key_the_arcade_binds_is_in_the_list` can hold the list
    /// against the dispatch. Deriving `shown` from these instead would put
    /// "W A S D" on the screen, and deriving these from `shown` would make
    /// the check a check of a string it had just parsed.
    #[allow(dead_code)]
    pub keys: &'static [KeyboardKey],
    pub does: &'static str,
    pub group: Group,
}

/// The heading over each group.
pub fn heading(group: Group) -> &'static str {
    match group {
        Group::About => "Getting about",
        Group::Looking => "Looking at the building itself",
    }
}

use Group::{About, Looking};
use KeyboardKey as K;

pub const BOUND: &[Bound] = &[
    Bound {
        shown: "WASD",
        keys: &[K::W, K::S, K::A, K::D],
        does: "walk",
        group: About,
    },
    Bound {
        shown: "Shift",
        keys: &[K::LShift, K::RShift],
        does: "run",
        group: About,
    },
    Bound {
        shown: "arrow keys",
        keys: &[K::Up, K::Down, K::Left, K::Right],
        does: "look around, or work the toy you are looking at",
        group: About,
    },
    Bound {
        shown: "Return",
        keys: &[K::Return],
        does: "use what you are looking at",
        group: About,
    },
    Bound {
        shown: "R",
        keys: &[K::R],
        does: "put back what you are looking at, or go back to the door while flying",
        group: About,
    },
    Bound {
        shown: "L",
        keys: &[K::L],
        does: "swing the solar system out, and line it up again",
        group: About,
    },
    Bound {
        shown: "Tab",
        keys: &[K::Tab],
        does: "list everything out past the door, and Return to go to one",
        group: About,
    },
    Bound {
        shown: "H",
        keys: &[K::H],
        does: "show this list",
        group: About,
    },
    Bound {
        shown: "Escape",
        keys: &[K::Escape],
        does: "quit",
        group: About,
    },
    Bound {
        shown: "O",
        keys: &[K::O],
        does: "outline everything solid over what is drawn",
        group: Looking,
    },
    Bound {
        shown: "F",
        keys: &[K::F],
        does: "show what a frame costs, and which pass of it is the dear one",
        group: Looking,
    },
    Bound {
        shown: "P",
        keys: &[K::P],
        does: "look for drawn faces sharing a plane, and outline what it finds",
        group: Looking,
    },
    Bound {
        shown: "M",
        keys: &[K::M],
        does: "say where you are standing, on the screen and in the log",
        group: Looking,
    },
    Bound {
        shown: "G",
        keys: &[K::G],
        does: "fly, and walk again",
        group: Looking,
    },
];

/// The list as the corner draws it: a heading, then a line a binding.
pub fn lines() -> Vec<String> {
    let mut out = Vec::new();

    for group in [About, Looking] {
        out.push(heading(group).to_owned());

        for bound in BOUND.iter().filter(|bound| bound.group == group) {
            out.push(format!("{}  {}", bound.shown, bound.does));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What `process_keyboard` is, as text, so a test can read which keys it
    /// actually answers to.
    ///
    /// **Source rather than behaviour, because behaviour cannot be asked.**
    /// A key that does nothing and a key that is not bound look identical
    /// from outside `Arcade`: both leave every field as they found it. What
    /// can be asked is which keys the dispatch names, and that is the thing
    /// that drifts from this list.
    ///
    /// It reads `process_keyboard` alone and not the whole file. The toys
    /// match on arrows in their own code and the arrows are one row here, so
    /// a sweep of the file would say nothing this does not and would break
    /// every time a toy learned a key the keyboard already carries.
    fn the_dispatch() -> &'static str {
        let file = include_str!("main.rs");
        let from = file
            .find("fn process_keyboard")
            .expect("process_keyboard has been renamed");
        let to = file[from..]
            .find("fn process_mouse")
            .expect("process_mouse has been renamed");

        &file[from..from + to]
    }

    /// Spec 0116: nothing is bound that the list does not name.
    ///
    /// This is the whole reason the list is allowed to be a second account
    /// of the dispatch. Without it, a key added in a hurry is a key nobody
    /// is told about, which is how twenty of them came to be bound with five
    /// named.
    #[test]
    fn every_key_the_arcade_binds_is_in_the_list() {
        let said = the_dispatch();
        let mut named: Vec<String> = Vec::new();

        for bound in BOUND {
            for key in bound.keys {
                named.push(format!("{:?}", key));
            }
        }

        let mut missing: Vec<&str> = Vec::new();

        for (at, _) in said.match_indices("KeyboardKey::") {
            let rest = &said[at + "KeyboardKey::".len()..];
            let end = rest
                .find(|one: char| !one.is_ascii_alphanumeric())
                .unwrap_or(rest.len());
            let key = &rest[..end];

            if key.is_empty() {
                continue;
            }
            if !named.iter().any(|one| one == key) && !missing.contains(&key) {
                missing.push(key);
            }
        }

        assert!(
            missing.is_empty(),
            "process_keyboard answers to {:?}, which the list does not name",
            missing
        );
    }

    /// And that the reading above finds anything at all.
    ///
    /// **Because an empty haystack passes the test above.** A rename that
    /// moved the dispatch, a slice that came back empty, a match that found
    /// no keys: every one of those reports that nothing is unnamed.
    #[test]
    fn the_dispatch_is_actually_being_read() {
        let said = the_dispatch();

        assert!(
            said.matches("KeyboardKey::").count() >= 10,
            "only {} keys were found in process_keyboard, which is too few to \
             be the real dispatch",
            said.matches("KeyboardKey::").count()
        );
    }

    /// Every row says something, and no key is in two rows.
    #[test]
    fn each_key_is_named_once() {
        let mut seen: Vec<String> = Vec::new();

        for bound in BOUND {
            assert!(!bound.shown.is_empty(), "a row has no name");
            assert!(!bound.does.is_empty(), "{} says nothing", bound.shown);
            assert!(!bound.keys.is_empty(), "{} is no key at all", bound.shown);

            for key in bound.keys {
                let key = format!("{:?}", key);

                assert!(!seen.contains(&key), "{:?} is in two rows", key);
                seen.push(key);
            }
        }
    }
}
