use ic_cdk_macros;

use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
};

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
    pub scores: BTreeMap<u32, String>,
}

// //Score contains the user name and the score . The score is the shortest time in miliseconds of the user to solve the board
#[derive(Default, CandidType, Clone, Deserialize, Debug)]
pub struct Score {
    pub user: String,
    pub miliseconds: u32,
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
        for (board_layout, leaderboard) in state.leaderboards.iter() {
            ic_cdk::println!(
                "get_best_scores_for_all_boards: Iterating over leaderboards with board_layout: {}",
                board_layout
            );
            if let Some((&miliseconds, _)) = leaderboard.scores.iter().next() {
                ic_cdk::println!(
                    "get_best_scores_for_all_boards: Found score for board_layout: {}, miliseconds: {}",
                    board_layout,
                    miliseconds
                );
                result.insert(board_layout.clone(), miliseconds);
            }
            ic_cdk::println!("get_best_scores_for_all_boards: Iteration done");
        }
    });

    STATE.with(|state| {
        let state = state.borrow();
        for (board_layout, leaderboard) in state.leaderboards.iter() {
            ic_cdk::println!("board_layout: {}", board_layout);
            for (&miliseconds, user) in leaderboard.scores.iter() {
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
        let leaderboard = state
            .leaderboards
            .entry(board_layout)
            .or_insert(Leaderboard {
                scores: BTreeMap::new(),
            });
        ic_cdk::println!("set_score: Leaderboard entry created");

        // Insert the score into the map
        leaderboard.scores.insert(miliseconds, user.clone());

        if leaderboard.scores.len() > MAX_LEADERBOARD_ENTRIES {
            let last_key = *leaderboard.scores.keys().rev().next().unwrap();
            leaderboard.scores.remove(&last_key);
        }
        ic_cdk::println!("set_score: Score inserted");
    });
}
#[ic_cdk_macros::update]
pub fn debug_delete_all_data() {
    ic_cdk::println!("debug_delete_all_data: Function called");

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        ic_cdk::println!("debug_delete_all_data: State borrowed");
        state.leaderboards.clear();
        ic_cdk::println!("debug_delete_all_data: State cleared");
    })
}

ic_cdk::export_candid!();
