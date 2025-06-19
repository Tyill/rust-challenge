#!/bin/bash

PASSW=""
if [ -n "$1" ]
  then
    PASSW="--password="$1
fi

clickhouse-client $PASSW --query="CREATE DATABASE IF NOT EXISTS stockdb";

clickhouse-client $PASSW -d "stockdb" --query="CREATE TABLE IF NOT EXISTS tblUserTransfer( \
    ts        UInt64 CODEC(DoubleDelta),                       \
    from      String,                                          \
    to        String,                                          \
    amount    Float64 CODEC(Gorilla),                          \
    usd_price Float64 CODEC(Gorilla)                           \
) ENGINE = MergeTree()                                         \
ORDER BY (from, ts)";
