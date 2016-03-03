from waflib.Build import BuildContext
from waflib import TaskGen, Task, Configure


@TaskGen.extension('.scss')
def sass_hook(self, node):
    return self.create_task('sass', node, node.change_ext('.css'))


class sass(Task.Task):
    "Compile Sass files into css files"
    vars = ['SASSC']
    ext_out = ['.css']
    run_str = '${SASSC} ${SASS_FLAGS} ${SASSPATH_ST:INCPATHS} ${SRC} ${TGT}'


@Configure.conf
def sass_common_flags(cfg):
    cfg.env['SASSPATH_ST'] = '-I%s'
    cfg.env['SASS_FLAGS'] = '-Eutf-8'


@Configure.conf
def sass(self, *args, **kwargs):
    kwargs['features'] = ['includes', 'sass']
    return self(*args, **kwargs)


def configure(cfg):
    cfg.sass_common_flags()
    cfg.find_program('sass', var='SASSC')
