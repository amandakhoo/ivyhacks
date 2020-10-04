# ivyhacks
Ivyhacks 2020 project

[Link to brainstorming document](https://docs.google.com/document/d/1SjrBZELyl4gQaV18hnx0B0nIb23NMlnKwpz0xekUDBc/edit?ts=5f768ce1).

## API's

Ideally a user would provide a query of the type of experiment they're
interested in, and we could scrape relevant papers as necessary.
This details the API's that we are interested in trying.

### Google Scholar API

Has 3rd party API, nothing official.

### Pubmed API

Second choice if we can't scrape google scholar.

https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=pmc&query_key=1&webenv=MCID_5f7917aac3bc2a2d306e9f98&retmax=20

### Nature-springer open access API

`http://api.springernature.com/openaccess/jats?q=subject:Chemistry&api_key=..`

https://dev.springernature.com/adding-constraints for constraints, potentially keywords or contains