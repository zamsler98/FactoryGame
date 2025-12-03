---
description: Create a new branch in Git. 
agent: build
---

Here is the current status of git. 
!`git status`
If there are uncommitted changes, stop and inform the user to commit or stash them first.
Now check out branch $1 and pull the latest changes from the remote repository.
Here is the name of the branch to create: $2
If branch name is empty, create a branch that follows the pattern "feature/{description}", where {description} is a brief summary of the changes to be made on this branch.
Create a branch with this name and switch to it.

