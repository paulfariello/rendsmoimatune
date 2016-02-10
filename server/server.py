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
"""A rest API providing access to Rendsmoimatune multiuser account management

Request to this REST API must be done with Content-Type: application/json as
all sent data is expected to be well formed json.
"""

import argparse
import bottle
import json
import datetime

import uniqid
import rmmt

ISO8601_FMT = "%Y-%m-%d"
STATIC_ROOT = None


def strpdate(date):
    return datetime.datetime.strptime(date[:10], ISO8601_FMT).date()

@bottle.get("/")
@bottle.get(r"/<path:re:.*\.(html|js|css|woff2|woff|ttf)>")
def static(path=None):
    """Unsafe method used only for dev"""
    if path is None:
        path = "index.html"
    return bottle.static_file(path, root=STATIC_ROOT)

@bottle.post(r"/api/account/")
def create_account():
    """Create a new account

    Exemple:
    curl -X POST -H "Content-Type:application/json" -d '{"name": "my new account"}' http://localhost:8080/api/account/
    """
    uid = uniqid.generate()
    name = bottle.request.json['name']
    account = rmmt.Account.create(uid=uid, name=name)
    return json.dumps(account.json)

@bottle.get(r"/api/account/<account_id:re:[a-zA-Z0-9_=-]+>")
def get_account(account_id):
    """Get account description

    Exemple:
    curl -X GET -H "Content-Type:application/json" http://localhost:8080/api/account/PoP93u9ktzqIP5-cJx1D9D
    """
    try:
        uid = uniqid.decode(account_id)
        account = rmmt.Account.get(rmmt.Account.uid == uid)
    except rmmt.Account.DoesNotExist as e:
        bottle.response.status = 404
        return {"error": "Account %s not found" % account_id}
    return json.dumps(account.json, indent="  ")

@bottle.post(r"/api/account/<account_id:re:[a-zA-Z0-9_=-]+>/users/")
def create_user(account_id):
    """Create user for an account

    Exemple:
    curl -X POST -H "Content-Type:application/json" -d '{"name": "paul"}' http://localhost:8080/api/account/PoP93u9ktzqIP5-cJx1D9D/users/
    """
    try:
        uid = uniqid.decode(account_id)
        account = rmmt.Account.get(rmmt.Account.uid == uid)
        name = bottle.request.json['name']
        user = rmmt.User.create(account=account, name=name)
    except rmmt.Account.DoesNotExist as e:
        bottle.response.status = 404
        return {"error": "Account %s not found" % account_id}
    return json.dumps(user.json)

@bottle.get(r"/api/account/<account_id:re:[a-zA-Z0-9_=-]+>/user/<name>")
def get_user(account_id, name):
    """Get user description

    Exemple:
    curl -X GET -H "Content-Type:application/json" http://localhost:8080/api/account/PoP93u9ktzqIP5-cJx1D9D/user/paul
    """
    try:
        uid = uniqid.decode(account_id)
        user = rmmt.User.select().join(rmmt.Account).where(rmmt.User.name == name,
                                                           rmmt.Account.uid == uid).get()
    except rmmt.Account.DoesNotExist as e:
        bottle.response.status = 404
        return {"error": "Account %s not found" % account_id}
    return json.dumps(user.json)

@bottle.post(r"/api/account/<account_id:re:[a-zA-Z0-9_=-]+>/expenditures/")
def create_expenditure(account_id):
    """Create expenditure

    Exemple:
    curl -X POST -H "Content-Type:application/json" -d '{"name": "patate", "date": "2016-01-28", "amount": 1200, "payer": "paul", "debts": [{"debtor": "paul", "share": 1}, {"debtor": "test", "share": 1}]}' http://localhost:8080/api/account/PoP93u9ktzqIP5-cJx1D9D/expenditures/
    """
    try:
        uid = uniqid.decode(account_id)
        account = rmmt.Account.get(rmmt.Account.uid == uid)
        name = bottle.request.json['name']
        date = strpdate(bottle.request.json['date'])
        amount = int(bottle.request.json['amount'])
        payer_name = bottle.request.json['payer']
        debts = bottle.request.json['debts']

        if len(debts) == 0:
            raise ValueError("Expenditure without debt")

        payer = rmmt.User.select().join(rmmt.Account).where(rmmt.User.name == payer_name,
                                                            rmmt.Account.uid == uid).get()

        expenditure = rmmt.Expenditure.create(account=account, name=name, date=date,
                                              amount=amount, payer=payer)

        for debt in debts:
            debtor = rmmt.User.select().join(rmmt.Account).where(rmmt.User.name == debt['debtor'],
                                                                 rmmt.Account.uid == uid).get()
            rmmt.Debt.create(debtor=debtor, expenditure=expenditure, share=debt['share'])
    except rmmt.Account.DoesNotExist as e:
        bottle.response.status = 404
        return {"error": "Account %s not found" % account_id}

    return json.dumps(expenditure.json)

def main():
    """Start server"""
    parser = argparse.ArgumentParser(description="Rendsmoimatune")
    parser.add_argument("-l", "--listen", dest="host", default="0.0.0.0", help="IP address to bind to")
    parser.add_argument("-p", "--port", dest="port", default=8080, type=int, help="Port to listen to")
    parser.add_argument("--db", dest="db", default="sqlite:///rmmt.db", help="Database scheme to connect to")
    parser.add_argument("--static", dest="static", default=None, type=str, help="Path to static files")
    parser.add_argument("--init", dest="init", type=bool, help="Initialize database")
    args = parser.parse_args()

    rmmt.connect(args.db)
    if args.init:
        rmmt.create_tables()

    global STATIC_ROOT
    STATIC_ROOT = args.static

    bottle.run(host=args.host, port=args.port)

if __name__ == "__main__":
    main()
