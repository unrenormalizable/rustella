/* eslint-disable react-refresh/only-export-components */
/* eslint-disable no-plusplus */
/* eslint-disable no-bitwise */
/* eslint-disable no-alert */
import { useEffect, useRef, useState } from 'react'
import init, { ThreeVectors } from 'rustella-wasm'

const padNumber = (num, places) => String(num).padStart(places, '0')

const WIDTH = 300
const HEIGHT = 300

const draw = (context, data, buffer, x1, y1, x2, y2) => {
  let i = 0
  for (let y = 0; y < HEIGHT; y++)
    for (let x = 0; x < WIDTH; x++) {
      const d1 = (Math.sqrt((x - x1) * (x - x1) + (y - y1) * (y - y1)) / 10) & 1
      const d2 = (Math.sqrt((x - x2) * (x - x2) + (y - y2) * (y - y2)) / 10) & 1
      buffer[i++] = d1 === d2 ? 0xff000000 : 0xffffffff
    }
  context.putImageData(data, 0, 0)
}

const drawFrame = (count, setFpsText, context, data, buffer) => {
  const start = Date.now()
  draw(
    context,
    data,
    buffer,
    300 + 300 * Math.sin((count * Math.PI) / 180),
    300 + 300 * Math.cos((count * Math.PI) / 180),
    500 + 100 * Math.sin((count * Math.PI) / 100),
    500 + 100 * Math.cos((count * Math.PI) / 100)
  )
  setFpsText(Date.now() - start)
}

const TV = () => {
  const canvasRef = useRef(null)
  const [fps, setFps] = useState(0)
  useEffect(() => {
    init()

    const canvas = canvasRef.current
    const context = canvas.getContext('2d')
    const data = context.createImageData(WIDTH, HEIGHT)
    const buffer = new Uint32Array(data.data.buffer)

    let count = 10
    const interval = setInterval(() => {
      drawFrame(count, setFps, context, data, buffer)
      count++
    }, 20)

    return () => {
      clearInterval(interval)
    }
  }, [])

  return (
    <div className="flex flex-col">
      <canvas className="mx-10 my-3" ref={canvasRef} />
      <button
        type="button"
        onClick={() => alert(`${new ThreeVectors().add_all()} must be 65524`)}
      >
        {`${padNumber(Math.trunc(1000 / fps), 3)} fps`}
      </button>
    </div>
  )
}

export default TV
