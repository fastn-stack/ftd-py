from django.template.backends.base import BaseEngine
from django.template.backends.utils import csrf_input_lazy, csrf_token_lazy
from django.conf import settings
import library



class TemplateBackend(BaseEngine):

    def __init__(self, params):
        params = params.copy()
        options = params.pop("OPTIONS").copy()
        options.setdefault("autoescape", True)
        options.setdefault("debug", settings.DEBUG)
        options.setdefault("file_charset", "utf-8")
        super().__init__(params)

    def get_template(self, template_name):
        path = template_name
        # todo: Check if this is valid template
        return Template(path)


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
        dir_path = settings.FPM_PACKAGE_NAME.split('/')[-1]
        filename = self.template[1:-1]
        if self.template == "/":
            filename = "index.ftd"
        elif filename.startswith("-"):
            filename = "FPM.ftd"
        else:
            filename += ".ftd"
        library.ftd_build(dir_path, filename, "/app")
        doc_id = dir_path + "/.build" + self.template + "index.html"
        if dir_path.endswith('/'):
            doc_id = dir_path + ".build" + self.template + "index.html"
        f = open(doc_id)
        return HttpResponse(f.read(), content_type="text/html")

        # return self.template.render(context)
