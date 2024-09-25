/* eslint-disable no-bitwise */
import { useEffect, useRef, useState } from 'react'
import useSWR from 'swr'
import init, { ntscColorMap, Atari } from 'rustella-wasm'
import { fetcher } from '../utils'
import ROMS from '../roms'

const TV_WIDTH = 228
const TV_HEIGHT = 262

const fillRect = (ctx, x, y, w, h, color) => {
  ctx.fillStyle = color
  ctx.fillRect(x, y, w, h)
}

const renderFrame = (setTotalFrames, colorMap, context) => (pixels) => {
  const imgData = context.createImageData(TV_WIDTH, TV_HEIGHT)

  for (let x = 0; x < TV_WIDTH; x += 1) {
    for (let y = 0; y < TV_HEIGHT; y += 1) {
      const i = x * TV_WIDTH + y
      const color = colorMap[pixels[i] / 2]
      imgData.data[4 * i + 0] = (color >> 24) & 0xff
      imgData.data[4 * i + 1] = (color >> 16) & 0xff
      imgData.data[4 * i + 2] = (color >> 8) & 0xff
      imgData.data[4 * i + 3] = 255
    }
  }

  context.putImageData(imgData, 0, 0)
  fillRect(context, 68, 0, 160, 3, 'rgba(255, 0, 0, 0.3)')
  fillRect(context, 68, 3, 160, 37, 'rgba(0, 255, 0, 0.3)')
  fillRect(context, 68, 232, 160, 30, 'rgba(0, 0, 255, 0.3)')
  fillRect(context, 0, 0, 68, 262, 'rgba(255, 255, 255, 0.3)')
  setTotalFrames((x) => x + 1)
}

const TV = () => {
  const [selectedROM, setSelectedROM] = useState(0)
  const [initialized, setInitialized] = useState(false)
  const [colorMap, setColorMap] = useState([])
  const [totalTime, setTotalTime] = useState(0)
  const [totalFrames, setTotalFrames] = useState(0)
  const canvasRef = useRef(null)
  const { data: romData } = useSWR(ROMS[selectedROM].url, fetcher, {
    suspense: true,
  })

  useEffect(() => {
    ;(async () => {
      await init()
      setInitialized(true)
      setTotalTime(0)
      setColorMap(ntscColorMap())
    })()
  }, [])

  useEffect(() => {
    if (!initialized) {
      return () => {}
    }

    const canvas = canvasRef.current
    const context = canvas.getContext('2d')
    const atari = new Atari(renderFrame(setTotalFrames, colorMap, context))
    atari.loadROM(
      ROMS[selectedROM].name,
      ROMS[selectedROM].start_addr,
      new Uint8Array(romData)
    )

    const interval = setInterval(() => {
      const start = Date.now()
      atari.tick(20000)
      setTotalTime((x) => x + Date.now() - start)
    }, 10)

    return () => {
      clearInterval(interval)
    }
  }, [initialized, colorMap, selectedROM, romData])

  const dropDownItems = (type, startValue) =>
    ROMS.filter((x) => x.type === type).map((r, i) => {
      const suffix = r.size ? ` (${r.size}K)` : ''
      return (
        <option key={r.name} value={i + startValue}>
          {`${r.name}${suffix}`}
        </option>
      )
    })

  return (
    <div className="flex flex-col items-center">
      <div className="flex flex-row">
        <select
          className="mb-1"
          value={selectedROM}
          onChange={(e) => setSelectedROM(e.target.value)}
        >
          {dropDownItems('test', 0)}
          <option disabled>──────────</option>
          {dropDownItems('game', ROMS.filter((x) => x.type === 'test').length)}
        </select>{' '}
      </div>
      <canvas
        className="bg-black"
        style={{ transform: 'scale(2.0, 1.0)' }}
        width={TV_WIDTH}
        height={TV_HEIGHT}
        ref={canvasRef}
      />
      <figcaption className="mb-2 text-xs">{ROMS[selectedROM].name}</figcaption>
      <div>{`${String(Math.trunc((totalFrames * 1000) / totalTime)).padStart(3, '0')} fps`}</div>
    </div>
  )
}

// eslint-disable-next-line react-refresh/only-export-components
export default TV
