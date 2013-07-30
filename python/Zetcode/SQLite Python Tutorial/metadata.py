#!/usr/bin/env python
#-*- encoding: utf-8 -*-


import sqlite3
import sys


with sqlite3.connect('test.db') as con:
    cur = con.cursor()
    cur.execute('PRAGMA table_info(Cars)')

    data = cur.fetchall()
    for d in data:
        print d[0], d[1], d[2]