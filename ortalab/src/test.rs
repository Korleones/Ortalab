#![allow(unused)]
#[doc(hidden)]
#[cfg(test)]
use ortalib::*;
use std::collections::{hash_map, HashMap};
use std::env;
use std::hash::Hash;
use crate::poker_base::{is_flushfive, CardFilter};
use crate::poker_base;


#[test]
fn find_the_poker_hand(){
    let card_info = r#"
    cards_played:
    - 2♥ Wild
    - 2♠
    - 5♥
    - 4♥
    - 3♥

    cards_held_in_hand: []

    jokers: []
    "#;
    let round :Round = serde_yaml::from_str(&card_info).expect("Can't be deserialized from .yaml");
    dbg!(&round);
    dbg!(poker_base::find_the_poker_hand(&round.cards_played,&round.jokers));
}

#[test]
fn test_card_filter_rank_or_suit_include_wild() {
    let card_info = r#"
    cards_played:
    - A♥ Wild
    - 8♠
    - 7♦
    - 6♣
    - 5♥

    cards_held_in_hand: []

    jokers: []
    "#;
    let round :Round = serde_yaml::from_str(&card_info).expect("Can't be deserialized from .yaml");
    dbg!(&round);

    let mut filter_suit = CardFilter::<Suit>::new();
    filter_suit.filter_suit(&round.cards_played,&round.jokers);
    dbg!(filter_suit.dict);

    let mut filter_rank = CardFilter::<Rank>::new();
    filter_rank.filter_rank(&round.cards_played);
    dbg!(filter_rank.dict);

}
    

#[test]
fn blueprint_transfer(){
    let card_info = r#"
    cards_played:
    - Q♦
    - Q♣
    - 3♠
    - 3♣
    cards_held_in_hand: []
    jokers:
    - Joker
    - Blueprint Foil
    - Photograph Holographic
    - Sock And Buskin Polychrome
    - Flower Pot Polychrome
    
    "#;
    let round :Round = serde_yaml::from_str(&card_info).expect("Can't be deserialized from .yaml");
    dbg!(poker_base::blueprint_tranfer(&round.jokers));
    
}

#[test]
fn blueprint_transfer2(){
    let card_info = r#"
    cards_played:
    - Q♦
    - Q♣
    - 3♠
    - 3♣
    cards_held_in_hand: []
    jokers:
    - Blueprint
    - Blueprint Foil
    - Blueprint
    - Blueprint
    - Blueprint
    
    "#;
    let round :Round = serde_yaml::from_str(&card_info).expect("Can't be deserialized from .yaml");
    dbg!(poker_base::blueprint_tranfer(&round.jokers));
    
}

#[test]
fn blueprint_transfer3(){
    let card_info = r#"
    cards_played:
    - Q♦
    - Q♣
    - 3♠
    - 3♣
    cards_held_in_hand: []
    jokers:
    - Joker
    
    "#;
    let round :Round = serde_yaml::from_str(&card_info).expect("Can't be deserialized from .yaml");
    dbg!(poker_base::blueprint_tranfer(&round.jokers));
    
}

#[test]
fn score_pokerhand(){
    let card_info = r#"
    cards_played:
    - Q♦
    - Q♣
    - 3♠
    - 3♣
    cards_held_in_hand: []
    jokers:
    - Smeared Joker
    - Flower Pot
    - Blackboard
    "#;
    dbg!(poker_base::cal_test(card_info));
}

#[test]
fn joker(){
    let card_info = r#"
    cards_played:
    - Q♦
    - Q♣
    - 3♠
    - 3♣
    cards_held_in_hand: []
    jokers:
    - Joker
    "#;
    dbg!(poker_base::cal_test(card_info));
}

#[test]
fn jollyjoker(){
    let card_info = r#"
    cards_played:
    - Q♦
    - Q♣
    - 2♠
    - 3♣
    cards_held_in_hand: []
    jokers:
    - Jolly Joker
    "#;
    dbg!(poker_base::cal_test(card_info));
}

#[test]
fn zanyjoker(){
    let card_info = r#"
    cards_played:
    - 10♦
    - Q♣
    - 3♠
    - 3♣
    - 3♦
    cards_held_in_hand: []
    jokers:
    - Zany Joker
    "#;
    dbg!(poker_base::cal_test(card_info));
}

#[test]
fn madjoker(){
    let card_info = r#"
    cards_played:
    - Q♦
    - Q♣
    - 3♠
    - 3♣
    cards_held_in_hand: []
    jokers:
    - Mad Joker
    "#;
    dbg!(poker_base::cal_test(card_info));
}

#[test]
fn crazyjoker(){
    let card_info = r#"
    cards_played:
    - 7♣
    - 6♦
    - 5♣
    - 4♠
    - 3♣
    cards_held_in_hand: []
    jokers:
    - Crazy Joker
    "#;
    dbg!(poker_base::cal_test(card_info));
}

#[test]
fn drolljoker(){
    let card_info = r#"
    cards_played:
    - 7♣
    - 6♣
    - 5♣
    - 4♣
    - 3♣
    cards_held_in_hand: []
    jokers:
    - Droll Joker
    "#;
    dbg!(poker_base::cal_test(card_info));
}

#[test]
fn slyjoker(){
    let card_info = r#"
    cards_played:
    - 7♣
    - 6♣
    - 5♣
    - 4♣
    - 3♣
    cards_held_in_hand: []
    jokers:
    - Sly Joker
    "#;
    dbg!(poker_base::cal_test(card_info));
}


#[test]
fn wilyjoker(){
    let card_info = r#"
    cards_played:
    - 7♣
    - 6♣
    - 5♣
    - 4♣
    - 3♣
    cards_held_in_hand: []
    jokers:
    - Wily Joker
    "#;
    dbg!(poker_base::cal_test(card_info));
}

#[test]
fn cleverjoker(){
    let card_info = r#"
    cards_played:
    - 7♣
    - 6♣
    - 5♣
    - 4♣
    - 3♣
    cards_held_in_hand: []
    jokers:
    - Clever Joker
    "#;
    dbg!(poker_base::cal_test(card_info));
}

#[test]
fn deviousjoker(){
    let card_info = r#"
    cards_played:
    - 7♣
    - 6♣
    - 5♣
    - 4♣
    - 3♣
    cards_held_in_hand: []
    jokers:
    - Devious Joker
    "#;
    dbg!(poker_base::cal_test(card_info));
}

#[test]
fn craftyjoker(){
    let card_info = r#"
    cards_played:
    - 7♣
    - 6♣
    - 5♣
    - 4♣
    - 3♣
    cards_held_in_hand: []
    jokers:
    - Crafty Joker
    "#;
    dbg!(poker_base::cal_test(card_info));
}


#[test]
fn abstractjoker(){
    let card_info = r#"
    cards_played:
    - 7♣
    - 6♣
    - 5♣
    - 4♣
    - 3♣
    cards_held_in_hand: []
    jokers:
    - Abstract Joker
    "#;
    dbg!(poker_base::cal_test(card_info));
}

#[test]
fn enhancement_edition_score_played_hands(){
    let card_info = r#"
    cards_played:
    - A♥ Bonus Foil
    - K♠ Mult Holographic
    - Q♦ Glass Polychrome
    - J♣ Wild
    - 10♥ Steel
    cards_held_in_hand: 
    - 5♥ Steel
    jokers:
    - Abstract Joker
    "#;
    dbg!(poker_base::cal_test(card_info));
}


#[test]
fn enhancement_edition_joker(){
    let card_info = r#"
    cards_played:
    - A♥ Bonus Foil
    - K♠ Mult Holographic
    - Q♦ Glass Polychrome
    - J♣ Wild
    - 10♥ Steel
    cards_held_in_hand: 
    - 5♥ Steel
    jokers:
    - Abstract Joker Foil
    - Abstract Joker Holographic
    - Abstract Joker Polychrome
    "#;
    dbg!(poker_base::cal_test(card_info));
}


#[test]
fn raisedfist(){
    let card_info = r#"
    cards_played:
    - Q♦
    - Q♣
    - 3♠
    - 3♣
    cards_held_in_hand: 
    - Q♦ Steel
    - Q♣
    - 3♠
    - 3♣
    jokers:
    - Raised Fist
    - Flower Pot
    - Blackboard
    "#;
    dbg!(poker_base::cal_test(card_info));
        
}


#[test]
fn baron(){
    let card_info = r#"
    cards_played:
    - Q♦
    - Q♣
    - 3♠
    - 3♣
    cards_held_in_hand: 
    - K♦ Steel
    - K♣
    - K♠
    - K♣
    jokers:
    - Smeared Joker
    - Flower Pot
    - Baron
    "#;
    dbg!(poker_base::cal_test(card_info));
        
}

#[test]
fn greedy_lusty_wrathful_gluttonous(){
    let card_info = r#"
    cards_played:
    - Q♦
    - Q♣
    - 3♠
    - 3♥
    cards_held_in_hand: 
    - Q♦ Steel
    - Q♣
    - 3♠
    - 3♣
    jokers:
    - Greedy Joker
    - Lusty Joker
    - Wrathful Joker
    - Gluttonous Joker
    "#;
    dbg!(poker_base::cal_test(card_info));
}


#[test]
fn fibonacci(){
    let card_info = r#"
    cards_played:
    - 2♦
    - 3♣
    - 5♠
    - 8♥
    - A♦

    cards_held_in_hand: 
    - Q♦ Steel
    - Q♣
    - 3♠
    - 3♣
    jokers:
    - Fibonacci 

    "#;
    dbg!(poker_base::cal_test(card_info));
}


#[test]
fn even_and_odd(){
    let card_info = r#"
    cards_played:
    - 2♦
    - 3♣
    - 5♠
    - 8♥
    - 6♦

    cards_held_in_hand: 
    - Q♦ Steel
    - Q♣
    - 3♠
    - 3♣
    jokers:
    - Even Steven
    - Odd Todd 

    "#;
    dbg!(poker_base::cal_test(card_info));
}



#[test]
fn photograph_smile(){
    let card_info = r#"
    cards_played:
    - K♦
    - 3♣
    - 5♠
    - 8♥
    - A♦

    cards_held_in_hand: 
    - Q♦ Steel
    - Q♣
    - 3♠
    - 3♣
    jokers:
    - Photograph 
    - Smiley Face

    "#;
    dbg!(poker_base::cal_test(card_info));
}





#[test]
fn blackboard(){
    let card_info = r#"
    cards_played:
    - Q♦
    - Q♣
    - 3♠
    - 3♣
    cards_held_in_hand: 
    - Q♦ Steel
    - Q♣
    - 3♠
    - 3♣
    jokers:
    - Smeared Joker
    - Flower Pot
    - Blackboard
    "#;
    dbg!(poker_base::cal_test(card_info));
        
}




#[test]//
fn test_flowerpot(){ 
    let card_info = r#"
    cards_played:
    - A♣ 
    - A♠ 
    - A♦ 
    - A♣ 
    - 6♠

    cards_held_in_hand:
    - 2♠
    - 3♦ Wild

    jokers:
    - Flower Pot
    "#;
    dbg!(poker_base::cal_test(card_info));
}



#[test]
fn fourfingers(){
    let card_info = r#"
    cards_played:
    - 2♦
    - 3♣
    - 4♠
    - 5♥
    - A♦

    cards_held_in_hand: 
    - Q♦ Steel
    - Q♣
    - 3♠
    - 3♣
    jokers:
    - Four Fingers 

    "#;
    dbg!(poker_base::cal_test(card_info));
}

#[test]
fn fourfingers_suit(){
    let card_info = r#"
    cards_played:
    - 2♦
    - 6♥
    - 9♥
    - 2♥
    - 4♥

    cards_held_in_hand: 
    - Q♦ Steel
    - Q♣
    - 3♠
    - 3♣
    jokers:
    - Four Fingers 

    "#;
    dbg!(poker_base::cal_test(card_info));
}


#[test]
fn shortcut(){
    let card_info = r#"
    cards_played:
    - 2♦
    - 6♥
    - 9♥
    - 8♥
    - 4♥

    cards_held_in_hand: 
    - Q♦ Steel
    - Q♣
    - 3♠
    - 3♣
    jokers:
    - Shortcut

    "#;
    dbg!(poker_base::cal_test(card_info));
}

#[test]
fn shortcut1(){
    let card_info = r#"
    cards_played:
    - 2♦
    - 6♥
    - A♥
    - 8♥
    - 4♥

    cards_held_in_hand: 
    - Q♦ Steel
    - Q♣
    - 3♠
    - 3♣
    jokers:
    - Shortcut

    "#;
    dbg!(poker_base::cal_test(card_info));
}

#[test]
fn shortcut_and_fourfingers(){
    let card_info = r#"
    cards_played:
    - 2♦
    - 6♥
    - A♥
    - 9♥
    - 4♥

    cards_held_in_hand: 
    - Q♦ Steel
    - Q♣
    - 3♠
    - 3♣
    jokers:
    - Shortcut
    - Four Fingers

    "#;
    dbg!(poker_base::cal_test(card_info));
}


#[test]
fn mime_raisedfist(){
    let card_info = r#"
    cards_played:
    - 2♦
    - 6♥
    - A♥
    - 9♥
    - 4♥

    cards_held_in_hand: 
    - Q♦ Steel
    - Q♣
    - 3♠
    - 3♣
    jokers:
    - Raised Fist
    - Mime

    "#;
    dbg!(poker_base::cal_test(card_info));
}

#[test]
fn pareidolia_sock_and_buskin(){
    let card_info = r#"
    cards_played:
    - 2♦
    - 6♥
    - A♥
    - 9♥
    - 4♥

    cards_held_in_hand: 
    - Q♦ Steel
    - Q♣
    - 3♠
    - 3♣
    jokers:
    - Sock And Buskin
    - Pareidolia

    "#;
    dbg!(poker_base::cal_test(card_info));
}


#[test]
fn smeared(){
    let card_info = r#"
    cards_played:
    - 2♦
    - 6♥
    - A♥
    - 9♥
    - 4♥

    cards_held_in_hand: 
    - Q♦ Steel
    - Q♣
    - 3♠
    - 3♣
    jokers:
    - Smeared Joker
    - Pareidolia

    "#;
    dbg!(poker_base::cal_test(card_info));
}


#[test]
fn splash(){
    let card_info = r#"
    cards_played:
    - K♥
    - Q♠
    - J♦
    - 10♣
    - 9♥

    cards_held_in_hand: []

    jokers:
    - Photograph 

    "#;
    dbg!(poker_base::cal_test(card_info));
}


#[test]
fn fourfingers_a(){
    let card_info = r#"
    cards_played:
    - 2♦
    - J♣
    - Q♠
    - K♥
    - A♦

    cards_held_in_hand: []
 
    jokers:
    - Four Fingers 

    "#;
    dbg!(poker_base::cal_test(card_info));
}

#[test]
fn a_fourfingers(){
    let card_info = r#"
    cards_played:
    - 2♦
    - 3♣
    - 4♠
    - 6♥
    - A♦

    cards_held_in_hand: []
    jokers:
    - Four Fingers 

    "#;
    dbg!(poker_base::cal_test(card_info));
}
#[test]
fn is_flush_five(){
    let card_info = r#"
    cards_played:
    - A♠ Wild
    - A♥
    - A♠ Wild
    - A♥
    - A♥

    cards_held_in_hand: []

    jokers: []
    "#;
    let round :Round = serde_yaml::from_str(&card_info).expect("Can't be deserialized from .yaml");
    dbg!(&round);

    dbg!(is_flushfive(&round.cards_played,&round.jokers));
    assert!(is_flushfive(&round.cards_played,&round.jokers).0 == Some(PokerHand::FlushFive));
}

#[test]
fn is_flush_house(){
    let card_info = r#"
    cards_played:
    - 2♠ Wild
    - 2♥
    - 2♥
    - A♠ Wild
    - A♥

    cards_held_in_hand: []

    jokers: []
    "#;
    let round :Round = serde_yaml::from_str(&card_info).expect("Can't be deserialized from .yaml");
    dbg!(&round);

    dbg!(poker_base::is_flushhouse(&round.cards_played,&round.jokers));
    assert!(poker_base::is_flushhouse(&round.cards_played,&round.jokers).0 == Some(PokerHand::FlushHouse));
}

#[test]
fn is_five_akind(){
    let card_info = r#"
    cards_played:
    - 2♠ Wild
    - 2♥
    - 2♠
    - 2♠ Wild
    - 2♥

    cards_held_in_hand: []

    jokers: []
    "#;
    let round :Round = serde_yaml::from_str(&card_info).expect("Can't be deserialized from .yaml");
    dbg!(&round);

    dbg!(poker_base::is_five_of_akind(&round.cards_played));
    assert!(poker_base::is_five_of_akind(&round.cards_played).0 == Some(PokerHand::FiveOfAKind));
}

#[test]
fn is_straight_flush(){
    let card_info = r#"
    cards_played:
    - J♠
    - 9♠
    - 7♠
    - 5♠

    cards_held_in_hand: []

    jokers: 
    - Four Fingers
    - Shortcut
    "#;
    let round :Round = serde_yaml::from_str(&card_info).expect("Can't be deserialized from .yaml");
    dbg!(&round);

    dbg!(poker_base::is_straight_flush(&round.cards_played,&round.jokers));
    assert!(poker_base::is_straight_flush(&round.cards_played,&round.jokers).0 == Some(PokerHand::StraightFlush));
}

#[test]
fn is_straight(){
    let card_info = r#"
    cards_played:
    - A♥
    - K♠
    - J♦
    - 9♣
    - 7♥

    cards_held_in_hand: []

    jokers:
    - Shortcut 
    "#;
    let round :Round = serde_yaml::from_str(&card_info).expect("Can't be deserialized from .yaml");
    dbg!(&round);

    dbg!(poker_base::is_straight(&round.cards_played,&round.jokers));
    assert!(poker_base::is_straight(&round.cards_played,&round.jokers).0 == Some(PokerHand::Straight));
}

#[test]
fn is_fullhouse(){
    let card_info = r#"
    cards_played:
    - 2♠ Wild
    - 2♥
    - 3♠
    - 3♠ Wild
    - 3♥

    cards_held_in_hand: []

    jokers: []
    "#;
    let round :Round = serde_yaml::from_str(&card_info).expect("Can't be deserialized from .yaml");
    dbg!(&round);

    dbg!(poker_base::is_fullhouse(&round.cards_played));
    assert!(poker_base::is_fullhouse(&round.cards_played).0 == Some(PokerHand::FullHouse));
}

#[test]
fn is_flush(){
    let card_info = r#"
    cards_played:
    - 2♣
    - 5♣
    - 5♣
    - 6♣
    - 6♠

    cards_held_in_hand: []

    jokers: 
    - Smeared Joker
    "#;
    let round :Round = serde_yaml::from_str(&card_info).expect("Can't be deserialized from .yaml");
    dbg!(&round);

    dbg!(poker_base::is_flush(&round.cards_played,&round.jokers));
    assert!(poker_base::is_flush(&round.cards_played,&round.jokers).0 == Some(PokerHand::Flush));
}

#[test]
fn is_fourakind(){
    let card_info = r#"
    cards_played:
    - 10♠ Wild
    - 10♥
    - 10♥
    - 10♠ Wild
    - A♥

    cards_held_in_hand: []

    jokers: []
    "#;
    let round :Round = serde_yaml::from_str(&card_info).expect("Can't be deserialized from .yaml");
    dbg!(&round);

    dbg!(poker_base::is_fourakind(&round.cards_played));
    assert!(poker_base::is_fourakind(&round.cards_played).0 == Some(PokerHand::FourOfAKind));
}

#[test]
fn is_threeakind(){
    let card_info = r#"
    cards_played:
    - 10♠ Wild
    - 10♥
    - 10♥
    - 2♠ Wild
    - A♥

    cards_held_in_hand: []

    jokers: []
    "#;
    let round :Round = serde_yaml::from_str(&card_info).expect("Can't be deserialized from .yaml");
    dbg!(&round);

    dbg!(poker_base::is_threeakind(&round.cards_played));
    assert!(poker_base::is_threeakind(&round.cards_played).0 == Some(PokerHand::ThreeOfAKind));
}

#[test]
fn is_twopair(){
    let card_info = r#"
    cards_played:
    - 10♠ Wild
    - 10♥
    - 2♥
    - 2♠ Wild
    - 2♥

    cards_held_in_hand: []

    jokers: []
    "#;
    let round :Round = serde_yaml::from_str(&card_info).expect("Can't be deserialized from .yaml");
    //dbg!(&round);

    dbg!(poker_base::is_twopair(&round.cards_played));
    assert!(poker_base::is_twopair(&round.cards_played).0 == Some(PokerHand::TwoPair));
}

#[test]
fn is_pair(){
    let card_info = r#"
    cards_played:
    - 10♠ Wild
    - 10♥
    - A♥
    - K♠ Wild
    - 2♥

    cards_held_in_hand: []

    jokers: []
    "#;
    let round :Round = serde_yaml::from_str(&card_info).expect("Can't be deserialized from .yaml");
    //dbg!(&round);

    dbg!(poker_base::is_pair(&round.cards_played));
    assert!(poker_base::is_pair(&round.cards_played).0 == Some(PokerHand::Pair));
}

#[test]
fn is_high(){
    let card_info = r#"
    cards_played:
    - 6♠ Wild
    - 3♥
    - 8♥
    - K♠ Wild
    - 2♥

    cards_held_in_hand: []

    jokers: []
    "#;
    let round :Round = serde_yaml::from_str(&card_info).expect("Can't be deserialized from .yaml");
    //dbg!(&round);

    dbg!(poker_base::is_high(&round.cards_played));
    assert!(poker_base::is_high(&round.cards_played).0 == Some(PokerHand::HighCard));
}

#[test]
fn calculate_score_based_on_poker_hand(){
    let card_info = r#"
    cards_played:
    - 2♥
    - 5♣ Wild
    - 5♥
    - 6♥
    - 6♠ Wild

    cards_held_in_hand: []

    jokers: []
    "#;
    let round : Round = serde_yaml::from_str(&card_info).expect("Can't be deserialized from .yaml");
    //dbg!(&round);

    
    let result: Result<(Chips, Mult), Box<dyn std::error::Error>> = poker_base::calculate_score_based_on_poker_hand(round);
    assert_eq!((59.0,4.0),(result.unwrap()))
}