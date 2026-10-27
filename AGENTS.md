# Instructions

## Git workflow

-   When fixing a GitHub issue, add the issue author as a co-author of the fix commit, unless they are already its author.
-   Use `gh` to verify the issue author's GitHub login, numeric account ID, and display name. Use the login when no display name is available.
-   Add a `Co-authored-by: Name <ID+login@users.noreply.github.com>` trailer with the verified account details. Do not guess an email address or use the pull request author in place of the issue author. Credit each distinct author when fixing multiple issues.
-   After pushing, verify that GitHub recognizes each co-author's account in the commit authors. Preserve these trailers in the final merge commit, including squash merges, so GitHub can attribute the contribution.
