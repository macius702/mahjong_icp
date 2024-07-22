
use ic_cdk_macros;

use std::{cell::RefCell, collections::HashMap};

use candid::{CandidType};

use serde::Deserialize;

use std::collections::BinaryHeap;
use std::cmp::Reverse;


thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(Default, Debug)]
pub struct State {
    // a list of  user leaderboards for each board setup
    pub leaderboards: HashMap<String, Leaderboard>,
}

//Leaderboard is a collection containing best scores for a given board setup
 
#[derive(Default, Clone, Debug)]
pub struct Leaderboard {
    pub scores: BinaryHeap<(Reverse<u32>, String)>,
}

// //Score contains the user name and the score . The score is the shortest time in miliseconds of the user to solve the board
#[derive(Default, CandidType, Clone, Deserialize, Debug)]
pub struct Score {
    pub user: String,
    pub miliseconds : u32,
}




#[ic_cdk_macros::query]
pub fn get_times() -> HashMap<String, u32> {
    ic_cdk::println!("get_times: Function called");
    

    //take from every leaderboard only the first score (if any)
    // and make an entry of it - leaderbord name, miliseconds
    // then pack them into a Hashmap

    let mut result = HashMap::new();
    STATE.with(|state| {
        let state = state.borrow();
        for (board_setup, leaderboard) in state.leaderboards.iter() {
            ic_cdk::println!("get_times: Iterating over leaderboards with board_setup: {}", board_setup);
            if let Some((Reverse(miliseconds), _)) = leaderboard.scores.peek() {
                ic_cdk::println!("get_times: Found score for board_setup: {}, miliseconds: {}", board_setup, miliseconds);
                result.insert(board_setup.clone(), *miliseconds);
            }
            ic_cdk::println!("get_times: Iteration done");
        }
    });

    //only print full STATE
    STATE.with(|state| {
       let state = state.borrow();
       ic_cdk::println!("get_times: Full state: {:?}", state);
    });





    ic_cdk::println!("get_times: Function done returning {:?}", result);

    result
    
}


#[ic_cdk_macros::update]
pub fn set_time(board_setup: String, miliseconds: u32, user: String) {
    ic_cdk::println!("set_time: Function called with board_setup: {}, miliseconds: {}, user: {}", board_setup, miliseconds, user);


    STATE.with(|state| {
        let mut state = state.borrow_mut();
        ic_cdk::println!("set_time: State borrowed");
        let leaderboard = state.leaderboards.entry(board_setup).or_insert(Leaderboard { scores: BinaryHeap::new() });
        ic_cdk::println!("set_time: Leaderboard entry created");

        // Push the score into the heap
        leaderboard.scores.push((Reverse(miliseconds), user.clone()));
        ic_cdk::println!("set_time: Score pushed");
    })
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
