import * as index from ".";
import { parent } from "./parent";

describe("index", () => {
  beforeEach(() => {
    jest.spyOn(index.children, "child");
    jest.spyOn(index.children, "niño");
  });

  it("called", () => {
    parent();

    expect(index.children.child).toHaveBeenCalled();
    expect(index.children.niño).toHaveBeenCalled();
  });
});
