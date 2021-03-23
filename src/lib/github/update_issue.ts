import {IssueInfo} from "../types.ts";
import {getGitHubAccessToken} from "../env.ts";
import {verbose} from "../args.ts";

const GitHubURL = "https://api.github.com"

export interface UpdateIssueData {
  labels?: string[]
}

// https://docs.github.com/en/rest/reference/issues#update-an-issue
export const updateIssue = async (issue: IssueInfo, data: UpdateIssueData): Promise<void> => {
  const {organization, repository, issueNumber} = issue
  const path = `/repos/${organization}/${repository}/issues/${issueNumber}`
  const response = await fetch(`${GitHubURL}${path}`, {
    method: 'PATCH',
    headers: {
      Authorization: `token ${getGitHubAccessToken()}`,
      Accept: "application/vnd.github.v3+json"
    },
    body: JSON.stringify(data)
  })

  if (!response.ok) {
    throw new Error("GitHub の Issue データの更新に失敗しました。")
  }
  if (verbose()) {
    const responseData = await response.json()
    console.log("Response data:", JSON.stringify(responseData, null, 2))
  }
}

