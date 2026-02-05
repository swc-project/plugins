import { useExperimentalFlags } from '@their/library';

function App() {
  const { featureA } = useExperimentalFlags();
  const flags = useExperimentalFlags();
  const { featureB } = flags;
  const useC = flags.featureC;

  if (featureA && featureB && useC && flags.featureD) {
    console.log('All enabled');
  }
}
