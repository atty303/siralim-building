:; tailwindcss -i public/tailwind.css -o "$TRUNK_STAGING_DIR/tailwind.css"; exit $?
@ECHO OFF
tailwindcss-windows-x64.exe -i public\tailwind.css -o %TRUNK_STAGING_DIR%\tailwind.css