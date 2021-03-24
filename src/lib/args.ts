interface Args {
  command: string;
  verbose: boolean;
  labels: string[] | null;
  pipeline: string | null;
  workspace: string | null;
  epicUrl: string | null;
  urls: string[];
}

const args: Args = {
  command: "help",
  verbose: false,
  labels: null,
  pipeline: null,
  workspace: null,
  epicUrl: null,
  urls: [],
}

args.command = Deno.args[0] ?? "help"

for (let i = 1; i < Deno.args.length; i++) {
  const arg: string = Deno.args[i]
  const next: string | undefined = Deno.args[i + 1]
  switch (arg) {
    case "-v":
    case "--verbose":
      args.verbose = true
      break;
    case "--pipeline":
      if (next == null) {
        throw new Error("パイプラインが指定されていません。")
      }
      args.pipeline = next
      i++
      break;
    case "--epic-url":
      if (next == null) {
        throw new Error("エピックURLが指定されていません。")
      }
      args.epicUrl = next
      i++
      break;
    case "--workspace":
      if (next == null) {
        throw new Error("ワークスペースが指定されていません。")
      }
      args.workspace = next
      i++
      break;
    case "--labels":
      if (next == null) {
        throw new Error("ラベルの内容が指定されていません。")
      }
      args.labels = next.split(",")
      i++
      break;
    default:
      if (arg.startsWith("https://")) {
        args.urls.push(arg)
        break;
      }
      throw new Error(`不明な引数です: ${arg}`)
  }
}

export const verbose = (): boolean => args.verbose
export const getCommand = (): string => args.command
export const getEpicUrl = (): string | null => args.epicUrl
export const getLabels = (): string[] | null => args.labels
export const getPipeline = (): string | null => args.pipeline
export const getUrls = (): string[] => args.urls
export const getWorkspace = (): string | null => args.workspace
