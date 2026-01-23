import { useExperimentalFlags } from '@their/library';

function App() {
  const { featureA, featureB } = useExperimentalFlags();

  if (featureA) {
    console.log('Feature A is enabled');
  }

  return featureB ? 'Beta' : 'Stable';
}
