import * as index from ".";
import { parent } from "./parent";

describe("index", () => {
  beforeEach(() => {
    jest.spyOn(index, "child");
    jest.spyOn(index, "niño");
  });

  it("called", () => {
    parent();

    expect(index.child).toHaveBeenCalled();
    expect(index.niño).toHaveBeenCalled();
  });
});
