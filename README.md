# Ortalab Scoring System (COMP6991 Assignment 1)

A scoring engine implemented in Rust for a simplified version of the rogue-lite poker game **Balatro**, as part of UNSW's COMP6991: *Solving Modern Programming Problems with Rust*.

## 🃏 Project Overview

This project simulates scoring for a single round of **Ortalab**, a Balatro-inspired poker game. The player plays a five-card hand, holds additional cards, and may use up to five **Jokers** — each of which modifies scoring in unique ways.

The program parses game state from a YAML file and computes the final score by evaluating:
- The poker hand formed
- Effects of enhancements and editions on cards
- Joker effects (over 30 supported)
- Card scoring logic per round

## 🎯 Key Features

- ✅ Scoring for all standard and custom Ortalab poker hands
- ✅ Full support for card modifiers (editions & enhancements)
- ✅ All 34 Joker effects implemented (including "hard" ones like Splash, Blueprint, Mime)
- ✅ Modular and testable Rust design using Ortalib types and traits
- ✅ Passes all 180+ official autotests

## 💡 Example

```yaml
cards_played:
  - 10♥
  - 10♠
  - 10♦
  - A♣
  - J♥

cards_held_in_hand:
  - 2♥
  - 5♦
  - 9♦

jokers:
  - Joker
  - Mad Joker
  - Zany Joker
  - Raised Fist
```
The key detail is here:
For part1: https://www.youtube.com/watch?v=N-UPiTi3-_I&t=267s
For part2: https://www.youtube.com/watch?v=YyXQ2-nYyVk
