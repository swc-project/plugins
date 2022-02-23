const fs = require("fs");
const prettier = require("prettier");

const expected = prettier.format(fs.readFileSync(process.argv[2], "utf-8").split("\n")
  .filter(x => x)
  .join("\n"), { parser: "babel" });
const actual = prettier.format(fs.readFileSync(process.argv[3], "utf-8")
  .split("Actual:\n")[1]
  .split("\x1b")[0]
  .split("\n")
  .filter(x => x)
  .join("\n"), { parser: "babel" });

fs.writeFileSync("clean-expected.js", expected);
fs.writeFileSync("clean-actual.js", actual);
