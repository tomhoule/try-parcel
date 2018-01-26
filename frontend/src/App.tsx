import * as React from 'react'
import * as ReactDOM from 'react-dom'
import * as ReactRedux from 'react-redux'
import { BrowserRouter } from 'react-router-dom'
import { Route } from 'react-router'
import { store } from './store'
import Texts from './components/Texts'

export class App extends React.Component<{}, {}> {
  render() {
    return (
      <BrowserRouter>
        <Route path='/' component={Texts} />
      </BrowserRouter>
    )
  }
}

export function render() {
  const root = document.querySelector('[data-app-root]')
  ReactDOM.render(
    <ReactRedux.Provider store={store}>
      <App />
    </ReactRedux.Provider>,
    root
  )
}
