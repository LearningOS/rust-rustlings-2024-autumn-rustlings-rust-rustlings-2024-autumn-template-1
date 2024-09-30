// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.


mod delicious_snacks {
    // TODO: Fix these use statements
    pub use self::fruits::PEAR as fruit;   // 将 fruits 模块中的 PEAR 公开，并为其起一个别名 `fruit`
    pub use self::veggies::CUCUMBER as veggie;  // 将 veggies 模块中的 CUCUMBER 公开，并为其起一个别名 `veggie`

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie

        //// 使用从 delicious_snacks 模块中导出的 fruit 和 veggie 常量
    );
}
