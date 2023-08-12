mod front_of_house;

fn _serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, //private
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }
    fn _fix_incorrect_order() {
        _cook_order();
        super::_serve_order();
    }

    fn _cook_order() {}
}

// use crate::front_of_house::hosting; //modulos externos
pub use crate::front_of_house::hosting; //outro modulo que importar poderá usar;

// use self::front_of_house::hosting; //mesmo modulo;
// use self::front_of_house::hosting::add_to_waitlist; //toda funcao

pub fn eat_at_restaurant() {
    // //path absoluto
    // crate::front_of_house::hosting::add_to_waitlist();
    // //path relativo
    // front_of_house::hosting::add_to_waitlist();
    //use trouxe o contexto
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("i'd like {} toast please", meal.toast);

    let _order1 = back_of_house::Appetizer::Salad;
    let _order2 = back_of_house::Appetizer::Soup;
}
