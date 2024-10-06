/* eslint-disable no-bitwise */
import { useEffect, useRef, useState } from 'react'
import { useSearchParams } from 'react-router-dom'
import useSWR from 'swr'
import humanizeString from 'humanize-string'
import init, { ntscColorMap, Atari } from 'rustella-wasm'
import { fetcher, getStartAddress } from '../utils'
import ROMS from '../roms'
import RomUploader from './RomUploader'

const TV_WIDTH = 228
const TV_HEIGHT = 262

const fillRect = (ctx, x, y, w, h, color) => {
  ctx.fillStyle = color
  ctx.fillRect(x, y, w, h)
}

const renderFrame = (setTotalFrames, colorMap, context) => (pixels) => {
  const imgData = context.createImageData(TV_WIDTH, TV_HEIGHT)

  for (let i = 0; i < imgData.data.length; i += 4) {
    const color = colorMap[pixels[Math.floor(i / 4)] / 2]
    imgData.data[i + 0] = (color >> 24) & 0xff
    imgData.data[i + 1] = (color >> 16) & 0xff
    imgData.data[i + 2] = (color >> 8) & 0xff
    imgData.data[i + 3] = 0xff
  }

  context.putImageData(imgData, 0, 0)
  fillRect(context, 0, 0, 68, 262, 'rgba(255, 255, 255, 0.2)')
  setTotalFrames((x) => x + 1)
}

const defaultUploadedRomInfo = { name: '', data: new Uint8Array() }

const TV = () => {
  const [searchParams, setSearchParams] = useSearchParams()
  const canvasRef = useRef(null)
  const [wasmInitialized, setWasmInitialized] = useState(false)
  const [colorMap, setColorMap] = useState([])
  const [totalTime, setTotalTime] = useState(0)
  const [totalFrames, setTotalFrames] = useState(0)
  const [selectedStockRomId, setSelectedStockRomId] = useState(0)
  const [uploadedRomInfo, setUploadedRomInfo] = useState(defaultUploadedRomInfo)
  const [romName, setRomName] = useState('')
  const { data: stockRomData } = useSWR(ROMS[selectedStockRomId].url, fetcher, {
    suspense: true,
  })

  useEffect(() => {
    ;(async () => {
      await init()
      setWasmInitialized(true)
      setColorMap(ntscColorMap())
    })()
  }, [])

  useEffect(() => {
    if (!wasmInitialized) {
      return
    }

    const id = parseInt(searchParams.get('id'), 10)
    setSelectedStockRomId(Number.isNaN(id) ? 0 : id)
  }, [wasmInitialized, searchParams])

  useEffect(() => {
    if (!wasmInitialized) {
      return () => {}
    }

    const romData = uploadedRomInfo.data.length
      ? uploadedRomInfo.data
      : stockRomData
    const name = uploadedRomInfo.data.length
      ? `${uploadedRomInfo.name} (uploaded)`
      : ROMS[selectedStockRomId].name
    setRomName(name)
    const startAddr = uploadedRomInfo.data.length
      ? getStartAddress(uploadedRomInfo.data.length)
      : ROMS[selectedStockRomId].start_addr

    const atari = new Atari(
      renderFrame(setTotalFrames, colorMap, canvasRef.current.getContext('2d'))
    )
    atari.loadROM(name, startAddr, new Uint8Array(romData))

    setTotalTime(0)
    setTotalFrames(0)
    const interval = setInterval(() => {
      const start = Date.now()
      atari.tick(20000)
      setTotalTime((x) => x + Date.now() - start)
    }, 10)

    return () => {
      clearInterval(interval)
    }
  }, [
    wasmInitialized,
    colorMap,
    selectedStockRomId,
    stockRomData,
    uploadedRomInfo,
  ])

  const stockRomDropDownItems = (type, startValue) =>
    ROMS.filter((x) => x.type === type).map((r, i) => {
      const suffix = r.size ? ` (${r.size}K)` : ''
      const index = i + startValue
      return (
        <option key={r.name} value={index}>
          {`[${index}] ${r.name}${suffix}`}
        </option>
      )
    })

  return (
    <div className="flex flex-col items-center">
      <div className="mb-1 flex flex-row">
        <select
          className="max-w-60"
          value={selectedStockRomId}
          onChange={(e) => {
            const id = e.target.value
            setSearchParams({ id })
            setSelectedStockRomId(id)
            setUploadedRomInfo(defaultUploadedRomInfo)
          }}
        >
          {stockRomDropDownItems('test', 0)}
          <option disabled>──────────</option>
          {stockRomDropDownItems(
            'game',
            ROMS.filter((x) => x.type === 'test').length
          )}
        </select>
        <div className="mx-4">OR</div>
        <RomUploader
          setRomInfo={(x) => {
            // setSearchParams({})
            setUploadedRomInfo(x)
          }}
        />
      </div>
      <canvas
        className="bg-black"
        style={{ transform: 'scale(2.0, 1.0)' }}
        width={TV_WIDTH}
        height={TV_HEIGHT}
        ref={canvasRef}
      />
      <figcaption className="mb-2 text-xs">
        {humanizeString(romName)}
      </figcaption>
      <div>{`${String(Math.trunc((totalFrames * 1000) / totalTime)).padStart(3, '0')} fps`}</div>
    </div>
  )
}

// eslint-disable-next-line react-refresh/only-export-components
export default TV
