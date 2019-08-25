# echo off
rm ./local-db/*
echo "sqlite3 ./local-db/match_base.db" | sqlite3
cat create.sql | sqlite3 ./local-db/match_base.db