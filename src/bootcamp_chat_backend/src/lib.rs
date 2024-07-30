use std::{cell::RefCell, collections::HashMap};
use candid::Principal;
use ic_cdk::caller;

thread_local! {
    static NOTES: RefCell<HashMap<Principal, Vec<String>>> = RefCell::default();
    static CHAT: RefCell<HashMap<[Principal; 2], Vec<String>>> = RefCell::default();
}

#[ic_cdk::query]
fn get_chat(chat_path: [Principal; 2]) -> Option<Vec<String>> {
    CHAT.with_borrow(|chats| chats.get(&chat_path).cloned())
}

#[ic_cdk::update]
fn add_chat_msg(msg: String, user2: Principal) {
    let user1: Principal = caller();

    if user1 == Principal::anonymous() {
        panic!("Anonymus principal!")
    }

    let mut principals = [user1, user2];
    principals.sort();

    CHAT.with_borrow_mut(|chats| {
        let mut_chats = chats.get_mut(&principals);

        if let Some(chats_msgs) = mut_chats{
            chats_msgs.push(msg)
        } else {
            chats.insert(principals, vec![msg]);
        }
    })
}

