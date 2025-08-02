
pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins:f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut total_fats = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_kcal = 0.;

    for food in foods {
        let kl = food.calories.1.trim_end_matches("kcal")
                        .parse::<f64>().unwrap_or(0.0);

        total_kcal += kl * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
    }
    
    let mut obj = json::JsonValue::new_object();

    obj["cals"] = round_flaot(total_kcal).into();
    obj["carbs"] = round_flaot(total_carbs).into();
    obj["proteins"] = round_flaot(total_proteins).into();
    obj["fats"] = round_flaot(total_fats).into();

    obj
}

fn round_flaot(n: f64) -> f64 {
    (n * 100.0).round() / 100.0
}