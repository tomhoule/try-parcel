import * as React from 'react'
import * as ReactDOM from 'react-dom'
import TextForm from './TextForm'

function attach(selector: string, Component: React.ComponentClass): void {
  const anchor = document.querySelector(selector)
  if (anchor) {
    ReactDOM.render(<Component />, anchor)
  }
}

attach('[data-text-new-form]', TextForm)
