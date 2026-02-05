function App() {
    const flags = useExperimentalFlags();
    const { excludedFlag } = flags;
    if (flags.excludedFlag) {
        console.log('This should not be transformed');
    }
    if (excludedFlag) {
        console.log('This should also not be transformed');
    }
}
