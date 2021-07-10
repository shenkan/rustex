use thiserror::Error;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Stat {
    Strength,
    Dexterity,
    Stamina,
    Energy
}

#[derive(Debug, PartialEq, Eq)]
enum EquipmentSlot {
    Head,
    Body,
    Legs
}

#[derive(Debug, PartialEq, Eq)]
enum InventorySlotStatus {
    Occupied(Item),
    Empty
}

#[derive(Debug, PartialEq, Eq, Error)]
enum InventorySlotError {
    #[error("Slot is occupied")]
    SlotOccupied(InventorySlotStatus)
}

#[derive(Debug, PartialEq, Eq, Error)]
enum InventoryError {
    #[error("Inventory is full")]
    InventoryFull
}

#[derive(Debug, PartialEq, Eq, Error)]
enum EquipError {
    #[error("Item cannot be equipped in this slot")]
    WrongSlot(EquipmentSlot),
    #[error("Item requirements not met")]
    Requirements(RequirementsError)
}

#[derive(Debug, PartialEq, Eq)]
struct RequirementsError {
    requirements: HashMap<Stat, u16>
 }

// do I need these 3 impls for RequirementsError?  Since it isn't a standalone error but a value for an error enum
// I think I need new, but i guess not the others?
impl RequirementsError {
    fn new(requirements: HashMap<Stat, u16>, details: String) -> Self {
        RequirementsError{
            requirements
        }
    }
}

impl fmt::Display for RequirementsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for RequirementsError {
    fn description(&self) -> &str {
        &self.requirements // not sure what goes here
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Player {
    name: String,
    inventory: Inventory::new(),
    equipped: HashMap<EquipmentSlot, Equipment>,
    stats: HashMap<Stat, u16>, // 
}

struct Inventory {
    slots: HashMap<u8, InventorySlotStatus>
}

impl Inventory {
    fn new(&self) -> Self {
        // how do i initialize the HashMap to have 40 slots with InventorySlotStatus::Empty?
        let inventory = HashMap::<u8, InventorySlotStatus>new();
        for i in 1..=40 {
            inventory.insert(i, InventorySlotStatus::Empty);
        }
        inventory
    }
    // self is &mut because I'm changing what's in the hashmap??
    fn add<T: CanHold>(&mut self, item: &T) -> Result<(), InventoryError> {
        // checking for open slot, add it to the hashmap otherwise return error InventoryFull
        self.slots.iter().map(|k, v| 
            if v == InventorySlotStatus::Empty { 
                v = InventorySlotStatus::Occupied(&item) 
            } else { 
                InventorySlotError::SlotOccupied(InventorySlotStatus::Occupied(&item))
            });
        // how do i return Ok and Err from this? (let alone any fn that returns Result<>)
    }
}

trait CanHold {
    fn add_to_inventory(&self, player: &Player) -> Result<(), InventoryError>;
}

trait CanEquip {
    fn equip(&self) -> Result<(), EquipError>;
    fn check_requirements(&self, player: &Player) -> Result<(), EquipError>;
}

struct Item {

}

impl Item {

}

struct Equipment {
    name: String,
    owner: Player, // should this be &Player do i use & in struct field type definitions?
    requirements: HashMap<Stat, u16>
}

impl Equipment {
    name: String,
    owner: Player
}

impl CanEquip for Equipment {
    fn equip(&self) -> Result<(), Self::Error> {
        self.owner.
    }
    // i just know i'm doing something wrong here lol
    fn check_requirements(&self, player: &Player) -> Result<(), EquipError> {
        self.requirements.all(|k, v| 
            if player.stats.contains_key(k) { 
                if player.stats.get(k) < requirements.get(k) {
                    Err(EquipError:Requirements(RequirementsError::new(requirements)))
                } else {
                    Ok()
                }
            } else  {
                Err(EquipError:Requirements(RequirementsError::new(requirements)))
            });
    }
}

impl CanHold for Equipment {
    fn add_to_inventory(&self, player: &Player) -> Result<(), InventoryError>; {
            Ok(player.inventory.add(&self)?) // how can I match on the Result return value of fn add() to return either Ok or Err will ? operator work?
        }
}

impl CanHold for Item {
    fn add_to_inventory(&self, player: &Player) -> Result<(), InventoryError>; {
            Ok(player.inventory.add(&self)?) 
        }
}

impl Player {
    fn new(name: String) -> Self {
        Player {
            name,
            Inventory::new(),
            HashMap::<EquipmentSlot, Equipment>new(),
            HashMap::<Stat, u16>new()
        }
    }
}

fn equip<T: CanEquip>(player: &Player, equipment: &T) {
    equipment.equip(&player);
}   

fn add_to_inventory<T: CanHold>(player: &Player, item: &T) {
    item.add_to_inventory(&player);
}   

fn main() {
    println!("Hello, world!");
}