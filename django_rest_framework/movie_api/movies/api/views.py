from django.http import HttpResponse, JsonResponse
from django.views.decorators.csrf import csrf_exempt
from rest_framework.renderers import JSONRenderer
from rest_framework.parsers import JSONParser
from rest_framework import viewsets
from rest_framework.generics import (
    ListAPIView,
    RetrieveAPIView,
    UpdateAPIView,
    DestroyAPIView,
    CreateAPIView,
)

from movies.models import Movie
from .serializers import *

"""
클래스형 뷰 (CBV, Class Based View)

class MovieListAPIView(ListAPIView):
    queryset = Movie.objects.all()
    serializer_class = MovieSerializer


class MovieDetailAPIView(RetrieveAPIView):
    lookup_field = "id"
    queryset = Movie.objects.all()
    serializer_class = MovieSerializer


class MovieUpdateAPIView(UpdateAPIView):
    lookup_field = "id"
    queryset = Movie.objects.all()
    serializer_class = MovieSerializer


class MovieDestroyAPIView(DestroyAPIView):
    lookup_field = "id"
    queryset = Movie.objects.all()
    serializer_class = MovieSerializer


class MovieCreateAPIView(CreateAPIView):
    queryset = Movie.objects.all()
    serializer_class = MovieSerializer
"""

# 함수형 뷰 (FBV, Function Based View)
def index(request):
    """ List all movies or create a new movie """

    if request.method == "GET":
        queryset = Movie.objects.all()
        serializer = MovieSerializer(queryset, many=True)

        return JsonResponse(
            serializer.data, safe=False, json_dumps_params={"ensure_ascii": False}
        )

    elif request.method == "POST":
        data = JSONParser().parse(request)
        serializer = MovieSerializer(data=data)

        if serializer.is_valid():
            serializer.save()
            return JsonResponse(
                serializer.data, status=201, json_dumps_params={"ensure_ascii": False}
            )

        return JsonResponse(
            serializer.errors, status=400, json_dumps_params={"ensure_ascii": False}
        )
