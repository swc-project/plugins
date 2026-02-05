function App() {
    if (__SWC_FLAGS__.featureA) {
        console.log('Outer');
        const flags = {
            featureA: false
        };
        if (flags.featureA) {
            console.log('Inner - should use local flags');
        }
    }
}
