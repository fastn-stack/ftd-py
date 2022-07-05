import json
import os
from typing import Optional, Callable

import ftd_sys


class Document:
    # noinspection PyShadowingBuiltins
    def __init__(
        self,
        id: str,
        handle_processor: Callable,
        handle_foreign_variable: Callable,
        handle_import: Callable,
        root: Optional[str] = None,
        base_url: Optional[str] = None,
        **data
    ):
        self.id = id
        if not root:
            try:
                root = os.environ["FPM_PACKAGE_ROOT"]
            except KeyError:
                pass
        self.root = root
        self.data = data
        self.base_url = base_url if base_url else "/"
        self.handle_processor = handle_processor
        self.handle_foreign_variable = handle_foreign_variable
        self.handle_import = handle_import

    def render(self, **data) -> str:
        all_data = self.data
        all_data.update(data)
        all_data = json.dumps(all_data)
        # noinspection PyUnresolvedReferences
        return interpret(
            self.id,
            self.handle_processor,
            self.handle_foreign_variable,
            self.handle_import,
            self.root,
            self.base_url,
            all_data,
        )


# noinspection PyShadowingBuiltins
def parse(
    id: str,
    handle_processor: Callable,
    handle_foreign_variable: Callable,
    handle_import: Callable,
    root: Optional[str] = None,
    base_url: Optional[str] = None,
    **data
) -> Document:
    return Document(
        id,
        handle_processor,
        handle_foreign_variable,
        handle_import,
        root,
        base_url,
        **data,
    )


# noinspection PyShadowingBuiltins
def render(
    id: str,
    handle_processor: Callable,
    handle_foreign_variable: Callable,
    handle_import: Callable,
    root: Optional[str] = None,
    base_url: Optional[str] = None,
    **data
) -> str:
    d = parse(
        id,
        handle_processor,
        handle_foreign_variable,
        handle_import,
        root,
        base_url,
        **data,
    )
    return d.render()


# noinspection PyShadowingBuiltins
def render_sync(
    id: str,
    handle_processor: Callable,
    handle_foreign_variable: Callable,
    handle_import: Callable,
    root: Optional[str] = None,
    base_url: Optional[str] = None,
    **data
) -> str:
    res = render(
        id,
        handle_processor,
        handle_foreign_variable,
        handle_import,
        root,
        base_url,
        **data,
    )
    return res


# rename it to parse
def interpret(
    id: str,
    handle_processor: Callable,
    handle_foreign_variable: Callable,
    handle_import: Callable,
    root: Optional[str] = None,
    base_url: Optional[str] = None,
    data: Optional[str] = None,
):
    try:
        interpreter = ftd_sys.interpret(id, root, base_url, data)
        while True:
            state = interpreter.state_name()
            if state == "done":
                return interpreter.render()

            if state == "stuck_on_import":
                print("stuck on import")
                module = interpreter.get_module_to_import()
                # It will call Rust resolve import
                # ftd_sys.resolve_import
                source = resolve_import(module)
                if not source:
                    source = handle_import(module)
                interpreter.continue_after_import(module, source)
                print("stuck on import done")

            if state == "stuck_on_foreign_variable":
                pass

            if state == "stuck_on_processor":
                print("stuck_on_processor")
                section = interpreter.get_processor_section()
                processor_value = interpreter.resolve_processor(section)
                """
                If value returned from fpm processors is None, we will call to
                python application processor 
                """
                if not processor_value:
                    processor_value = handle_processor(section)
                interpreter.continue_after_processor(processor_value)
                print("stuck_on_processor done")

    except Exception as e:
        print("Exception in interpreter: ", e)


def resolve_processor(section):
    pass


def resolve_foreign_variable(section):
    pass


def resolve_import(path) -> str:
    print("getting module: ", path + ".ftd")
    with open(path + ".ftd", "r") as f:
        content = f.read()
        print("module content", path, content)
        return content


doc = """
-- import: foo

-- ftd.text: Hello World


-- ftd.toc-item list toc:
$processor$: toc

- index.html
  fpm: FTD Package Manager
- about/
  About `fpm`
  - why-not/
    Why Not `fpm`?
    
"""

print(render_sync("foo/", resolve_processor, resolve_foreign_variable, resolve_import))
