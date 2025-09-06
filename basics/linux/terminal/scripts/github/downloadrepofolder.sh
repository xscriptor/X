#!/bin/bash

download_folder() {
  local user="$1"
  local repo="$2"
  local branch="$3"
  local path="$4"

  mkdir -p "$path"

  curl -s "https://api.github.com/repos/$user/$repo/contents/$path?ref=$branch" \
  | jq -r '.[] | @base64' | while read item; do
      _jq() { echo $item | base64 --decode | jq -r $1; }

      name=$(_jq '.name')
      type=$(_jq '.type')
      url=$(_jq '.download_url')

      if [[ "$type" == "file" ]]; then
        echo "Descargando $path/$name"
        wget -q -O "$path/$name" "$url"
      elif [[ "$type" == "dir" ]]; then
        download_folder "$user" "$repo" "$branch" "$path/$name"
      fi
  done
}

read -p "Url/folder you want to download from git: " url

user=$(echo "$url" | sed -E 's#https://github.com/([^/]+)/([^/]+)/tree/([^/]+)/(.+)#\1#')
repo=$(echo "$url" | sed -E 's#https://github.com/([^/]+)/([^/]+)/tree/([^/]+)/(.+)#\2#')
branch=$(echo "$url" | sed -E 's#https://github.com/([^/]+)/([^/]+)/tree/([^/]+)/(.+)#\3#')
path=$(echo "$url" | sed -E 's#https://github.com/([^/]+)/([^/]+)/tree/([^/]+)/(.+)#\4#')

download_folder "$user" "$repo" "$branch" "$path"