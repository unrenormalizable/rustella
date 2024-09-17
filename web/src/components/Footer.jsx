const Footer = ({ appVersion, commitId }) => {
  const repoUrl = 'https://github.com/unrenormalizable/rustella'
  return (
    <div className="mt-0 text-center text-xs text-blue-500">
      <a href={repoUrl} target="_blank">
        rustella
      </a>
      <span> &bull; </span>
      <a
        href={`${repoUrl}/commit/${commitId}`}
        target="_blank"
      >{`${appVersion}`}</a>
      <span> &bull; </span>
      <a href="https://discord.gg/8FMz2ZpSXK" target="_blank">
        ideas? contribute?
      </a>
    </div>
  )
}

export default Footer
