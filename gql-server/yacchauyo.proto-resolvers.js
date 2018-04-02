const grpc = require('grpc')
const Yacchauyo = grpc.load('./yacchauyo.proto')

const YacchauyoStub = new Yacchauyo.Yacchauyo(process.env.YACCHAUYO_BACKEND_URL, grpc.credentials.createInsecure())

console.log(YacchauyoStub.TextsIndex)

module.exports = {
  Query: {
    yacchauyo: () => ({
      textsIndex: ({ texts_query: req }) => {
        return new Promise((resolve, reject) => YacchauyoStub.TextsIndex({...req}, (err, res) => err ? reject(err) : resolve(res)))
      },
      createText: ({ text: req }) => {
        return new Promise((resolve, reject) => YacchauyoStub.CreateText({...req}, (err, res) => err ? reject(err) : resolve(res)))
      },
      patchText: ({ text: req }) => {
        return new Promise((resolve, reject) => YacchauyoStub.PatchText({...req}, (err, res) => err ? reject(err) : resolve(res)))
      },
      textSchema: ({ texts_query: req }) => {
        return new Promise((resolve, reject) => YacchauyoStub.TextSchema({...req}, (err, res) => err ? reject(err) : resolve(res)))
      },
      patchSchema: ({ schema: req }) => {
        return new Promise((resolve, reject) => YacchauyoStub.PatchSchema({...req}, (err, res) => err ? reject(err) : resolve(res)))
      },
      queryFragments: ({ fragments_query: req }) => {
        return new Promise((resolve, reject) => YacchauyoStub.QueryFragments({...req}, (err, res) => err ? reject(err) : resolve(res)))
      },
    }),
  },
}
