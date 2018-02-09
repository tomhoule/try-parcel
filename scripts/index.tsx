import * as React from 'react'
import * as ReactDOM from 'react-dom'
import TextForm from './TextForm'

declare global {
  interface TopLevel {
    data: DOMStringMap
  }
}

function attach(selector: string, Component: React.ComponentClass<TopLevel>): void {
  const anchors = document.querySelectorAll(selector)
  anchors.forEach(anchor => {
    ReactDOM.render(<Component data={(anchor as HTMLElement).dataset} />, anchor)
  })
}

attach('[data-text-new-form]', TextForm)
