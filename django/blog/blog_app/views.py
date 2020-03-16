from django.http import HttpResponse, JsonResponse
from django.shortcuts import render
from .models import Post


def index(request):
    return render(request, "blog_app/index.html")


def post_list(request):
    posts = Post.objects.all()
    context = {"posts": posts}
    return render(request, "blog_app/post_list.html", context)


def post_detail(request, pk):
    post = Post.objects.get(pk=pk)
    context = {"post": post}
    return render(request, "blog_app/post_detail.html", context)


def post_create(request):
    return render(request, "blog_app/post_create.html")

