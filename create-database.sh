#!/bin/bash

database_file="db_main.db"

# Check if the database file exists
if [ -e "$database_file" ]; then
  echo "Error: The database file $database_file already exists. Aborting."
  exit 1
fi

# If the database file does not exist, execute the sqlite3 command
sqlite3 "$database_file" < dump.sql

# Check the exit status of the sqlite3 command
if [ $? -eq 0 ]; then
  echo "Database import successful."
else
  echo "Error: Database import failed."
fi
