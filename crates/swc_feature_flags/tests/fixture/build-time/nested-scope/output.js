function App() {
    if (__SWC_FLAGS__.featureA) {
        console.log('Outer feature A');
        // Shadowed variable - should NOT be replaced
        const featureA = false;
        if (featureA) {
            console.log('This should use the local featureA');
        }
    }
}
