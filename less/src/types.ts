import { Code } from "grpc-web-client/dist/Code"

export {}

declare global {
  interface RpcFailure {
    status: Code
    statusMessage: string
  }
}
