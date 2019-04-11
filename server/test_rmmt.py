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

from nose.tools import assert_equals, assert_not_equals, with_setup
import rmmt
import uniqid
import os

def connect_db():
    rmmt.connect("sqlite:///tests.db")
    rmmt.create_tables()

def delete_db():
    os.remove("tests.db")

def with_db(f):
    f.setup = connect_db
    f.teardown = delete_db

    return f

@with_db
def test_create_account():
    # Given
    account = rmmt.Account.create(name=test_create_account.__name__, uid=uniqid.generate())

    # When
    json = account.json

    # Then
    assert_equals(json["name"], test_create_account.__name__)

@with_db
def test_create_user():
    # Given
    uid = uniqid.generate()
    account = rmmt.Account.create(name=test_create_user.__name__, uid=uid)

    # When
    user = rmmt.User.create(name="Alice", account=account)

    # Then
    account = rmmt.Account.get(uid=uid)
    json = account.json
    assert_equals(json["users"][0]["name"], "Alice")

@with_db
def test_create_expenditure():
    # Given
    uid = uniqid.generate()
    account = rmmt.Account.create(name=test_create_expenditure.__name__, uid=uid)

    alice = rmmt.User.create(name="Alice", account=account)
    bob = rmmt.User.create(name="Bob", account=account)

    # When
    alambic = rmmt.Expenditure.create(account=account, name="Alambic", date="2019-04-11", amount=215, payer=alice)

    # Then
    account = rmmt.Account.get(uid=uid)
    json = account.json
    assert_equals(json["expenditures"][0]["name"], "Alambic")
    assert_equals(json["expenditures"][0]["amount"], 215)
    assert_equals(json["expenditures"][0]["date"], "2019-04-11")
    assert_equals(json["expenditures"][0]["payer"], "Alice")

@with_db
def test_balance_is_stable():
    # Given
    uid = uniqid.generate()
    account = rmmt.Account.create(name=test_balance_is_stable.__name__, uid=uid)

    alice = rmmt.User.create(name="Alice", account=account)
    bob = rmmt.User.create(name="Bob", account=account)
    charles = rmmt.User.create(name="Charles", account=account)
    daniel = rmmt.User.create(name="Daniel", account=account)
    erwan = rmmt.User.create(name="Erwan", account=account)

    alambic = rmmt.Expenditure.create(account=account, name="Alambic", date="2019-04-11", amount=215, payer=alice)
    rmmt.Debt.create(debtor=alice, expenditure=alambic, share=1)
    rmmt.Debt.create(debtor=bob, expenditure=alambic, share=1)
    rmmt.Debt.create(debtor=charles, expenditure=alambic, share=1)
    rmmt.Debt.create(debtor=daniel, expenditure=alambic, share=1)
    rmmt.Debt.create(debtor=erwan, expenditure=alambic, share=1)

    # When
    repayments = account.balance

    # Then
    account = rmmt.Account.get(uid=uid)
    json = account.json
    assert_equals(repayments[0]["from"], "Erwan")
    assert_equals(repayments[0]["to"], "Alice")
    assert_equals(repayments[0]["amount"], 43)
    assert_equals(repayments[1]["from"], "Daniel")
    assert_equals(repayments[1]["to"], "Alice")
    assert_equals(repayments[1]["amount"], 43)
    assert_equals(repayments[2]["from"], "Charles")
    assert_equals(repayments[2]["to"], "Alice")
    assert_equals(repayments[2]["amount"], 43)
    assert_equals(repayments[3]["from"], "Bob")
    assert_equals(repayments[3]["to"], "Alice")
    assert_equals(repayments[3]["amount"], 43)

    # When
    repayment = rmmt.Repayment.create(account=account, payer=charles, amount=43, beneficiary=alice, date="2019-04-11")

    # Then
    account = rmmt.Account.get(uid=uid)
    repayments = account.balance
    assert_equals(repayments[0]["from"], "Erwan")
    assert_equals(repayments[0]["to"], "Alice")
    assert_equals(repayments[0]["amount"], 43)
    assert_equals(repayments[1]["from"], "Daniel")
    assert_equals(repayments[1]["to"], "Alice")
    assert_equals(repayments[1]["amount"], 43)
    assert_equals(repayments[2]["from"], "Bob")
    assert_equals(repayments[2]["to"], "Alice")
    assert_equals(repayments[2]["amount"], 43)
