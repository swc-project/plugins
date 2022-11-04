import withAlphaVariable from '../src/util/withAlphaVariable'

test('it adds the right custom property', () => {
  expect(
    withAlphaVariable({ color: '#ff0000', property: 'color', variable: '--tw-text-opacity' })
  ).toEqual({
    '--tw-text-opacity': '1',
    color: 'rgb(255 0 0 / var(--tw-text-opacity))',
  })
  expect(
    withAlphaVariable({
      color: 'hsl(240 100% 50%)',
      property: 'color',
      variable: '--tw-text-opacity',
    })
  ).toEqual({
    '--tw-text-opacity': '1',
    color: 'hsl(240 100% 50% / var(--tw-text-opacity))',
  })
})

test('it ignores colors that cannot be parsed', () => {
  expect(
    withAlphaVariable({
      color: 'currentColor',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    'background-color': 'currentColor',
  })
  expect(
    withAlphaVariable({
      color: 'rgb(255, 0)',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    'background-color': 'rgb(255, 0)',
  })
  expect(
    withAlphaVariable({
      color: 'rgb(255)',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    'background-color': 'rgb(255)',
  })
  expect(
    withAlphaVariable({
      color: 'rgb(255, 0, 0, 255)',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    'background-color': 'rgb(255, 0, 0, 255)',
  })
  expect(
    withAlphaVariable({
      color: 'rgb(var(--color))',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    'background-color': 'rgb(var(--color))',
  })
})

test('it ignores colors that already have an alpha channel', () => {
  expect(
    withAlphaVariable({
      color: '#ff0000ff',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    'background-color': '#ff0000ff',
  })
  expect(
    withAlphaVariable({
      color: '#ff000080',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    'background-color': '#ff000080',
  })
  expect(
    withAlphaVariable({
      color: '#f00a',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    'background-color': '#f00a',
  })
  expect(
    withAlphaVariable({
      color: '#f00f',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    'background-color': '#f00f',
  })
  expect(
    withAlphaVariable({
      color: 'rgba(255, 255, 255, 1)',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    'background-color': 'rgba(255, 255, 255, 1)',
  })
  expect(
    withAlphaVariable({
      color: 'rgba(255, 255, 255, 0.5)',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    'background-color': 'rgba(255, 255, 255, 0.5)',
  })
  expect(
    withAlphaVariable({
      color: 'rgba(255 255 255 / 0.5)',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    'background-color': 'rgba(255 255 255 / 0.5)',
  })
  expect(
    withAlphaVariable({
      color: 'hsla(240, 100%, 50%, 1)',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    'background-color': 'hsla(240, 100%, 50%, 1)',
  })
  expect(
    withAlphaVariable({
      color: 'hsla(240, 100%, 50%, 0.5)',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    'background-color': 'hsla(240, 100%, 50%, 0.5)',
  })
  expect(
    withAlphaVariable({
      color: 'hsl(240 100% 50% / 0.5)',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    'background-color': 'hsl(240 100% 50% / 0.5)',
  })
})

test('it allows a closure to be passed', () => {
  expect(
    withAlphaVariable({
      color: ({ opacityVariable }) => `rgba(0, 0, 0, var(${opacityVariable}))`,
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    '--tw-bg-opacity': '1',
    'background-color': 'rgba(0, 0, 0, var(--tw-bg-opacity))',
  })
  expect(
    withAlphaVariable({
      color: ({ opacityValue }) => `rgba(0, 0, 0, ${opacityValue})`,
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    '--tw-bg-opacity': '1',
    'background-color': 'rgba(0, 0, 0, var(--tw-bg-opacity))',
  })
})

test('it transforms rgb and hsl to space-separated rgb and hsl', () => {
  expect(
    withAlphaVariable({
      color: 'rgb(50, 50, 50)',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    '--tw-bg-opacity': '1',
    'background-color': 'rgb(50 50 50 / var(--tw-bg-opacity))',
  })
  expect(
    withAlphaVariable({
      color: 'rgb(50 50 50)',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    '--tw-bg-opacity': '1',
    'background-color': 'rgb(50 50 50 / var(--tw-bg-opacity))',
  })
  expect(
    withAlphaVariable({
      color: 'hsl(50, 50%, 50%)',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    '--tw-bg-opacity': '1',
    'background-color': 'hsl(50 50% 50% / var(--tw-bg-opacity))',
  })
  expect(
    withAlphaVariable({
      color: 'hsl(50 50% 50%)',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    '--tw-bg-opacity': '1',
    'background-color': 'hsl(50 50% 50% / var(--tw-bg-opacity))',
  })
})

test('it transforms named colors to rgb', () => {
  expect(
    withAlphaVariable({
      color: 'red',
      property: 'background-color',
      variable: '--tw-bg-opacity',
    })
  ).toEqual({
    '--tw-bg-opacity': '1',
    'background-color': 'rgb(255 0 0 / var(--tw-bg-opacity))',
  })
})
