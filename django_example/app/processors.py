import ftd
import ftd_django

# Used by to-do table as list of tasks => [name, status] tuples
data = [
    {
        "task_name": "Task Name",
        "status": "Status",
        "id": "1"
    },
    {
        "task_name": "ftd application processor for toc data",
        "status": "Done",
        "id": "2"
    },
    {
        "task_name": "call an api to update todo list",
        "status": "In Progress",
        "id": "3"
    },
    {
        "task_name": "documentation for all",
        "status": "Pending",
        "id": "4"
    }
]

@ftd_django.processor("hello_world")
def hello_world(doc_id, section, interpreter):
    return ftd.string_to_value("This is coming from processor")


@ftd_django.processor
def todo_data(doc_id, section, interpreter):
    return ftd.object_to_value(data, section, interpreter)
