import { useExperimentalFlags } from '@their/library';

function App() {
  const flags = useExperimentalFlags();
  const { featureA } = flags;

  if (featureA) {
    console.log('Feature A enabled');
  }
}
