import styled from '@emotion/styled'

function makeOptions() {
  return {
    shouldForwardProp: (propertyName: string) => !propertyName.startsWith('$'),
  }
}
const ContainerWithOptions = styled('div', makeOptions())`
  color: hotpink;
`

const ContainerWithOptions2 = styled('div', makeOptions())({
  color: "hotpink"
})
