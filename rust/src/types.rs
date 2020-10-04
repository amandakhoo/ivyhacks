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
    let input = std::fs::read_to_string("../data/test-search.json")?;
    let response: SearchResponse = serde_json::from_str(&input)?;
    let result: SearchResult = response.e_search_result;
    assert_eq!(
        result,
        SearchResult {
            count: 586547,
            retmax: 20,
            retstart: 0,
            querykey: 1,
            webenv: "MCID_5f79127686702466ba37cb38".to_string(),
            idlist: vec![
                7531585, 7531260, 7531195, 7531068, 7531033, 7530964, 7530962, 7530949, 7530896,
                7530769, 7530732, 7530720, 7530715, 7530657, 7530583, 7530549, 7530125, 7529917,
                7529892, 7529886
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
