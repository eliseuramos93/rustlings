// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

mod delicious_snacks {
    // TODO: Add the following two `use` statements after fixing them.
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;
    pub use self::veggies::CARROT as rabbit_food;
    pub use self::fruits::APPLE as overpriced_hardware;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}. rabbits prefer {}, and hipsters prefer {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
        delicious_snacks::rabbit_food,
        delicious_snacks::overpriced_hardware,
    );
}
