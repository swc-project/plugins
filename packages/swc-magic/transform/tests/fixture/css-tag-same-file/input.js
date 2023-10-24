import css from 'swc-magic/css'

export default ({ children }) => (
  <div>
    <p>{children}</p>
    <style jsx>{styles}</style>
  </div>
)

const styles = css`
  p {
    color: red;
  }
`

class Test extends React.Component {
  render() {
    return (
      <div>
        <p>{this.props.children}</p>
        <style jsx>{styles}</style>
      </div>
    )
  }
}
