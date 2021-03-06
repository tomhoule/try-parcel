import * as React from 'react'
import * as ReactDOM from 'react-dom'
import ConfirmModal from './ConfirmModal'
import TextForm from './TextForm'
import SchemaEditor from './SchemaEditor'

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
attach('[data-confirm]', ConfirmModal)
attach('[data-schema-editor]', SchemaEditor)
