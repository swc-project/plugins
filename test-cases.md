# Test Cases for SWC's Styled JSX Plugin

## 1. Basic Variables in CSS
```jsx
<style jsx>{`
  .component {
    width: ${width}px;
    height: ${height}px;
    color: ${color};
  }
`}</style>
```

## 2. Nested Selectors
```jsx
<style jsx>{`
  .parent {
    position: relative;

    &:hover {
      background-color: red;
    }

    .child {
      margin-top: 10px;
    }
    
    div {
      padding: 15px;
    }
    
    h1 {
      font-size: 24px;
    }
  }
`}</style>
```

## 3. Media Queries with Variables
```jsx
<style jsx>{`
  .component {
    width: 100%;

    @media (max-width: ${ResponsiveBreakpoint[breakpoint]}) {
      width: ${mobileWidth}px;

      &.active {
        color: blue;
      }
      
      div {
        display: block;
      }
    }
  }
`}</style>
```

## 4. Complex Animation Keyframes
```jsx
<style jsx>{`
  .wrapper {
    @keyframes customAnimation {
      0% {
        opacity: 0;
        transform: scale(0);
      }
      50% {
        opacity: ${middleOpacity};
        transform: rotate(${rotation}deg);
      }
      100% {
        opacity: 1;
        transform: scale(1);
      }
    }

    .animated {
      animation: customAnimation ${duration}ms ${easing} forwards;
      animation-delay: ${delay}ms;
    }
  }
`}</style>
```

## 5. CSS Variables and Functions
```jsx
<style jsx>{`
  .container {
    --local-var: ${dynamicValue};
    color: var(--text-color);
    background: linear-gradient(to right, ${color1}, ${color2});

    .item {
      transform: translate(
        calc(var(--x) + ${offset}px),
        calc(var(--y) + ${offset}px)
      );
    }
    
    div {
      margin: calc(10px + ${spacing}px);
    }
  }
`}</style>
```

## 6. Global Styles with Dynamic Values
```jsx
<style jsx global>{`
  .scope-${id} {
    ${stringifyCssVariablesObject(cssVariables)}

    button {
      color: ${buttonColor};
    }
    
    div {
      background-color: ${backgroundColor};
    }
  }
`}</style>
```