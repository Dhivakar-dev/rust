// mod for front of the house logic
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }
        pub fn seat_at_table() {
            println!("Seated at table");
        }
    }

   pub mod serving {
        pub fn take_order() {
            println!("Order taken");
        }
        pub fn serve_order() {
            println!("Order served");
        }
        pub fn take_payment() {
            println!("Payment taken");
        }
    }   
}

pub mod back_of_house {
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

    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {
        println!("Cooking order");
    }
}

pub fn eat_at_restaurant() {
    // Using absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Using relative path
    front_of_house::hosting::seat_at_table();
}