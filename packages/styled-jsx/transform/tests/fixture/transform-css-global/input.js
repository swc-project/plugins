export default () => (
  <div>
    <p>test</p>
    <style jsx>{`
      html {
        background-image:
          linear-gradient(0deg, rgba(255, 255, 255, 0.8), rgba(255, 255, 255, 0.8)),
          url(/static/background.svg);
      }
      
      :global(p) {
        color: #001
      }
      
      :global(h1){
        color: #002;
      }
      
      :global(h2), a {
        color: #003;
      }
      
      :global(.foo + a) {
        color: #004;
      }
      
      :global(body) {
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif;
      }
    `}</style>
  </div>
)