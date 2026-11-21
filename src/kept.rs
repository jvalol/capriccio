//! Where the building keeps what it remembers, and the reading and
//! writing of it. Spec 0120.
//!
//! **The file half, and the trail half is in `trail`.** What a trail
//! looks like written down is a fact about a trail; where that goes on
//! this machine is a fact about this machine, and neither wants to be
//! read while looking at the other.
//!
//! **Nothing here returns an error.** A missing file is somebody who has
//! not been here, an unreadable one is the same, and a write that fails
//! is a run that will write again the next time a room answers. What none
//! of them may ever be is a reason the building does not open.
//!
//! **And nothing here is reached from a test.** `Arcade::new` remembers
//! nowhere, `main` is what hands it a path, and the risk this guards is
//! worth more than the feature: a thousand tests build an arcade and two
//! of them compare pictures of the field, so a `new` that read a save
//! would make the gate pass or fail by whatever was last played.

use crate::trail::Trail;
use std::path::{Path, PathBuf};

/// What the file is called inside its folder.
pub const KEPT: &str = "trail";

/// The folder the arcade keeps its own things in.
pub const UNDER: &str = "capriccio";

/// Where this machine puts a program's data, with the arcade's folder on
/// the end of it, or nothing if it cannot be worked out.
///
/// **Written out rather than taken from a crate.** This is two
/// environment variables and a join, and the alternative is a dependency
/// in a game whose whole list is the engine and `glam`.
///
/// **Nothing rather than a fallback.** A home-less environment is a build
/// server or a sandbox, and the right thing to do there is forget, which
/// is what the building did everywhere until today.
pub fn where_it_remembers() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").map(PathBuf::from)?;

    // macOS keeps a program's data under Application Support, and the
    // rest of them under the XDG share folder. Asked of the target rather
    // than of the environment, because a Mac has no `XDG_DATA_HOME` and
    // would otherwise land in a `.local` nobody on a Mac looks in.
    let folder = if cfg!(target_os = "macos") {
        home.join("Library").join("Application Support")
    } else {
        std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".local").join("share"))
    };

    Some(folder.join(UNDER).join(KEPT))
}

/// What is written there, or an empty trail.
pub fn read(at: &Path) -> Trail {
    std::fs::read_to_string(at)
        .map(|text| Trail::read_back(&text))
        .unwrap_or_default()
}

/// Write it there, making the folder if it is not there yet.
///
/// **Whole file at a time, and it is five lines.** There is nothing here
/// worth appending to and an append would double every station the second
/// time a run wrote.
pub fn write(at: &Path, trail: &Trail) {
    if let Some(folder) = at.parent() {
        let _ = std::fs::create_dir_all(folder);
    }

    let _ = std::fs::write(at, trail.written());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trail::Asked;

    /// Spec 0120: what is written comes back in the order it went.
    ///
    /// **Through the file and not only through the string.** The two
    /// halves are tested apart in `trail`; what this adds is that the
    /// thing on the disk is the thing that comes off it.
    #[test]
    fn what_goes_to_the_file_comes_back_off_it() {
        let at = std::env::temp_dir()
            .join("capriccio-kept-test")
            .join("trail");
        let mut trail = Trail::new();

        trail.gave(Asked::ReachedTheSun);
        trail.gave(Asked::OpenedTheCase);

        write(&at, &trail);

        assert_eq!(
            read(&at).given(),
            trail.given(),
            "what came back off the disk is not what went on it"
        );

        let _ = std::fs::remove_file(&at);
    }

    /// Spec 0120: nothing to read is somebody who has not been here.
    #[test]
    fn nothing_to_read_is_a_first_visit() {
        let nowhere = std::env::temp_dir().join("capriccio-kept-no-such-file");

        let _ = std::fs::remove_file(&nowhere);

        assert!(
            read(&nowhere).is_empty(),
            "a file that is not there answered something"
        );

        // and a folder where a file should be, which is the unreadable
        // case that can actually happen: `read_to_string` of a directory
        // is an error on every platform this builds for
        let folder = std::env::temp_dir().join("capriccio-kept-a-folder");
        let _ = std::fs::create_dir_all(&folder);

        assert!(
            read(&folder).is_empty(),
            "a file that could not be read answered something"
        );

        let _ = std::fs::remove_dir(&folder);
    }

    /// Spec 0120: the place is under the arcade's own folder and is named.
    ///
    /// **Not asserted to be any one path**, which would be this machine's
    /// answer written down twice. What it does check is that it is not the
    /// home folder itself and that both names are in it, a join that lost
    /// one of them being the way this goes wrong.
    #[test]
    fn it_remembers_somewhere_of_its_own() {
        let Some(at) = where_it_remembers() else {
            // a home-less environment forgets, per the note above
            return;
        };

        assert_eq!(
            at.file_name().and_then(|one| one.to_str()),
            Some(KEPT),
            "the file is not called {}",
            KEPT
        );
        assert_eq!(
            at.parent().and_then(|one| one.file_name()?.to_str()),
            Some(UNDER),
            "it is not in a folder of the arcade's own"
        );
        assert!(
            at.components().count() > 3,
            "it is at {}, which is somebody's home folder rather than a \
             place for a program's data",
            at.display()
        );
    }
}
