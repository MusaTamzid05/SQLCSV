#!/bin/bash

./target/debug/csv_sql "./uspop.csv"  "SELECT COUNT(Population)"
./target/debug/csv_sql "./uspop.csv"  "SELECT MAX(Population)"
./target/debug/csv_sql "./uspop.csv"  "SELECT MIN(Population)"
./target/debug/csv_sql "./uspop.csv"  "SELECT AVG(Population)"
