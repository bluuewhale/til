from urllib.parse import urlparse

from django.contrib import messages
from django.http.response import (
    HttpResponseRedirect,
    HttpResponseForbidden,
    Http404,
    HttpResponseBadRequest,
)
from django.urls import reverse_lazy, reverse
from django.shortcuts import redirect, get_object_or_404
from django.views.generic import View
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
        return super(PhotoDelete, self).dispatch(request, *args, **kwargs)


class PhotoDetail(DetailView):
    model = Photo
    template_name_suffix = "_detail"


class PhotoLike(View):
    def get(self, request, *args, **kwargs):
        photo = get_object_or_404(Photo, pk=kwargs.get("pk"))

        user = request.user
        like_users = photo.like.all()
        if user in like_users:  # 이미 좋아요를 누른 상태면
            photo.like.remove(user)  # 좋아요 취소
        else:
            photo.like.add(user)  # 좋아요 등록
        return HttpResponseRedirect(photo.get_absolute_url())

    def dispatch(self, request, *args, **kwargs):
        if not request.user.is_authenticated:  # 로그인 확인
            return HttpResponseRedirect(reverse("accounts:login"))
        return super(PhotoLike, self).dispatch(request, *args, **kwargs)

    def post(self, request, *args, **kwargs):
        return HttpResponseBadRequest()


class PhotoFavorite(View):
    def get(self, request, *args, **kwargs):
        photo = get_object_or_404(Photo, pk=kwargs.get("pk"))
        user = request.user

        if user in photo.favorite.all():  # 이미 fav를 누른 상태면
            photo.favorite.remove(user)  # 취소
        else:
            photo.favorite.add(user)  # fav등록
        return HttpResponseRedirect(photo.get_absolute_url())

    def dispatch(self, request, *args, **kwargs):
        if not request.user.is_authenticated:  # 로그인 확인
            return HttpResponseRedirect(reverse("accounts:login"))
        return super(PhotoFavorite, self).dispatch(request, *args, **kwargs)


class PhotoLikeList(ListView):
    model = Photo
    template_name_suffix = "_index"

    def dispatch(self, request, *args, **kwargs):
        if not request.user.is_authenticated:
            messages.warning(request, "로그인이 필요합니다")
            return HttpResponseRedirect("/")

    def get_queryset(self):
        return self.request.user.like.post.all()
