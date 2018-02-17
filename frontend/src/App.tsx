import * as React from 'react'
import * as ReactDOM from 'react-dom'
import * as ReactRedux from 'react-redux'
import { Route, Switch } from 'react-router'
import { BrowserRouter } from 'react-router-dom'
import Texts from './components/Texts'
import EditText from './components/EditText'
import { store } from './store'

export class App extends React.Component<{}, {}> {
  render() {
    return (
      <BrowserRouter>
        <Switch>
          <Route path='/t/:textId/edit' component={EditText} />
          <Route path='/' component={Texts} />
        </Switch>
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
    root,
  )
}
