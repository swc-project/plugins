"use cache";

import gql from "gql-tag";

const GetUser = gql(/* GraphQL */ `
  query GetUser {
    user {
      id
      name
    }
  }
`);

const CreatePost = gql(/* GraphQL */ `
  mutation CreatePost($input: PostInput!) {
    createPost(input: $input) {
      id
      title
    }
  }
`);