import json
import django.http
from django.views.generic import FormView, TemplateView
from django.views.decorators.csrf import csrf_exempt
import ftd

# Create your views here.


def handle_processor(doc_id, section, interpreter=None):
    name = section.header_string(doc_id, "$processor$")
    if name == "hello_world":
        return ftd.string_to_value("This is coming from processor")

    if name == "todo_data":
        data = [
            {
                "task_name": "Task Name",
                "status": "Status"
            },
            {
                "task_name": "ftd application processor for toc data",
                "status": "Done"
            },
            {
                "task_name": "call an api to update todo list",
                "status": "In Progress"
            },
            {
                "task_name": "documentation for all",
                "status": "Pending"
            }
        ]
        return ftd.object_to_value(json.dumps(data), section, interpreter)

    return None


class IndexView(TemplateView):
    template_name = "/foo/"

    def get_context_data(self, *args, **kwargs):
        context = super(IndexView, self).get_context_data(*args, **kwargs)
        context["asd"] = "Message from context"
        return context

    def get(self, request, *args, **kwargs):
        from django.template import loader
        from django.http import HttpResponse

        template = loader.get_template(self.template_name)
        return HttpResponse(template.render(
            self.get_context_data(),
            request,
            handle_processor=handle_processor
        ))


def get_data(_):
    from django.http import JsonResponse

    response_data = {
        "data": {"apis#name": "Abrar Khan", "apis#age": 28},
        "success": True,
    }
    return JsonResponse(response_data, status=200)


@csrf_exempt
def post_data(req: django.http.HttpRequest):
    from django.http import JsonResponse

    body = json.loads(req.body)
    print(body)
    return JsonResponse(
        {
            "data": {
                "apis#post-done": True,
            }
        },
        status=200,
    )
