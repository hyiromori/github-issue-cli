import { fetchGitHubGraphQL } from "./graphql.ts";
import { decodeBase64 } from "../base64.ts";
import { debugLog } from "../logger.ts";

interface Repository {
  id: number;
}

const cache: { [key: string]: Repository } = {};

export const getRepository = async (
  organization: string,
  repository: string,
): Promise<Repository> => {
  const cacheKey = `${organization}/${repository}`;
  if (cache[cacheKey] != null) {
    return cache[cacheKey];
  }
  const json = await fetchGitHubGraphQL(`
    {
      repository(owner: "${organization}", name: "${repository}") {
        id
      }
    }`);
  if ("errors" in json) {
    throw new Error(`リポジトリデータの取得に失敗しました: ${cacheKey}`)
  }
  const id: number = parseInt(
    (decodeBase64(json.data.repository.id).match(/[0-9]+$/) ??
      [""])[0],
    10,
  );
  debugLog(`Repository ID: ${id} (${repository})`);

  const data: Repository = { id };
  cache[cacheKey] = data;
  return data;
};
