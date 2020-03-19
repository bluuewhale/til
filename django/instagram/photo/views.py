from django.views.generic.list import ListView
from django.views.generic.edit import CreateView, UpdateView, DeleteView
from django.views.generic.detail import DetailView
from django.urls import reverse_lazy
from django.shortcuts import redirect
from .models import Photo


class PhotoList(ListView):
    model = Photo
    template_name_suffix = "_index"  # {cls.model.__name__}_index.html
    # paginated_by = 20


class PhotoCreate(CreateView):
    model = Photo
    fields = ["text", "image"]
    template_name_suffix = "_create"
    success_url = "/"

    def form_valid(self, form):
        form.instance.author_id = self.request.user.id
        if form.is_valid():
            form.instance.save()
            return redirect("/")
        return self.render_to_response({"form": form})


class PhotoUpdate(UpdateView):
    model = Photo
    fields = ["text", "image"]
    template_name_suffix = "_update"
    success_url = "/"


class PhotoDelete(DeleteView):
    model = Photo
    template_name_suffix = "_delete"
    success_url = "/"


class PhotoDetail(DetailView):
    model = Photo
    template_name_suffix = "_detail"

