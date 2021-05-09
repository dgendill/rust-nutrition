#[derive(PartialEq, Debug)]
enum FoodCategory {
    Meat,
    Starch,
    Plant,
    Animal,
    Grain,
    Legume,
    Other
}
impl Eq for FoodCategory {}

#[derive(Debug)]
struct Food {
    id: String,
    protein: f32,
    fiber: Option<f32>,
    calories: f32,
    category: FoodCategory
}

fn show_foods(foods: Vec<&Food>) -> String {
    foods.into_iter().fold(String::new(), |a, b| {
        a + &b.id + "\n"
    })
}

fn get_foods() -> Vec<Food> {
    vec![
        Food { id: "1 Cup Black Beans".to_string(), fiber: None, calories: 218.0, protein: 14.5,
            category: FoodCategory::Legume },
        Food { id: "1 Cup Kale".to_string(), fiber: Some(1.3), calories: 34.0, protein: 2.1,
            category: FoodCategory::Plant
        },
        Food { id: "1 Cup Cabbage".to_string(), fiber: None, calories: 21.0, protein: 1.2,
            category: FoodCategory::Plant
        },
        Food { id: "1 Egg".to_string(), fiber: None, calories: 78.0, protein: 6.0,
            category: FoodCategory::Animal
        },
        Food { id: "1 Russet Potato".to_string(), fiber: None, calories: 168.0, protein: 4.5,
            category: FoodCategory::Starch
        },
        Food { id: "1 Cup All Purpose Flour".to_string(), fiber: None, calories: 455.0, protein: 12.9,
              category: FoodCategory::Grain
        },
        Food { id: "1 Cup Whole Milk".to_string(), fiber: None, calories: 149.0, protein: 8.1,
            category: FoodCategory::Animal
        },
        Food { id: "1 Banana".to_string(), fiber: None, calories: 89.0, protein: 1.1,
            category: FoodCategory::Plant
        },
        Food { id: "1 Medium Chicken Thigh".to_string(), fiber: None, calories: 152.0, protein: 15.41,
            category: FoodCategory::Meat
        },
        Food { id: "1 Cup Cooked Chickpeas".to_string(), fiber: None, calories: 220.0, protein: 12.0,
            category: FoodCategory::Legume
        },
        Food { id: "1 Cup Full Fat Yogurt".to_string(), fiber: None, calories: 154.0, protein: 12.86,
            category: FoodCategory::Animal
        },
        Food { id: "1 Medium Jicama".to_string(), fiber: Some(32.3), calories: 250.0, protein: 4.7,
            category: FoodCategory::Plant
        },
        Food { id: "1 Cup Oatmeal".to_string(), fiber: Some(3.7), calories: 145.0, protein: 6.06,
            category: FoodCategory::Grain
        },
        Food { id: "1 Cup Full Fat Cottage Cheese".to_string(), fiber: None, calories: 232.0, protein: 28.1,
            category: FoodCategory::Animal
        },
        Food { id: "1 Cup Broccoli".to_string(), fiber: None, calories: 31.0, protein: 2.57,
            category: FoodCategory::Plant
        },
        Food { id: "1 oz Raw Ground Beef".to_string(), fiber: None, calories: 58.0, protein: 5.32,
            category: FoodCategory::Meat
        },
        Food { id: "1/2 Cup Popcorn (unpopped)".to_string(), fiber: Some(9.8), calories: 261.0, protein: 8.73,
            category: FoodCategory::Grain
        },
        Food { id: "2 T Nutritional Yeast".to_string(), fiber: None, calories: 60.0, protein: 8.0,
            category: FoodCategory::Other
        }
    ]
}

fn get_food_by_name<'a>(foods: &'a Vec<Food>, name: &str) -> Option<&'a Food> {
    
    let food : Vec<&Food> = foods.iter().filter(|food| {
        food.id == name
    }).collect();

    if food.len() > 0 {
        Some(food[0])
    } else {
        None
    }
}

fn get_foods_by_category<'a>(foods: &'a Vec<Food>, category: FoodCategory) -> Vec<&'a Food> {
    
    let food : Vec<&Food> = foods.iter().filter(|food| {
        food.category == category
    }).collect();

    food
}

fn main() {
    let foods = get_foods();

    let meat : Vec<&Food> = foods.iter().filter(|food| {
        food.category == FoodCategory::Meat
    }).collect();

    println!("Meats: \n{}", show_foods(meat));

    println!("Food: \n{:?}", get_food_by_name(&foods, "1 Cup Black Beans"));

    println!("Foods: \n{:?}", get_foods_by_category(&foods, FoodCategory::Legume));

}
