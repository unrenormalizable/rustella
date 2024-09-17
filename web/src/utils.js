export const sleep = (ms) =>
  new Promise((r) => {
    setTimeout(r, ms)
  })

export const fetcher = async (url) => {
  const res = await fetch(url)
  if (!res.ok) {
    const error = new Error('An error occurred while fetching the data.')
    error.info = await res.json()
    error.status = res.status
    throw error
  }

  const json = await res.json()
  return json
}
