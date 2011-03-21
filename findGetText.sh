#!/bin/sh
( find . -name "*.php" -not -path "./inc/cots/*" -not -path "./templates_c/*" -not -path "./inc/engine/*" | xargs cat | grep -E "getText(.*)" -o | perl -pe "s/getText\(([\"'])((\\\\\1|[^\1])*?)\1.*/\"\2\"/" && find ./templates/ -name "*.tpl" | xargs cat | grep -E "\{getText id=.*\}" -o | perl -pe "s/{getText id=([\"'])((\\\\\1|[^\1])*?)\1.*/\"\2\"/" ) | perl -pe "s/\\\'/'/g" | perl -pe "s/(.+?)\"(.+?)/\1\\\\\"\2/g" | sort | uniq | sed "s/^/msgid /" | sed 's/$/\
msgstr ""/'
