from django.template.backends.base import BaseEngine
from django.template.backends.utils import csrf_input_lazy, csrf_token_lazy
from django.template import TemplateDoesNotExist
from django.core.exceptions import ImproperlyConfigured

import ftd
from ftd_django import helpers


(BASE, FPM_FOLDER) = helpers.validate_settings()


class Template:
    def __init__(self, template):
        self.template = template

    def render(self, context=None, request=None):
        if context is None:
            context = {}
        if request is not None:
            context["request"] = request
            context["csrf_token"] = csrf_token_lazy(request)
        # noinspection PyUnresolvedReferences
        return ftd.render_sync(BASE, FPM_FOLDER, self.template, context)


class TemplateBackend(BaseEngine):
    # noinspection PyMethodMayBeStatic
    def get_template(self, template_name: str) -> Template:
        if not (template_name.startswith("/") and template_name.endswith("/")):
            raise TemplateDoesNotExist()
        return Template(template_name)



