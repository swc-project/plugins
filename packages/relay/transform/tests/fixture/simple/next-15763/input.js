import { graphql } from "relay-runtime";
import { revalidatePath } from "next/cache";
import type { actionsSetValueMutation } from '@/relay/actionsSetValueMutation.graphql';

export async function setValue(string: string): Promise<void> {
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
            } satisfies actionsSetValueMutation["variables"],
        }),
        headers: {
            "Content-Type": "application/json",
        },
        method: "POST",
    });

    const data = await response.json() as actionsSetValueMutation["response"];

    // the ui should flush this value when revalidated
    console.log(data);

    revalidatePath("/", 'page');
};