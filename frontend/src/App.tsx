import * as React from 'react'
import * as ReactDOM from 'react-dom'
import { css } from 'react-emotion'
import * as ReactRedux from 'react-redux'
import { Route, Switch } from 'react-router'
import { BrowserRouter } from 'react-router-dom'
import Texts from './components/Texts'
import CreateText from './components/CreateText'
import EditText from './components/EditText'
import { store } from './store'

const appRoot = css({
  backgroundColor: 'rgb(50, 80, 70)',
  minHeight: '100vh',
  position: 'absolute',
  width: '100%',
})

const content = css({
  backgroundColor: 'rgb(240, 240, 240)',
  border: 'solid 1px rgba(0, 0, 0, .3)',
  margin: '5em auto 0',
  padding: '1em',
  width: '50em',
})

export class App extends React.Component<{}, {}> {
  render() {
    return (
      <div className={appRoot}>
        <div className={content}>
          <BrowserRouter>
            <Switch>
              <Route path='/t/:textId/edit' component={EditText} />
              <Route path='/texts/new' component={CreateText} />
              <Route path='/' component={Texts} exact />
            </Switch>
          </BrowserRouter>
        </div>
      </div>
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
