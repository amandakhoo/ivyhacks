#[cfg(test)]
use anyhow::Error;
use serde::{de, Deserialize, Deserializer, Serialize};

/// Contains count, idlist, and webenv information
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    #[serde(deserialize_with = "from_str")]
    pub count: usize,
    #[serde(deserialize_with = "from_str")]
    pub retmax: usize,
    #[serde(deserialize_with = "from_str")]
    pub retstart: usize,
    #[serde(deserialize_with = "from_str")]
    pub querykey: usize,
    pub webenv: String,
    #[serde(deserialize_with = "from_strs")]
    pub idlist: Vec<usize>,
}

/// Entire json wrapper around search result
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SearchResponse {
    #[serde(rename = "esearchresult")]
    pub e_search_result: SearchResult,
}

#[test]
fn test_from_search() -> Result<(), Error> {
    let input = include_str!("../../data/test-search.json");
    let response: SearchResponse = serde_json::from_str(input)?;
    let result: SearchResult = response.e_search_result;
    assert_eq!(
        result,
        SearchResult {
            count: 418202,
            retmax: 20,
            retstart: 18,
            querykey: 1,
            webenv: "MCID_5f77bdfcb48029616024b3fb".to_string(),
            idlist: vec![
                33001611, 33001587, 33001583, 33001531, 33001390, 33001337, 33001336, 33001328,
                33001309, 33001307, 33001299, 33001269, 33001242, 33001241, 33001187, 33001186,
                33001012, 33000910, 33000898, 33000732
            ]
        }
    );
    Ok(())
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

fn from_strs<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
    D: Deserializer<'de>,
{
    let s: Vec<&str> = Vec::deserialize(deserializer)?;
    s.iter()
        .map(|s| T::from_str(s).map_err(de::Error::custom))
        .collect()
}
