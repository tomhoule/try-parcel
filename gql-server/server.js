const { GraphQLServer } = require('graphql-yoga')
const typeDefs = require('./yacchauyo.proto-type-defs').all
const resolvers = require('./yacchauyo.proto-resolvers')

const server = new GraphQLServer({ typeDefs, resolvers })
server.start(() => console.log('Server is running on localhost:4000'))
