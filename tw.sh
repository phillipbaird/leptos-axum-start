#!/usr/bin/env bash
case "$1" in
    watch)
      tailwindcss -i ./tailwind.css -o ./css/main.css --watch
      ;;
    build)
      tailwindcss --minify -i ./tailwind.css -o ./css/main.css
      ;;
    *)
      echo "Usage: ./tw.sh [OPTION]"
      echo "  watch - watch for changes"
      echo "  build - build with minify"
      ;;
esac