import { ExclamationCircleIcon } from '@heroicons/react/20/solid'

const Error = ({ children }) => (
  <div className="flex h-full justify-center">
    <div className="flex content-center items-center text-center text-sm font-semibold text-red-500">
      <ExclamationCircleIcon className="mr-2 h-5 w-5" />
      {children}
    </div>
  </div>
)

export default Error
