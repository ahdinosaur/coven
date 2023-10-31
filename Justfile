dev:
  dx serve --hot-reload

expand:
  cargo expand --lib > ./dist/expanded.rs

expand_watch:
  watchexec -e rs cargo expand --lib

tailwind: expand
  npm run tailwind

tailwind_watch: expand_watch
  npm run tailwind:watch
