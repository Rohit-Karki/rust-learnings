// Earlier, we mentioned that src/main.rs and src/lib.rs are called crate roots.
// The reason for their name is that the contents of either of
// these two files form a module named crate at the root of the crate’s module structure, known as the module tree.


//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

// While front_of_house isn’t public, because the eat_at_restaurant function is defined in the same module as front_of_house 
// (that is, eat_at_restaurant and front_of_house are siblings), we can refer to front_of_house from eat_at_restaurant.

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

mod back_of_house {

    pub struct Breakfast{
        toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }


    fn fix_incorrect_order() {
        cook_order()
        super::cook_order()
    }
    fn cook_order() {

    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();


    // Order a breakfast in the summer with Rye toast
    let meal = back_of_house::Breakfast::summer("rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast)
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
    
}