import { Action } from 'typescript-fsa'
import { reducerWithInitialState } from 'typescript-fsa-reducers'
import { texts as a } from '../actions/texts'

export const texts = reducerWithInitialState<TextsState>({
  index: { textsList: [] },
  single: null,
}).case(a.fetchIndex.done, (state, payload) => ({
  ...state,
  index: payload.result,
}))
  .case(a.fetchSingle.done, (state, payload) => ({
    ...state,
    single: payload.result,
  }))
