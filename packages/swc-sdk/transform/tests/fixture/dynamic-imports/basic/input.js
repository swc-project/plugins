/*#__DYNAMIC__*/
import { log } from "./logger";

export async function work() {
  if (process.env.NODE_ENV !== "production") {
    await log("Starting work");
  }

  if (process.env.NODE_ENV !== "production") {
    log("Without await");
  }
}

export function sync() {
  if (process.env.NODE_ENV !== "production") {
    log("Without async");
  }
}