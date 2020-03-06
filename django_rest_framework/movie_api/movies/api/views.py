from rest_framework import viewsets
from rest_framework.generics import (
    ListAPIView, RetrieveAPIView, UpdateAPIView, DestroyAPIView
    )

from movies.models import Movie
from .serializers import *


class MovieListAPIView(ListAPIView):
    queryset = Movie.objects.all()
    serializer_class = MovieListSerializer


class MovieDetailAPIView(RetrieveAPIView):
    lookup_field = "id"
    queryset = Movie.objects.all()
    serializer_class = MovieDetailSerializer
    
class MovieUpdateAPIView(UpdateAPIView):
    lookup_field = "id"
    queryset = Movie.objects.all()
    serializer_class = MovieListSerializer

class MovieDestroyAPIView(DestroyAPIView):
    lookup_field = "id"
    queryset = Movie.objects.all()
    serializer_class = MovieListSerializer

