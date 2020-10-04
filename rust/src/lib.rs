mod types;
use anyhow::Error;
use reqwest::{Client, Response};
use roxmltree::Document;
pub use types::Search;
use types::*;

const BASE_URL: &str = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/";

pub async fn pubmed_search(query: &str) -> Result<Response, Error> {
    let client = Client::new();
    let query = [
        ("db", "pmc"),
        ("term", query),
        ("usehistory", "y"),
        ("retmode", "json"),
    ];
    let base_url = BASE_URL.to_string() + "esearch.fcgi";

    client
        .get(&base_url)
        .query(&query)
        .send()
        .await
        .map_err(|e| e.into())
}

/// maybe just need webenv?
/// ie: https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=pubmed&query_key=1&usehistory=y&WebEnv=MCID_5f77bdfcb48029616024b3fb&retmode=xml&retmax=100&retstart=18
pub async fn pubmed_fetch(
    query_key: usize,
    web_env: &str,
    retmax: usize,
    retstart: usize,
) -> Result<Response, Error> {
    let client = Client::new();
    let query = [
        ("db", "pmc"),
        ("query_key", &query_key.to_string()),
        ("WebEnv", web_env),
        ("retmode", "xml"),
        ("retmax", &retmax.to_string()),
        ("retstart", &retstart.to_string()),
    ];
    let base_url = BASE_URL.to_string() + "efetch.fcgi";

    client
        .get(&base_url)
        .query(&query)
        .send()
        .await
        .map_err(|e| e.into())
}

/// Given a response whose body is xml, return the corresponding xml document
pub fn response_to_xml(s: &str) -> Result<Document<'_>, Error> {
    Document::parse(&s).map_err(|e| e.into())
}

pub async fn search_response_to_result(r: Response) -> Result<SearchResult, Error> {
    let text = r.text().await?;
    let response: SearchResponse = serde_json::from_str(&text)?;
    Ok(response.e_search_result)
}

pub fn method_paragraphs<'a>(doc: &'a Document) -> Vec<Option<String>> {
    let articles = doc
        .root()
        .children()
        .next()
        .unwrap()
        .children()
        .filter(|node| node.is_element());
    articles
        .map(|article| {
            let body = article
                .children()
                .find(|node| node.tag_name().name() == "body");
            let methods_node = body.and_then(|node| {
                node.descendants()
                    .find(|node| node.text() == Some("Methods"))
            });
            let methods_section = methods_node.and_then(|node| node.parent());
            let methods_paragraphs = methods_section.map(|node| {
                node.descendants()
                    .filter_map(|node| {
                        if node.tag_name().name() == "p" {
                            let texts = node
                                .children()
                                .filter_map(|node| node.text())
                                .collect::<Vec<_>>();
                            Some(texts.join(""))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            });
            methods_paragraphs.map(|paragraphs| paragraphs.join(" "))
        })
        .collect()
}

#[test]
fn xml_tests() -> Result<(), Error> {
    let response = std::fs::read_to_string("../data/test-fetch.xml")?;
    let doc = response_to_xml(&response)?;
    let method_paragraphs = method_paragraphs(&doc);
    assert_eq!(
        method_paragraphs.into_iter().take(4).collect::<Vec<_>>(),
        vec![None, None, None, Some(r#"The present study was conducted at the Radiation Oncology Unit of the Careggi University Hospital of Florence, Italy. This is a prospective monocenter study including cancer patients admitted to the department to receive either systemic and/or radiation treatment. All patients recruited underwent a survey approved by our institutional ethics review board. Two validated questionnaires (EORTC QLQ-C30, FACIT-TS-G version 1) [11, 12] were administered to the recruited outpatients. Additionally, an internally developed survey consisting of 14 questions evaluating patients’ perception of COVID-19 measures was administered. To be eligible, cancer patients had to be aged 18 years or older without cognitive impairment. Patients anonymously filled in the Italian version of the questionnaires at their hospital access. Both patients accessing our facility at treatment start and during their treatment were included. Patients were asked to deposit completed questionnaires into a closed survey box placed in the patient waiting area. Only one form was filled by each included patient. The aim of the present study was to evaluate Health-Related Quality of Life (HRQoL), patient satisfaction, and level of patient knowledge and satisfaction with COVID-19 precautions. EORTC QLQ-C30 was analyzed according to the scoring manual and a linear transformation of results on a 0 to 100 scale was performed. Scores for each of the following domains were assessed: Global Health Status (GHS), Functional Scales (Physical, Role, Emotional, Cognitive and Social Functioning), and Symptom Scales (Fatigue, Nausea, Pain, Dyspnea, Insomnia, Appetite Loss, Constipation, Diarrhea, Financial Difficulties). FACIT-TS-G results were reported in terms of the percentage of patients satisfied with the healthcare service provided. A specific survey regarding COVID-19 preventive measures was internally developed on the basis of a pre-existent questionnaire used during the 2003 SARS outbreak [13]. Our survey consisted of two main subgroups: patient’s level of information about the pandemic (5 questions) and patient’s level of satisfaction for health-related measures during the pandemic (9 questions), as shown in Table 4. Patients were asked to report their agreement regarding each different statement (strongly disagree, disagree, agree, strongly agree). Patients’ demographic information (sex, age range, level of education) and primary tumor diagnosis were also collected. Descriptive statistics was performed to report analysis results. MedCalc Software version 19.2.1 was employed for the statistical analysis. Questionnaires with missing data were not included in the analysis."#.into())]
    );

    Ok(())
}
