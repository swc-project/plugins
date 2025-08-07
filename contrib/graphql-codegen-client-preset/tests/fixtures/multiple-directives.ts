"use strict";
"use cache";

import gql from "gql-tag";

const GetData = gql(/* GraphQL */ `
  query GetData {
    data
  }
`);