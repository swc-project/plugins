import css from 'styled-jsx/css';


const { className: cardClassName, styles } = css.resolve`
.geist-card {
    padding: ${padding ? 'var(--geist-gap)' : '0'};
    border-radius: var(--geist-radius);
    background: var(--geist-background);
    box-shadow: ${shadow ? 'var(--ds-shadow-border-small)' : 'none'};
    transition: box-shadow 0.2s ease;
}
:hover {
    z-index: ${hoverAnimation ? '1' : 'auto'};
}
`;