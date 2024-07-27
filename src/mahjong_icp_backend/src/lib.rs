use ic_cdk_macros;

use std::{cell::RefCell, collections::HashMap, fmt};

use candid::CandidType;

use serde::Deserialize;

const MAX_LEADERBOARD_ENTRIES: usize = 10;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(Default, Debug)]
pub struct State {
    // a list of  user leaderboards for each board layout
    pub leaderboards: HashMap<String, Leaderboard>,
}
//Leaderboard is a collection containing best scores for a given board layout

#[derive(Default, CandidType, Clone, Debug)]
pub struct Leaderboard {
    pub scores: Vec<Score>,
}

// //Score contains the user name and the score . The score is the shortest time in miliseconds of the user to solve the board
#[derive(Default, CandidType, Clone, Deserialize, Debug)]
pub struct Score {
    pub miliseconds: u32,
    pub user: String,
}
impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User: {}, Score: {}", self.user, self.miliseconds)
    }
}

#[ic_cdk_macros::query]
pub fn get_scores_by_board(board_layout: String) -> Leaderboard {
    ic_cdk::println!(
        "get_scores_by_board: Function called with board_layout: {}",
        board_layout
    );

    let mut result = Leaderboard::default();
    STATE.with(|state| {
        let state = state.borrow();
        ic_cdk::println!("get_scores_by_board: State borrowed");
        if let Some(leaderboard) = state.leaderboards.get(&board_layout) {
            ic_cdk::println!(
                "get_scores_by_board: Found leaderboard for board_layout: {}",
                board_layout
            );
            result = leaderboard.clone();
        }
    });

    ic_cdk::println!("get_scores_by_board: Function done returning {:?}", result);

    result
}

#[ic_cdk_macros::query]
pub fn get_best_scores_for_all_boards() -> HashMap<String, u32> {
    ic_cdk::println!("get_best_scores_for_all_boards: Function called");

    let mut result = HashMap::new();
    STATE.with(|state| {
        let state = state.borrow();
        ic_cdk::println!("get_best_scores_for_all_boards: State borrowed");
        for (board_layout, leaderboard) in state.leaderboards.iter() {
            ic_cdk::println!(
                "get_best_scores_for_all_boards: Iterating over leaderboards with board_layout: {}",
                board_layout
            );
            if let Some(score) = leaderboard.scores.iter().next() {
                ic_cdk::println!(
                    "get_best_scores_for_all_boards: Found score for board_layout: {}, score: {}",
                    board_layout,
                    score
                );
                result.insert(board_layout.clone(), score.miliseconds);
            }
            ic_cdk::println!("get_best_scores_for_all_boards: Iteration done");
        }
    });

    STATE.with(|state| {
        let state = state.borrow();
        for (board_layout, leaderboard) in state.leaderboards.iter() {
            ic_cdk::println!("board_layout: {}", board_layout);
            for score in leaderboard.scores.iter() {
                let miliseconds = score.miliseconds;
                let user = &score.user;
                ic_cdk::println!("    miliseconds: {}, user: {}", miliseconds, user);
            }
        }
    });

    ic_cdk::println!(
        "get_best_scores_for_all_boards: Function done returning {:?}",
        result
    );

    result
}

#[ic_cdk_macros::update]
pub fn set_score(board_layout: String, miliseconds: u32, user: String) {
    ic_cdk::println!(
        "set_score: Function called with board_layout: {}, miliseconds: {}, user: {}",
        board_layout,
        miliseconds,
        user
    );

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        ic_cdk::println!("set_score: State borrowed");

        let leaderboard = state.leaderboards.entry(board_layout.clone()).or_default();
        ic_cdk::println!("set_score: Leaderboard entry created");

        let score = Score { user, miliseconds };
        ic_cdk::println!("set_score: Score created");

        let mut scores = leaderboard.scores.clone();
        ic_cdk::println!("set_score: Scores cloned");

        // insert score into the leaderboard, so that the leaderboard is sorted by score.miliseconds
        scores.push(score);
        scores.sort_by_key(|score| score.miliseconds);

        // remove the last score if the leaderboard has more than MAX_LEADERBOARD_ENTRIES
        if scores.len() > MAX_LEADERBOARD_ENTRIES {
            scores.pop();
        }

        leaderboard.scores = scores;

        ic_cdk::println!("set_score: Leaderboard scores updated");
    });

    ic_cdk::println!("set_score: Function done");
}
//#[ic_cdk_macros::update]
pub fn debug_delete_all_data() {
    ic_cdk::println!("debug_delete_all_data: Function called");

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        ic_cdk::println!("debug_delete_all_data: State borrowed");
        state.leaderboards.clear();
        ic_cdk::println!("debug_delete_all_data: State cleared");
    })
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaderboard_default() {
        let leaderboard = Leaderboard::default();
        assert!(leaderboard.scores.is_empty());
    }

    #[test]
    fn test_state_default() {
        let state = State::default();
        assert!(state.leaderboards.is_empty());
    }

    #[test]
    fn test_set_score() {
        let board_layout = "test_layout".to_string();
        let miliseconds = 100;
        let user = "test_user".to_string();

        set_score(board_layout.clone(), miliseconds, user.clone());

        STATE.with(|state| {
            let state = state.borrow();
            let leaderboard = state.leaderboards.get(&board_layout).unwrap();
            assert_eq!(leaderboard.scores.len(), 1);
            assert_eq!(leaderboard.scores[0].miliseconds, miliseconds);
            assert_eq!(leaderboard.scores[0].user, user);
        });
    }

    #[test]
    fn test_debug_delete_all_data() {
        let board_layout = "test_layout".to_string();
        let miliseconds = 100;
        let user = "test_user".to_string();

        set_score(board_layout.clone(), miliseconds, user.clone());

        debug_delete_all_data();

        STATE.with(|state| {
            let state = state.borrow();
            assert!(state.leaderboards.is_empty());
        });
    }

    // set scores more than 10 times and check that the leaderboard's length is still equal to 10
    #[test]
    fn test_set_score_max_entries() {
        let board_layout = "test_layout".to_string();
        let user = "test_user".to_string();

        for i in 0u32..(MAX_LEADERBOARD_ENTRIES as u32) + 1 {
            set_score(board_layout.clone(), i, user.clone());
        }

        STATE.with(|state| {
            let state = state.borrow();
            let leaderboard = state.leaderboards.get(&board_layout).unwrap();
            assert_eq!(leaderboard.scores.len(), MAX_LEADERBOARD_ENTRIES);
        });
    }

    // set scores a few times and see that the leaderboard is sorted by score
    #[test]
    fn test_set_score_sorted() {
        let board_layout = "test_layout".to_string();
        let user = "test_user".to_string();

        set_scores(&board_layout, vec![100, 50, 200, 150], &user);
        assert_layout_scores(vec![50, 100, 150, 200], &board_layout);

        // set a score with the same time as the first score
        set_score(board_layout.clone(), 50, user.clone());
        assert_layout_scores(vec![50, 50, 100, 150, 200], &board_layout);

        // set a score with the same time as the last score
        set_score(board_layout.clone(), 200, user.clone());
        assert_layout_scores(vec![50, 50, 100, 150, 200, 200], &board_layout);

        // set a score with the same time as the third score
        set_score(board_layout.clone(), 100, user.clone());
        assert_layout_scores(vec![50, 50, 100, 100, 150, 200, 200], &board_layout);

        //set a score after third score
        set_score(board_layout.clone(), 75, user.clone());
        assert_layout_scores(vec![50, 50, 75, 100, 100, 150, 200, 200], &board_layout);
    }

    // test pub fn get_scores_by_board(board_layout: String) -> Leaderboard
    #[test]
    fn test_get_scores_by_board() {
        // when no scores exist
        let leaderboard = get_scores_by_board("non_existent_layout".to_string());
        assert!(leaderboard.scores.is_empty());

        let board_layout = "test_layout".to_string();
        let user = "test_user".to_string();

        set_scores(&board_layout, vec![100, 50, 200, 150], &user);
        let leaderboard = get_scores_by_board(board_layout.clone());
        assert_leaderboard_scores(&[50, 100, 150, 200], &leaderboard);

        // try to get a leaderboard that does not exist
        let leaderboard = get_scores_by_board("non_existent_layout".to_string());
        assert!(leaderboard.scores.is_empty());

        // try another board layout
        let board_layout = "test_layout2".to_string();
        let user = "test_user".to_string();
        set_scores(&board_layout, vec![100, 50, 200, 150], &user);
        let leaderboard = get_scores_by_board(board_layout.clone());
        assert_leaderboard_scores(&[50, 100, 150, 200], &leaderboard);
    }

    // test pub fn get_best_scores_for_all_boards() -> Vec<Score>
    #[test]
    fn test_get_best_scores_for_all_boards() {
        //try to get best scores when there are no scores
        let best_scores = get_best_scores_for_all_boards();
        assert!(best_scores.is_empty());

        // now with some data
        let board_layout1 = "test_layout".to_string();
        let user = "test_user".to_string();

        set_scores(&board_layout1, vec![100, 50, 200, 150], &user);

        let best_scores = get_best_scores_for_all_boards();
        assert_eq!(best_scores.len(), 1);
        //assert that the only score is 50
        assert_eq!(best_scores[&board_layout1], 50);

        // try another board layout
        let board_layout2 = "test_layout2".to_string();
        let user = "test_user".to_string();

        set_scores(&board_layout2, vec![100, 50, 200, 150], &user);

        let best_scores = get_best_scores_for_all_boards();
        assert_eq!(best_scores.len(), 2);
        assert_eq!(best_scores[&board_layout1], 50);
        assert_eq!(best_scores[&board_layout2], 50);
    }

    // helper functions

    fn set_scores(board_layout: &String, scores: Vec<u32>, user: &String) {
        for score in scores {
            set_score(board_layout.clone(), score, user.clone());
        }
    }

    fn assert_layout_scores(expected_scores: Vec<u32>, board_layout: &String) {
        STATE.with(|state| {
            let state = state.borrow();
            let leaderboard = state.leaderboards.get(board_layout).unwrap();
            let scores = leaderboard
                .scores
                .iter()
                .map(|score| score.miliseconds)
                .collect::<Vec<u32>>();
            assert_eq!(scores, expected_scores);
        });
    }

    fn assert_leaderboard_scores(expected_scores: &[u32], leaderboard: &Leaderboard) {
        assert_eq!(leaderboard.scores.len(), expected_scores.len());
        for (i, score) in expected_scores.iter().enumerate() {
            assert_eq!(leaderboard.scores[i].miliseconds, *score);
        }
    }
}

ic_cdk::export_candid!();
