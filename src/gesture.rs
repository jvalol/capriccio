//! What somebody does while they are standing at a stop, per spec 0067.
//!
//! **The robot is the moment they arrive.** A round walks somebody to the
//! fire and then sets every joint to nought, so a figure that read as a
//! person while it was moving becomes a mannequin the instant it gets where
//! it was going. Standing is most of what anybody in this village is doing
//! at any second, so it is most of what they are.
//!
//! Jake, 2026-11-14: it only lands if it reads as natural and not robotic,
//! as though the motion was recorded by a human actor. Nothing here is
//! recorded and nothing here will be mistaken for it. What is written down
//! is the handful of things that actually separate a person standing from a
//! machine paused, each of them a rule rather than a keyframe.
//!
//! **Nothing is ever still.** A body holding a pose is breathing, swaying a
//! little, and carrying its weight on one foot until that foot complains.
//! Under every errand below there is an idle that never stops, and the
//! errand is added to it rather than replacing it.
//!
//! **The joints do not start together.** A shoulder leads and the elbow
//! follows it a moment later, which is the difference between an arm and a
//! lever. Every chain here is given that lag.
//!
//! **No two repetitions match.** The fourth stir is not the third stir. Each
//! turn of a cycle takes its own length and its own reach off a hash of who
//! is doing it and which turn it is.
//!
//! **And it starts and stops at rest.** The gesture fades up over the first
//! moments at a stop and back down before they leave, so nobody begins
//! mid-stroke or walks away with their arm out.

use std::f32::consts::{PI, TAU};

/// What somebody is at a stop for.
///
/// **Named for the errand and not for the motion.** `Stir` rather than
/// `swing the forearm`, because what the village wanted was a cook, and the
/// day the pot moves the cook should still be cooking.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errand {
    /// Standing about: at the end of a street, waiting for somebody.
    Wait,
    /// A pot over a fire.
    Stir,
    /// Something heavy off a wagon and onto the next thing.
    Lift,
    /// A bucket up a well, hand over hand.
    Draw,
    /// A horse's neck, which is slower than any of the others and stops to
    /// think halfway through.
    Tend,
    /// At a door: reach, and in.
    Knock,
}

/// Every joint this building can actually move on a standing figure, which
/// is fewer than a body has.
///
/// **No torso and no neck.** Both are drawn on the body's own yaw at a fixed
/// height, so leaning or looking about would be a change to every head
/// fitting in `main.rs` rather than a number here. `turn` leans on the whole
/// figure instead, which is cruder and is what there is. Spec 0067.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Stance {
    /// Right leg first, as the drawing counts them.
    pub hip: [f32; 2],
    pub knee: [f32; 2],
    pub shoulder: [f32; 2],
    pub elbow: [f32; 2],
    /// How far the hips are off where a standing body would hold them, in
    /// metres, which everything above them rides on.
    pub rise: f32,
    /// How far off the stop's own facing they have turned, in radians.
    pub turn: f32,
    /// How far the body is leaning forward from the hips, in radians.
    ///
    /// **The one thing that stops a gesture being an arm on a post.**
    /// Somebody reaching over a pot or lifting off a wagon takes their
    /// shoulders with them; a figure that holds a plumb torso while the arm
    /// goes out is doing neither of those and is visibly doing neither.
    pub lean: f32,
}

/// A number between -1 and 1 off two things that do not divide.
///
/// Not random: a stop is drawn again every frame and has to answer the same
/// way, and a picture test has to answer the same way tomorrow.
/// **`rem_euclid` and not `fract`.** Rust keeps the sign through `fract`, so
/// the obvious spelling of this returns -3 to 1 rather than -1 to 1: a
/// quarter of the cycle lengths below came out negative, which ran the
/// cycles backwards, and a quarter of the reaches came out inside out. It
/// looked like five separate faults in five separate errands.
fn wiggle(who: f32, what: f32) -> f32 {
    ((who * 12.9898 + what * 78.233).sin() * 43758.547).rem_euclid(1.0) * 2.0 - 1.0
}

/// Nought up to one, with the ends flat, which is how a body starts and
/// stops a movement.
fn eased(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);

    t * t * (3.0 - 2.0 * t)
}

/// Which turn of a cycle this is and how far through it, with every turn its
/// own length.
///
/// **The jitter is in the length and not only in the reach**, because a
/// motion that is always the same length is a metronome whatever its size.
fn turn_of(who: f32, stood: f32, every: f32) -> (f32, f32) {
    let mut from = 0.0;
    let mut n = 0.0;

    loop {
        let long = every * (1.0 + wiggle(who, n * 3.7) * 0.22);

        if stood < from + long || n > 64.0 {
            return (n, ((stood - from) / long).clamp(0.0, 1.0));
        }

        from += long;
        n += 1.0;
    }
}

/// What a body does when it is not doing anything: breathe, sway, and change
/// feet.
///
/// **The weight shift is the big one.** Standing on both feet evenly is a
/// thing people manage for a few seconds at a time; the rest of it is one
/// hip taking the load while the other leg goes soft, and swapping when that
/// gets tiring. It moves the hips, both knees and the whole body's facing at
/// once, which is why it reads as a person deciding rather than as a part
/// oscillating.
fn idle(who: f32, stood: f32) -> Stance {
    let mut out = Stance::default();

    // breath, which is four millimetres and the only thing here that is
    // regular, because breathing is
    let breath = ((stood / 3.7 + who) * TAU).sin();

    out.rise += breath * 0.004;

    // and a sway that never repeats, off two periods that do not divide
    out.turn += ((stood * 0.31 + who * 2.0).sin() * 0.013) + ((stood * 0.17 + who).sin() * 0.009);
    out.rise += (stood * 0.23 + who * 3.0).sin() * 0.003;

    // the foot they are standing on, which changes every six seconds or so
    let (n, through) = turn_of(who, stood, 6.5);
    let on = (n as usize) % 2;
    let other = 1 - on;
    // eased at both ends, so the change of feet is a movement and not a
    // switch
    let over = eased(through * 3.0) * (1.0 - eased(through * 3.0 - 2.0));
    let lean = 0.35 + wiggle(who, n * 1.3) * 0.25;

    out.knee[other] += (0.10 + wiggle(who, n * 5.1) * 0.04) * over;
    out.hip[other] += 0.07 * over;
    out.hip[on] -= 0.02 * over;
    out.rise -= 0.013 * over;
    out.turn += (if on == 0 { 1.0 } else { -1.0 }) * 0.045 * lean * over;
    // and the arm on the loose side hangs a little further back
    out.shoulder[other] += 0.06 * over;
    // and the shoulders come back a little over the loaded hip
    out.lean -= 0.012 * over;

    out
}

/// The two arms of a swing, with the elbow lagging the shoulder.
///
/// **The lag is the whole of it.** Driven off one phase the arm is a lever
/// with a hinge in it; a tenth of a cycle between the two and it is an arm.
fn swing(through: f32, lag: f32) -> (f32, f32) {
    let lead = ((through * TAU) - PI * 0.5).sin() * 0.5 + 0.5;
    let follow = (((through - lag).rem_euclid(1.0) * TAU) - PI * 0.5).sin() * 0.5 + 0.5;

    (lead, follow)
}

/// What somebody at a stop is doing with themselves at this second.
///
/// `stood` is how long they have been there and `stays` how long they are
/// staying, both in seconds. `who` is anything that tells one person from
/// another and stays the same from frame to frame.
pub fn standing(errand: Errand, who: f32, stood: f32, stays: f32) -> Stance {
    let mut out = idle(who, stood);

    // **Up on arriving and down before leaving.** A gesture at full size on
    // the first frame is somebody who was already stirring before they got
    // there, and one that stops dead is somebody switched off.
    let settled = eased(stood / 0.8) * eased((stays - stood) / 0.7);

    if settled <= 0.0 || errand == Errand::Wait {
        if errand == Errand::Wait {
            waiting(&mut out, who, stood, settled);
        }

        return out;
    }

    match errand {
        Errand::Wait => {}
        Errand::Stir => {
            let (n, through) = turn_of(who, stood, 1.9);
            let (shoulder, elbow) = swing(through, 0.12);
            let reach = 1.0 + wiggle(who, n * 2.3) * 0.18;

            // **The working arm, held out over the pot and not swinging
            // beside the hip.** The first build put the shoulder at a
            // quarter of a radian, which is an arm hanging down with a
            // twitch in it: a hand over a pot a stride away wants the
            // shoulder most of the way forward and the elbow folded, so the
            // forearm is the thing lying across the fire. What moves is
            // small and on top of that.
            out.shoulder[0] += (0.82 + shoulder * 0.17) * reach * settled;
            out.elbow[0] += (0.95 + elbow * 0.35) * reach * settled;
            // the other hand rests on something, and is not a mirror of it
            out.shoulder[1] += 0.10 * settled;
            out.elbow[1] += (0.5 + (stood * 0.4 + who).sin() * 0.06) * settled;
            // a cook leans over the pot, and sways with the stroke rather
            // than holding the lean still
            out.rise -= 0.035 * settled;
            out.lean += (0.20 + shoulder * 0.05) * settled;
        }
        Errand::Lift => {
            let (n, through) = turn_of(who, stood, 3.4);
            let (shoulder, elbow) = swing(through, 0.16);
            let reach = 1.0 + wiggle(who, n * 4.1) * 0.2;

            // both arms, because it is heavy, and the near one leads
            for side in 0..2 {
                let lag = if side == 0 { 0.0 } else { 0.05 };
                let (shoulder, elbow) = swing((through - lag).rem_euclid(1.0), 0.16);

                out.shoulder[side] += (0.15 + shoulder * 0.55) * reach * settled;
                out.elbow[side] += (0.55 + elbow * 0.75) * reach * settled;
            }

            let _ = (shoulder, elbow);

            // the knees take it at the bottom of the lift
            let down = (1.0 - (through * TAU).cos()) * 0.5;

            out.knee[0] += 0.22 * down * settled;
            out.knee[1] += 0.20 * down * settled;
            out.rise -= 0.09 * down * settled;
            // **Bent to it at the bottom and upright at the top**, which is
            // most of what says the thing has weight.
            out.lean += (0.08 + down * 0.30) * settled;
        }
        Errand::Draw => {
            let (n, through) = turn_of(who, stood, 1.5);
            let reach = 1.0 + wiggle(who, n * 1.7) * 0.15;

            // hand over hand, so the two arms are half a turn apart and
            // neither is ever where the other is
            for side in 0..2 {
                let at = (through + side as f32 * 0.5).rem_euclid(1.0);
                let (shoulder, elbow) = swing(at, 0.1);

                out.shoulder[side] += (0.30 + shoulder * 0.42) * reach * settled;
                out.elbow[side] += (1.25 - elbow * 0.85) * reach * settled;
            }

            out.rise -= 0.02 * settled;
            out.lean += 0.14 * settled;
        }
        Errand::Tend => {
            let (n, through) = turn_of(who, stood, 2.6);
            let (shoulder, elbow) = swing(through, 0.14);
            // **It stops to think.** Every third turn or so the hand comes
            // to rest where it is for a beat, which no amount of smoothing
            // does for a cycle that never pauses.
            let resting = if wiggle(who, n * 6.7) > 0.35 {
                0.25
            } else {
                1.0
            };
            let reach = (1.0 + wiggle(who, n * 2.9) * 0.2) * resting;

            out.shoulder[0] += (0.42 + shoulder * 0.20) * reach * settled;
            out.elbow[0] += (0.55 + elbow * 0.30) * reach * settled;
            out.shoulder[1] += -0.05 * settled;
            out.turn += (through - 0.5) * 0.06 * settled;
            out.lean += 0.07 * reach * settled;
        }
        Errand::Knock => {
            // once, not a cycle: reach for the latch, then let the arm down
            let reach = eased(stood / 0.9) * (1.0 - eased((stood - 1.4) / 0.9));

            out.shoulder[0] += 0.85 * reach;
            out.elbow[0] += 0.95 * reach;
            out.lean += 0.10 * reach;
            // and afterwards they stand there with their hands down, which
            // the idle under this is already doing
        }
    }

    out
}

/// Standing about, which is its own errand and not the absence of one.
///
/// Somebody waiting at the end of a street looks round, because a person
/// with nothing to do looks at things. Held for a beat and brought back,
/// rather than swept, which is what makes it a glance.
fn waiting(out: &mut Stance, who: f32, stood: f32, settled: f32) {
    let (n, through) = turn_of(who, stood, 4.2);
    let away = wiggle(who, n * 8.3);

    if away.abs() < 0.3 {
        return;
    }

    let held = eased(through * 4.0) * (1.0 - eased(through * 4.0 - 3.0));

    out.turn += away * 0.33 * held * settled;
    out.shoulder[0] += 0.05 * held;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EVERY: Errand = Errand::Wait;

    fn all() -> [Errand; 6] {
        [
            Errand::Wait,
            Errand::Stir,
            Errand::Lift,
            Errand::Draw,
            Errand::Tend,
            Errand::Knock,
        ]
    }

    fn apart(one: &Stance, two: &Stance) -> f32 {
        let mut most: f32 = 0.0;

        for n in 0..2 {
            most = most.max((one.hip[n] - two.hip[n]).abs());
            most = most.max((one.knee[n] - two.knee[n]).abs());
            most = most.max((one.shoulder[n] - two.shoulder[n]).abs());
            most = most.max((one.elbow[n] - two.elbow[n]).abs());
        }

        most.max((one.turn - two.turn).abs())
            .max((one.lean - two.lean).abs())
            .max((one.rise - two.rise).abs() * 10.0)
    }

    /// The fault this whole module is for: a figure that arrives and freezes.
    #[test]
    fn nobody_standing_still_is_actually_still() {
        let _ = EVERY;

        for errand in all() {
            for who in [0.3f32, 1.7, 4.9] {
                let mut most: f32 = 0.0;

                for step in 0..60 {
                    let stood = 1.0 + step as f32 * 0.1;
                    let now = standing(errand, who, stood, 9.0);
                    let next = standing(errand, who, stood + 0.1, 9.0);

                    most = most.max(apart(&now, &next));
                }

                assert!(
                    most > 0.004,
                    "{:?} moves at most {:.5} in a tenth of a second, which is a statue",
                    errand,
                    most
                );
            }
        }
    }

    /// And a figure that does the same thing over and over is the other
    /// half of the same fault.
    #[test]
    fn no_two_turns_of_a_cycle_are_the_same() {
        for (errand, every) in [
            (Errand::Stir, 1.9),
            (Errand::Lift, 3.4),
            (Errand::Draw, 1.5),
            (Errand::Tend, 2.6),
        ] {
            let who = 2.1;
            let mut most: f32 = 0.0;

            for step in 0..40 {
                let stood = 1.5 + step as f32 * 0.05;
                let now = standing(errand, who, stood, 30.0);
                let again = standing(errand, who, stood + every, 30.0);

                most = most.max(apart(&now, &again));
            }

            assert!(
                most > 0.02,
                "{:?} comes back to the same pose a cycle later, within {:.4}",
                errand,
                most
            );
        }
    }

    /// Arriving and leaving are movements too.
    #[test]
    fn a_gesture_fades_up_and_down_rather_than_switching_on() {
        for errand in all() {
            let stays = 6.0;
            let who = 0.9;
            let arriving = standing(errand, who, 0.0, stays);
            let settled = standing(errand, who, 3.0, stays);
            let leaving = standing(errand, who, stays, stays);
            let resting = idle(who, 0.0);

            assert!(
                apart(&arriving, &resting) < 0.05,
                "{:?} is already underway on the frame they arrive",
                errand
            );
            assert!(
                apart(&leaving, &idle(who, stays)) < 0.05,
                "{:?} is still going on the frame they leave",
                errand
            );

            // **Over the stay and not at one instant**, because `Tend`
            // rests its hand for a beat every few turns on purpose, and a
            // sample that lands in one of those says the errand does
            // nothing when what it is doing is pausing.
            if errand != Errand::Wait && errand != Errand::Knock {
                let most = (0..50)
                    .map(|step| {
                        let stood = 1.0 + step as f32 * 0.08;

                        apart(&standing(errand, who, stood, stays), &idle(who, stood))
                    })
                    .fold(0.0f32, f32::max);

                assert!(most > 0.2, "{:?} never actually does anything", errand);
            }

            let _ = settled;
        }
    }

    /// A shoulder leads and an elbow follows, which is what tells an arm
    /// from a lever.
    ///
    /// **Two signals rather than two peaks.** Comparing the instant each one
    /// is furthest over sounds like the same question and is not: a cycle
    /// that pauses, as `Tend` does on purpose, puts both maxima in whatever
    /// quiet stretch the idle happens to peak in, and the test reads that as
    /// one hinge. Normalising both curves and asking how far apart they stay
    /// does not care where the maxima are.
    #[test]
    fn the_shoulder_and_the_elbow_do_not_turn_together() {
        for errand in [Errand::Stir, Errand::Lift, Errand::Draw, Errand::Tend] {
            let who = 1.3;
            let taken: Vec<(f32, f32)> = (0..400)
                .map(|step| {
                    let now = standing(errand, who, 2.0 + step as f32 * 0.02, 30.0);

                    (now.shoulder[0], now.elbow[0])
                })
                .collect();

            let span = |pick: fn(&(f32, f32)) -> f32| {
                let low = taken.iter().map(pick).fold(f32::MAX, f32::min);
                let high = taken.iter().map(pick).fold(f32::MIN, f32::max);

                (low, (high - low).max(1e-6))
            };
            let (shoulder_low, shoulder_span) = span(|at| at.0);
            let (elbow_low, elbow_span) = span(|at| at.1);
            let apart = taken
                .iter()
                .map(|(shoulder, elbow)| {
                    ((shoulder - shoulder_low) / shoulder_span - (elbow - elbow_low) / elbow_span)
                        .abs()
                })
                .sum::<f32>()
                / taken.len() as f32;

            assert!(
                apart > 0.05,
                "{:?} moves its shoulder and its elbow as one signal, {:.4} apart on \
                 average, which is one hinge and not two",
                errand,
                apart
            );
        }
    }

    /// And none of it bends anywhere a body does not.
    #[test]
    fn nothing_bends_the_wrong_way_or_too_far() {
        for errand in all() {
            for who in [0.2f32, 2.6, 5.4] {
                for step in 0..400 {
                    let stood = step as f32 * 0.025;
                    let now = standing(errand, who, stood, 10.0);

                    for n in 0..2 {
                        assert!(
                            (-0.2..=0.6).contains(&now.knee[n]),
                            "{:?} bends a knee to {:.2}",
                            errand,
                            now.knee[n]
                        );
                        assert!(
                            (-0.1..=2.4).contains(&now.elbow[n]),
                            "{:?} bends an elbow to {:.2}",
                            errand,
                            now.elbow[n]
                        );
                        assert!(
                            (-0.8..=1.6).contains(&now.shoulder[n]),
                            "{:?} swings a shoulder to {:.2}",
                            errand,
                            now.shoulder[n]
                        );
                    }

                    assert!(
                        now.rise.abs() < 0.2,
                        "{:?} moves the hips {:.3} metres",
                        errand,
                        now.rise
                    );
                    assert!(
                        (-0.1..=0.55).contains(&now.lean),
                        "{:?} leans {:.2}, which is a bow or a backflip",
                        errand,
                        now.lean
                    );
                    assert!(
                        now.turn.abs() < 0.6,
                        "{:?} turns {:.2} off their facing",
                        errand,
                        now.turn
                    );
                }
            }
        }
    }
}
