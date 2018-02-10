import * as React from 'react'
import * as ReactDOM from 'react-dom'

interface State {
  modalOpen: boolean
}

export default class ConfirmModal extends React.Component<TopLevel, State> {
  state = {
    modalOpen: false,
  }

  modal = () => (
    <div className='absolute pin'>
      <div
        className='z-10 opacity-25 shadow-md absolute pin bg-black'
        onClick={() => this.setState({ modalOpen: false })}
      />
      <form
        action={this.props.data['action']}
        className='block relative z-20 bg-white p-6 m-auto w-4/5 mt-8'
        method='post'
        onClick={event => event.preventDefault()}
      >
        <button
          className='bg-blue p-4 rounded text-white'
          type='submit'
        >
          I'm sure
      </button>
      </form>
    </div>)

  render() {
    if (this.state.modalOpen) {
      const target = document.querySelector('[data-modal-container]')
      if (target) {
        return ReactDOM.createPortal(this.modal(), target)
      } else {
        return 'error - cannot create modal'
      }
    } else {
      return (
        <button onClick={() => this.setState({ modalOpen: true })}>
          {this.props.data['button-text'] || 'no button text'}
        </button>)
    }
  }
}
