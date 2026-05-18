# Git Merge Conflict Resolution Guide

When pulling from a remote repository and you have divergent branches, conflicts may occur. This guide provides three different strategies to resolve merge conflicts based on your needs.

---

## Prerequisites

```bash
# First, configure your git to use merge strategy
git config pull.rebase false

# Then attempt to pull
git pull origin <branch-name>
```

If conflicts occur, you'll see messages like:
```
CONFLICT (content): Merge conflict in <filename>
Automatic merge failed; fix conflicts and then commit the result.
```

---

## Solution 1: Ignore Remote and Keep Local

**Use this when:** You want to discard all remote changes and keep only your local version.

### Step-by-step:

```bash
# 1. View conflicted files
git status

# 2. Resolve conflicts by taking your local version
# For each conflicted file, use:
git checkout --ours <filename>

# Or resolve all conflicts at once:
git diff --name-only --diff-filter=U | xargs git checkout --ours

# 3. Stage all resolved files
git add .

# 4. Complete the merge
git commit -m "Merge: keeping local version, discarding remote changes"

# 5. Push your changes
git push origin <branch-name>
```

### Alternative (one-liner):

```bash
git checkout --ours . && git add . && git commit -m "Merge: using local version"
```

---

## Solution 2: Ignore Local and Keep Remote

**Use this when:** You want to discard all your local changes and accept the remote version.

### Step-by-step:

```bash
# 1. View conflicted files
git status

# 2. Resolve conflicts by taking the remote version
# For each conflicted file, use:
git checkout --theirs <filename>

# Or resolve all conflicts at once:
git diff --name-only --diff-filter=U | xargs git checkout --theirs

# 3. Stage all resolved files
git add .

# 4. Complete the merge
git commit -m "Merge: accepting remote version, discarding local changes"

# 5. Push the merged state
git push origin <branch-name>
```

### Alternative (one-liner):

```bash
git checkout --theirs . && git add . && git commit -m "Merge: using remote version"
```

---

## Solution 3: Keep Both Local and Remote, Then Push

**Use this when:** You want to manually merge both versions, combining the best of both local and remote changes.

### Step-by-step:

```bash
# 1. View conflicted files
git status

# 2. Open each conflicted file and manually resolve conflicts
# Look for conflict markers:
# <<<<<<<< HEAD          (your local version)
# your local content
# ========
# remote content
# >>>>>>>> origin/branch-name

# Example file with conflicts:
# <<<<<<< HEAD
# const version = "1.0.0"
# ========
# const version = "1.1.0"
# >>>>>>> origin/main

# 3. Edit the file to keep what you want from both versions:
nano <filename>
# or
code <filename>
# or your preferred editor

# 4. Remove the conflict markers and keep desired content
# After editing, the file should look like:
# const version = "1.0.0"  // or "1.1.0" or both if applicable

# 5. Stage the resolved files
git add <filename>

# Or stage all resolved files:
git add .

# 6. View the merge state
git status

# 7. Complete the merge
git commit -m "Merge: combining local and remote changes"

# 8. Push the merged result
git push origin <branch-name>
```

### Manual Resolution Tips:

- **Understand conflict markers:**
  - `<<<<<<< HEAD` = Start of your local changes
  - `=======` = Divider between local and remote
  - `>>>>>>> origin/branch` = End of remote changes

- **Common merge scenarios:**
  - **Same file, different changes:** Combine both changes
  - **Same line changed:** Choose the most recent or better version
  - **One added, one modified:** Keep both modifications

### Example:

**Before resolution:**
```javascript
<<<<<<< HEAD
function calculateTotal(items) {
  return items.reduce((sum, item) => sum + item.price, 0);
}
=======
function calculateTotal(items, tax = 0.1) {
  return items.reduce((sum, item) => sum + item.price, 0) * (1 + tax);
}
>>>>>>> origin/main
```

**After resolution (combining both):**
```javascript
function calculateTotal(items, tax = 0.1) {
  return items.reduce((sum, item) => sum + item.price, 0) * (1 + tax);
}
```

---

## Abort Merge If Needed

If you want to cancel the merge process entirely and start over:

```bash
git merge --abort
```

This will return your repository to the state before the merge started.

---

## Useful Commands During Merge

```bash
# See current merge status
git status

# View conflicted files
git diff --name-only --diff-filter=U

# View conflicts in a specific file
git diff <filename>

# View your local version
git show :1:<filename>

# View remote version
git show :3:<filename>

# View the merged base
git show :2:<filename>
```

---

## Summary Table

| Scenario | Command | Use When |
|----------|---------|----------|
| **Keep Local Only** | `git checkout --ours .` | You want to discard all remote changes |
| **Keep Remote Only** | `git checkout --theirs .` | You want to discard all local changes |
| **Manual Merge** | Edit files manually | You want to combine both versions |
| **Abort Everything** | `git merge --abort` | You made a mistake and want to start over |

---

## Best Practices

1. **Always review conflicts** before committing
2. **Test your code** after resolving conflicts
3. **Communicate with your team** about significant merges
4. **Keep commit messages clear** about what was merged
5. **Use version control tools** (VS Code, GitHub Desktop, etc.) for visual conflict resolution

---

## Additional Resources

- [Git Official Documentation: Merge Conflicts](https://git-scm.com/book/en/v2/Git-Branching-Basic-Branching-and-Merging)
- [GitHub: Resolving a merge conflict](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/addressing-merge-conflicts)

---

*Created for ruslink project - Git workflow guide*
