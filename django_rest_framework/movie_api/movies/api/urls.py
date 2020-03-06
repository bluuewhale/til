from django.conf.urls import url
from django.contrib import admin

from movies.api.views import *


urlpatterns = [
    url(r"^$", MovieListAPIView.as_view(), name="movie_list"),
    url(r"^(?P<id>\d+)$", MovieDetailAPIView.as_view(), name="movie_detail"),
    url(r"^(?P<id>\d+)/edit$", MovieUpdateAPIView.as_view(), name="movie_detail"),
    url(r"^(?P<id>\d+)/delete$", MovieDestroyAPIView.as_view(), name="movie_detail"),
]

