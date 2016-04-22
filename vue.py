from waflib.Build import BuildContext
from waflib import TaskGen, Task, Configure
from html.parser import HTMLParser
from enum import Enum
import json

class DomNode:
    def __init__(self):
        self.children = []
        self.parent = None

    def add_child(self, child):
        assert(child.parent is None)
        self.children.append(child)
        child.parent = self


class RootNode(DomNode):
    pass


class HtmlNode(DomNode):
    def __init__(self, tag, attrs=None):
        super().__init__()
        self.tag = tag
        self.attrs = attrs or []

    @property
    def html_attrs(self):
        return ["=".join(t) if t[1] is not None else t[0] for t in self.attrs]

    @property
    def html(self):
        html = "<" + " ".join([self.tag] + self.html_attrs) + ">"
        html += self.html_content
        html += "</" + self.tag + ">"
        return html

    @property
    def html_content(self):
        html = ""
        for child in self.children:
            html += child.html
        return html



class EmptyNode(HtmlNode):
    def add_child(self, child):
        raise AssertionError("Cannot add child to empty node")

    @property
    def html(self):
        return "<" + " ".join([self.tag] + self.html_attrs) + "/>"


class DataNode(DomNode):
    def __init__(self, data):
        super().__init__()
        self.data = data

    @property
    def html(self):
        return self.data


class VueParser(HTMLParser):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._template = None
        self._script = None
        self._style = None
        self.dom = RootNode()
        self._current = self.dom

    def handle_startendtag(self, tag, attrs):
        node = EmptyNode(tag, attrs)
        self._current.add_child(node)

    def handle_starttag(self, tag, attrs):
        node = HtmlNode(tag, attrs)
        self._current.add_child(node)
        self._current = node

    def handle_endtag(self, tag):
        assert(self._current.parent is not None)
        self._current = self._current.parent

    def handle_data(self, data):
        self._current.add_child(DataNode(data))

    def get_fragment(self, tag):
        for node in self.dom.children:
            if isinstance(node, HtmlNode) and node.tag == tag:
                return node
        return None


@TaskGen.extension('.vue')
def vue_hook(self, node):
    return self.create_task('vue', node, node.change_ext('.js'))


class vue(Task.Task):
    "Compile vue files into js files"
    ext_out = ['.js']

    def run(self):
        for src, dst in zip(self.inputs, self.outputs):
            parser = VueParser()
            parser.feed(src.read())
            template = parser.get_fragment("template")
            style = parser.get_fragment("style")
            script = parser.get_fragment("script")

            output = 'var home = (function() {\n'
            if style is not None:
                # TODO build style, trigger new task ?
                output += 'var __vueify_style__ = require("vueify-insert-css").insert('
                output += style.data + ')\n'

            if script is not None:
                # TODO build style, trigger new task ?
                output += script.data + '\n'
                          # babel 6 compat
                output += 'if (module.exports.__esModule) module.exports = module.exports.default\n'

            if template is not None:
                output += ';(typeof module.exports === "function"'
                output += '? module.exports.options'
                output += ': module.exports).template = '
                output += json.dumps(template.html_content) + '\n'

            output += '}());'
            dst.write(output)

@Configure.conf
def vue(self, *args, **kwargs):
    kwargs['features'] = ['vue']
    return self(*args, **kwargs)
