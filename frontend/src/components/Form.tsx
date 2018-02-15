import * as React from 'react'

interface MinimalEvent<N extends string> {
  target: { name: N, value: string }
}

type OnChange = <N extends string>(event: MinimalEvent<N>) => void

interface Props {
  errors: any[]
  onChange: OnChange
}

export class Form extends React.Component<Props> {
  render() {
    const { errors, onChange } = this.props
    return (
      <form>
        {this.props.children &&
          React.cloneElement(this.props.children as any, (props: any): any => ({
            errors: props.name ? errors[name] : undefined,
            onChange: props.name ? onChange : undefined,
        }))}
      </form>
    )
  }
}
