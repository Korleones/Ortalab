#![allow(unused)]
//!Include all the function would be needed to calculate the chip and mult
use std::{clone, env};
use std::collections::{hash_map, HashMap, HashSet};
use std::hash::Hash;
use std::ops::Mul;
use clap::builder::styling::Color;
use ortalib::{Card, Chips, Edition, Enhancement, Joker, JokerCard, Mult, PokerHand, Rank, Round, Suit, SuitColor};
use std::{
    error::Error,
    fs::File,
    io::{Read, stdin},
    path::{Path, PathBuf},
};

/// A card filter that groups cards by `T`.
///
/// This filter returns a `HashMap` where each key is a `T` value,  
/// and the corresponding value is a `Vec<Card>` containing cards that match the key.
/// T could be: [`Suit`] or [`Rank`]

pub struct CardFilter<'a, T> {
     pub dict: HashMap<T, Vec<&'a Card>>,
}
/// Implements `new` for `CardFilter` based on `T`.
impl<'a, T: Eq + Hash + Copy> CardFilter<'a ,T> {
    pub fn new() -> Self {
        Self {dict: HashMap::new()}
    }
}
/// Implements filtering logic for `CardFilter` based on `Rank`.
impl <'a> CardFilter<'a,Rank> {
   pub fn filter_rank(&mut self, hands: &'a Vec<Card>){
        for card in hands{
            let rank = &card.rank;
            self.dict.entry(rank.clone()).or_insert_with(Vec::new).push(card);
        }
   }
}
/// Implements filtering logic for `CardFilter` based on `Suit`.
impl <'a> CardFilter<'a,Suit> {
    pub fn filter_suit(&mut self, hands: &'a Vec<Card>,jokers:&Vec<JokerCard>){
        let mut smeared = false;
        for jcard in jokers{
            if jcard.joker == Joker::SmearedJoker{
                smeared = true;
                break;
            }
        }


         for card in hands{
            if card.enhancement == Some(ortalib::Enhancement::Wild){
                // self.dict.entry(Suit::Clubs).or_insert_with(Vec::new).push(card);
                // self.dict.entry(Suit::Diamonds).or_insert_with(Vec::new).push(card);
                // self.dict.entry(Suit::Spades).or_insert_with(Vec::new).push(card);
                // self.dict.entry(Suit::Hearts).or_insert_with(Vec::new).push(card);
            }
            else{
                if smeared{
                    let suit = &card.suit;
                    if suit.color() == ortalib::SuitColor::Black{
                        self.dict.entry(Suit::Spades).or_insert_with(Vec::new).push(card);
                    }
                    else if suit.color() == ortalib::SuitColor::Red{
                        self.dict.entry(Suit::Hearts).or_insert_with(Vec::new).push(card);
                    }
                }
                else{
                    let suit = &card.suit;
                    self.dict.entry(suit.clone()).or_insert_with(Vec::new).push(card);
                }
               
            }
           
         }
    }
 }

#[doc(hidden)]
pub fn cal_test(card_info :&str)-> Result<(Chips, Mult), Box<dyn Error>> {
    let round :Round = serde_yaml::from_str(&card_info).expect("Can't be deserialized from .yaml");
    let cards_played = &round.cards_played;
    if cards_played.is_empty() {
        return Err("No cards is played!".into());
    }
    else{

        let vec: Vec< (Option<PokerHand>,Option<Vec<Card>>)> = find_the_poker_hand(&cards_played, &round.jokers);

        let mut map: HashMap<PokerHand, Vec<Card>> = HashMap::new();
        for (hand, cards) in vec.clone() {
            if let (Some(poker_hand), Some(card_list)) = (hand, cards) {
                map.insert(poker_hand, card_list);
            }
        }

        let (mut chip, mut mult) = vec[0].0.unwrap().hand_value();
        let counted_cards = vec[0].1.as_ref().unwrap();

        let args: Vec<String> = env::args().collect(); //检查explain flag
        if args.contains(&"--explain".to_string()) {
        println!("{} ({} x {})",vec[0].0.unwrap(), chip, mult);
    } 
        let new_jokers = blueprint_tranfer(&round.jokers);
        

        score_pokerhand(&mut chip, &mut mult,&counted_cards, &round.cards_held_in_hand,&round,&new_jokers);
        retrigger_scores_played(&mut chip, &mut mult,&counted_cards, &round.cards_held_in_hand,&round,&new_jokers);
        score_onheld(&mut chip, &mut mult,&counted_cards,&round,&map,&new_jokers);
        retrigger_scores_in_hand(&mut chip, &mut mult,&counted_cards,&round,&map,&new_jokers);
        calculate_independent_jokers(&mut chip, &mut mult,&counted_cards,&round,&map,&new_jokers);

        return Ok((chip,mult))
    
}
}

/// Calculate the score based on pokerhand, enhancement, edition, Jokers
/// 
/// Take [`Round`] as input, ouput a `Result<(Chips, Mult), Box<dyn Error>>`.
/// 
/// If the card_played in round is empty, the function will return error. Or it will return `Some(Chips, Mult)`.
pub fn calculate_score_based_on_poker_hand(round :Round)-> Result<(Chips, Mult), Box<dyn Error>> {
    let cards_played = &round.cards_played;
    if cards_played.is_empty() {
        return Err("No cards is played!".into());
    }
    else{

        let vec: Vec< (Option<PokerHand>,Option<Vec<Card>>)> = find_the_poker_hand(&cards_played, &round.jokers);

        let mut map: HashMap<PokerHand, Vec<Card>> = HashMap::new();
        for (hand, cards) in vec.clone() {
            if let (Some(poker_hand), Some(card_list)) = (hand, cards) {
                map.insert(poker_hand, card_list);
            }
        }

        let (mut chip, mut mult) = vec[0].0.unwrap().hand_value();
        let counted_cards = vec[0].1.as_ref().unwrap();

        let args: Vec<String> = env::args().collect(); //检查explain flag
        if args.contains(&"--explain".to_string()) {
        println!("{} ({} x {})",vec[0].0.unwrap(), chip, mult);
    } 
        let new_jokers = blueprint_tranfer(&round.jokers);
        

        score_pokerhand(&mut chip, &mut mult,&counted_cards, &round.cards_held_in_hand,&round,&new_jokers);
        retrigger_scores_played(&mut chip, &mut mult,&counted_cards, &round.cards_held_in_hand,&round,&new_jokers);
        score_onheld(&mut chip, &mut mult,&counted_cards,&round,&map,&new_jokers);
        retrigger_scores_in_hand(&mut chip, &mut mult,&counted_cards,&round,&map,&new_jokers);
        calculate_independent_jokers(&mut chip, &mut mult,&counted_cards,&round,&map,&new_jokers);

        return Ok((chip,mult))
    
}
}
///Transfer all the blueprint joker cards to the duplicapable cards.
pub fn blueprint_tranfer(jokers:&Vec<JokerCard>) -> Vec<JokerCard>{
    if jokers.is_empty(){return vec![]}

    let mut is_duplicable = false;
    let mut temp = *jokers.last().unwrap();
    
    let impossible_jokers = vec![Joker::SmearedJoker, Joker::Splash, Joker::Pareidolia, Joker::Shortcut, Joker::FourFingers];
    let mut new_jokercards:Vec<JokerCard> = vec![];

    for jcard in jokers.iter().rev() {
        if impossible_jokers.contains(&jcard.joker){ //if it cant be dup by blueprint, just insert it in the head
            new_jokercards.insert(0, jcard.clone());
            is_duplicable = false;
            continue;
        }
        else if !impossible_jokers.contains(&jcard.joker) && jcard.joker != Joker::Blueprint{ // it means the card is normal and dupulicable.
            is_duplicable = true;
            temp = jcard.clone();
            new_jokercards.insert(0, jcard.clone());
        }
        else if jcard.joker == Joker::Blueprint{
            if is_duplicable{
                new_jokercards.insert(0, temp.clone());
                is_duplicable = true;
            }
            else{
                new_jokercards.insert(0, jcard.clone());
                is_duplicable = false;
            }
        }
    }


    new_jokercards
}

///Step 1 of all the calculation steps. Calculate all the scoring cards played.
pub fn score_pokerhand(chip:& mut f64, mut mult:& mut f64,counted_cards:&Vec<Card>,cards_in_hand:&Vec<Card>,round :&Round,newjokers:&Vec<JokerCard>){
    //base chips
    let args: Vec<String> = env::args().collect(); //检查explain flag
    let mut pareidolia = false;
    let mut splash = false;
    let mut smeard = false;

    for jcard in newjokers{
        if jcard.joker == Joker::Pareidolia{
            pareidolia = true;
        }
        else if jcard.joker == Joker::Splash{
            splash = true;
        }
        else if jcard.joker == Joker::SmearedJoker{
            smeard = true;
        }
    }


    for card in &round.cards_played{
        if !splash{
            if !counted_cards.contains(&card){
                continue;
            }
        }


        *chip += card.rank.rank_value();
        if args.contains(&"--explain".to_string()) {
            println!("{}{} +{} Chips ({} x {})" ,card.rank,card.suit, card.rank.rank_value(),chip,mult);
        } 


    //enhancements
    match card.enhancement{
        Some(Enhancement::Bonus) =>{
            *chip += 30.0;
            if args.contains(&"--explain".to_string()) {
                println!("{} +{} Chips ({} x {})" ,card, 30, chip,mult);
            } 
        },
        Some(Enhancement::Glass) => {
            *mult *=  2.0;
            if args.contains(&"--explain".to_string()) {
                println!("{} x{} Mult ({} x {})" ,card, 2, chip,mult);
            } 
        }
        Some(Enhancement::Mult) => {
            *mult += 4.0;
            if args.contains(&"--explain".to_string()) {
                println!("{} +{} Mult ({} x {})" ,card, 4, chip,mult);
            } 

        },
        _ => {}

    }
    //editions
    match card.edition{
        Some(Edition::Foil) =>{
            *chip += 50.0;
            if args.contains(&"--explain".to_string()) {
                println!("{} +{} Chips ({} x {})" ,card, 50, chip,mult);
            } 
        },
        Some(Edition::Holographic) => {
            *mult += 10.0;
            if args.contains(&"--explain".to_string()) {
                println!("{} +{} Mult ({} x {})" ,card, 10, chip,mult);
            } 
        },
        Some(Edition::Polychrome) => {
            *mult *=  1.5;
            if args.contains(&"--explain".to_string()) {
                println!("{} x{} Mult ({} x {})" ,card, 1.5, chip,mult);
            } 
        }
        _ => {}
    }
    
    // onscore joker
    let joker_cards = newjokers;
        for jcard in joker_cards{
            match jcard.joker {
                Joker::GreedyJoker => {
                    if (card.suit == Suit::Diamonds && !smeard) || (card.suit.color() == SuitColor::Red && smeard){
                        *mult += 3.0;
                        if args.contains(&"--explain".to_string()) {
                            println!("{} {} +{} Mult ({} x {})" ,jcard.joker,card, 3, chip,mult);
                        } 
                    }
                },
                Joker::LustyJoker =>{
                    if (card.suit == Suit::Hearts && !smeard) || (card.suit.color() == SuitColor::Red && smeard){
                        *mult += 3.0;
                        if args.contains(&"--explain".to_string()) {
                            println!("{} {} +{} Mult ({} x {})" ,jcard.joker,card, 3, chip,mult);
                        } 
                    }
                },
                Joker::WrathfulJoker =>{
                    if (card.suit == Suit::Spades && !smeard) || (card.suit.color() == SuitColor::Black && smeard){
                        *mult += 3.0;
                        if args.contains(&"--explain".to_string()) {
                            println!("{} {} +{} Mult ({} x {})" ,jcard.joker,card, 3, chip,mult);
                        } 
                    }
                    
                },
                Joker::GluttonousJoker => {
                    if (card.suit == Suit::Clubs && !smeard) || (card.suit.color() == SuitColor::Black && smeard){
                        *mult += 3.0;
                        if args.contains(&"--explain".to_string()) {
                            println!("{} {} +{} Mult ({} x {})" ,jcard.joker,card, 3, chip,mult);
                        } 
                    }
                },
                Joker::Fibonacci => {
                    if card.rank == Rank::Ace || card.rank == Rank::Two || card.rank == Rank::Three || card.rank == Rank::Five || card.rank == Rank::Eight{
                        *mult += 8.0;
                        if args.contains(&"--explain".to_string()) {
                            println!("{} {} +{} Mult ({} x {})" ,jcard.joker,card, 8, chip,mult);
                        } 
                    }

                    
                },
                Joker::ScaryFace => {
                    if pareidolia{
                        *chip += 30.0;
                        if args.contains(&"--explain".to_string()) {
                            println!("{} {} +{} Chips ({} x {})" ,jcard.joker,card, 30, chip,mult);
                        } 
                    }
                    else{
                        if card.rank.is_face() == true{
                            *chip += 30.0;
                            if args.contains(&"--explain".to_string()) {
                                println!("{} {} +{} Chips ({} x {})" ,jcard.joker,card, 30, chip,mult);
                            } 
                        }  
                    }
                 
                },
                Joker::EvenSteven => {
                    if card.rank == Rank::Two || card.rank == Rank::Four || card.rank == Rank::Six || card.rank == Rank::Eight || card.rank == Rank::Ten{
                        *mult += 4.0;
                        if args.contains(&"--explain".to_string()) {
                            println!("{} {} +{} Mult ({} x {})" ,jcard.joker,card, 4, chip,mult);
                        } 
                    }
                    
                },
                Joker::OddTodd => {
                    if card.rank == Rank::Ace || card.rank == Rank::Three || card.rank == Rank::Five || card.rank == Rank::Seven || card.rank == Rank::Nine{
                        *chip += 31.0;
                        if args.contains(&"--explain".to_string()) {
                            println!("{} {} +{} Chips ({} x {})" ,jcard.joker,card, 31, chip,mult);
                        } 
                    }
                },
                Joker::Photograph => {
                    if pareidolia{
                        if card == counted_cards.first().unwrap(){
                            *mult *= 2.0;
                            if args.contains(&"--explain".to_string()) {
                                println!("{} {} x{} Mult ({} x {})" ,jcard.joker,card, 2, chip,mult);
                            }
                        }
                    }
                    else{
                        let mut the_first_face_card:Option<&Card> = None;
                        for crd in counted_cards{
                            if crd.rank.is_face() == true{
                                the_first_face_card = Some(crd);
                                break;
                            }
                        }
                        if !the_first_face_card.is_none() && card == the_first_face_card.unwrap(){
                            *mult *= 2.0;
                            if args.contains(&"--explain".to_string()) {
                                println!("{} {} x{} Mult ({} x {})" ,jcard.joker,card, 2, chip,mult);
                            } 
                        }
                        
                    }
                    
                    
              
                },
                Joker::SmileyFace => {
                    if pareidolia{
                        *mult += 5.0;
                            if args.contains(&"--explain".to_string()) {
                                println!("{} {} +{} Mult ({} x {})" ,jcard.joker,card, 5, chip,mult);
                            } 
                    }
                    else{
                        if card.rank.is_face() == true{
                            *mult += 5.0;
                            if args.contains(&"--explain".to_string()) {
                                println!("{} {} +{} Mult ({} x {})" ,jcard.joker,card, 5, chip,mult);
                            } 
                        }
                    }

                    

                },
                _ => {}
            }
        }


    //retrigger
    
    }
}

///Step 2 of all the calculation steps. Retrigger all the effect of cards played.
pub fn retrigger_scores_played(chip:& mut f64, mut mult:& mut f64,counted_cards:&Vec<Card>,cards_in_hand:&Vec<Card>,round :&Round,newjokers:&Vec<JokerCard>){
    let mut pareidolia = false;
    for jcard in newjokers{
        if jcard.joker == Joker::Pareidolia{
            pareidolia = true;
        }
    }

    
    
    for jcard in newjokers{
        if jcard.joker == Joker::SockAndBuskin{
            if pareidolia{
                score_pokerhand(chip, mult, counted_cards, cards_in_hand, round,newjokers);
            }
            else{
                let mut facecards:Vec<Card> = vec![];
                for card in counted_cards{
                    if card.rank.is_face(){
                        facecards.push(card.clone());
                    }
                }
                score_pokerhand(chip, mult, &facecards, cards_in_hand, round,newjokers);
            }
           

        }
    }
}

///Step 3 of all the calculation steps. Score all the effect of cards held in hands.
pub fn score_onheld(chip:& mut f64, mut mult:& mut f64,counted_cards:&Vec<Card>,round:&Round,all_possible_pokerhand:&HashMap<PokerHand, Vec<Card>>,newjokers:&Vec<JokerCard>){
    let args: Vec<String> = env::args().collect(); //检查explain flag

    for card in &round.cards_held_in_hand{
        if card.enhancement == Some(Enhancement::Steel){
            *mult *=  1.5;
            if args.contains(&"--explain".to_string()) {
                println!("{} x{} Mult ({} x {})" ,card, 1.5, chip,mult);
            } 
        }
    

    let joker_cards = newjokers;
    for jcard in joker_cards{
        match jcard.joker {
            Joker::RaisedFist =>{
                let mut cards_in_hands = round.cards_held_in_hand.clone();
                cards_in_hands.sort_by(|a, b| a.rank.cmp(&b.rank));
                let mut lowest_rank_card = cards_in_hands[0].clone();
                for c in cards_in_hands{
                    if c != lowest_rank_card{
                        if c.rank == lowest_rank_card.rank{
                            lowest_rank_card = c.clone();
                        }
                    }
                   
                }
                if card == &lowest_rank_card{
                    *mult += (2.0* lowest_rank_card.rank.rank_value());
                    if args.contains(&"--explain".to_string()) {
                        println!("{} {} +{} Mult ({} x {})" ,jcard.joker,card,2.0* lowest_rank_card.rank.rank_value() , chip,mult);
                    } 
                }
               
            },
            Joker::Baron => {
                if card.rank == Rank::King{
                    *mult *= 1.5; 
                    if args.contains(&"--explain".to_string()) {
                        println!("{} {} x{} Mult ({} x {})" ,jcard.joker,card, 1.5, chip,mult);
                    } 

                }
            },
            _=> {}
        }
    }
}
    
}

///Step 4 of all the calculation steps. Retrigger all the scoring effect of cards held in hands.
pub fn retrigger_scores_in_hand(chip:& mut f64, mut mult:& mut f64,counted_cards:&Vec<Card>,round:&Round,all_possible_pokerhand:&HashMap<PokerHand, Vec<Card>>,newjokers:&Vec<JokerCard>){
    for jcard in newjokers{
        if jcard.joker == Joker::Mime{
            score_onheld(chip, mult,&counted_cards,&round,&all_possible_pokerhand,newjokers);
        }
    }
}

///Step 5 of all the calculation steps.  Calculate the independent jokers and its enhancement,edition.
pub fn calculate_independent_jokers(chip:& mut f64, mut mult:& mut f64,counted_cards:&Vec<Card>,round:&Round,all_possible_pokerhand:&HashMap<PokerHand, Vec<Card>>,newjokers:&Vec<JokerCard>){ 
    let args: Vec<String> = env::args().collect(); //检查explain flag
    
    let mut smeard = false;
    for jcard in newjokers{
        if jcard.joker == Joker::SmearedJoker{
            smeard = true;
        }
    }

    let card_in_hand = &round.cards_held_in_hand;
    let joker_cards = newjokers;
    
        for (origin, new) in round.jokers.iter().zip(joker_cards.iter()) {
         match origin.edition{
            Some(Edition::Foil) =>{
                *chip += 50.0;
                if args.contains(&"--explain".to_string()) {
                    println!("{} +{} Chips ({} x {})" ,origin, 50, chip,mult);
                } 
            },
            Some(Edition::Holographic) => {
                *mult += 10.0;
                if args.contains(&"--explain".to_string()) {
                    println!("{} +{} Mult ({} x {})" ,origin, 10, chip,mult);
                } 
            },
            _ => {}
        }

        match new.joker {
            Joker::Joker => {
                *mult += 4.0;
                if args.contains(&"--explain".to_string()) {
                    println!("{} +{} Mult ({} x {})" ,origin, 4, chip,mult);
                } 
             },
            Joker::JollyJoker =>{
                        if all_possible_pokerhand.contains_key(&PokerHand::Pair){
                            *mult += 8.0;
                        }
                    }
            Joker::ZanyJoker =>{
                        if all_possible_pokerhand.contains_key(&PokerHand::ThreeOfAKind){
                            *mult += 12.0;
                        }
                    }
            Joker::MadJoker =>{
                        if all_possible_pokerhand.contains_key(&PokerHand::TwoPair){
                            *mult += 10.0;
                        }
                    }
            Joker::CrazyJoker =>{
                        if all_possible_pokerhand.contains_key(&PokerHand::Straight){
                            *mult += 12.0;
                        }
                    }
            Joker::DrollJoker =>{
                        if all_possible_pokerhand.contains_key(&PokerHand::Flush){
                            *mult += 10.0;
                        }
                    }
            Joker::SlyJoker =>{
                        if all_possible_pokerhand.contains_key(&PokerHand::Pair){
                            *chip += 50.0;
                        }
                    }
            Joker::WilyJoker =>{
                        if all_possible_pokerhand.contains_key(&PokerHand::ThreeOfAKind){
                            *chip += 100.0;
                        }
                    }
            Joker::CleverJoker =>{
                        if all_possible_pokerhand.contains_key(&PokerHand::TwoPair){
                            *chip += 80.0;
                        }
                    }
            Joker::DeviousJoker =>{
                        if all_possible_pokerhand.contains_key(&PokerHand::Straight){
                            *chip += 100.0;
                        }
                    }
            Joker::CraftyJoker =>{
                        if all_possible_pokerhand.contains_key(&PokerHand::Flush){
                            *chip += 80.0;
                        }
                    } 
            Joker::AbstractJoker =>{
                        *mult += joker_cards.len() as f64 *3.0 ;
                        
                    }
            Joker::Blackboard => {
                let mut all_black = true;
                for i in &round.cards_held_in_hand{
                    if i.enhancement != Some(Enhancement::Wild) && i.suit.color() == SuitColor::Red{
                        all_black = false;
                        break;
                    }
                }

                if all_black{
                    *mult *= 3.0;
                }

            },
            Joker::FlowerPot =>{
                if smeard{
                    let mut wildcount = 0;
                    let mut redcount = 0;
                    let mut blackcount = 0;
                    for card in counted_cards{
                        if card.enhancement == Some(Enhancement::Wild){
                            wildcount += 1; 
                        }
                        else if card.suit.color() == ortalib::SuitColor::Black{
                            blackcount +=1;
                        }
                        else if card.suit.color() == ortalib::SuitColor::Red{
                            redcount +=1;
                        }
                    }
                    if redcount <= 2 && blackcount <= 2 && wildcount + redcount + blackcount  >= 4{ *mult *= 3.0;}
                }
                else{
                    let mut countdict = HashMap::<Suit,u8>::new();
                    for card in counted_cards{
                        if card.enhancement != Some(Enhancement::Wild){
                            *countdict.entry(card.suit).or_insert(0) += 1; 
                        }
                    }
                    let suits_alreadyhave = countdict.keys().len();
                    
                    let mut wildcount = 0;
                    for card in counted_cards{
                        if card.enhancement == Some(Enhancement::Wild){
                            wildcount += 1; 
                        }
                    }
                    if wildcount + suits_alreadyhave >= 4{ *mult *= 3.0}
    
                }
             
            },
            _ => {},
                    }

            match origin.edition{
                Some(Edition::Polychrome) => {
                    *mult *=  1.5;
                    if args.contains(&"--explain".to_string()) {
                        println!("{} x{} Mult ({} x {})" ,origin, 1.5, chip,mult);
                 } 
             }
                _ => {}
         }
        
    }



}





// region: all good
///Find all the possible pokerhand for the played cards.
/// Take the played cards as input, it will return a vec of `(Option<PokerHand>,Option<Vec<Card>>)`
/// `Option<PokerHand>` refers to possible pokerhands, `Option<Vec<Card>>`refers to cards should be calculated.
pub fn find_the_poker_hand(hands:&Vec<Card>,jokers:&Vec<JokerCard>)-> Vec<(Option<PokerHand>,Option<Vec<Card>>)>{
   let mut pokerhands = vec![];
   if(!is_flushfive(hands,jokers).0.is_none()){pokerhands.push(is_flushfive(hands,jokers));}
   if(!is_flushhouse(hands,jokers).0.is_none()){pokerhands.push(is_flushhouse(hands,jokers));}
   if(!is_five_of_akind(hands).0.is_none()){pokerhands.push(is_five_of_akind(hands));}
   if(!is_straight_flush(hands,jokers).0.is_none()){pokerhands.push(is_straight_flush(hands,jokers));}
   if(!is_straight(hands,jokers).0.is_none()){pokerhands.push(is_straight(hands,jokers));}
   if(!is_flush(hands,jokers).0.is_none()){pokerhands.push(is_flush(hands,jokers));}
   if(!is_fullhouse(hands).0.is_none()){pokerhands.push(is_fullhouse(hands));}
   if(!is_fourakind(hands).0.is_none()){pokerhands.push(is_fourakind(hands));}
   if(!is_threeakind(hands).0.is_none()){pokerhands.push(is_threeakind(hands));}
   if(!is_twopair(hands).0.is_none()){pokerhands.push(is_twopair(hands));}
   if(!is_pair(hands).0.is_none()){pokerhands.push(is_pair(hands));}
   if(!is_high(hands).0.is_none()){pokerhands.push(is_high(hands));}

   pokerhands.sort();
   pokerhands.reverse();
   pokerhands
}
///Similar to [`is_five_of_akind`]
pub fn is_flushfive(hands:&Vec<Card>,jokers:&Vec<JokerCard>) -> (Option<PokerHand>,Option<Vec<Card>>){
    if matches!(is_five_of_akind(hands).0,Some(PokerHand::FiveOfAKind)) && matches!(is_flush(hands,jokers).0,Some(PokerHand::Flush)){
        return (Some(PokerHand::FlushFive),  Some(hands.clone()))
    }
    else {(None,None)}
    
  
}

///Similar to [`is_five_of_akind`]
pub fn is_flushhouse(hands:&Vec<Card>,jokers:&Vec<JokerCard>)-> (Option<PokerHand>,Option<Vec<Card>>){
    if matches!(is_flush(hands,jokers).0,Some(PokerHand::Flush)) && matches!(is_fullhouse(hands).0,Some(PokerHand::FullHouse)){
        return (Some(PokerHand::FlushHouse),Some(hands.clone()))
    }
    else {(None,None)}
    
}
/// Judge whether the played card could be a `[PokerHand::FiveOfAKind]`
/// If it can't, the function will return `(None,None)`
/// Or it will return the `FiveofAKind` and Counted cards
pub fn is_five_of_akind(hands:&Vec<Card>)-> (Option<PokerHand>,Option<Vec<Card>>){
    if hands.len() < 5 {return (None,None) }

    let std_rank = &hands.last().unwrap().rank;

    for card in hands.iter() {
        if &card.rank != std_rank {
            return (None,None); 
        }
    }
    (Some(PokerHand::FiveOfAKind),Some(hands.clone()))
}
///Similar to [`is_five_of_akind`]
pub fn is_straight_flush(hands:&Vec<Card>,jokers:&Vec<JokerCard>)-> (Option<PokerHand>,Option<Vec<Card>>){
    //无小丑版本
    if matches!(is_straight(hands,jokers).0,Some(PokerHand::Straight)) && matches!(is_flush(hands,jokers).0,Some(PokerHand::Flush)){
        let vec1 = is_straight(hands,jokers).1.unwrap();
        let vec2 = is_flush(hands,jokers).1.unwrap();
        let mut vec3:Vec<Card> = vec![];
        for card in hands{
            if vec1.contains(card) || vec2.contains(card){
                vec3.push(card.clone());
            }
        }
        return (Some(PokerHand::StraightFlush), Some(vec3))
    }
    else {(None,None)}
}
///Similar to [`is_five_of_akind`]
pub fn is_straight(hands:&Vec<Card>,jokers:&Vec<JokerCard>)-> (Option<PokerHand>,Option<Vec<Card>>){
    let mut is_shortcut = false;
    let mut isfourfingers = false;
    for jcard in jokers{
        if jcard.joker == Joker::Shortcut{
            is_shortcut = true;
        }
        else if jcard.joker == Joker::FourFingers{
            isfourfingers = true;
        }
    }
        if !isfourfingers && hands.len() < 4 {
            return (None,None);
        }
        // 按 rank 排序
        let mut myhands = hands.clone();
        myhands.sort_by_key(|card| card.rank as u8);
    
        // 获取 rank 值
        let mut ranks: Vec<i8> = myhands.iter().map(|card| card.rank as i8).collect();

        if ranks.contains(&12){
            ranks.insert(0, -1);
        }  
        
        let mut filtered: Vec<i8> = vec![];
        for i in 0..ranks.len() - 1 {
            let current = &ranks[i];
            let next = &ranks[i + 1];
            if is_shortcut{
                if next - current <= 2 && next - current >=1{
                    if !filtered.contains(&ranks[i]){
                        filtered.push(ranks[i]);
                    }
                    filtered.push(ranks[i+1]);
                }
                else{
                    continue;
                }
            }
            else{
                if next - current == 1{
                    if !filtered.contains(&ranks[i]){
                        filtered.push(ranks[i]);
                    }
                    filtered.push(ranks[i+1]);
                }
                else{
                    continue;
                }
            }
            
        }
        if filtered.len() > 5{
            filtered.remove(0);
            filtered.remove(0);
        }
        if !filtered.windows(2).all(|window| {
            // 比较相邻的两个元素
            let (first, second) = (window[0], window[1]);
    
            // 判断第二个元素比第一个大最多 2 且不相等
            if is_shortcut{
                second > first && second - first <= 2
            }
            else{
                second > first && second - first == 1
            }
            
        })  {
            return (None,None);
        }

        else if filtered.len() == 5{
            return (Some(PokerHand::Straight),Some(hands.clone()))
        }
        else if filtered.len() == 4 && isfourfingers{
            let mut returncards:Vec<Card> = vec![];
            for f in filtered{
                for card in hands{
                    if card.rank as i8 == f{
                        returncards.push(card.clone());
                        break;
                    }
            
                }
                if f == -1{
                    for card in hands{
                        if card.rank == Rank::Ace{
                            returncards.push(card.clone());
                            break;
                        }
                
                    }
                }
            }
            let mut true_returncards:Vec<Card> = vec![]; 
            for card in hands{
                if returncards.contains(card){
                    true_returncards.push(card.clone())
                }
            }
            return (Some(PokerHand::Straight),Some(true_returncards))
        }
        else{
            return (None,None); 
        }
    
    
    }
    
///Similar to [`is_five_of_akind`]
pub fn is_flush(hands:&Vec<Card>,jokers:&Vec<JokerCard>)-> (Option<PokerHand>,Option<Vec<Card>>){
    let mut smeard = false;
    for jcard in jokers{
        if jcard.joker == Joker::SmearedJoker{
            smeard = true;
        }
    }

    for jcard in jokers{
        if jcard.joker == Joker::FourFingers{
            if hands.len() < 4 {return (None,None) }
            let mut is_all_wild = true;
            let mut std_suit = &Suit::Clubs;
            
            for card in hands.iter() {
                if card.enhancement.is_none() || !matches!(card.enhancement, Some(Enhancement::Wild)) {
                    std_suit = &card.suit;
                    is_all_wild = false;
                    break;
                }
            }
            if !is_all_wild{
                let mut wildcount = 0;
                for card in hands.iter() {
                    if matches!(card.enhancement, Some(Enhancement::Wild)){
                        wildcount += 1; 
                    }
                }//get all the wild cards
                let mut sfilter = CardFilter:: <Suit> ::new();
                sfilter.filter_suit(hands,jokers);
                let dict = sfilter.dict.clone();
                let (max_suit, max_vec) = dict.iter().max_by_key(|(_, v)| v.len()).unwrap();
                if (max_vec.len() + wildcount) < 4{
                    return (None,None)
                }
                else{//如果是4张牌 就直接全部返回，如果5张牌 要剔除那张不是的牌
                    if hands.len() == 4{
                        return (Some(PokerHand::Flush),Some(hands.clone()))
                    }
                    else{
                        let mut hands_return : Vec<Card> = vec![];
                        for card in hands{
                            if card.enhancement == Some(Enhancement::Wild) || max_vec.contains(&&card) {
                                hands_return.push(card.clone());
                            }
                        }
                        return (Some(PokerHand::Flush),Some(hands_return.clone()));
                    }   
                }
            }
           
        }
    }
    
    if hands.len() < 5 {return (None,None) }
    let mut is_all_wild = true;
    let mut std_suit = &Suit::Clubs;
    
    for card in hands.iter() {
        if card.enhancement.is_none() || !matches!(card.enhancement, Some(Enhancement::Wild)) {
            std_suit = &card.suit;
            is_all_wild = false;
            break;
        }
    }
    if !is_all_wild{
        for card in hands.iter() {

            if smeard{
                if &card.suit.color() != &std_suit.color() && !matches!(card.enhancement, Some(Enhancement::Wild)){
                    return (None,None); 
                }
            }
            else{
                if &card.suit != std_suit && !matches!(card.enhancement, Some(Enhancement::Wild)){
                    return (None,None); 
                }
            }
            
        }
    }
    (Some(PokerHand::Flush),Some(hands.clone()))
}

///Similar to [`is_five_of_akind`]
pub fn is_fullhouse(hands:&Vec<Card>)-> (Option<PokerHand>,Option<Vec<Card>>){
    if hands.len() < 5 {return (None,None) }

    let mut dict: HashMap<Rank, u8> = HashMap::new();
    for i in hands.iter(){
        *dict.entry(i.rank).or_insert(0) += 1;
    }
    let mut counts: Vec<u8> = dict.values().cloned().collect();
    counts.sort(); 

    if counts == vec![2,3]{(Some(PokerHand::FullHouse),Some(hands.clone()))}
    else {(None,None)}
}

///Similar to [`is_five_of_akind`]
pub fn is_fourakind(hands:&Vec<Card>)-> (Option<PokerHand>,Option<Vec<Card>>){
    if hands.len() < 4 {return (None,None) }

    let mut dict: HashMap<Rank, u8> = HashMap::new();
    for i in hands.iter(){
        *dict.entry(i.rank).or_insert(0) += 1;
    }
    let mut counts: Vec<u8> = dict.values().cloned().collect();
    counts.sort(); 



    if *counts.last().unwrap() >= 4 {
        let max_entry = dict.iter().max_by_key(|&(_, value)| value).unwrap().0;
        let mut returncards:Vec<Card> = vec![];
        for i in hands.iter(){
            if i.rank == *max_entry{
                returncards.push(i.clone());
            }
        }
        
        (Some(PokerHand::FourOfAKind),Some(returncards))
    }
    else {(None,None)}
}

///Similar to [`is_five_of_akind`]
pub fn is_threeakind(hands:&Vec<Card>)->(Option<PokerHand>,Option<Vec<Card>>){
    if hands.len() < 3 {return (None,None) }

    let mut dict: HashMap<Rank, u8> = HashMap::new();
    for i in hands.iter(){
        *dict.entry(i.rank).or_insert(0) += 1;
    }
    let mut counts: Vec<u8> = dict.values().cloned().collect();
    counts.sort(); 



    if *counts.last().unwrap() >= 3 {
        let max_entry = dict.iter().max_by_key(|&(_, value)| value).unwrap().0;
        let mut returncards:Vec<Card> = vec![];
        for i in hands.iter(){
            if i.rank == *max_entry{
                returncards.push(i.clone());
            }
        }
        
        (Some(PokerHand::ThreeOfAKind),Some(returncards))
    }
    else {(None,None)}
}

///Similar to [`is_five_of_akind`]
pub fn is_twopair(hands:&Vec<Card>)-> (Option<PokerHand>,Option<Vec<Card>>){
    if hands.len() < 4 {return (None,None) }

    let mut dict: HashMap<Rank, u8> = HashMap::new();
    for i in hands.iter(){
        *dict.entry(i.rank).or_insert(0) += 1;
    }
    let mut counts: Vec<u8> = dict.values().cloned().collect();
    counts.sort(); 

    if counts == vec![2,2] || counts == vec![1,2,2] || counts == vec![2,3]{
        let mut sorted: Vec<(&Rank, &u8)> = dict.iter().collect();
        sorted.sort_by_key(|&(_, value)| value);
        sorted.reverse(); // 从大到小排序

        let firstrank = &sorted[0].0.clone();
        let secondrank = &sorted[1].0.clone();
        let mut returncards:Vec<Card> = vec![];
        for i in hands.iter(){
            if i.rank == *firstrank || i.rank == *secondrank{
                returncards.push(i.clone());
            }
        }

        (Some(PokerHand::TwoPair), Some(returncards))
    }
    else {(None,None)}

}

///Similar to [`is_five_of_akind`]
pub fn is_pair(hands:&Vec<Card>)-> (Option<PokerHand>,Option<Vec<Card>>){
    if hands.len() < 2 {return (None,None) }

    let mut dict: HashMap<Rank, u8> = HashMap::new();
    for i in hands.iter(){
        *dict.entry(i.rank).or_insert(0) += 1;
    }
    let mut counts: Vec<u8> = dict.values().cloned().collect();
    counts.sort(); 

    if *counts.last().unwrap() >= 2 {
        let max_entry = dict.iter().max_by_key(|&(_, value)| value).unwrap().0;
        let mut returncards:Vec<Card> = vec![];
        for i in hands.iter(){
            if i.rank == *max_entry{
                returncards.push(i.clone());
            }
        }
        
        (Some(PokerHand::Pair),Some(returncards))
    }
    else {(None,None) }
}

///Similar to [`is_five_of_akind`]
pub fn is_high(hands:&Vec<Card>)-> (Option<PokerHand>,Option<Vec<Card>>){
    if hands.len() < 1 {return (None,None) }
    let mut returncards:Vec<Card> = vec![];
    let mut max:Rank = Rank::Two; 
    for i in hands{
        if i.rank > max{
            max = i.rank;
        }
    }
    for i in hands{
        if i.rank == max{
            returncards.push(i.clone());
        }
    }
    (Some(PokerHand::HighCard), Some(returncards))

}

// endregion: all good



