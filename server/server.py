#!/usr/bin/env python3
#
# Copyright (c) 2016 Paul Fariello <paul@fariello.eu>
#
# Permission to use, copy, modify, and distribute this software for any
# purpose with or without fee is hereby granted, provided that the above
# copyright notice and this permission notice appear in all copies.
#
# THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
# WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
# MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
# ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
# WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
# ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
# OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
"""A rest API providing access to Rendsmoimatune multiuser account management"""

import argparse
import bottle
import json

import uniqid
import rmmt

@bottle.post(r"/api/account/")
def create_account():
    """Create a new account"""
    uid = uniqid.generate()
    name = bottle.request.POST.get('name')
    account = rmmt.Account.create(uid=uid, name=name)
    return json.dumps(account.json)

@bottle.get(r"/api/account/<account_id:re:[a-zA-Z0-9_=-]+>")
def get_account(account_id):
    """Get account description"""
    uid = uniqid.decode(account_id)
    account = rmmt.Account.get(rmmt.Account.uid == uid)
    return json.dumps(account.json)

@bottle.post(r"/api/account/<account_id:re:[a-zA-Z0-9_=-]+>/user/")
def create_user(account_id):
    """Create user for an account"""
    uid = uniqid.decode(account_id)
    account = rmmt.Account.get(rmmt.Account.uid == uid)
    name = bottle.request.POST.get('name')
    user = rmmt.User.create(account=account, name=name)
    return json.dumps(user.json)

@bottle.get(r"/api/account/<account_id:re:[a-zA-Z0-9_=-]+>/user/<name>")
def get_user(account_id, name):
    """Get user description"""
    uid = uniqid.decode(account_id)
    user = rmmt.User.select().join(rmmt.Account).where(rmmt.User.name == name, rmmt.Account.uid == uid).get()
    return json.dumps(user.json)

def main():
    """Start server"""
    parser = argparse.ArgumentParser(description="Rendsmoimatune")
    parser.add_argument("-l", "--listen", dest="host", default="0.0.0.0", help="IP address to bind to")
    parser.add_argument("-p", "--port", dest="port", default=8080, type=int, help="Port to listen to")
    parser.add_argument("--db", dest="db", default="sqlite:///rmmt.db", help="Database scheme to connect to")
    parser.add_argument("--init", dest="init", type=bool, help="Initialize database")
    args = parser.parse_args()

    rmmt.connect(args.db)
    if args.init:
        rmmt.create_tables()

    bottle.run(host=args.host, port=args.port)

if __name__ == "__main__":
    main()
