import { Code } from 'grpc-web-client/dist/Code'
import * as proto from './rpc/yacchauyo_pb'

declare global {

  interface TextSingle {
    schema: proto.Schema.AsObject
    text: proto.Text.AsObject
  }

  interface TextsState {
    index: proto.Texts.AsObject
    single: TextSingle | null
  }

  interface FragmentsState {
    query: proto.FragmentsQuery.AsObject | null
  }

  interface AppState {
    texts: TextsState
    fragments: FragmentsState
  }

  interface RpcFailure {
    status: Code
    statusMessage: string
  }

  namespace ycy {
    type Path = string
    type Uuid = string
    type PgTimestamp = number
    type Option<T> = T | null

    /**
     * [API docs](http://localhost:8000/docs/yacchauyo/models/texts/struct.Text.html)
     */
    interface Text {
      id: Uuid
      title: string
      slug: string
      authors: string
      description: string
      created_at: string
      updated_at: string
    }

    /**
     * [API docs](http://localhost:8000/docs/yacchauyo/models/schemas/struct.Schema.html)
     */
    interface Schema {
      id: Uuid
      text_id: Uuid
      paths: Path[]
      created_at: string
    }

    /**
     * [API docs](http://localhost:8000/docs/yacchauyo/models/fragments/struct.Fragment.html)
     */
    interface Fragment {
      id: Uuid
      schema_path: string
      text_id: Option<Uuid>
      value: Option<string>
      created_at: PgTimestamp

    }
  }
}
