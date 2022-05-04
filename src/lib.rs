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
//! TheMealDB API client
//!
//! An open, crowd-sourced database
//! of Recipes from around the world
mod meal;
mod error;
pub use error::MealError;
pub use meal::search;
pub use meal::search_by_letter;
pub use meal::search_by_id;
pub use meal::random;
pub use meal::meal_categories;
pub use meal::filter_by_area;
pub use meal::filter_by_category;
pub use meal::filter_by_ingredient;
pub use meal::categories_filter;
pub use meal::area_filter;
pub use meal::ingredients_filter;
pub use meal::Meal;
pub use meal::Category;
pub use meal::Filter;
pub use meal::Ingredient;