import json
import os
from typing import Optional

import ftd_sys
import asyncio


class Document:
    # noinspection PyShadowingBuiltins
    def __init__(
            self,
            id: str,
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

    async def render(self, **data) -> str:
        all_data = self.data
        all_data.update(data)
        all_data = json.dumps(all_data)
        # noinspection PyUnresolvedReferences
        return await ftd_sys.render(self.id, self.root, self.base_url, all_data)


# noinspection PyShadowingBuiltins
def parse(
        id: str, root: Optional[str] = None, base_url: Optional[str] = None,
        **data
) -> Document:
    return Document(id, root, base_url, **data)


# noinspection PyShadowingBuiltins
async def render(
        id: str, root: Optional[str] = None, base_url: Optional[str] = None,
        **data
) -> str:
    d = parse(id, root, base_url, **data)
    return await d.render()


# noinspection PyShadowingBuiltins
def render_sync(
        id: str, root: Optional[str] = None, base_url: Optional[str] = None,
        **data
) -> str:
    res = asyncio.run(render(id, root, base_url, **data))
    return res


def interpret(name, source, handle_processor, handle_foreign_variable):
    try:
        interpreter = ftd_sys.interpret(name, source)
        while True:
            state = interpreter.state_name()
            if state == "done":
                 return interpreter.render()

            if state == "stuck_on_import":
                print("stuck on import")
                module = interpreter.get_module_to_import()
                source = resolve_import(module)
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


def handle_processor(section):
    pass


def handle_foreign_variable(section):
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


\-- ftd.toc-item list toc:
$processor$: toc

- index.html
  fpm: FTD Package Manager
- about/
  About `fpm`
  - why-not/
    Why Not `fpm`?
    
"""

print(interpret("hello.ftd", doc, handle_processor, handle_foreign_variable))
