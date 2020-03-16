from django.urls import path, re_path
from blog_app import views

urlpatterns = [
    re_path("^$", views.post_list),
    re_path(r"/(?P<pk>\d+)", views.post_detail),
    re_path(r"^/create", views.post_create),
]
