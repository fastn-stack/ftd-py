from django.views.generic import FormView, TemplateView


# Create your views here.


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
        return HttpResponse(template.render(self.get_context_data(), request))
