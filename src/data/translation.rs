use serde::Deserialize;
use std::{collections::HashMap, fmt::Display};

use super::DataError;

pub(super) trait Translatable {
    fn translate(self, dict: &FlattenedTranslationDictionary) -> Self;
}

pub(super) trait Dictionary {
    fn lookup(&self, id: &str) -> Result<String, DataError>;
}

type TranslationMap = HashMap<String, TranslationContent>;

pub type FlattenedTranslationDictionary = HashMap<String, String>;

impl Dictionary for FlattenedTranslationDictionary {
    fn lookup(&self, id: &str) -> Result<String, DataError> {
        self.get(id)
            .ok_or_else(|| DataError::TranslationError(id.to_string()))
            .cloned()
    }
}

impl From<TranslationData> for FlattenedTranslationDictionary {
    fn from(value: TranslationData) -> Self {
        let mut res: HashMap<String, String> = HashMap::new();
        for translation_map in [value.bosses, value.stages, value.rules] {
            translation_map
                .iter()
                .for_each(|(id, translation_content)| {
                    res.insert(id.to_string(), translation_content.name.clone());
                })
        }

        res
    }
}

#[derive(Deserialize)]
pub struct TranslationData {
    // gear: TranslationMap,
    stages: TranslationMap,
    rules: TranslationMap,
    // weapons: TranslationMap,
    bosses: TranslationMap,
    // brands: TranslationMap,
    // powers: TranslationMap,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TranslationContent {
    name: String,
}

#[cfg(test)]
mod test {
    use crate::data::translation::Dictionary;
    use crate::data::translation::{
        FlattenedTranslationDictionary, TranslationContent, TranslationData,
    };

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

    #[test]
    fn test_translation_lookup_with_dictionary() {
        let dict: FlattenedTranslationDictionary = FlattenedTranslationDictionary::from([
            ("VnNTdGFnZS0x".to_string(), "温泉花大峡谷".to_string()),
            ("VnNTdGFnZS0y".to_string(), "鳗鲶区".to_string()),
        ]);

        assert_eq!(
            dict.lookup("VnNTdGFnZS0x").unwrap(),
            "温泉花大峡谷".to_string()
        );

        assert!(dict.lookup("nonexistent").is_err());
    }
}
