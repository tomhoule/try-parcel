import { Action } from 'typescript-fsa'
import { reducerWithInitialState } from 'typescript-fsa-reducers'
import { fragments as a } from '../actions/fragments'

export const fragments = reducerWithInitialState<FragmentsState>({
  query: null,
}).case(a.query.done, (state, payload) => ({
  ...state,
  query: payload.result,
}))
