function App() {
    if (__SWC_FLAGS__.featureA) {
        console.log('Feature A is enabled');
    }
    return __SWC_FLAGS__.featureB ? 'Beta' : 'Stable';
}
