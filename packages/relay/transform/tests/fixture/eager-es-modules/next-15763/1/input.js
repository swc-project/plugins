import { graphql } from "relay-runtime";
import { revalidatePath } from "next/cache";

export async function setValue(string) {
    "use server";

    // comment out the next two statements to successfully compile
    const taggedNode = graphql`
    mutation actionsSetThingMutation($value: String!) {
      setRocket(value: $value) {
        name
      }
    }
  `
    console.log({ taggedNode })

    const response = await fetch("http://localhost:3000/graphql", {
        body: JSON.stringify({
            // replace this query with the taggedNode value
            query: `
        mutation actionsSetThingMutation($value: String!) {
          setThing(value: $value) {
            name
          }
        }
      `,
            variables: {
                value: string,
            },
        }),
        headers: {
            "Content-Type": "application/json",
        },
        method: "POST",
    });

    const data = await response.json();

    // the ui should flush this value when revalidated
    console.log(data);

    revalidatePath("/", 'page');
};