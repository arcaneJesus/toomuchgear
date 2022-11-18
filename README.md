# Too Much Gear

## Mechanics

### Multipliers

#### Penetration

A weapon's penetration is a positive integer 0 to 100.  It is combined with the targets armor value to determine the final amount of armor.  A penetration of 0 would result in the full armor value remaining while a penetration of 100 would fully negate any armor.

## Items

Each item has a name, ID, description, and type.  Type determines what further characteristics the item has.

### Types

#### Weapon

Each weapon has three damage values, primary, secondary, and tertiary and a pattern.  The pattern determines what type of weapon it is.

##### Patterns

Each pattern consists of base damage, multipliers to be applied, and a description of the action.

###### Melee

Basic pattern for hand to hand combat weapons like swords or axes.

- Swing
- Stab
- Blunt

###### Ranged

Basic pattern for ranged weapons like bows or muskets.

- Shot
- Stab
- Blunt

###### Defensive

Basic pattern for defensive items which need to be actively used like shields.

- Hardness
- Comfort
- Blunt

The mean of hardness and comfort is applied to all primary damage.  Hardness is applied to all secondary damage.  Comfort is applied to all tertiary damage.

#### Armor

Each armor piece has a protection value and multipliers