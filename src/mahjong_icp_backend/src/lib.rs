
use ic_cdk_macros;

use std::{cell::RefCell, collections::HashMap};

use candid::{CandidType};

use serde::Deserialize;


thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(Default)]
pub struct State {
    // a list of  user leaderboards for each board setup
    pub leaderboards: HashMap<String, Score>,
}

//Leaderboard is a struct containing best scores for a given board setup
 
// #[derive(Default, Clone)]
// pub struct Leaderboard {
//     pub score: Score,
// }

// //Score contains the user name and the score . The score is the shortest time in miliseconds of the user to solve the board
// #[derive(Default, CandidType, Clone, Deserialize)]
// pub struct Score {
//     //pub user: String,
//     pub miliseconds : u64,
// }

type Score = u64;

// abstract class IHighscoreDB {
//     Future<Map<String, int>> getTimes();
//     Future<void> set(String layout, int time);
//   }


#[ic_cdk_macros::query]
pub fn get_times() -> HashMap<String, Score> {
    STATE.with(|state| {
        let state = state.borrow();
        state.leaderboards.clone()
    })
}



#[ic_cdk_macros::update]
pub fn set_time(board_setup: String, score: Score) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let a_score = state.leaderboards.entry(board_setup).or_insert(Score::default());
        *a_score = score; // Dereference a_score before assigning
    })
}

ic_cdk::export_candid!();
