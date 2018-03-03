import * as React from 'react'
import * as ReactDOM from 'react-dom'
import * as ReactRouter from 'react-router'
import * as ReactRouterDOM from 'react-router-dom'
import { Home } from './components/Home'

const Show = () => <>show<ReactRouterDOM.Link to='/'>index</ReactRouterDOM.Link></>

export class App extends React.Component<{}, {}> {
  render() {
    return (
      <>
        <ReactRouterDOM.BrowserRouter>
          <ReactRouter.Switch>
            <ReactRouter.Route path='/' exact component={Home} />
            <ReactRouter.Route path='/show' component={Show} />
          </ReactRouter.Switch>
        </ReactRouterDOM.BrowserRouter>
      </>
    )
  }
}

export function start() {
  const root = document.querySelector('[data-app-root]')
  ReactDOM.render(<App />, root)
}
