function App() {
  // featureA is true
  if (__SWC_FLAGS__.featureA) {
    console.log('Feature A enabled');
  } else {
    console.log('Feature A disabled');
  }

  // featureB is false
  if (__SWC_FLAGS__.featureB) {
    console.log('Feature B enabled');
  } else {
    console.log('Feature B disabled');
  }

  // featureB is false, no else
  if (__SWC_FLAGS__.featureB) {
    console.log('Should be removed');
  }
}
