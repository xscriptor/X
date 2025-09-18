# Alias
---
## Definitions to increase the productivity

*   Note:
    * To install, clone `chmod +x script.sh` and run `./script` or just copy on the terminal or use what you want to use.

    * For Bash and Zsh.

1. [Github Alias](./github/script.sh)
<details>
<summary>Github Aliases list</summary>

```bash
#Base
 alias gc="git clone" #write the link of the repository after of this
 alias ga="git add ."
 alias gcom="git commit -m" #write the commit after
 alias gp="git push"
 alias gpuom="git push -u origin main"
 alias gpuod="git push -u origin dev"
 #Habitual
 alias gs="git status"
 alias gl="git log --online --graph --decorate"
 alias gco="git checkout" #write the branch name
 alias gcb="git checkout -b" #change the branch creating
 alias gd="git diff"
 #Pull & fetch
 alias gpl="git pull"
 alias gf="git fetch"
```

</details>

1. [Github Alias](./navigation/script.sh)
<details>
<summary>Navigation Aliases list</summary>

```bash
alias ..="cd .."
alias ...="cd ../.."
alias ....="cd ../../.."
alias ~="cd ~"
alias c="clear"
alias ll="ls -lh"
alias la="ls -A"
alias l="ls -CF"
```

</details>