from pathlib import Path
import mimetypes
import posixpath

from django.views.generic import FormView, TemplateView
from django.http import HttpResponse
from django.conf import settings
from django.http import FileResponse
from django.utils.http import http_date

# import ftd_django
import ftd

from os.path import abspath, dirname, join, normcase, sep


# Create your views here.


class IndexView(TemplateView):
    template_name = "/foo/"

    def get_context_data(self, *args, **kwargs):
        context = super(IndexView, self).get_context_data(*args, **kwargs)
        context["asd"] = "Message from context"
        return context


def render(req, path="/"):
    if not path:
        path = "/"

    print("request: ", path)
    if path.startswith("-/"):
        # content = ftd_django.static()
        return serve_static(path)

    try:
        content = ftd.render(
            path,
            root=settings.FPM_PACKAGE_ROOT,
            handle_processor=resolve_processor,
            handle_foreign_variable=resolve_foreign_variable,
            handle_import=resolve_import,
        )
        return HttpResponse(content, content_type="text/html", status=200)
    except Exception as e:
        return HttpResponse(e, content_type="text/html", status=200)


def serve_static(path):
    BASE = "/"
    FPM_FOLDER = settings.BASE_DIR
    path = path.lstrip("-/")

    def _serve(fullpath: Path):
        content_type, encoding = mimetypes.guess_type(str(fullpath))
        content_type = content_type or "application/octet-stream"
        statobj = fullpath.stat()
        response = FileResponse(fullpath.open("rb"), content_type=content_type)
        response.headers["Last-Modified"] = http_date(statobj.st_mtime)
        if encoding:
            response.headers["Content-Encoding"] = encoding
        return response

    def view(path):
        path = posixpath.normpath(path)
        fullpath = Path(join(FPM_FOLDER, path))
        if fullpath.is_dir():
            return _serve(fullpath.join("index.html"))
        return _serve(fullpath)

    # val = [re_path(r"^%s(?P<path>.*)$" % re.escape(BASE.lstrip("/")), view)]
    return view(path)
