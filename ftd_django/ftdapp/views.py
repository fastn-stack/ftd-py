from django.http import HttpResponse
import ftd

# Create your views here.


def render(req, doc_path="/"):
    if not len(doc_path):
        print("Hello")
    print(doc_path)
    return HttpResponse(doc_path, content_type="text/html")
