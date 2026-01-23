function App() {
  // Logical AND
  const andTrue = __SWC_FLAGS__.featureA && expensiveFunction();
  const andFalse = __SWC_FLAGS__.featureB && shouldNotRun();

  // Logical OR
  const orTrue = __SWC_FLAGS__.featureA || fallback();
  const orFalse = __SWC_FLAGS__.featureB || defaultValue();

  // Negation
  const notTrue = !__SWC_FLAGS__.featureA;
  const notFalse = !__SWC_FLAGS__.featureB;

  return { andTrue, andFalse, orTrue, orFalse, notTrue, notFalse };
}
