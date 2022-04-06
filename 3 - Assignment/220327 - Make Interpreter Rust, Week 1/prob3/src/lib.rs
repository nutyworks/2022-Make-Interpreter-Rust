#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub enum ThrowResult {
    Normal(u16),
    Spare(u16),
    Bonus(u16),
    Strike,
}

impl ThrowResult {
    fn to_score(&self) -> u16 {
        match self {
            Self::Normal(n) => *n,
            Self::Spare(n) => *n,
            Self::Bonus(n) => *n,
            Self::Strike => 10,
        }
    }
}

pub struct BowlingGame {
    frame: u8,
    is_bonus_throw: bool,
    throws_in_frame: u8,
    pins_left: u16,
    result: Vec<ThrowResult>,
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frame: 1,
            is_bonus_throw: false,
            throws_in_frame: 0,
            pins_left: 10,
            result: vec![ThrowResult::Normal(0), ThrowResult::Normal(0)],
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_complete() {
            return Err(Error::GameComplete);
        }

        if !self.has_enough_pins(pins) {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.remove_pins(pins);

        let throw_result = self.get_throw_result_with_pins(pins);
        self.add_throw_result(throw_result);

        self.increase_throws_in_frame();

        if self.should_proceed_to_bonus_throw() {
            self.set_bonus_throw();
        }

        if self.should_proceed_to_next_frame() {
            self.set_new_frame();
        }

        Ok(())
    }

    fn increase_throws_in_frame(&mut self) {
        self.throws_in_frame += 1;
    }

    fn remove_pins(&mut self, pins: u16) {
        self.pins_left -= pins;
    }

    fn add_throw_result(&mut self, throw_result: ThrowResult) {
        self.result.push(throw_result);
    }

    fn get_throw_result_with_pins(&self, pins: u16) -> ThrowResult {
        if self.is_bonus_throw {
            ThrowResult::Bonus(pins)
        } else if pins == 10 {
            ThrowResult::Strike
        } else if self.pins_left == 0 {
            ThrowResult::Spare(pins)
        } else {
            ThrowResult::Normal(pins)
        }
    }

    fn has_enough_pins(&self, pins: u16) -> bool {
        self.pins_left >= pins
    }

    fn set_bonus_throw(&mut self) {
        self.pins_left = 10;
        self.is_bonus_throw = true;
    }

    fn should_proceed_to_bonus_throw(&self) -> bool {
        self.pins_left == 0 && self.frame == 10
    }

    fn should_proceed_to_next_frame(&self) -> bool {
        if self.frame == 10 && self.is_bonus_throw {
            self.throws_in_frame >= 3
        } else {
            self.pins_left == 0 || self.throws_in_frame >= 2
        }
    }

    fn is_game_complete(&self) -> bool {
        self.frame > 10
    }

    fn set_new_frame(&mut self) {
        self.frame += 1;
        self.throws_in_frame = 0;
        self.pins_left = 10;
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_game_complete() {
            Some(self.calculate_score())
        } else {
            None
        }
    }

    fn calculate_score(&self) -> u16 {
        self.result
            .windows(3)
            .map(Self::calculate_score_of_throw)
            .sum()
    }

    fn calculate_score_of_throw(x: &[ThrowResult]) -> u16 {
        Self::calculate_completed_strike_or_zero(x)
            + Self::calculate_completed_spare_or_zero(x)
            + Self::calculate_normal_hits_or_zero(x)
    }

    fn calculate_completed_strike_or_zero(x: &[ThrowResult]) -> u16 {
        if let [ThrowResult::Strike, a, b] = x {
            10 + a.to_score() + b.to_score()
        } else {
            0
        }
    }

    fn calculate_completed_spare_or_zero(x: &[ThrowResult]) -> u16 {
        if let [_, ThrowResult::Spare(spare), a] = x {
            spare + a.to_score()
        } else {
            0
        }
    }

    fn calculate_normal_hits_or_zero(x: &[ThrowResult]) -> u16 {
        if let [_, _, ThrowResult::Normal(normal)] = x {
            *normal
        } else {
            0
        }
    }
}

#[test]
fn roll_returns_a_result() {
    let mut game = BowlingGame::new();
    assert!(game.roll(0).is_ok());
}

#[test]
fn you_cannot_roll_more_than_ten_pins_in_a_single_roll() {
    let mut game = BowlingGame::new();

    assert_eq!(game.roll(11), Err(Error::NotEnoughPinsLeft));
}

#[test]
fn a_game_score_is_some_if_ten_frames_have_been_rolled() {
    let mut game = BowlingGame::new();

    for _ in 0..10 {
        let _ = game.roll(0);
        let _ = game.roll(0);
    }

    assert!(game.score().is_some());
}

#[test]
fn you_cannot_score_a_game_with_no_rolls() {
    let game = BowlingGame::new();

    assert_eq!(game.score(), None);
}

#[test]
fn a_game_score_is_none_if_fewer_than_ten_frames_have_been_rolled() {
    let mut game = BowlingGame::new();

    for _ in 0..9 {
        let _ = game.roll(0);
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), None);
}

#[test]
fn a_roll_is_err_if_the_game_is_done() {
    let mut game = BowlingGame::new();

    for _ in 0..10 {
        let _ = game.roll(0);
        let _ = game.roll(0);
    }

    assert_eq!(game.roll(0), Err(Error::GameComplete));
}

#[test]
fn twenty_zero_pin_rolls_scores_zero() {
    let mut game = BowlingGame::new();

    for _ in 0..20 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(0));
}

#[test]
fn ten_frames_without_a_strike_or_spare() {
    let mut game = BowlingGame::new();

    for _ in 0..10 {
        let _ = game.roll(3);
        let _ = game.roll(6);
    }

    assert_eq!(game.score(), Some(90));
}

#[test]
fn spare_in_the_first_frame_followed_by_zeros() {
    let mut game = BowlingGame::new();

    let _ = game.roll(6);
    let _ = game.roll(4);

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(10));
}

#[test]
fn points_scored_in_the_roll_after_a_spare_are_counted_twice_as_a_bonus() {
    let mut game = BowlingGame::new();

    let _ = game.roll(6);
    let _ = game.roll(4);
    let _ = game.roll(3);

    for _ in 0..17 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(16));
}

#[test]
fn consecutive_spares_each_get_a_one_roll_bonus() {
    let mut game = BowlingGame::new();

    let _ = game.roll(5);
    let _ = game.roll(5);
    let _ = game.roll(3);
    let _ = game.roll(7);
    let _ = game.roll(4);

    for _ in 0..15 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(31));
}

#[test]
fn if_the_last_frame_is_a_spare_you_get_one_extra_roll_that_is_scored_once() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(5);
    let _ = game.roll(5);
    let _ = game.roll(7);

    assert_eq!(game.score(), Some(17));
}

#[test]
fn a_strike_earns_ten_points_in_a_frame_with_a_single_roll() {
    let mut game = BowlingGame::new();

    let _ = game.roll(10);

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(10));
}

#[test]
fn points_scored_in_the_two_rolls_after_a_strike_are_counted_twice_as_a_bonus() {
    let mut game = BowlingGame::new();

    let _ = game.roll(10);
    let _ = game.roll(5);
    let _ = game.roll(3);

    for _ in 0..16 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(26));
}

#[test]
fn consecutive_strikes_each_get_the_two_roll_bonus() {
    let mut game = BowlingGame::new();

    let _ = game.roll(10);
    let _ = game.roll(10);
    let _ = game.roll(10);
    let _ = game.roll(5);
    let _ = game.roll(3);

    for _ in 0..12 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(81));
}

#[test]
fn a_strike_in_the_last_frame_earns_a_two_roll_bonus_that_is_counted_once() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);
    let _ = game.roll(7);
    let _ = game.roll(1);

    assert_eq!(game.score(), Some(18));
}

#[test]
fn a_spare_with_the_two_roll_bonus_does_not_get_a_bonus_roll() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);
    let _ = game.roll(7);
    let _ = game.roll(3);

    assert_eq!(game.score(), Some(20));
}

#[test]
fn strikes_with_the_two_roll_bonus_do_not_get_a_bonus_roll() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);
    let _ = game.roll(10);
    let _ = game.roll(10);

    assert_eq!(game.score(), Some(30));
}

#[test]
fn a_strike_with_the_one_roll_bonus_after_a_spare_in_the_last_frame_does_not_get_a_bonus() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(7);
    let _ = game.roll(3);
    let _ = game.roll(10);

    assert_eq!(game.score(), Some(20));
}

#[test]
fn all_strikes_is_a_perfect_score_of_300() {
    let mut game = BowlingGame::new();

    for _ in 0..12 {
        let _ = game.roll(10);
    }

    assert_eq!(game.score(), Some(300));
}

#[test]
fn you_cannot_roll_more_than_ten_pins_in_a_single_frame() {
    let mut game = BowlingGame::new();

    assert!(game.roll(5).is_ok());
    assert_eq!(game.roll(6), Err(Error::NotEnoughPinsLeft));
}

#[test]
fn first_bonus_ball_after_a_final_strike_cannot_score_an_invalid_number_of_pins() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert_eq!(game.roll(11), Err(Error::NotEnoughPinsLeft));
}

#[test]
fn the_two_balls_after_a_final_strike_cannot_score_an_invalid_number_of_pins() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert!(game.roll(5).is_ok());
    assert_eq!(game.roll(6), Err(Error::NotEnoughPinsLeft));
}

#[test]
fn the_two_balls_after_a_final_strike_can_be_a_strike_and_non_strike() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert!(game.roll(10).is_ok());
    assert!(game.roll(6).is_ok());
}

#[test]
fn the_two_balls_after_a_final_strike_cannot_be_a_non_strike_followed_by_a_strike() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert!(game.roll(6).is_ok());
    assert_eq!(game.roll(10), Err(Error::NotEnoughPinsLeft));
}

#[test]
fn second_bonus_ball_after_a_final_strike_cannot_score_an_invalid_number_of_pins_even_if_first_is_strike(
) {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert!(game.roll(10).is_ok());
    assert_eq!(game.roll(11), Err(Error::NotEnoughPinsLeft));
}

#[test]
fn if_the_last_frame_is_a_strike_you_cannot_score_before_the_extra_rolls_are_taken() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert_eq!(game.score(), None);

    let _ = game.roll(10);

    assert_eq!(game.score(), None);

    let _ = game.roll(10);

    assert!(game.score().is_some());
}

#[test]
fn if_the_last_frame_is_a_spare_you_cannot_create_a_score_before_extra_roll_is_taken() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(5);
    let _ = game.roll(5);

    assert_eq!(game.score(), None);

    let _ = game.roll(10);

    assert!(game.score().is_some());
}

#[test]
fn cannot_roll_after_bonus_roll_for_spare() {
    let mut game = BowlingGame::new();

    for _ in 0..9 {
        let _ = game.roll(0);
        let _ = game.roll(0);
    }
    let _ = game.roll(7);
    let _ = game.roll(3);
    assert!(game.roll(2).is_ok());

    assert_eq!(game.roll(2), Err(Error::GameComplete));
}

#[test]
fn cannot_roll_after_bonus_roll_for_strike() {
    let mut game = BowlingGame::new();

    for _ in 0..9 {
        let _ = game.roll(0);
        let _ = game.roll(0);
    }
    let _ = game.roll(10);
    let _ = game.roll(3);
    assert!(game.roll(2).is_ok());

    assert_eq!(game.roll(2), Err(Error::GameComplete));
}

#[test]
fn last_two_strikes_followed_by_only_last_bonus_with_non_strike_points() {
    let mut game = BowlingGame::new();
    for _ in 0..16 {
        let _ = game.roll(0);
    }
    let _ = game.roll(10);
    let _ = game.roll(10);
    let _ = game.roll(0);
    let _ = game.roll(1);

    assert_eq!(game.score(), Some(31));
}
