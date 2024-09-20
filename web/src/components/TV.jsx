/* eslint-disable react-refresh/only-export-components */
/* eslint-disable no-plusplus */
import { useEffect, useRef, useState } from 'react'
import useSWR from 'swr'
import init, { Atari } from 'rustella-wasm'
import { fetcher } from '../utils'

const padNumber = (num, places) => String(num).padStart(places, '0')

const WIDTH = 228
const HEIGHT = 262

const renderScanline = (setFps, context) => (scanline, pixels) => {
  const start = Date.now()

  const data = context.getImageData(0, scanline, scanline + WIDTH, 1)
  const buffer = new Uint32Array(data.data.buffer)

  for (let i = 0; i < pixels.length; i++) {
    buffer[i] = pixels[i]
  }
  context.putImageData(data, 0, scanline)
  setFps(Date.now() - start)
}

const TV = () => {
  const [initialized, setInitialized] = useState(false)
  const canvasRef = useRef(null)
  const [fps, setFps] = useState(0)
  const { data: romData } = useSWR('collect.bin', fetcher, {
    suspense: true,
  })

  useEffect(() => {
    init().then(() => setInitialized(true))
  })

  useEffect(() => {
    if (!initialized) {
      return () => {}
    }

    const canvas = canvasRef.current
    const context = canvas.getContext('2d', { willReadFrequently: true })
    const atari = new Atari(renderScanline(setFps, context))
    atari.loadROM(0xf800, new Uint8Array(romData))

    const interval = setInterval(() => {
      atari.tick()
    }, 5)

    return () => {
      clearInterval(interval)
    }
  }, [romData, initialized])

  return (
    <div className="flex flex-col">
      <canvas
        className="mx-auto my-3 h-[50%] w-[50%] bg-black"
        width={WIDTH}
        height={HEIGHT}
        ref={canvasRef}
      />
      <div className="mx-auto">{`${padNumber(Math.trunc(1000 / fps), 3)} fps`}</div>
    </div>
  )
}

export default TV
