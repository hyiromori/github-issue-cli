const usage = () => [
  "More information: https://github.com/hyiromori/github-update-issue/blob/main/README.md",
  "",
  "USAGE",
  "  github-issue <command> [options]",
  "",
  "CORE COMMANDS",
  "  create",
  "  update",
  "  help",
].join("\n")

export const help = (): void => {
  console.log(usage())
}

