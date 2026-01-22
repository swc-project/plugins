function App() {
    // Logical AND
    const andTrue = expensiveFunction();
    const andFalse = false;
    // Logical OR
    const orTrue = true;
    const orFalse = defaultValue();
    // Negation
    const notTrue = false;
    const notFalse = true;
    return {
        andTrue,
        andFalse,
        orTrue,
        orFalse,
        notTrue,
        notFalse
    };
}
