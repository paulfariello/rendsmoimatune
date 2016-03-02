from waflib.Build import BuildContext
from waflib import TaskGen, Task, Configure


@TaskGen.extension('.js')
def uglify_hook(self, node):
    task = self.create_task('uglify', node, node.change_ext('.js.min'))
    self.source.extend(task.outputs)
    return task


class uglify(Task.Task):
    "Minify js files"
    vars = ['UGLIFY']
    ext_out = ['.min.js']
    run_str = '${UGLIFY} -o ${TGT} ${SRC}'


class fake_concat(Task.Task):
    def runnable_status(self):
        return Task.SKIP_ME


@TaskGen.extension('.js.min')
def concat_this_js_file(self, node):
    task = self.create_task('fake_concat', node)
    try:
        self.concat_tasks.append(task)
    except AttributeError:
        self.concat_tasks = [task]


@TaskGen.feature('concat')
@TaskGen.after_method('process_source')
def apply_concat(self):
    srcs = [task.inputs[0] for task in getattr(self, 'concat_tasks', [])]
    self.concat_task = self.create_task('concat', srcs)
    target = self.path.find_or_declare(self.target)
    self.concat_task.set_outputs(target)


class concat(Task.Task):
    run_str = 'cat ${SRC} > ${TGT}'


@Configure.conf
def uglify(self, *args, **kwargs):
    kwargs['features'] = ['uglify', 'concat']
    return self(*args, **kwargs)


def configure(cfg):
    cfg.find_program('uglifyjs', var='UGLIFY')
