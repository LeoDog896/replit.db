# Replit base API

Set a key (key, value): `curl $REPLIT_DB_URL -d 'key=value'`

Grab a value (key): `curl $REPLIT_DB_URL/key`

Delete a key (key): `curl -XDELETE $REPLIT_DB_URL/key`

List all keys starting with (key): `curl "$REPLIT_DB_URL?prefix=key"`