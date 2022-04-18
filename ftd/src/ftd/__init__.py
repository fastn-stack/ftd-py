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
