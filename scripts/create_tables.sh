#!/bin/bash

PASSW=""
if [ -n "$1" ]
  then
    PASSW="--password="$1
fi

clickhouse-client $PASSW --query="CREATE DATABASE IF NOT EXISTS stockdb";

clickhouse-client $PASSW -d "stockdb" --query="CREATE TABLE IF NOT EXISTS tblUserTransfer( \
    from      Int32 CODEC(DoubleDelta),                        \
    to        Int32 CODEC(DoubleDelta),                        \
    ts        UInt64 CODEC(DoubleDelta),                       \
    amount    Float64 CODEC(Gorilla),                          \
    usd_price Float64 CODEC(Gorilla)                           \
) ENGINE = MergeTree()                                         \
ORDER BY (ts)";

clickhouse-client $PASSW -d "stockdb" --query="CREATE TABLE IF NOT EXISTS tblUserBalance( \
    uid       Int32 CODEC(DoubleDelta),                        \
    ts        UInt64 CODEC(DoubleDelta),                       \
    balance   Float64 CODEC(Gorilla),                          \
) ENGINE = MergeTree()                                         \
ORDER BY (uid, ts)";
