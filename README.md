# ivyhacks
Ivyhacks 2020 project

[Link to brainstorming document](https://docs.google.com/document/d/1SjrBZELyl4gQaV18hnx0B0nIb23NMlnKwpz0xekUDBc/edit?ts=5f768ce1).

## API's

Ideally a user would provide a query of the type of experiment they're
interested in, and we could scrape relevant papers as necessary.
This details the API's that we are interested in trying.

### Google Scholar API

SkiMethods uses a Google API, AutoML Natural Language (particularily Entity extraction), to create a new categorization/grouping of those scientific words found in "method" section in papers that a user is interested in. A user will be able to evauale which method to use for their research by going through these AI-made categories as decision factors. 

Step 1: Upload texts of "method" section (in JSONL file format) to Cloud Storage (https://console.cloud.google.com/storage/browser/ivyhackssum;tab=objects?forceOnBucketsSortingFiltering=false&authuser=1&project=ivyhackssum&supportedpurview=project&prefix=&forceOnObjectsSortingFiltering=false)

STep 2: Select a CSV file, which is a list of GSC paths to JSONL files, on Cloud Storage 
(https://console.cloud.google.com/natural-language/locations/us-central1/datasets/TEN8197724500721139712/import?authuser=1&project=ivyhackssum&supportedpurview=project)

Reference: AutoML Natural Laungage Quickstart Guide (https://cloud.google.com/natural-language/automl/docs/quickstart)

Note: Has 3rd party API, nothing official.

### Pubmed API

Second choice if we can't scrape google scholar.

https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=pmc&query_key=1&webenv=MCID_5f7917aac3bc2a2d306e9f98&retmax=20

### Nature-springer open access API

`http://api.springernature.com/openaccess/jats?q=subject:Chemistry&api_key=..`

https://dev.springernature.com/adding-constraints for constraints, potentially keywords or contains

## Implementation notes

This repo contains a Rust command line tool for searching
for and saving the methods of articles matching the search.
Ideally we would use the core functionality of this tool in the 
web app version of this project. However, it looks like there
are dependency issues when compiling this project to web assembly.
As a workaround we will use the tool locally to download some articles
then render the web app with these saved results. In the future,
the web app would actually call out to the tool (compiled to web assembly)
and render the results directly.