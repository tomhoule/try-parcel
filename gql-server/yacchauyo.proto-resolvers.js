const grpc = require('grpc')
const services = grpc.load('./yacchauyo.proto')

const YacchauyoStub = new services.Yacchauyo(process.env.YACCHAUYO_BACKEND_URL, grpc.credentials.createInsecure())

module.exports = {
  Query: {
    yacchauyo: () => ({
      textsIndex: async ({ texts_query: req }) => {
        const res = await new Promise((resolve) => YacchauyoStub.TextsIndex({...req}, (err, res) => resolve(res)))
        return res
      },
      createText: async (req) => {
        const res = await YacchauyoStub.call(services.Yacchauyo.CreateText, req)
        return res.toJson()
      },
      patchText: async (req) => {
        const res = await YacchauyoStub.call(services.Yacchauyo.PatchText, req)
        return res.toJson()
      },
      textSchema: async (req) => {
        const res = await YacchauyoStub.call(services.Yacchauyo.TextSchema, req)
        return res.toJson()
      },
      patchSchema: async (req) => {
        const res = await YacchauyoStub.call(services.Yacchauyo.PatchSchema, req)
        return res.toJson()
      },
      queryFragments: async (req) => {
        const res = await YacchauyoStub.call(services.Yacchauyo.QueryFragments, req)
        return res.toJson()
      },
    }),
  },
}
