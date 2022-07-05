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


def render(req, doc_path="/"):
    if not len(doc_path):
        print("Hello")
    print(doc_path)
    return HttpResponse(doc_path, content_type="text/html")
