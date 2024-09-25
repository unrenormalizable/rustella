const makeFileReaderPromise = (file) =>
  new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onload = () => resolve(reader.result)
    reader.onerror = reject
    reader.readAsArrayBuffer(file)
  })

const RomUploader = ({ setRomInfo }) => {
  const changeHandler = async (e) => {
    if (!e.target || e.target.length === 0) {
      return
    }
    const data = await makeFileReaderPromise(e.target.files[0])
    setRomInfo({ name: e.target.files[0].name, data: new Uint8Array(data) })
  }

  return <input type="file" onChange={changeHandler} />
}

export default RomUploader
