import ftd
import ftd_django


@ftd_django.processor("hello_world")
def hello_world(doc_id, section, interpreter):
    return ftd.string_to_value("This is coming from processor")


@ftd_django.processor
def todo_data(doc_id, section, interpreter):
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
    return ftd.object_to_value(data, section, interpreter)
