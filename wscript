def configure(ctx):
    ctx.env.RMMT_PREFIX = ctx.env.PREFIX + '/share/rmmt'
    ctx.env.SERVER_PATH = ctx.env.RMMT_PREFIX + '/server'
    ctx.env.STATIC_PATH = ctx.env.RMMT_PREFIX + '/static'
    ctx.recurse('server')


def build(ctx):
    ctx.recurse('server')
