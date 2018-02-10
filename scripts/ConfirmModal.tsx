import * as React from 'react'
import * as ReactDOM from 'react-dom'

export default class ConfirmModal extends React.Component<TopLevel> {
  render() {
    const target = document.querySelector('[data-modal-container]')
    if (target) {
      const modal = (
        <div className='absolute pin'>
          <div className='z-10 opacity-25 shadow-md absolute pin bg-black'></div>
          <form
            action={this.props.data['action']}
            className='block relative z-20 bg-white p-6 m-auto w-4/5 mt-8'
            method='post'
          >
            <button
              className='bg-blue p-4 rounded text-white'
              type='submit'
            >
              I'm sure
            </button>
          </form>
        </div>
      )

      return ReactDOM.createPortal(modal, target)
    } else {
      return 'error - cannot create modal'
    }
  }
}
