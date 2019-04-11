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

import sys
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
@bottle.get(r"/<path:re:.*\.(html|js|css|woff2|woff|ttf|jpg)>")
def static(path=None):
    """Unsafe method used only for dev"""
    if path is None:
        path = "index.html"
    return bottle.static_file(path, root=STATIC_ROOT)

@bottle.get(r"/api/status")
def get_status():
    """Get server status

    Exemple:
    curl -X GET -H "Content-Type:application/json" http://localhost:8001/api/status
    """
    return json.dumps({'status': 'OK'})


@bottle.post(r"/api/account/")
def create_account():
    """Create a new account

    Exemple:
    curl -X POST -H "Content-Type:application/json" -d '{"name": "my new account"}' http://localhost:8001/api/account/
    """
    uid = uniqid.generate()
    name = bottle.request.json['name']
    account = rmmt.Account.create(uid=uid, name=name)
    return json.dumps(account.json)


@bottle.get(r"/api/account/<account_id:re:[a-zA-Z0-9_=-]+>")
def get_account(account_id):
    """Get account description

    Exemple:
    curl -X GET -H "Content-Type:application/json" http://localhost:8001/api/account/PoP93u9ktzqIP5-cJx1D9D
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
    curl -X POST -H "Content-Type:application/json" -d '{"name": "paul"}' http://localhost:8001/api/account/PoP93u9ktzqIP5-cJx1D9D/users/
    """
    try:
        uid = uniqid.decode(account_id)
        account = rmmt.Account.get(rmmt.Account.uid == uid)
        name = bottle.request.json['name']
        user = rmmt.User.create(account=account, name=name)
        return json.dumps(user.json)
    except rmmt.Account.DoesNotExist as e:
        bottle.response.status = 404
        return {"error": "Account %s not found" % account_id}


@bottle.get(r"/api/account/<account_id:re:[a-zA-Z0-9_=-]+>/users/<name>")
def get_user(account_id, name):
    """Get user description

    Exemple:
    curl -X GET -H "Content-Type:application/json" http://localhost:8001/api/account/PoP93u9ktzqIP5-cJx1D9D/users/paul
    """
    try:
        uid = uniqid.decode(account_id)
        user = rmmt.User.select().join(rmmt.Account).where(rmmt.User.name == name,
                                                           rmmt.Account.uid == uid).get()
        return json.dumps(user.json)
    except rmmt.User.DoesNotExist as e:
        bottle.response.status = 404
        return {"error": "User %s not found" % name}
    except rmmt.Account.DoesNotExist as e:
        bottle.response.status = 404
        return {"error": "Account %s not found" % account_id}


@bottle.delete(r"/api/account/<account_id:re:[a-zA-Z0-9_=-]+>/users/<name>")
def delete_user(account_id, name):
    """Delete user

    Exemple:
    curl -X DELETE -H "Content-Type:application/json" http://localhost:8001/api/account/PoP93u9ktzqIP5-cJx1D9D/users/paul
    """
    try:
        uid = uniqid.decode(account_id)
        user = rmmt.User.select().join(rmmt.Account).where(rmmt.User.name == name,
                                                           rmmt.Account.uid == uid).get()
        user.delete_instance()
    except rmmt.User.DoesNotExist as e:
        bottle.response.status = 404
        return {"error": "User %s not found" % name}
    except rmmt.Account.DoesNotExist as e:
        bottle.response.status = 404
        return {"error": "Account %s not found" % account_id}


def validate_expenditure():
    debts = bottle.request.json['debts']
    amount = int(bottle.request.json['amount'])

    if len(debts) == 0:
        raise ValueError("no debts")

    share_sum = sum(debt['share'] for debt in debts)
    if share_sum <= 0:
        raise ValueError("invalid share sum %s" % share_sum)

    for debt in debts:
        if debt['share'] < 0:
            raise ValueError("invalid share for user %s" % debt['debtor'])

    if amount <= 0:
        raise ValueError("invalid amount %s" % amount)


@rmmt.atomic
def create_expenditure(account_id):
    validate_expenditure()

    uid = uniqid.decode(account_id)
    account = rmmt.Account.get(rmmt.Account.uid == uid)
    name = bottle.request.json['name']
    date = strpdate(bottle.request.json['date'])
    amount = int(bottle.request.json['amount'])
    user_name = bottle.request.json['payer']
    debts = bottle.request.json['debts']

    payer = rmmt.User.select().where(rmmt.User.name == user_name,
                                     rmmt.User.account == account).get()

    expenditure = rmmt.Expenditure.create(account=account, name=name, date=date,
                                          amount=amount, payer=payer)

    for debt in debts:
        if debt['share'] > 0:
            user_name = debt['debtor']
            debtor = rmmt.User.select().where(rmmt.User.name == user_name,
                                              rmmt.User.account == account).get()
            rmmt.Debt.create(debtor=debtor, expenditure=expenditure, share=debt['share'])

    return expenditure

@bottle.post(r"/api/account/<account_id:re:[a-zA-Z0-9_=-]+>/expenditures/")
def rest_create_expenditure(account_id):
    """Create expenditure

    Exemple:
    curl -X POST -H "Content-Type:application/json" -d '{"name": "patate", "date": "2016-01-28", "amount": 1200, "payer": "paul", "debts": [{"debtor": "paul", "share": 1}, {"debtor": "test", "share": 1}]}' http://localhost:8001/api/account/PoP93u9ktzqIP5-cJx1D9D/expenditures/
    """
    try:
        expenditure = create_expenditure(account_id)
        return json.dumps(expenditure.json)
    except rmmt.Account.DoesNotExist:
        bottle.response.status = 404
        return {"error": "Account %s not found" % account_id}
    except rmmt.User.DoesNotExist:
        bottle.response.status = 404
        return {"error": "User not found"}
    except ValueError as e:
        bottle.response.status = 400
        return {"error": e.msg}


@rmmt.atomic
def update_expenditure(account_id, expenditure_id):
    validate_expenditure()

    uid = uniqid.decode(account_id)
    account = rmmt.Account.get(rmmt.Account.uid == uid)
    expenditure = rmmt.Expenditure.select().where(rmmt.Expenditure._id == expenditure_id,
                                         rmmt.Expenditure.account == account).get()

    user_name = bottle.request.json['payer']
    payer = rmmt.User.select().where(rmmt.User.name == user_name,
                                     rmmt.User.account == account).get()

    expenditure.name = bottle.request.json['name']
    expenditure.date = strpdate(bottle.request.json['date'])
    expenditure.amount = int(bottle.request.json['amount'])
    expenditure.payer = payer
    expenditure.save()

    debts = bottle.request.json['debts']

    rmmt.Debt.delete().where(rmmt.Debt.expenditure == expenditure).execute()
    for debt in debts:
        if debt['share'] > 0:
            user_name = debt['debtor']
            debtor = rmmt.User.select().where(rmmt.User.name == user_name,
                                              rmmt.User.account == account).get()
            rmmt.Debt.create(debtor=debtor, expenditure=expenditure, share=debt['share'])

    return expenditure


@bottle.put(r"/api/account/<account_id:re:[a-zA-Z0-9_=-]+>/expenditures/<expenditure_id:re:[0-9]+>")
def rest_update_expenditure(account_id, expenditure_id):
    """Update expenditure

    Exemple:
    curl -X PUT -H "Content-Type:application/json" -d '{"name": "patate", "date": "2016-01-28", "amount": 1200, "payer": "paul", "debts": [{"debtor": "paul", "share": 1}, {"debtor": "test", "share": 1}]}' http://localhost:8001/api/account/PoP93u9ktzqIP5-cJx1D9D/expenditures/11
    """
    try:
        expenditure = update_expenditure(account_id, expenditure_id)
        return json.dumps(expenditure.json)
    except rmmt.Account.DoesNotExist as e:
        bottle.response.status = 404
        return {"error": "Account %s not found" % account_id}
    except rmmt.Expenditure.DoesNotExist as e:
        bottle.response.status = 404
        return {"error": "Expenditure %s not found" % expenditure_id}
    except rmmt.User.DoesNotExist as e:
        bottle.response.status = 404
        return {"error": "User not found"}
    except ValueError as e:
        bottle.response.status = 400
        return {"error": e.args[0]}


@rmmt.atomic
def delete_expenditure(account_id, expenditure_id):
    uid = uniqid.decode(account_id)
    account = rmmt.Account.get(rmmt.Account.uid == uid)
    expenditure = rmmt.Expenditure.select().where(rmmt.Expenditure._id == expenditure_id,
                                         rmmt.Expenditure.account == account).get()

    expenditure.delete_instance()


@bottle.delete(r"/api/account/<account_id:re:[a-zA-Z0-9_=-]+>/expenditures/<expenditure_id:re:[0-9]+>")
def rest_delete_expenditure(account_id, expenditure_id):
    """Delete expenditure

    Exemple:
    curl -X DELETE -H "Content-Type:application/json" http://localhost:8001/api/account/PoP93u9ktzqIP5-cJx1D9D/expenditures/11
    """
    try:
        expenditure = delete_expenditure(account_id, expenditure_id)
        return
    except rmmt.Account.DoesNotExist as e:
        bottle.response.status = 404
        return {"error": "Account %s not found" % account_id}
    except rmmt.Expenditure.DoesNotExist as e:
        bottle.response.status = 404
        return {"error": "Expenditure %s not found" % expenditure_id}
    except ValueError as e:
        bottle.response.status = 400
        return {"error": e.args[0]}


def validate_repayment():
    amount = int(bottle.request.json['amount'])

    if amount <= 0:
        raise ValueError("invalid amount %s" % amount)

    payer_name = bottle.request.json['payer']
    beneficiary_name = bottle.request.json['beneficiary']

    if payer_name == beneficiary_name:
        raise ValueError("cannot create a repayment to yourself")


@rmmt.atomic
def create_repayment(account_id):
    validate_repayment()

    uid = uniqid.decode(account_id)
    account = rmmt.Account.get(rmmt.Account.uid == uid)
    date = strpdate(bottle.request.json['date'])
    amount = int(bottle.request.json['amount'])
    payer_name = bottle.request.json['payer']
    beneficiary_name = bottle.request.json['beneficiary']

    payer = rmmt.User.select().where(rmmt.User.name == payer_name,
                                     rmmt.User.account == account).get()

    beneficiary = rmmt.User.select().where(rmmt.User.name == beneficiary_name,
                                     rmmt.User.account == account).get()

    repayment = rmmt.Repayment.create(account=account, date=date,
                                      amount=amount, payer=payer,
                                      beneficiary=beneficiary)

    return repayment

@bottle.post(r"/api/account/<account_id:re:[a-zA-Z0-9_=-]+>/repayments/")
def rest_create_repayment(account_id):
    """Create repayment

    Exemple:
    curl -X POST -H "Content-Type:application/json" -d '{"date": "2016-01-28", "amount": 1200, "payer": "paul", "beneficiary": "emilie"}' http://localhost:8001/api/account/PoP93u9ktzqIP5-cJx1D9D/repayments/
    """
    try:
        repayment = create_repayment(account_id)
        return json.dumps(repayment.json)
    except rmmt.Account.DoesNotExist:
        bottle.response.status = 404
        return {"error": "Account %s not found" % account_id}
    except rmmt.User.DoesNotExist:
        bottle.response.status = 404
        return {"error": "User not found"}
    except ValueError as e:
        bottle.response.status = 400
        return {"error": e.msg}


@rmmt.atomic
def update_repayment(account_id, repayment_id):
    validate_repayment()

    uid = uniqid.decode(account_id)
    account = rmmt.Account.get(rmmt.Account.uid == uid)
    repayment = rmmt.Repayment.select().where(rmmt.Repayment._id == repayment_id,
                                         rmmt.Repayment.account == account).get()

    payer_name = bottle.request.json['payer']
    payer = rmmt.User.select().where(rmmt.User.name == payer_name,
                                     rmmt.User.account == account).get()

    beneficiary_name = bottle.request.json['beneficiary']
    beneficiary = rmmt.User.select().where(rmmt.User.name == beneficiary_name,
                                     rmmt.User.account == account).get()

    repayment.date = strpdate(bottle.request.json['date'])
    repayment.amount = int(bottle.request.json['amount'])
    repayment.payer = payer
    repayment.beneficiary = beneficiary
    repayment.save()

    return repayment


@bottle.put(r"/api/account/<account_id:re:[a-zA-Z0-9_=-]+>/repayments/<repayment_id:re:[0-9]+>")
def rest_update_repayment(account_id, repayment_id):
    """Update repayment

    Exemple:
    curl -X PUT -H "Content-Type:application/json" -d '{"date": "2016-01-28", "amount": 1200, "payer": "paul", "beneficiary": "test"}' http://localhost:8001/api/account/PoP93u9ktzqIP5-cJx1D9D/repayments/11
    """
    try:
        repayment = update_repayment(account_id, repayment_id)
        return json.dumps(repayment.json)
    except rmmt.Account.DoesNotExist as e:
        bottle.response.status = 404
        return {"error": "Account %s not found" % account_id}
    except rmmt.Repayment.DoesNotExist as e:
        bottle.response.status = 404
        return {"error": "Repayment %s not found" % repayment_id}
    except rmmt.User.DoesNotExist as e:
        bottle.response.status = 404
        return {"error": "User not found"}
    except ValueError as e:
        bottle.response.status = 400
        return {"error": e.args[0]}


@rmmt.atomic
def delete_repayment(account_id, repayment_id):
    uid = uniqid.decode(account_id)
    account = rmmt.Account.get(rmmt.Account.uid == uid)
    repayment = rmmt.Repayment.select().where(rmmt.Repayment._id == repayment_id,
                                              rmmt.Repayment.account == account).get()

    repayment.delete_instance()


@bottle.delete(r"/api/account/<account_id:re:[a-zA-Z0-9_=-]+>/repayments/<repayment_id:re:[0-9]+>")
def rest_delete_repayment(account_id, repayment_id):
    """Delete repayment

    Exemple:
    curl -X DELETE -H "Content-Type:application/json" http://localhost:8001/api/account/PoP93u9ktzqIP5-cJx1D9D/repayments/11
    """
    try:
        repayment = delete_repayment(account_id, repayment_id)
        return
    except rmmt.Account.DoesNotExist as e:
        bottle.response.status = 404
        return {"error": "Account %s not found" % account_id}
    except rmmt.Repayment.DoesNotExist as e:
        bottle.response.status = 404
        return {"error": "Repayment %s not found" % repayment_id}
    except ValueError as e:
        bottle.response.status = 400
        return {"error": e.args[0]}

def main():
    """Start server"""
    parser = argparse.ArgumentParser(description="Rendsmoimatune")
    parser.add_argument("-l", "--listen", dest="host", default="0.0.0.0", help="IP address to bind to")
    parser.add_argument("-p", "--port", dest="port", default=8001, type=int, help="Port to listen to")
    parser.add_argument("--db", dest="db", default="sqlite:///rmmt.db", help="Database scheme to connect to")
    parser.add_argument("--static", dest="static", default=None, type=str, help="Path to static files")
    parser.add_argument("--server", dest="server", default='auto', type=str, help="Bottle server type")
    parser.add_argument("--init", dest="init", action="store_true", help="Initialize database")
    args, remaining = parser.parse_known_args()
    sys.argv = [sys.executable] + remaining

    rmmt.connect(args.db)
    if args.init:
        rmmt.create_tables()

    global STATIC_ROOT
    STATIC_ROOT = args.static

    bottle.run(server=args.server, host=args.host, port=args.port)

if __name__ == "__main__":
    main()
