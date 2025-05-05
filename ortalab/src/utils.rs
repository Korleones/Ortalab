//!Welcome to dk_utils! This is a public crate for main function to call.
//!You can find some straight-forward and powerful public function here. 
#![allow(unused)]
use crate::poker_base::{self, find_the_poker_hand};
use ortalib::{Chips, Mult, Round};
use std::{
    error::Error,
    fs::File,
    io::{Read, stdin},
    path::{Path, PathBuf},
};
///Caculate chips and mult accoding to the 12 poker hands
/// Now it's still the basic version cuz it does not take other fancy factors into account.
pub fn caculate_poker_hand_base(round :Round) -> Result<(Chips, Mult), Box<dyn Error>> {
      // let cards_played = round.cards_played;
      // if cards_played.is_empty() {return Err("No cards is played!".into());}

      // else{
      //       let vec: Vec< (Option<PokerHand>,Option<Vec<Card>>)> = find_the_poker_hand(&cards_played);
      //       let (mut chip, mut mult) = pokerhand.unwrap().hand_value();

            return Ok((3.0,4.0))
      
}

