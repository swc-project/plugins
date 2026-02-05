import { useExperimentalFlags } from '@their/library';

function App() {
  const flags = useExperimentalFlags();

  if (flags.featureA) {
    console.log('Feature A enabled');
  }

  return flags.featureB ? 'Beta' : 'Stable';
}
