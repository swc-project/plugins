

export default ({ breakPoint }) => (
  <div>
    <style jsx>{`@media (${breakPoint}) {
      .test {
        margin-bottom: 1em;
      }
    }`}</style>
  </div>
)
