import json
import django.http
from django.views.generic import FormView, TemplateView
from django.views.decorators.csrf import csrf_exempt

# Copy of initialized todo list (Testing)
data = [
    {
        "task_name": "Task Name",
        "status": "Status",
        "tid": 1
    },
    {
        "task_name": "ftd application processor for toc data",
        "status": "Done",
        "tid": 2
    },
    {
        "task_name": "call an api to update todo list",
        "status": "In Progress",
        "tid": 3
    },
    {
        "task_name": "documentation for all",
        "status": "Pending",
        "tid": 4
    }
]

def update_todo_item(target_tid, status):
    for task in data:
        curr_id = task['tid']
        if curr_id == target_tid:
            task['status'] = status
            return True

    return False

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


@csrf_exempt
def update_todo(req: django.http.HttpRequest):
    from django.http import JsonResponse

    body = json.loads(req.body)
    target_tid = body['tid']
    status = body['status']
    print(f"body received = {body}")
    print(f"tid received = {target_tid}")
    print(f"status received = {status}")

    update_status = update_todo_item(target_tid,status)
    s = "success" if update_status else "failure"
    print(f"update status = {s}")
    print(f"updated task list = {data}")

    json_data = json.dumps(data)
    print(json_data)

    return JsonResponse(
        {
            "data": {
                "foo#items": json_data,
            }
        },
        status=200,
    )

    # print(json.loads(req.body))
