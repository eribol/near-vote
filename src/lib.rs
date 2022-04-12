use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default, serde::Serialize)]
pub struct Votes{
    a: u32,
    b: u32,
    c: u32,
    d: u32
}

#[near_bindgen]
impl Votes{
    #[init(ignore_state)]
    pub fn new_state() -> Self {
        //use rand::prelude::*;
        //let mut rng = rand::thread_rng();
        //let number: u16 = rng.gen_range(1..5);
        Self::default()
    }
    pub fn get_options(&mut self) -> &mut Self{
        self
    }

    pub fn try_new(&mut self, vote: u8) -> &mut Self{
        if vote == 1{
            self.a += 1;
        }
        else if vote == 2{
            self.b += 1
        }
        else if vote == 2{
            self.c += 1
        }
        else if vote == 2{
            self.d += 1
        }
        return self
    }
}
