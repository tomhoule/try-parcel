import axios from 'axios'

const textsBase = '/api/texts'
const schemasBase = '/api/schemas'

export async function texts(): Promise<ycy.Text> {
  const res = await axios.get(textsBase)
  return res.data
}

export async function text(id: string): Promise<{ text: ycy.Text, schema: ycy.Schema }> {
  const res = await axios.get(`${textsBase}${id}`)
  return res.data
}

export async function createText(initial: Partial<ycy.Text>): Promise<ycy.Text> {
  const res = await axios.post(textsBase, initial)
  return res.data
}

export async function editText(id: string, patch: Partial<ycy.Text>): Promise<ycy.Text> {
  const res = await axios.patch(`${textsBase}/${id}`, {
    body: patch,
  })
  return res.data
}

export async function deleteText(id: string): Promise<{}> {
  await axios.delete(`${textsBase}/${id}`)
  return {}
}

export async function editPaths(id: string, newPaths: string[]): Promise<ycy.Schema> {
  const res = await axios.patch(`${schemasBase}/${id}`, {
    body: { paths: newPaths },
  })
  return res.data
}

export async function textFragments(textId: string, basePath: string = ''): Promise<ycy.Fragment[]> {
  const res = await axios.get(`${textsBase}/${textId}/fragments`, {
    params: { basePath },
  })
  return res.data
}
