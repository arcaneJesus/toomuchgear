pub mod Inv {
    pub mod Item {

        enum Pattern {
            Melee,
            Ranged,
            Defensive,
        }

        struct Attack {
            damage: i32,
            speed: i32,
            penetration: i32,
            description: String,
        }

        impl Attack {
            fn calc(&self, armor: Armor) -> HitOrMiss {}
        }

        enum HitOrMiss {
            // Yah I bet you never miss ha.  You got a boyfriend? I bet he doesn't kiss ya.
            Hit(i32),
            Miss,
        }

        struct Weapon {
            primary: Attack,
            secondary: Attack,
            tertiary: Attack,
            pattern: Pattern,
            description: String,
        }

        struct Armor {}
    }
}

#[cfg(test)]
mod Test {
    use super::*;
}
