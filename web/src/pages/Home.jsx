import * as C from '../components'
import Logo from '../assets/icon-black.svg'

const Home = () => (
  <main className="flex h-screen w-screen flex-col">
    <header className="flex flex-none flex-col text-center">
      <h1 className="my-0 mb-2 flex flex-row justify-center text-3xl font-bold">
        <img
          className="mr-2 mt-2 flex-none"
          src={Logo}
          width="25px"
          height="25px"
          alt="rustella logo"
        />
        <div>rustella</div>
      </h1>
    </header>
    <section className="flex-grow overflow-y-auto px-3">
      <C.TV />
    </section>
    <footer className="flex-none">
      <C.Footer
        appVersion={`${import.meta.env.VITE_APP_VERSION}`}
        commitId={import.meta.env.VITE_APP_COMMIT_ID}
      />
    </footer>
  </main>
)

export default Home
