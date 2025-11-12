# Scoundrel

A single-player card game implemented in Rust.

## Rules
Red (hearts and diamonds) face cards (and aces) are removed from a standard 52-card deck. The remaining 44 cards are shuffled. Gameplay proceeds by dealing four-card "rooms." In each room, the player opts to handle 3 of the 4 cards. Once one card remains in the room, three more cards are dealt into the room from the top of th deck. The player begins the game with 20 health points. The goal of the game is to empty the deck without losing all health points.

1. Clubs and spades are *monsters*. The player can "fight" a monster. (See *Combat* below.) After combat, the monster is placed in the graveyard.
2. Hearts are *potions*. Playing a potion heals the player for a value equal to the rank of the card (up to a maximum of 20). After usage, the potion is placed in the graveyard. Only one potion may be played per room unless there are no other legal moves.
3. Diamonds are *weapons*. A player can "equip" a weapon. Only one weapon may be equipped at a time. If a weapon is equipped when another weapon is played, the old weapon and all monsters it has slain are placed in the graveyard, and then the new weapon is equipped.

### Ranks

Card ranks are given by their value (if numeric), or as follows, for face cards:

- Ace: 14 points
- King: 13 points
- Queen: 12 points
- Jack: 11 points

### Combat

When a player chooses a *monster* card, they may fight the monster one of two ways:

1. In "barehanded" combat, the monsters rank is subtracted from the player's health points. The monster is then placed in the graveyard.
2. If the player has a weapon equipped, the player may choose to use the weapon to slay the monster. In this case, the weapon's rank is subtracted from the monster's rank, and that value is subtracted from the player's health points. If a weapon slays a monster, then the monster is placed on top of the weapon. Weapons may only be used against monsters of equal or lesser rank than the weakest monster slain by that weapon. A new (unused) weapon may be used against a monster of any rank.

### Fleeing

At the start of a room (when there are four cards remaining), the player may choose to *flee*. If the player flees, the room is placed at the bottom of the deck and a new room of four cards is dealt. A player may not flee from two rooms in a row.

### End of game

The game ends when the player's health points are reduced to zero (or below) or when all cards in the deck have been played. The final score is calculated as follows.

- If the player dies (loses all their health points), then the score is *negative* the sum of the ranks of the remaining monsters in the room and the deck. Monsters are slain immediately in combat, so the monster that kills the player does not contribute to the score.
- If the player is alive, then the score is the player's health total. If the last card played is a potion, then the potion's value is added to the score.

## Input

Input is through stdio only. You will be prompted with a list of legal moves indexed by an integer. Any input besides one of those integers is invalid.
