/* eslint-disable react-refresh/only-export-components */
/* eslint-disable no-plusplus */
import { useEffect, useRef, useState } from 'react'
import useSWR from 'swr'
import init, { ntscColorMap, Atari } from 'rustella-wasm'
import { fetcher } from '../utils'

const padNumber = (num, places) => String(num).padStart(places, '0')

const WIDTH = 228
const HEIGHT = 262

const renderFrame = (setFrames, frames, colorMap, context) => (pixels) => {
  const data = context.createImageData(WIDTH, HEIGHT)
  const buffer = new Uint32Array(data.data.buffer)
  for (let i = 0; i < pixels.length; i++) {
    buffer[i] = colorMap.map[pixels[i]]
  }
  context.putImageData(data, 0, 0)
  setFrames(frames + 1)
}

const TV = () => {
  const [initialized, setInitialized] = useState(false)
  const [colorMap, setColorMap] = useState({})
  const [startTime, setStartTime] = useState({ start: Date.now() })
  const canvasRef = useRef(null)
  const [frames, setFrames] = useState(0)
  const { data: romData } = useSWR('collect.bin', fetcher, {
    suspense: true,
  })

  useEffect(() => {
    ;(async () => {
      await init()
      setInitialized(true)
      setStartTime({ start: Date.now() })
      setColorMap({ map: ntscColorMap() })
    })()
  }, [])

  useEffect(() => {
    if (!initialized) {
      return () => {}
    }

    const canvas = canvasRef.current
    const context = canvas.getContext('2d', { willReadFrequently: true })
    const atari = new Atari(renderFrame(setFrames, frames, colorMap, context))
    atari.loadROM(0xf800, new Uint8Array(romData))

    const interval = setInterval(() => {
      atari.tick(20000)
    }, 10)

    return () => {
      clearInterval(interval)
    }
  }, [initialized, frames, colorMap, romData])

  return (
    <div className="flex flex-col">
      <canvas
        className="mx-auto my-3 h-[50%] w-[50%] bg-black"
        width={WIDTH}
        height={HEIGHT}
        ref={canvasRef}
      />
      <div className="mx-auto">{`${padNumber(Math.trunc((frames * 1000) / (Date.now() - startTime.start)), 3)} fps`}</div>
    </div>
  )
}

export default TV
