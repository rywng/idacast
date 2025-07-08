use std::collections::HashMap;

use serde::Deserialize;

type TranslationMap = HashMap<String, TranslationContent>;

#[derive(Deserialize)]
pub struct TranslationData {
    gear: TranslationMap,
    stages: TranslationMap,
    rules: TranslationMap,
    weapons: TranslationMap,
    bosses: TranslationMap,
    brands: TranslationMap,
    powers: TranslationMap,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TranslationContent {
    name: String,
}

#[cfg(test)]
mod test {

    use crate::data::translation_data::{TranslationContent, TranslationData};

    use super::TranslationMap;

    #[test]
    fn test_translation_map() {
        let example_map_data = r#"
        {
            "e8ab0e3e655b8391": {
              "name": "Bream-Brim Cap"
            },
            "017f4bb178ae2555": {
              "name": "Moto Shades"
            },
            "ababea5943c18ea9": {
              "name": "Barazushi Wrap"
            },
            "d1ddf6a044e31f77": {
              "name": "Suede Basics"
            },
            "f78714f08fa31b1f": {
              "name": "Blue Moto Boots"
            },
            "59705e7558d8dddd": {
              "name": "Red Cuttlegear LS"
            }
        }"#;
        let result: TranslationMap = serde_json::from_str(example_map_data).unwrap();
        assert_eq!(
            result["e8ab0e3e655b8391"],
            TranslationContent {
                name: "Bream-Brim Cap".to_string()
            }
        );
        assert_eq!(
            result["017f4bb178ae2555"],
            TranslationContent {
                name: "Moto Shades".to_string()
            }
        );
        assert_eq!(
            result["59705e7558d8dddd"],
            TranslationContent {
                name: "Red Cuttlegear LS".to_string()
            }
        )
    }

    #[tokio::test]
    async fn test_parsing_online() {
        let test_data = reqwest::get("https://splatoon3.ink/data/locale/en-US.json")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let result: TranslationData = serde_json::from_str(&test_data).unwrap();
        assert_eq!(
            result.stages["VnNTdGFnZS0xNA=="],
            TranslationContent {
                name: "Sturgeon Shipyard".to_string()
            }
        )
    }
}
