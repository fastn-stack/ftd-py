from django.views.generic import FormView, TemplateView
from django.http import HttpResponse
import ftd

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

    content = ftd.render(
        doc_path,
        resolve_processor,
        resolve_foreign_variable,
        resolve_import,
        root="/Users/wilderbit/github/ftd-py/django_example/ui"
    )

    return HttpResponse(content, content_type="text/html")
