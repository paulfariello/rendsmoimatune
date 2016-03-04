Rends moi ma tune
=================

Rends moi ma tune (abbreviated Rmmt) is a free software multi-user account management.
Rmmt allow you to organize some expenditures and track who owes money to who in a group.

Rmmt aims to be accurate, simple and ergonomic.

Installation
------------

Rmmt REST server being based on peewee it is supposed to support a wide range of database systems.
Currently, only SQLite3 and Postegresql are eavily tested.
You are more than welcome to test Rmmt with another database systems.

Here is the 3 steps required to run Rmmt web app.

Install web app dependencies:
```
cd client/ && npm install
```

Build web app and install Rmmt:
```
./waf-1.8.19 configure build install -v --destdir=$(pwd)
```

> `waf 1.8.19` seems to have a bug and requires `-v` option in order to build Rmmt

Then go to built server directory and run Rmmt server:
```
cd usr/local/share/rmmt/server/
./server.py --static ../static/ --init
```

> `--init` option should only be used on first run as it will create the database tables

Here you go, Rmmt should be running on http://127.0.0.1:8080

Durable installation
--------------------

In order to have a more durable installation you should consider using another web server and database.
Here the procedure to run Rmmt server behind nginx, with gunicorn and postgresql.

First you should build and install Rmmt to a standard directory:
```
./waf-1.8.19 configure build install -v
```

Ensure your database is configured to use UTF-8 encoding, either with initdb:
```
initdb -D /var/postgresql/data -U postgres -E UTF8 -A md5 -W
```
or when creating the database:
```SQL
CREATE DATABASE rmmt WITH ENCODING 'UTF-8' OWNER rmmt;
```

Configure your nginx to proxy request to gunicorn and serve static files:
```
server {
        listen 80;
        root /usr/local/share/rmmt/static/;

        location /api/ {
                proxy_pass http://127.0.0.1:8080;
        }
}
```

And finally run Rmmt with gunicorn (here with 4 workers):
```
/usr/local/share/rmmt/server/server.py --db postgresql://user:password@localhost:5432/rmmt --server gunicorn -w 4
```


Architecture
------------

Rmmt is mainly a REST server based on:
 - [peewee](https://github.com/coleifer/peewee)
 - [bottle](http://bottlepy.org/docs/dev/index.html)

We provide a default web client based on:
 - [angularjs](https://angularjs.org/)
 - [foundation-sites](http://foundation.zurb.com/sites/docs/)

API
---

Up-to-date documentation of the API can be found in server/server.py.

Server manual
-------------

Up-to-date server manual is available with:
```
./server.py --help
```

```
usage: server.py [-h] [-l HOST] [-p PORT] [--db DB] [--static STATIC]
                 [--server SERVER] [--init]

Rendsmoimatune

optional arguments:
  -h, --help            show this help message and exit
  -l HOST, --listen HOST
                        IP address to bind to
  -p PORT, --port PORT  Port to listen to
  --db DB               Database scheme to connect to
  --static STATIC       Path to static files
  --server SERVER       Bottle server type
  --init                Initialize database
```
