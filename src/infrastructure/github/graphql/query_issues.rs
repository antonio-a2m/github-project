pub const QUERY_ISSUES: &str = r#"
query($organization: String!, $projectNumber: Int!, $after: String, $limit: Int) {
  organization(login: $organization) {
    projectV2(number: $projectNumber) {
      id
      title
      number
      url
      createdAt
      updatedAt
      items(first: $limit, after: $after) {
        pageInfo {
          endCursor
          hasNextPage
        }
        nodes {
          content {
            ... on Issue {
              id
              url
              title
              body # Added body
              state
              number
              createdAt
              updatedAt
              closedAt
              labels(first: 10) {
                nodes {
                  id
                  name
                  color
                }
              }
              assignees(first: 10) {
                nodes {
                  id
                  login
                  url
                }
              }
              projectItems(first: 10) {
                nodes {
                  fieldValues(first: 20) {
                    nodes {
                      ... on ProjectV2ItemFieldValueCommon {
                        field {
                          ... on ProjectV2FieldCommon {
                            id
                            name
                          }
                        }
                      }
                      ... on ProjectV2ItemFieldTextValue {
                        text
                      }
                      ... on ProjectV2ItemFieldSingleSelectValue {
                        name
                      }
                      ... on ProjectV2ItemFieldNumberValue {
                        number
                      }
                    }
                  }
                }
              }
            } 
          }
        }
      }
    }
  }
}
"#;
