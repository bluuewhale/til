from django.contrib import messages
from django.http.response import HttpResponseRedirect
from django.urls import reverse_lazy
from django.shortcuts import redirect
from django.views.generic.list import ListView
from django.views.generic.edit import CreateView, UpdateView, DeleteView
from django.views.generic.detail import DetailView
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
    # success_url = "/"

    def dispatch(self, request, *args, **kwargs):
        object = self.get_object()
        if object.author != request.user:
            messages.warning(request, "수정할 권한이 없습니다.")
            return HttpResponseRedirect("/")
        else:
            return super(PhotoUpdate, self).dispatch(request, *args, **kwargs)


class PhotoDelete(DeleteView):
    model = Photo
    template_name_suffix = "_delete"
    success_url = "/"

    def dispatch(self, request, *args, **kwargs):
        """ request를 검사하고 HTTP 매서드를 찾아서 중계하는 역할 """

        object = self.get_object()
        if object.author != request.user:
            messages.warning(request, "수정할 권한이 없습니다.")
            return HttpResponseRedirect("/")
        else:
            return super(PhotoDelete, self).dispatch(request, *args, **kwargs)


class PhotoDetail(DetailView):
    model = Photo
    template_name_suffix = "_detail"

