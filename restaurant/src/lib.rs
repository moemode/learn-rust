mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn cook_order() {}
}

fn deliver_order() {}


pub fn eat_at_restaurant() {
    //Choosing whether to use a relative or absolute path 
    //based on your project, and depends on whether you’re 
    // more likely to move item definition code separately from or together with 
    //the code that uses the item.
    //Items in a parent module can’t use the private items inside child modules.
    crate::front_of_house::hosting::add_to_wait();
    front_of_house::hosting::add_to_wait();
        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);
    
        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");
}
