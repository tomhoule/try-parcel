import { Action } from 'typescript-fsa'
import { reducerWithInitialState } from 'typescript-fsa-reducers'
import { texts as a } from '../actions/texts'
import { schemas as s } from '../actions/schemas'

export const texts = reducerWithInitialState<TextsState>({
  index: { textsList: [] },
  single: null,
}).case(a.fetchIndex.done, (state, payload) => ({
  ...state,
  index: payload.result,
}))
  .case(a.receive, (state, payload) => ({
    ...state,
    single: state.single ? {
      ...state.single,
      text: payload.toObject(),
    } : state.single,
  }))
  .case(a.fetchSingle.done, (state, payload) => ({
    ...state,
    single: payload.result,
  }))
  .case(s.patchSchema.done, (state, payload) => ({
    ...state,
    single: state.single
      ? { ...state.single, schema: payload.result }
      : state.single,
  }))
  .case(a.patch.done, (state, payload) => ({
    ...state,
    single: state.single
      ? { ...state.single, text: payload.result }
      : state.single,
  }))
