from django.template.backends.base import BaseEngine
from django.template.backends.utils import csrf_token_lazy
from django.template import TemplateDoesNotExist
from django.urls import re_path
from django.http import FileResponse
from django.utils.http import http_date
import re
from pathlib import Path

import mimetypes

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

        return ftd.render(
            self.template,
            root=FPM_FOLDER,
            base_url=BASE,
            handle_processor=helpers.processor(request),
            **context,
        )


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
        from django.http import HttpResponse
        import traceback

        try:
            (content, content_type) = ftd.file_content(FPM_FOLDER, path)
            if content_type == "ftd":
                context = {}
                return HttpResponse(
                    ftd.render(
                        path,
                        root=FPM_FOLDER,
                        base_url=BASE,
                        handle_processor=helpers.processor,
                        **context,
                    )
                )
            return HttpResponse(bytes(content), content_type=content_type)
        except Exception as e:
            traceback.print_stack()
            return HttpResponse(e, status=500)

    val = [re_path(r"^%s(?P<path>.*)$" % re.escape(BASE.lstrip("/")), view)]
    return val


def processor(fn_or_str):
    if isinstance(fn_or_str, str):

        def f(fn):
            fn.processor_name = fn_or_str
            return fn

        return f
    else:
        fn_or_str.processor_name = fn_or_str.__name__
        return fn_or_str
