from django.views.generic import FormView, TemplateView
from django.http import HttpResponse
import ftd
# import ftd_django
from django.conf import settings

# Create your views here.


class IndexView(TemplateView):
    template_name = "/foo/"

    def get_context_data(self, *args, **kwargs):
        context = super(IndexView, self).get_context_data(*args, **kwargs)
        context["asd"] = "Message from context"
        return context


def resolve_processor(section):
    pass


def resolve_foreign_variable(section):
    pass


def resolve_import(path) -> str:
    pass


def render(req, doc_path="/"):
    if not doc_path:
        doc_path = "/"

    print("request: ", doc_path)
    # content = ftd_django.static()
    try:
        content = ftd.render(
            doc_path,
            resolve_processor,
            resolve_foreign_variable,
            resolve_import,
            root=settings.FPM_PACKAGE_ROOT
        )
        return HttpResponse(content, content_type="text/html", status=200)
    except Exception as e:
        return HttpResponse(e, content_type="text/html", status=200)
