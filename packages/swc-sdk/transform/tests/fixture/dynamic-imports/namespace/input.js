/*#__DYNAMIC__*/
import * as logger from "./logger";

export async function work() {
  if (process.env.NODE_ENV !== "production") {
    await logger.log("Starting work");
  }

  if (process.env.NODE_ENV !== "production") {
    logger.log("Without await");
  }
}

export function sync() {
  if (process.env.NODE_ENV !== "production") {
    logger.log("Without async");
  }
}