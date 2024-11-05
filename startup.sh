#!/bin/bash
set -e
refinery migrate -e DATABASE_URL -p ./migrations/V1__init_up.sql
exec ./target/release/wb_tech_l0 --database $DATABASE_URL