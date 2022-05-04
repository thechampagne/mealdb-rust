/*
 * Copyright 2022 XXIV
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::io::Read;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use urlencoding::encode;
use crate::error::MealError;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Meals {
    meals: Vec<Meal>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meal {
    pub id_meal: Option<String>,
    pub str_meal: Option<String>,
    pub str_drink_alternate: Option<String>,
    pub str_category: Option<String>,
    pub str_area: Option<String>,
    pub str_instructions: Option<String>,
    pub str_meal_thumb: Option<String>,
    pub str_tags: Option<String>,
    pub str_youtube: Option<String>,
    pub str_ingredient1: Option<String>,
    pub str_ingredient2: Option<String>,
    pub str_ingredient3: Option<String>,
    pub str_ingredient4: Option<String>,
    pub str_ingredient5: Option<String>,
    pub str_ingredient6: Option<String>,
    pub str_ingredient7: Option<String>,
    pub str_ingredient8: Option<String>,
    pub str_ingredient9: Option<String>,
    pub str_ingredient10: Option<String>,
    pub str_ingredient11: Option<String>,
    pub str_ingredient12: Option<String>,
    pub str_ingredient13: Option<String>,
    pub str_ingredient14: Option<String>,
    pub str_ingredient15: Option<String>,
    pub str_ingredient16: Option<String>,
    pub str_ingredient17: Option<String>,
    pub str_ingredient18: Option<String>,
    pub str_ingredient19: Option<String>,
    pub str_ingredient20: Option<String>,
    pub str_measure1: Option<String>,
    pub str_measure2: Option<String>,
    pub str_measure3: Option<String>,
    pub str_measure4: Option<String>,
    pub str_measure5: Option<String>,
    pub str_measure6: Option<String>,
    pub str_measure7: Option<String>,
    pub str_measure8: Option<String>,
    pub str_measure9: Option<String>,
    pub str_measure10: Option<String>,
    pub str_measure11: Option<String>,
    pub str_measure12: Option<String>,
    pub str_measure13: Option<String>,
    pub str_measure14: Option<String>,
    pub str_measure15: Option<String>,
    pub str_measure16: Option<String>,
    pub str_measure17: Option<String>,
    pub str_measure18: Option<String>,
    pub str_measure19: Option<String>,
    pub str_measure20: Option<String>,
    pub str_source: Option<String>,
    pub str_image_source: Option<String>,
    pub str_creative_commons_confirmed: Option<String>,
    pub date_modified: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Categories {
    categories: Vec<Category>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id_category: Option<String>,
    pub str_category: Option<String>,
    pub str_category_thumb: Option<String>,
    pub str_category_description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Ingredients {
    meals: Vec<Ingredient>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
    pub id_ingredient: Option<String>,
    pub str_ingredient: Option<String>,
    pub str_description: Option<String>,
    pub str_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FilterMeals {
    meals: Vec<Filter>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub str_meal: Option<String>,
    pub str_meal_thumb: Option<String>,
    pub id_meal: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CategoriesFilter {
    meals: Vec<CategoryFilter>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CategoryFilter {
    str_category: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AreaFilter {
    meals: Vec<Area>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Area {
    str_area: String,
}

fn http(endpoint: &str) -> Option<String> {
    match reqwest::blocking::Client::new().get(format!("https://themealdb.com/api/json/v1/1/{}", endpoint))
        .send() {
        Ok(mut response) => {
            let mut body = String::new();
            match response.read_to_string(&mut body) {
                Ok(_) => Some(body),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

/// Search meal by name
pub fn search(s: &str) -> Result<Vec<Meal>, MealError> {
    match http(format!("search.php?s={}", encode(s)).as_str()) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let meal: Meals = json;
                    for i in meal.meals {
                        vector.push(i)
                    }
                    if vector.is_empty() {
                        Err(MealError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(MealError::Error(String::from("null")))
            }
        },
        None => Err(MealError::Error(String::from("null")))
    }
}

/// Search meals by first letter
pub fn search_by_letter(c: char) -> Result<Vec<Meal>, MealError> {
    match http(format!("search.php?f={}", c).as_str()) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let meal: Meals = json;
                    for i in meal.meals {
                        vector.push(i)
                    }
                    if vector.is_empty() {
                        Err(MealError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(MealError::Error(String::from("null")))
            }
        },
        None => Err(MealError::Error(String::from("null")))
    }
}

/// Search meal details by id
pub fn search_by_id(i: i64) -> Result<Meal, MealError> {
    match http(format!("lookup.php?i={}", i).as_str()) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let meal: Meals = json;
                    if meal.meals.is_empty() {
                        Err(MealError::Error(String::from("null")))
                    } else {
                        Ok(meal.meals[0].clone())
                    }
                },
                Err(_) => Err(MealError::Error(String::from("null")))
            }
        },
        None => Err(MealError::Error(String::from("null")))
    }
}

/// Search a random meal
pub fn random() -> Result<Meal, MealError> {
    match http("random.php") {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let meal: Meals = json;
                    if meal.meals.is_empty() {
                        Err(MealError::Error(String::from("null")))
                    } else {
                        Ok(meal.meals[0].clone())
                    }
                },
                Err(_) => Err(MealError::Error(String::from("null")))
            }
        },
        None => Err(MealError::Error(String::from("null")))
    }
}

/// List the meals categories
pub fn meal_categories() -> Result<Vec<Category>, MealError> {
    match http("categories.php") {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let meal: Categories = json;
                    for i in meal.categories {
                        vector.push(i)
                    }
                    if vector.is_empty() {
                        Err(MealError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(MealError::Error(String::from("null")))
            }
        },
        None => Err(MealError::Error(String::from("null")))
    }
}

/// Filter by ingredient
pub fn filter_by_ingredient(s: &str) -> Result<Vec<Filter>, MealError> {
    match http(format!("filter.php?i={}", encode(s)).as_str()) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let meal: FilterMeals = json;
                    for i in meal.meals {
                        vector.push(i)
                    }
                    if vector.is_empty() {
                        Err(MealError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(MealError::Error(String::from("null")))
            }
        },
        None => Err(MealError::Error(String::from("null")))
    }
}

/// Filter by area
pub fn filter_by_area(s: &str) -> Result<Vec<Filter>, MealError> {
    match http(format!("filter.php?a={}", encode(s)).as_str()) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let meal: FilterMeals = json;
                    for i in meal.meals {
                        vector.push(i)
                    }
                    if vector.is_empty() {
                        Err(MealError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(MealError::Error(String::from("null")))
            }
        },
        None => Err(MealError::Error(String::from("null")))
    }
}

/// Filter by Category
pub fn filter_by_category(s: &str) -> Result<Vec<Filter>, MealError> {
    match http(format!("filter.php?c={}", encode(s)).as_str()) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let meal: FilterMeals = json;
                    for i in meal.meals {
                        vector.push(i)
                    }
                    if vector.is_empty() {
                        Err(MealError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(MealError::Error(String::from("null")))
            }
        },
        None => Err(MealError::Error(String::from("null")))
    }
}

/// List the categories filter
pub fn categories_filter() -> Result<Vec<String>, MealError> {
    match http("list.php?c=list") {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let meal: CategoriesFilter = json;
                    for i in meal.meals {
                        vector.push(i.str_category)
                    }
                    if vector.is_empty() {
                        Err(MealError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(MealError::Error(String::from("null")))
            }
        },
        None => Err(MealError::Error(String::from("null")))
    }
}

/// List the ingredients filter
pub fn ingredients_filter() -> Result<Vec<Ingredient>, MealError> {
    match http("list.php?i=list") {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let meal: Ingredients = json;
                    for i in meal.meals {
                        vector.push(i)
                    }
                    if vector.is_empty() {
                        Err(MealError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(MealError::Error(String::from("null")))
            }
        },
        None => Err(MealError::Error(String::from("null")))
    }
}

/// List the area filter
pub fn area_filter() -> Result<Vec<String>, MealError> {
    match http("list.php?a=list") {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let meal: AreaFilter = json;
                    for i in meal.meals {
                        vector.push(i.str_area)
                    }
                    if vector.is_empty() {
                        Err(MealError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(MealError::Error(String::from("null")))
            }
        },
        None => Err(MealError::Error(String::from("null")))
    }
}