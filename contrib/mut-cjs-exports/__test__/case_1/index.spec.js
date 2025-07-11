import * as index from ".";

describe("index", () => {
  it("called", () => {
    const spiedChild = jest.spyOn(index, "child");
    index.callChild();

    expect(spiedChild).toHaveBeenCalled();
  });
});
