import * as index from ".";

describe("index", () => {
  it("enums should be transformed correctly", () => {
    expect(index.TagType.tag).toBe("tag");
  });
});
