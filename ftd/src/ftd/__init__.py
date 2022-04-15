import json
import os
from typing import Optional

import ftd_sys
import asyncio


class Document:
    # noinspection PyShadowingBuiltins
    def __init__(self, id: str, root: Optional[str] = None, **data):
        self.id = id
        if not root:
            try:
                root = os.environ["FPM_PACKAGE_ROOT"]
            except KeyError:
                pass
        self.root = root
        self.data = data

    async def render(self, **data) -> str:
        all_data = self.data
        all_data.update(data)
        all_data = json.dumps(all_data)
        # noinspection PyUnresolvedReferences
        return await ftd_sys.render(self.id, self.root, all_data)


# noinspection PyShadowingBuiltins
def parse(id: str, root: Optional[str] = None, **data) -> Document:
    return Document(id, root, **data)


# noinspection PyShadowingBuiltins
async def render(id: str, root: Optional[str] = None, **data) -> str:
    d = parse(id, root, **data)
    return await d.render()


# noinspection PyShadowingBuiltins
def render_sync(id: str, root: Optional[str] = None, **data) -> str:
    loop = asyncio.get_event_loop()
    res = loop.run_until_complete(render(id, root, **data))
    return res
