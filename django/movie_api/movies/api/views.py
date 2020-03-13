import random

from django.http import HttpResponse, JsonResponse
from django.shortcuts import get_object_or_404
from rest_framework.renderers import JSONRenderer
from rest_framework.parsers import JSONParser
from rest_framework import viewsets
from rest_framework.decorators import action
from rest_framework.response import Response
from rest_framework.generics import (
    ListAPIView,
    RetrieveAPIView,
    UpdateAPIView,
    DestroyAPIView,
    CreateAPIView,
)

from movies.models import Movie
from .serializers import MovieSerializer

"""
클래스형 뷰 (CBV, Class Based View)
"""


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
클래스형 ViewSet

기존에 클래스 View로 작성한 코드를 리팩토링하여 하나로 모아줌
"""


class MovieViewSet(viewsets.ViewSet):
    """
     A simple ViewSet for listing or retrieving users.
     """

    queryset = Movie.objects.all()

    def list(self, request):

        serializers = MovieSerializer(self.queryset, many=True)
        return Response(serializers.data)

    def retrieve(self, request, pk=None):

        movie = get_object_or_404(self.queryset, pk=pk)
        serializers = MovieSerializer(movie)
        return Response(serializers.data)

    @action(detail=False)
    def say_hi(self, request):

        data = "Hello Django rest Framework"
        return Response(data)

    @action(detail=False)
    def get_random(self, request):

        serializer = MovieSerializer(self.queryset, many=True)
        data = serializer.data

        return Response(random.sample(data, 1))


"""
함수형 뷰 (FBV, Function Based View)
"""


def index(request):
    """ List all movies or create a new movie """
    params = {"ensure_ascii": False}

    if request.method == "GET":
        queryset = Movie.objects.all()
        serializer = MovieSerializer(queryset, many=True)

        return JsonResponse(serializer.data, safe=False, json_dumps_params=params)

    elif request.method == "POST":
        data = JSONParser().parse(request)
        serializer = MovieSerializer(data=data)

        if serializer.is_valid():
            serializer.save()
            return JsonResponse(serializer.data, status=201, json_dumps_params=params)

        return JsonResponse(serializer.errors, status=400, json_dumps_params=params)
