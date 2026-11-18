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

/// What `--help` says about the arcade's own options, above the engine's.
///
/// **One block with one marker on a line of its own**, which is the form
/// that leaves nothing behind when `sign-copy` takes the marker out. A
/// marker per line would sign off into eight stray gaps.
///
/// Spec 0069: every one of these was an environment variable until Jake
/// said the arguments seem to belong in different places. They do not any
/// more, and `tests::the_help_names_every_option_the_arcade_takes` holds
/// this list against the one the code actually reads.
pub const OPTIONS: &str = "\
The arcade takes these of its own:

  --done                   every question on the trail already answered, so
                           the bridge and the hall at the top of it are there
  --pose <seven numbers>   stand here, at this moment, rather than at the
                           door in the afternoon: the line the plate prints
                           under M, with or without commas
  --where                  print where you are standing, twice a second, on
                           stderr
  --cost                   open with the frame readout up, rather than
                           reaching for F
  --at <0 to 1>            what time it is when the run starts: nought dawn,
                           a quarter noon, a half dusk
  --day <seconds>          how long a day lasts
  --sky <name>             hold the weather: clear, fair, overcast or storm
  --fly                    fly rather than walk
  --open-case              hold the hall of mirrors' bookcase open
  --dump-mirrors           write every mirror's view into /tmp, once
  --screenshot [<name>]    take the poster, or one of the others by name
  --screenshots            print the pictures and their files, and stop
  --walk <route>           walk a written route with no window, and fail if
                           it no longer goes where it says
  --tour <x,y,z>           work a route out to a place, and print it
  --film <clip>            film a clip

";

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
///
/// `WentOutIntoTheField` has a line it can never show, this stone
/// standing out in the field: nobody can read it without having answered
/// that one. It is written anyway, because "the stone is in the field" is
/// a fact about where the stone is today and leaving the line out would
/// tie the copy to it. Move the stone and the hole would be silent.
pub fn still_held(what: crate::trail::Asked) -> &'static str {
    use crate::trail::Asked;

    match what {
        Asked::WentOutIntoTheField => "The grass past the garden wall has not been walked.",
        Asked::OpenedTheCase => "A book is a handle, and nobody has pulled it.",
        Asked::WadedInThePool => "Nobody has been in the water.",
        Asked::SolvedTheCube => "The cube on the bench is not as it was handed over.",
        Asked::GotBehindTheWall => "The hall's end wall has a secret nobody has walked through.",
        Asked::RodeToTheSun => "Nobody has ridden out as far as the sun.",
    }
}
