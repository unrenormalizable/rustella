/* eslint-disable react-refresh/only-export-components */
/* eslint-disable no-plusplus */
import { useEffect, useRef, useState } from 'react'
import useSWR from 'swr'
import init, { ntscColorMap, Atari } from 'rustella-wasm'
import { fetcher } from '../utils'

const padNumber = (num, places) => String(num).padStart(places, '0')

const WIDTH = 228
const HEIGHT = 262

const drawLine = (ctx, x1, y1, x2, y2, color) => {
  ctx.beginPath()
  ctx.moveTo(x1, y1)
  ctx.lineTo(x2, y2)
  ctx.strokeStyle = color
  ctx.stroke()
}

const renderFrame = (setTotalFrames, colorMap, context) => (pixels) => {
  const data = context.createImageData(WIDTH, HEIGHT)
  const buffer = new Uint32Array(data.data.buffer)
  for (let i = 0; i < pixels.length; i++) {
    buffer[i] = colorMap.map[pixels[i]]
  }
  context.putImageData(data, 0, 0)
  drawLine(context, 0, 3, WIDTH, 3, 'red')
  drawLine(context, 0, 40, WIDTH, 40, 'red')
  drawLine(context, 0, 232, WIDTH, 232, 'red')
  setTotalFrames((x) => x + 1)
}

const TV = () => {
  const [initialized, setInitialized] = useState(false)
  const [colorMap, setColorMap] = useState({})
  const [totalTime, setTotalTime] = useState(0)
  const [totalFrames, setTotalFrames] = useState(0)
  const canvasRef = useRef(null)
  const { data: romData } = useSWR('collect.bin', fetcher, {
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
    atari.loadROM(0xf800, new Uint8Array(romData))

    const interval = setInterval(() => {
      const start = Date.now()
      atari.tick(20000)
      setTotalTime((x) => x + Date.now() - start)
    }, 10)

    return () => {
      clearInterval(interval)
    }
  }, [initialized, colorMap, romData])

  return (
    <div className="flex flex-col">
      <canvas
        className="mx-auto my-3 h-[50%] w-[50%] bg-black"
        width={WIDTH}
        height={HEIGHT}
        ref={canvasRef}
      />
      <div className="mx-auto">{`${padNumber(Math.trunc((totalFrames * 1000) / totalTime), 3)} fps`}</div>
    </div>
  )
}

export default TV
