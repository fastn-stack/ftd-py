import ftd_sys
import json


class Document:
    def __init__(self, f, **data):
        self.f = f
        self.data = data

    async def render(self, **data):
        all_data = self.data
        all_data.update(data)
        all_data = json.dumps(all_data)
        await ftd_sys.render(self.f, all_data)


def parse(f, **data):
    return Document(f, **data)
