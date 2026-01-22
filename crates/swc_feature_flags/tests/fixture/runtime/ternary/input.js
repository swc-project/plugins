function App() {
  // featureA is true
  const resultA = __SWC_FLAGS__.featureA ? 'A enabled' : 'A disabled';

  // featureB is false
  const resultB = __SWC_FLAGS__.featureB ? 'B enabled' : 'B disabled';

  return { resultA, resultB };
}
