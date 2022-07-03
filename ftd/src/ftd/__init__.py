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
    id: str, root: Optional[str] = None, base_url: Optional[str] = None, **data
) -> Document:
    return Document(id, root, base_url, **data)


# noinspection PyShadowingBuiltins
async def render(
    id: str, root: Optional[str] = None, base_url: Optional[str] = None, **data
) -> str:
    d = parse(id, root, base_url, **data)
    return await d.render()


# noinspection PyShadowingBuiltins
def render_sync(
    id: str, root: Optional[str] = None, base_url: Optional[str] = None, **data
) -> str:
    res = asyncio.run(render(id, root, base_url, **data))
    return res


def interpret(name: str, source: str):
    try:
        interpreted_object = ftd_sys.interpret(name, source)
        # name = interpreted_object.hello("Amitu")
        while True:
            state = interpreted_object.get_state()
            if state == "done":
                print("state is done")
                break

            if state == "stuck_on_import":
                print("stuck on import")
                module = interpreted_object.get_module_to_import()
                source = resolve_import(module)
                print("module source", source)
                interpreted_object.continue_after_import(module, source)

            if state == "stuck_on_foreign_variable":
                pass

            if state == "stuck_on_processor":
                pass

    except Exception as e:
        print("Exception in interpreter: ", e)


def resolve_import(path) -> str:
    print("getting module: ", path + ".ftd")
    with open(path + ".ftd", "r") as f:
        return f.read()


interpret("hello.ftd", "-- import: foo\n -- ftd.text: Hello World")
