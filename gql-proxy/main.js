const { graphqlExpress, graphiqlExpress } = require('apollo-server-express');
const express = require('express');
const { makeExecutableSchema } = require('graphql-tools');
const bodyParser = require('body-parser');
const Client = require('./src/rpc/proto/yacchauyo_grpc_pb').YacchauyoClient
const grpc = require('grpc')
const pb = require('./src/rpc/proto/yacchauyo_pb.js')

const TextT = `
  type Text {
    id: String!,
    title: String!,
    slug: String!,
    authors: String!,
    description: String!,
  }
`

const TextsQuery = `
  input TextsQuery {
    id: String!,
    title: String!,
  }
`

const Texts = `
  type Texts {
    texts: [Text!]!
  }
`

const Query = `
  type Query {
    textsIndex(q: TextsQuery): [Text!]!
  }
`

const typeDefs = [TextT, TextsQuery, Query]
const resolvers = {
  Query: {
    textsIndex: (input) => {
      const client = new Client('localhost:4443', grpc.credentials.createInsecure(), {})
      const q = new pb.TextsQuery()
      return new Promise(resolve => client.textsIndex(q, (err, feat) => {
        console.log('err', err)
        console.log('feat', feat.toObject())
        resolve(feat.toObject().textsList)
      }))
  },
}

const schema = makeExecutableSchema({
  typeDefs,
  resolvers,
})

const app = express()


// The GraphQL endpoint
app.use('/graphql', bodyParser.json(), graphqlExpress({ schema }));

// GraphiQL, a visual editor for queries
app.use('/graphiql', graphiqlExpress({ endpointURL: '/graphql' }));

// Start the server
app.listen(3000, () => {
  console.log('Go to http://localhost:3000/graphiql to run queries!');
});
