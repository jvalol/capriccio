//! What the building says, in the words it says it in.
//!
//! **One file, because the signature is a hash of a file.** Every string
//! drafted for somebody to read carries a marker until Jake takes it out,
//! and `sign-copy` takes it out by showing him the words and writing the
//! file's hash into `.copy-signed`. That works on a file whose contents
//! are the copy. Pointed at `src/bridge.rs` it would make him re-approve
//! the Bifröst inscription every time somebody moved a vertex in the same
//! file, which is the opposite of easy.
//!
//! The marker is written above the strings below and nowhere in this
//! prose, because `sign-copy` strikes the token off whatever line it finds
//! it on. A sentence that quotes it comes back with a hole in it, which is
//! what the sentence above had.
//!
//! So drafted copy is written here and read from here. The hash moves
//! when the words move and at no other time.
//!
//! **And `.copy-files` names this file**, which is what makes
//! `sign-copy --all` find it. Before that there were two scanners with
//! two file sets: `check-all` read `src` and held the gate, while
//! `check-voice` read the readme and reported everything signed. The
//! one with the reading and editing in it was looking at the other
//! half.

use blitzkit::{Asked, Options};

/// The name on the runestone at the foot of the bridge, in the Latin
/// letters under the runes. Spec 0054.
pub const BIFROST: &str = "BIFROST";

/// What the watchman says when you come up to him.
///
/// **He asked a riddle until spec 0063.** It was "tell me what it looks
/// like from here", and what it took for an answer was "a light": the
/// building is a dark block with one lit lantern on it from up there, per
/// spec 0057. Jake read that and said the riddle is very weak, and he is
/// right. The answer comes from turning round rather than from having
/// walked anything, and spec 0058 set its own test as "an answer you can
/// only have from having walked the building", which it fails. The bridge
/// is already the gate: it only comes up once every station has been
/// given, so the riddle was a second lock on a door that was locked.
///
/// Four of his lines went with it: the question, the line after an answer,
/// the line after a wrong one, and the four answers it took.
///
/// What stands here is his own: he struck the line I drafted out of those
/// four and wrote this one in its place.
pub const WATCHMAN_LETS_YOU_BY: &str =
    "I have stood at this gate since the beginning of time. You may go on through.";

/// What the arcade takes of its own, which `--help` prints above the
/// engine's. Spec 0069.
///
/// **A list rather than a paragraph, since engine spec 0109.** The columns
/// used to be typed into the text and the whole block printed as it stood,
/// so nothing could ask it what is *not* an option and `--flyy` opened the
/// building and walked. The words are the same words; where they sit is
/// worked out where they are printed.
pub const OPTIONS: Options = Options {
    says: "The arcade takes these of its own:",
    each: &[
        Asked {
            name: "--done",
            also: "",
            takes: "",
            says: "every question on the trail already answered, so the bridge and the hall at the top of it are there",
        },
        Asked {
            name: "--forget",
            also: "",
            takes: "",
            says: "open with nothing answered, whatever the building remembers, and keep this run instead",
        },
        Asked {
            name: "--pose",
            also: "",
            takes: "<seven numbers>",
            says: "stand here, at this moment, rather than at the door in the afternoon: the line the plate prints under M, with or without commas",
        },
        Asked {
            name: "--where",
            also: "",
            takes: "",
            says: "print where you are standing, twice a second, on stderr",
        },
        Asked {
            name: "--windowed",
            also: "",
            takes: "",
            says: "open in a window rather than on the whole screen",
        },
        Asked {
            name: "--cost",
            also: "",
            takes: "",
            says: "open with the frame readout up, rather than reaching for F",
        },
        Asked {
            name: "--at",
            also: "",
            takes: "<0 to 1>",
            says: "what time it is when the run starts: nought dawn, a quarter noon, a half dusk",
        },
        Asked {
            name: "--day",
            also: "",
            takes: "<seconds>",
            says: "how long a day lasts",
        },
        Asked {
            name: "--sky",
            also: "",
            takes: "<name>",
            says: "hold the weather: clear, fair, overcast or storm",
        },
        Asked {
            name: "--fly",
            also: "",
            takes: "",
            says: "fly rather than walk",
        },
        Asked {
            name: "--open-case",
            also: "",
            takes: "",
            says: "hold the hall of mirrors' bookcase open",
        },
        Asked {
            name: "--dump-mirrors",
            also: "",
            takes: "",
            says: "write every mirror's view into /tmp, once",
        },
        Asked {
            name: "--screenshot",
            also: "",
            takes: "[<name>]",
            says: "take the poster, or one of the others by name",
        },
        Asked {
            name: "--screenshots",
            also: "",
            takes: "",
            says: "print the pictures and their files, and stop",
        },
        Asked {
            name: "--walk",
            also: "",
            takes: "<route>",
            says: "walk a written route with no window, and fail if it no longer goes where it says",
        },
        Asked {
            name: "--tour",
            also: "",
            takes: "<x,y,z>",
            says: "work a route out to a place, and print it",
        },
        Asked {
            name: "--film",
            also: "",
            takes: "<clip>",
            says: "film a clip",
        },
    ],
};

/// What the stone at the foot of the bridge says the building is still
/// holding. Spec 0118.
///
/// **And never a compass direction.** The field's line said the grass west
/// of the garden. Jake, 2026-11-18: nobody knows west from east or south
/// or north. Nothing in this building tells you which way you are facing
/// except spec 0115's brass mark on the nook's plan, which is one wall in
/// one room, so a direction is a word the player cannot cash. A landmark
/// they can see and walk to is one they can.
///
/// **The gallery has no line, because it is no longer asked.** Three were
/// written for it and each failed differently: "one of the six has not
/// turned you" read as predatory, "five of the six turn where they stand"
/// could not be parsed, and "one of the six stands still" was simply
/// false, `display::Spin::facing` turning every one of them that is not
/// in your hands. The one that would have worked is "one of the six
/// changes shape", the sixth turning itself in the fourth at
/// `tesseract::TURNS_BY` while the rest only spin on their plinths. It is
/// not here because Jake took the room out of the requirements rather
/// than the line out of the room: see `trail::Asked`.
///
/// **And never more than the trail asks for.** The cellar's line said one
/// book on the shelf was a handle. Two are: `room::handle` is
/// `bookcases[cellar::CASE]` and `room::mirror_handle` is
/// `bookcases[mirrors::CASE]`, which are different cases in the row, so
/// "the shelf" was wrong as well as the count. Saying two are handles
/// would be worse than wrong: spec 0113 keeps the cellar and the hall of
/// mirrors unsigned on purpose, a sign over either being the building
/// giving away its own secret, and the trail never asks for the mirrors.
/// So the line says a book without counting, which is true of the one
/// this station is about and tells you nothing about the other.
///
/// **And never the room doing something to the player.** Jake, 2026-11-18:
/// "the water has not had you" and "one of the six has not turned you"
/// sound predatory. Both put the thing in the room on the acting end of
/// the sentence with the player on the receiving one, which is a voice
/// nothing else in this building uses. The register that works is the one
/// the other five were already in: nobody has done the thing yet.
///
/// **In the voice of the thing undone, and never an instruction.** Spec
/// 0036 forbids signposting outright: a clue that says GO LEFT is a sign.
/// What is allowed is the model that spec sets, which is spec 0020's worn
/// lane in the carpet, a thing that happened rather than a thing somebody
/// put up. So each of these is a fact about the building with nothing in
/// it telling you what to do about it, and working out what to do is the
/// whole of what the player is being asked for.
///
/// **A match and not a table**, so a station added to `trail::Asked`
/// stops this compiling rather than quietly carving nothing.
/// And the same sentence once somebody has. Spec 0119.
///
/// **The same sentence with the negative taken out**, which is the whole
/// of the design and not a shortcut. You read a row, you do the thing, and
/// next time you walk out the stone says it happened. Nothing anywhere
/// tells you that is what the bridge is waiting on; you watch one line
/// turn over and you have the rule.
///
/// **So the register is `still_held`'s and must stay it.** That one was
/// settled on 2026-11-18 after three lines were thrown out for sounding
/// predatory, and what it landed on is a fact about the building with the
/// player nowhere in the sentence: nobody has done the thing yet. Its
/// mirror is somebody who has, and the player is still not in it. "You
/// pulled it" would put them there, and would also be a lie the first time
/// somebody else's run is watched.
///
/// **A match and not a table**, for the reason `still_held` gives: a
/// station added to `trail::Asked` stops this compiling rather than
/// quietly leaving a row of the ledger blank.
pub fn given(what: crate::trail::Asked) -> &'static str {
    use crate::trail::Asked;

    match what {
        Asked::OpenedTheCase => "A book was a handle, and somebody pulled it.",
        Asked::WadedInThePool => "Somebody has been in the water.",
        Asked::SolvedTheCube => "Somebody has played with the cube and solved it.",
        Asked::GotBehindTheWall => "The room you started in has a secret somebody walked through.",
        Asked::ReachedTheSun => "Somebody has walked all the way in to the sun.",
    }
}

pub fn still_held(what: crate::trail::Asked) -> &'static str {
    use crate::trail::Asked;

    match what {
        Asked::OpenedTheCase => "A book is a handle, and nobody has pulled it.",
        Asked::WadedInThePool => "Nobody has been in the water.",
        Asked::SolvedTheCube => "Nobody has played with the cube and then solved it.",
        Asked::GotBehindTheWall => {
            "The room you started in has a secret nobody has walked through."
        }
        Asked::ReachedTheSun => "Nobody has walked all the way in to the sun.",
    }
}
