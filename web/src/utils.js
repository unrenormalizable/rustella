export const sleep = (ms) =>
  new Promise((r) => {
    setTimeout(r, ms)
  })

export const fetcher = async (url) => {
  const res = await fetch(url, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/octet-stream',
    },
    responseType: 'arraybuffer',
  })

  if (!res.ok) {
    const error = new Error(
      `An error occurred while fetching the data from '${url}'.`
    )
    throw error
  }

  const bin = await res.arrayBuffer()
  return bin
}

export const getStartAddress = (romSize) => (romSize > 2048 ? 0xf000 : 0xf800)
