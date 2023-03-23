const myFragment = graphql`
  fragment FooFragment on Bar {
    id
  }
`
useQuery(graphql`
  query FooQuery {
    id
  }
`)