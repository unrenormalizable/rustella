import { useEffect, useRef, useState } from 'react'
import useSWR from 'swr'
import init, { ntscColorMap, Atari } from 'rustella-wasm'
import { fetcher } from '../utils'

const roms = [
  {
    name: 'Step 1 - Generate a Stable Display',
    url: '/1/collect.bin',
    start_addr: 0xf800,
  },
  {
    name: 'Step 2 - Timers',
    url: '/2/collect.bin',
    start_addr: 0xf800,
  },
  {
    name: 'Step 3 - Score & Timer display',
    url: '/3/collect.bin',
    start_addr: 0xf800,
  },
  {
    name: '8blit-s01e04-Playfield-01',
    url: '/3/8blit-s01e04-Playfield-01.bin',
    start_addr: 0xf000,
    info_url:
      'https://github.com/kreiach/8Blit/tree/main/s01e04%20-%20Playfield%20Registers',
  },
]

const TV_WIDTH = 228
const TV_HEIGHT = 262

const drawLine = (ctx, x1, y1, x2, y2, color) => {
  ctx.beginPath()
  ctx.moveTo(x1, y1)
  ctx.lineTo(x2, y2)
  ctx.strokeStyle = color
  ctx.stroke()
}

const renderFrame = (setTotalFrames, colorMap, context) => (pixels) => {
  const data = context.createImageData(TV_WIDTH, TV_HEIGHT)
  const buffer = new Uint32Array(data.data.buffer)
  for (let i = 0; i < pixels.length; i += 1) {
    buffer[i] = colorMap.map[pixels[i] / 2]
  }
  context.putImageData(data, 0, 0)
  drawLine(context, 67, 0, 67, TV_HEIGHT, 'red')
  drawLine(context, 0, 3, TV_WIDTH, 3, 'red')
  drawLine(context, 0, 39, TV_WIDTH, 40, 'red')
  drawLine(context, 0, 233, TV_WIDTH, 232, 'red')
  setTotalFrames((x) => x + 1)
}

const TV = () => {
  const [selectedROM, setSelectedROM] = useState(0)
  const [initialized, setInitialized] = useState(false)
  const [colorMap, setColorMap] = useState({})
  const [totalTime, setTotalTime] = useState(0)
  const [totalFrames, setTotalFrames] = useState(0)
  const canvasRef = useRef(null)
  const { data: romData } = useSWR(roms[selectedROM].url, fetcher, {
    suspense: true,
  })

  useEffect(() => {
    ;(async () => {
      await init()
      setInitialized(true)
      setTotalTime(0)
      setColorMap({ map: ntscColorMap() })
    })()
  }, [])

  useEffect(() => {
    if (!initialized) {
      return () => {}
    }

    const canvas = canvasRef.current
    const context = canvas.getContext('2d', { willReadFrequently: true })
    const atari = new Atari(renderFrame(setTotalFrames, colorMap, context))
    atari.loadROM(roms[selectedROM].start_addr, new Uint8Array(romData))

    const interval = setInterval(() => {
      const start = Date.now()
      atari.tick(20000)
      setTotalTime((x) => x + Date.now() - start)
    }, 10)

    return () => {
      clearInterval(interval)
    }
  }, [initialized, colorMap, selectedROM, romData])

  const dropDownItems = roms.map((r, i) => (
    <option key={r.name} value={i}>
      {r.name}
    </option>
  ))

  return (
    <div className="flex flex-col items-center">
      <select
        className="mb-1"
        value={selectedROM}
        onChange={(e) => setSelectedROM(e.target.value)}
      >
        {dropDownItems}
      </select>{' '}
      <canvas
        className="bg-black"
        style={{ transform: 'scale(2.0, 1.0)' }}
        width={TV_WIDTH}
        height={TV_HEIGHT}
        ref={canvasRef}
      />
      <figcaption className="mb-2 text-xs">{roms[selectedROM].name}</figcaption>
      <div>{`${String(Math.trunc((totalFrames * 1000) / totalTime)).padStart(3, '0')} fps`}</div>
    </div>
  )
}

// eslint-disable-next-line react-refresh/only-export-components
export default TV
