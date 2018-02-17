import * as React from 'react'
import { Schema } from '../rpc/yacchauyo_pb'

// TODO: Dim common prefixes
// TODO: API endpoint

interface State {
  active: number | null
  paths: string[]
}

interface TextInputProps extends React.InputHTMLAttributes<HTMLInputElement> {
}

class TextInput extends React.Component<TextInputProps> {
  elem: HTMLInputElement | null = null

  componentDidMount() {
    if (this.elem) {
      this.elem.focus()
      // Place cursor at the end of the text
      this.elem.selectionStart = this.elem.selectionEnd = this.elem.value.length
    }
  }

  render() {
    return <input ref={elem => (this.elem = elem)} {...this.props} />
  }
}

interface PathProps {
  addBelow: (path: string) => void
  deleteActive: () => void
  edit: (path: string) => void
  path: string
  active: boolean
  setActive: (offset?: number) => void
}

function handleKeyDown(props: PathProps): (event: React.KeyboardEvent<HTMLInputElement>) => void {
  return event => {
    if (event.key === 'Enter') {
      props.addBelow(props.path)
    }

    if (event.key === 'ArrowDown') {
      props.setActive(1)
    }

    if (event.key === 'ArrowUp') {
      props.setActive(-1)
    }

    if (event.key === 'Backspace' && props.path.trim() === '') {
      props.deleteActive()
    }
  }
}

const Path = (props: PathProps) =>
  props.active
    ? <TextInput
      className='block'
      type='text'
      onChange={event => props.edit(event.target.value)}
      onKeyDown={handleKeyDown(props)}
      value={props.path}
    />
    : <div onClick={() => props.setActive()}>{props.path}</div>

interface StateProps {
  patchSchema: (patch: Schema) => void
  schema: Schema.AsObject
}

type Props = StateProps

export default class SchemaEditor extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props)

    this.state = {
      active: null,
      paths: props.schema.pathsList,
    }
  }

  addBelow = (path: string): void => {
    const index = this.state.paths.indexOf(path)
    const before = this.state.paths.slice(0, index + 1)
    const after = this.state.paths.slice(index + 1)
    this.setState({
      active: index + 1,
      paths: [...before, path, ...after],
    })
  }

  deleteActive = (): void => {
    const { active, paths } = this.state
    if (active === 0 || active === null) { return }
    this.setState({
      active: active - 1,
      paths: [...paths.slice(0, active), ...paths.slice(active + 1)],
    })
  }

  edit = (newPath: string): void => {
    if (this.state.active !== null) {
      this.setState({
        paths: [
          ...this.state.paths.slice(0, this.state.active),
          newPath,
          ...this.state.paths.slice(this.state.active + 1),
        ],
      })
    }
  }

  submit = () => {
    const patch = new Schema()
    patch.setId(this.props.schema.id)
    patch.setPathsList(this.state.paths)
    this.props.patchSchema(patch)
  }

  validate = (): string | null => {
    const { paths } = this.state
    for (let i = 0; i < paths.length; i++) {
      const after = paths.slice(i + 1)
      if (after.includes(paths[i])) {
        return 'Paths must be unique'
      }
    }

    return null
  }

  render() {
    const error = this.validate()
    return (
      <>
        {error && <div>{error}</div>}
        {this.state.paths.map((path, idx) =>
          <Path
            active={idx === this.state.active}
            addBelow={this.addBelow}
            deleteActive={this.deleteActive}
            edit={this.edit}
            key={`${path}${idx}`}
            path={path}
            setActive={(offset = 0) => this.setState({ active: idx + offset })}
          />)}
      </>
    )
  }
}
