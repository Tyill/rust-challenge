#!/bin/bash

PASSW=""
if [ -n "$1" ]
  then
    PASSW="--password="$1
fi

clickhouse-client $PASSW -d "stockdb" --query="DROP TABLE IF EXISTS tblUserTransfer";
clickhouse-client $PASSW -d "stockdb" --query="DROP TABLE IF EXISTS tblUserBalance";