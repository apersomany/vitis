use std::{collections::VecDeque, time::SystemTime};

use dashmap::DashMap;

pub struct Token {}

pub struct Expirable {
    expiries: VecDeque<(SystemTime, u32)>,
    value: u32,
}

impl Expirable {
    pub fn consume(&mut self, amount: u32) -> bool {
        if amount > self.value {
            false
        } else {
            while let Some((at, amount)) = self.expiries.pop_front() {}
            true
        }
    }
}

pub struct Cash {
    permanent: u32,
    expirable: Expirable,
}

impl Cash {
    pub fn consume(&mut self, amount: u32) {}
}

pub struct Tickets {
    wait_free_available: bool,
    wait_free_at: SystemTime,
    permanent: u32,
    expirable: Expirable,
}

impl Tickets {
    pub fn consume(&mut self, amount: u32) {}
}

pub struct Account {
    token: Token,
    cash: Cash,
    tickets: DashMap<u32, Tickets>,
}

pub struct Series {}

pub struct State {
    accounts: DashMap<u32, Account>,
    serieses: DashMap<u32, Series>,
}
