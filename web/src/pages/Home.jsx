/* eslint-disable no-alert */

import { useEffect } from 'react'
import init, { ThreeVectors } from 'rustella-wasm'
import * as C from '../components'
import Logo from '../assets/icon.svg'

const Home = () => {
  useEffect(() => {
    init()
  }, [])

  return (
    <main className="flex h-screen w-screen flex-col">
      <header className="flex flex-none flex-col text-center">
        <h1 className="my-0 flex flex-row justify-center text-3xl font-bold">
          <img
            className="flex-none"
            src={Logo}
            width="40px"
            height="40px"
            alt="rustella logo"
          />
          <div>rustella</div>
        </h1>
        <h2 className="my-0 text-sm">Atari 2600 Emulator written in Rust.</h2>
      </header>
      <button type="button" onClick={() => alert(new ThreeVectors().add_all())}>
        hello wasm
      </button>
      <section className="flex-none">Calling into wasm</section>
      <footer className="flex-none">
        <C.Footer
          appVersion={`${import.meta.env.VITE_APP_VERSION}`}
          commitId={import.meta.env.VITE_APP_COMMIT_ID}
        />
      </footer>
    </main>
  )
}

export default Home
