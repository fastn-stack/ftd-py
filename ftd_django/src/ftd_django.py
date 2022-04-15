from django.template.backends.base import BaseEngine
from django.template.backends.utils import csrf_input_lazy, csrf_token_lazy
from django.conf import settings
from typing import Optional

import ftd_sys
import asyncio
import os
import json


def ftd_render(path: str, file: Optional[str] = None, base_url='/', **data) -> str:
    async def render(data):
        data = json.dumps(data)
        await ftd_sys.render(file, base_url)

    current_dir = os.getcwd()
    os.chdir(path)
    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    html = loop.run_until_complete(render(data))
    loop.close()
    os.chdir(current_dir)
    return html


class Template:
    def __init__(self, template):
        self.template = template

    def render(self, context=None, request=None):
        from django.http import HttpResponse

        if context is None:
            context = {}
        if request is not None:
            context["request"] = request
            context["csrf_input"] = csrf_input_lazy(request)
            context["csrf_token"] = csrf_token_lazy(request)

        ftd_render(dir_path, self.template, "/app")
        doc_id = dir_path + "/.build" + self.template + "index.html"
        if dir_path.endswith('/'):
            doc_id = dir_path + ".build" + self.template + "index.html"
        f = open(doc_id)
        return HttpResponse(f.read(), content_type="text/html")

        # return self.template.render(context)


class TemplateBackend(BaseEngine):
    # noinspection PyMethodMayBeStatic
    def get_template(self, template_name) -> Template:
        return Template(template_name)


