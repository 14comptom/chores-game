# Chores Game

Chores game allows users to perfom chores and answer trivia questions through an interactive console interphase.

| Project Info | Details |
| :--- | :--- |
| **Version** | `v0.1.0` |

### Chores/items available

* Chores
  * Laundry
  * Mowing
  * Electrical Repair
  * Cooking
* Sleeping(rest)
* Trivia questions.


### Awards

```rust 
const COOKING_HEALTH: u32 = 3;
const LAUNDRY_HEALTH: u32 = 3;
const ELECTRICAL_REPAIR_HEALTH: u32 = 6;
const MOWING_HEALTH: u32 = 6;
const COOKING_COINS: u32 = 3;
const LAUNDRY_COINS: u32 = 3;
const ELECTRICAL_REPAIR_COINS: u32 = 3;
const MOWING_COINS: u32 = 3;
const SLEEP_HEALTH: u32 = 3;
const SLEEPING_COINS: u32 = 3;

```

```rust 
enum AllActivitiesAdditions {
    Cooking(u32, u32),
    Laundry(u32, u32),
    ElectricalRepair(u32, u32),
    Mowing(u32, u32),
    Sleeping(u32, u32),
    Trivia(u32, u32),
}
```

## Warning
* No error handling in this version
* Not usable in this version

## Status
 * Work in progress
