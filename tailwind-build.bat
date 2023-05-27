:; npx tailwindcss -i src/input.css -o "$TRUNK_STAGING_DIR/tailwind.css"; exit $?
@ECHO OFF
npx tailwindcss -i src\input.css -o %TRUNK_STAGING_DIR%\tailwind.css