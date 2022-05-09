from django.template.backends.base import BaseEngine
from django.template.backends.utils import csrf_token_lazy
from django.template import TemplateDoesNotExist
from django.urls import re_path
from django.http import FileResponse
from django.utils.http import http_date
import re
from pathlib import Path

import mimetypes
import posixpath

import ftd
from ftd_django import helpers


class Template:
    def __init__(self, template):
        self.template = template

    def render(self, context=None, request=None):
        if context is None:
            context = {}
        if request is not None:
            # context["request"] = request
            context["csrf_token"] = str(csrf_token_lazy(request))
        try:
            del context["view"]
        except KeyError:
            pass
        # noinspection PyUnresolvedReferences
        (BASE, FPM_FOLDER) = helpers.validate_settings()
        return ftd.render_sync(FPM_FOLDER, self.template, BASE, **context)


class TemplateBackend(BaseEngine):
    def __init__(self, params):
        params = params.copy()
        params.pop("OPTIONS")
        super().__init__(params)

    # noinspection PyMethodMayBeStatic
    def get_template(self, template_name: str) -> Template:
        if not (template_name.startswith("/") and template_name.endswith("/")):
            raise TemplateDoesNotExist(f"Unable to find template for: {template_name}")
        return Template(template_name)


def _serve(fullpath: Path):
    content_type, encoding = mimetypes.guess_type(str(fullpath))
    content_type = content_type or "application/octet-stream"
    statobj = fullpath.stat()
    response = FileResponse(fullpath.open("rb"), content_type=content_type)
    response.headers["Last-Modified"] = http_date(statobj.st_mtime)
    if encoding:
        response.headers["Content-Encoding"] = encoding
    return response


def static():
    # noinspection PyUnresolvedReferences
    (BASE, FPM_FOLDER) = helpers.validate_settings()

    def view(_, path):
        path = posixpath.normpath(path)
        fullpath = Path(helpers.safe_join(FPM_FOLDER, ".build", path))
        if fullpath.is_dir():
            return _serve(fullpath.join("index.html"))
        return _serve(fullpath)

    val = [re_path(r"^%s(?P<path>.*)$" % re.escape(BASE.lstrip("/")), view)]
    return val
