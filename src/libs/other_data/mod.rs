
pub mod other_data {

    use std::collections::HashMap;
    use rocket::serde::json::Json;
    use serde::Serialize;

    /// Struct fo representing info from API about available locations 
    #[derive(Debug, Serialize)]
    pub struct RegionInfo {
        id: i32,
        name: String
    }

    /// Function that returns a hashmap of pairs of i32 and string
    /// where key (i32) is the location identifier and the string is the region name.
    pub fn get_all_cities() -> HashMap<i32, String> {
        let cities: HashMap<i32, String> = HashMap::from(
            [
                (3, "Хмельницька область".to_string()),
                (4, "Вінницька область".to_string()),
                (5, "Рівненська область".to_string()),
                (8, "Волинська область".to_string()),
                (9, "Дніпропетровська область".to_string()),
                (10, "Житомирська область".to_string()),
                (11, "Закарпатська область".to_string()),
                (12, "Запорізька область".to_string()),
                (13, "Івано-Франківська область".to_string()),
                (14, "Київська область".to_string()),
                (15, "Кіровоградська область".to_string()),
                (16, "Луганська область".to_string()),
                (17, "Миколаївська область".to_string()),
                (18, "Одеська область".to_string()),
                (19, "Полтавська область".to_string()),
                (20, "Сумська область".to_string()),
                (21, "Тернопільська область".to_string()),
                (22, "Харківська область".to_string()),
                (23, "Херсонська область".to_string()),
                (24, "Черкаська область".to_string()),
                (25, "Чернігівська область".to_string()),
                (26, "Чернівецька область".to_string()),
                (27, "Львівська область".to_string()),
                (28, "Донецька область".to_string()),
                (29, "Автономна Республіка Крим".to_string()),
                (30, "м. Севастополь".to_string()),
                (31, "м. Київ".to_string()),
            ]
        );
        cities
    }

    /// A function that returns data about available locations as a RegionInfo vector.
    pub fn get_all_cities_as_json() -> Json<Vec<RegionInfo>> {

        let mut result = Vec::<RegionInfo>::new();

        for (region_id, region_name) in get_all_cities() {
            let _r = RegionInfo{
                id: region_id,
                name: region_name
            };
            result.push(_r);
        }
        Json(result)
    }



}