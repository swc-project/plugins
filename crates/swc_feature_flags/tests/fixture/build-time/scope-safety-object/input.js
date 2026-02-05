import { useExperimentalFlags } from '@their/library';

function App() {
  const flags = useExperimentalFlags();

  if (flags.featureA) {
    console.log('Outer');

    const flags = { featureA: false };
    if (flags.featureA) {
      console.log('Inner - should use local flags');
    }
  }
}
