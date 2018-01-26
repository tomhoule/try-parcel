import { Action } from 'typescript-fsa'
import { reducerWithInitialState } from 'typescript-fsa-reducers'
import { texts as a } from '../actions/texts'

export const texts = reducerWithInitialState<TextsState>({
  index: []
}).case(a.fetchIndex.done, (state, payload) => ({
  index: [],
}))
